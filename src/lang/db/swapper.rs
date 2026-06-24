use std::collections::HashSet;
use std::fmt::Display;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use crossbeam::channel::{Receiver, RecvTimeoutError, Sender, TrySendError};
use lsp_types::Url;
use tracing::{error, info, trace, warn};

use crate::env_config;
use crate::ide::analysis_progress::AnalysisEvent;
use crate::lang::db::{AnalysisDatabase, migrate_to_fresh_database};
use crate::server::schedule::thread::{self, JoinHandle, ThreadPriority};

#[derive(Debug, Clone, Copy)]
pub enum SwapReason {
    Mutations(u64),
}

impl Display for SwapReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
/// The mutation threshold can be configured with an environment variable.
/// Consult [`env_config::db_replace_mutations`] for more information.
///
/// The new database has a clean state.
/// It is expected that diagnostics will be refreshed on it as quickly as possible, otherwise
/// the entire workspace would be recompiled at an undetermined time leading to bad UX delays.
pub struct AnalysisDatabaseSwapper {
    mutations_since_last_replace: u64,
    db_replace_min_mutations: u64,
    analysis_event_sender: Sender<AnalysisEvent>,
}

impl AnalysisDatabaseSwapper {
    pub fn new(analysis_event_sender: Sender<AnalysisEvent>) -> Self {
        Self {
            mutations_since_last_replace: 0,
            db_replace_min_mutations: env_config::db_replace_mutations(),
            analysis_event_sender,
        }
    }

    pub fn register_mutation(&mut self) {
        self.mutations_since_last_replace += 1;
    }

    /// Swaps the database unconditionally, triggered by the inactivity monitor.
    pub fn swap_on_inactivity(&mut self, db: &mut AnalysisDatabase, open_files: &HashSet<Url>) {
        if let Err(err) = self.analysis_event_sender.send(AnalysisEvent::DatabaseSwap) {
            error!("Could not send swap status: {err:?}");
        }

        self.swap(db, open_files);
        self.mutations_since_last_replace = 0;

        trace!("Database swapped due to inactivity");
    }

    /// Checks for the mutation-based swap criterion and swaps the database if it has been met.
    pub fn maybe_swap(
        &mut self,
        db: &mut AnalysisDatabase,
        open_files: &HashSet<Url>,
    ) -> Option<SwapReason> {
        let reason = self.check_for_swap()?;

        if let Err(err) = self.analysis_event_sender.send(AnalysisEvent::DatabaseSwap) {
            error!("Could not send swap status: {err:?}");
        }

        self.swap(db, open_files);
        self.mutations_since_last_replace = 0;

        trace!("Database swapped - {reason}");

        Some(reason)
    }

    fn check_for_swap(&self) -> Option<SwapReason> {
        let mutations = self.mutations_since_last_replace;
        (mutations >= self.db_replace_min_mutations).then_some(SwapReason::Mutations(mutations))
    }

    /// Swaps the database.
    #[tracing::instrument(skip_all)]
    fn swap(&self, db: &mut AnalysisDatabase, open_files: &HashSet<Url>) {
        let Ok(new_db) =
            catch_unwind(AssertUnwindSafe(|| migrate_to_fresh_database(db, open_files)))
        else {
            error!("caught panic when preparing new db for swap");
            return;
        };

        *db = new_db;
    }
}

/// Monitors wall-clock inactivity and sends swap requests to the main event loop.
///
/// This tracks real elapsed time since the last user activity (mutations or analysis starts).
/// When the server sits idle for longer than the configured threshold, it signals the main loop
/// to swap the database, freeing memory that would otherwise accumulate during long idle sessions.
pub struct InactivitySwapMonitor {
    last_activity_at: Arc<AtomicU64>,
    /// `true` while the user is considered active (i.e. no inactivity swap has fired yet).
    /// Flipped to `false` when an inactivity swap fires; restored to `true` on any real activity.
    /// Prevents repeated swaps during a single continuous idle session.
    user_active: Arc<AtomicBool>,
    swap_request_receiver: Receiver<()>,
    /// Dropped to signal the monitor thread to stop.
    _shutdown_sender: Sender<()>,
    _thread: JoinHandle<()>,
}

impl InactivitySwapMonitor {
    /// How often the monitor thread wakes up to check inactivity.
    const POLL_INTERVAL: Duration = Duration::from_secs(60);

    pub fn new() -> Self {
        let inactivity_threshold = env_config::db_replace_inactive_interval();
        let last_activity_at = Arc::new(AtomicU64::new(Self::unix_secs_now()));
        let user_active = Arc::new(AtomicBool::new(true));
        let (swap_request_sender, swap_request_receiver) = crossbeam::channel::bounded(1);
        let (shutdown_sender, shutdown_receiver) = crossbeam::channel::bounded::<()>(0);

        let thread = thread::Builder::new(ThreadPriority::Worker)
            .name("cairo-ls:inactivity-swap-monitor".into())
            .spawn({
                let last_activity_at = last_activity_at.clone();
                let user_active = user_active.clone();
                move || {
                    Self::monitor_loop(
                        last_activity_at,
                        user_active,
                        swap_request_sender,
                        shutdown_receiver,
                        inactivity_threshold,
                    );
                }
            })
            .expect("failed to spawn inactivity swap monitor thread");

        Self {
            last_activity_at,
            user_active,
            swap_request_receiver,
            _shutdown_sender: shutdown_sender,
            _thread: thread,
        }
    }

    /// Resets the inactivity timer and marks the user as active.
    ///
    /// Call this on any user-visible activity so a fresh idle period is acquired before the next
    /// inactivity swap.
    pub fn notify_activity(&self) {
        self.last_activity_at.store(Self::unix_secs_now(), Ordering::Relaxed);
        self.user_active.store(true, Ordering::Relaxed);
    }

    /// Marks that an inactivity swap has just occurred, suppressing further swaps until
    /// user activity is detected.
    pub fn notify_swap_triggered(&self) {
        info!(
            "inactivity swap triggered, suppressing further swaps until user activity is detected"
        );
        self.user_active.store(false, Ordering::Relaxed);
    }

    /// Returns a receiver that fires when the inactivity threshold has been exceeded.
    pub fn swap_request_receiver(&self) -> Receiver<()> {
        self.swap_request_receiver.clone()
    }

    fn monitor_loop(
        last_activity_at: Arc<AtomicU64>,
        user_active: Arc<AtomicBool>,
        swap_request_sender: Sender<()>,
        shutdown_receiver: Receiver<()>,
        inactivity_threshold: Duration,
    ) {
        loop {
            match shutdown_receiver.recv_timeout(Self::POLL_INTERVAL) {
                Ok(()) => unreachable!(),
                Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => {}
            }

            if !user_active.load(Ordering::Relaxed) {
                continue;
            }

            let inactive_secs =
                Self::unix_secs_now().saturating_sub(last_activity_at.load(Ordering::Relaxed));

            if inactive_secs >= inactivity_threshold.as_secs() {
                match swap_request_sender.try_send(()) {
                    Ok(()) => trace!("inactivity swap requested after {inactive_secs}s of idle"),
                    Err(TrySendError::Full(_)) => {
                        warn!("inactivity swap request dropped: channel full")
                    }
                    Err(TrySendError::Disconnected(_)) => break,
                }
            }
        }
    }

    fn unix_secs_now() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs()
    }
}
