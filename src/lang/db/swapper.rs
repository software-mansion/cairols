use std::collections::HashSet;
use std::fmt::Display;
use std::mem;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::time::{Duration, SystemTime};

use crossbeam::channel::Sender;
use lsp_types::Url;
use serde::Serialize;
use tracing::{error, trace};

use crate::env_config;
use crate::ide::analysis_progress::AnalysisEvent;
use crate::lang::db::{AnalysisDatabase, migrate_to_fresh_database};
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::project::ProjectController;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SwapReason {
    Time(Duration),
    Mutations(u64),
}

impl Display for SwapReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SwapReason::Time(duration) => {
                write!(f, "{}s passed since the last swap", duration.as_secs())
            }
            SwapReason::Mutations(mutations) => {
                write!(f, "{mutations} mutations applied since the last swap")
            }
        }
    }
}

/// Swaps entire [`AnalysisDatabase`] with empty one periodically.
///
/// Salsa does not perform GC, which means that whatever computes in query groups stays in memory
/// forever (with little exception being the LRU mechanism).
/// The usage patterns of Salsa queries in the Cairo compiler cause Salsa to steadily allocate
/// new memory, which is a problem if the user is having a long coding session.
/// This object realises a nuclear GC strategy by wiping the entire analysis database from time to
/// time.
///
/// The swapping criteria can be configured with environment variables.
/// Consult [`env_config::db_replace_interval`] and [`env_config::db_replace_mutations`]
/// for more information.
///
/// The new database has a clean state.
/// It is expected that diagnostics will be refreshed on it as quickly as possible, otherwise
/// the entire workspace would be recompiled at an undetermined time leading to bad UX delays.
pub struct AnalysisDatabaseSwapper {
    stopwatch: Stopwatch,
    mutations_since_last_replace: u64,
    db_replace_min_interval: Duration,
    db_replace_min_mutations: u64,
    analysis_event_sender: Sender<AnalysisEvent>,
}

impl AnalysisDatabaseSwapper {
    pub fn new(analysis_event_sender: Sender<AnalysisEvent>) -> Self {
        Self {
            stopwatch: Stopwatch::default(),
            mutations_since_last_replace: 0,
            db_replace_min_interval: env_config::db_replace_interval(),
            db_replace_min_mutations: env_config::db_replace_mutations(),
            analysis_event_sender,
        }
    }

    pub fn register_mutation(&mut self) {
        self.mutations_since_last_replace += 1;
    }

    pub fn start_stopwatch(&mut self) {
        self.stopwatch.start();
        trace!("Stopwatch started!");
    }

    pub fn stop_stopwatch(&mut self) {
        self.stopwatch.stop();
        trace!(
            "Stopwatch stopped! Total elapsed time: {}s",
            self.stopwatch.total_elapsed_time.as_secs()
        );
    }

    /// Checks for the swap criteria and swaps the database if they have been met.
    pub fn maybe_swap(
        &mut self,
        db: &mut AnalysisDatabase,
        open_files: &HashSet<Url>,
        project_controller: &mut ProjectController,
        proc_macro_client_controller: &ProcMacroClientController,
    ) -> Option<SwapReason> {
        let reason = self.check_for_swap()?;

        if let Err(err) = self.analysis_event_sender.send(AnalysisEvent::DatabaseSwap) {
            error!("Could not send swap status: {err:?}");
        };

        self.swap(db, open_files, project_controller, proc_macro_client_controller);

        self.mutations_since_last_replace = 0;
        self.stopwatch.reset();

        trace!("Database swapped - {reason}");

        Some(reason)
    }

    /// Checks whether any swap condition has been met. Returns the reason if swap is possible, `None` otherwise.
    fn check_for_swap(&self) -> Option<SwapReason> {
        let elapsed_time = self.stopwatch.total_elapsed_time;
        let mutations = self.mutations_since_last_replace;

        if mutations >= self.db_replace_min_mutations {
            Some(SwapReason::Mutations(mutations))
        } else if elapsed_time >= self.db_replace_min_interval {
            Some(SwapReason::Time(elapsed_time))
        } else {
            None
        }
    }

    /// Swaps the database.
    #[tracing::instrument(skip_all)]
    fn swap(
        &self,
        db: &mut AnalysisDatabase,
        open_files: &HashSet<Url>,
        project_controller: &mut ProjectController,
        proc_macro_client_controller: &ProcMacroClientController,
    ) {
        let Ok(new_db) = catch_unwind(AssertUnwindSafe(|| {
            migrate_to_fresh_database(
                db,
                open_files,
                project_controller,
                proc_macro_client_controller,
            )
        })) else {
            error!("caught panic when preparing new db for swap");
            return;
        };

        *db = new_db;
    }
}

#[derive(Default)]
struct Stopwatch {
    start_time: Option<SystemTime>,
    total_elapsed_time: Duration,
}

impl Stopwatch {
    fn start(&mut self) {
        self.start_time = Some(SystemTime::now());
    }

    fn stop(&mut self) {
        let Some(start_time) = mem::take(&mut self.start_time) else {
            error!("Tried to start a stopwatch which has not started");
            return;
        };

        let Ok(elapsed_time) = start_time.elapsed() else {
            // Unprobable case, would happen if system time somehow went backwards.
            error!("Failed to read the elapsed time of the stopwatch");
            return;
        };

        self.total_elapsed_time += elapsed_time;
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}
