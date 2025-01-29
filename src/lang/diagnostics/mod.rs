use std::collections::HashSet;
use std::iter;
use std::iter::zip;
use std::num::NonZero;
use std::panic::{AssertUnwindSafe, catch_unwind};

use cairo_lang_filesystem::ids::FileId;
use lsp_types::Url;
use tracing::{error, trace};

use self::project_diagnostics::ProjectDiagnostics;
use self::refresh::{clear_old_diagnostics, refresh_diagnostics};
use crate::ide::analysis_progress::AnalysisProgressController;
use crate::lang::diagnostics::file_batches::{batches, find_primary_files, find_secondary_files};
use crate::lang::lsp::LsProtoGroup;
use crate::server::client::Notifier;
use crate::server::panic::cancelled_anyhow;
use crate::server::schedule::thread::task_progress_monitor::TaskHandle;
use crate::server::schedule::thread::{self, JoinHandle, ThreadPriority};
use crate::server::trigger;
use crate::state::{State, StateSnapshot};

mod file_batches;
mod file_diagnostics;
mod lsp;
mod project_diagnostics;
mod refresh;

/// Schedules refreshing of diagnostics in a background thread.
///
/// This structure *owns* the worker thread and is responsible for its lifecycle.
/// Dropping it will ask the worker to stop and synchronously wait for it to finish.
pub struct DiagnosticsController {
    // NOTE: Member order matters here.
    //   The trigger MUST be dropped before worker's join handle.
    //   Otherwise, the controller thread will never be requested to stop, and the controller's
    //   JoinHandle will never terminate.
    trigger: trigger::Sender<StateSnapshots>,
    _thread: JoinHandle,
    state_snapshots_props: StateSnapshotsProps,
}

impl DiagnosticsController {
    /// Creates a new diagnostics controller.
    pub fn new(notifier: Notifier, analysis_progress_tracker: AnalysisProgressController) -> Self {
        let (trigger, receiver) = trigger::trigger();
        let (thread, parallelism) =
            DiagnosticsControllerThread::spawn(receiver, notifier, analysis_progress_tracker);
        Self {
            trigger,
            _thread: thread,
            state_snapshots_props: StateSnapshotsProps { parallelism },
        }
    }

    /// Schedules diagnostics refreshing on snapshot(s) of the current state.
    pub fn refresh(&self, state: &State) {
        self.trigger.activate(StateSnapshots::new(state, &self.state_snapshots_props));
    }
}

/// Stores entire state of diagnostics controller's worker thread.
struct DiagnosticsControllerThread {
    receiver: trigger::Receiver<StateSnapshots>,
    notifier: Notifier,
    pool: thread::Pool,
    project_diagnostics: ProjectDiagnostics,
    analysis_progress_controller: AnalysisProgressController,
    worker_handles: Vec<TaskHandle>,
}

impl DiagnosticsControllerThread {
    /// Spawns a new diagnostics controller worker thread
    /// and returns a handle to it and the amount of parallelism it provides.
    fn spawn(
        receiver: trigger::Receiver<StateSnapshots>,
        notifier: Notifier,
        analysis_progress_controller: AnalysisProgressController,
    ) -> (JoinHandle, NonZero<usize>) {
        let mut this = Self {
            receiver,
            notifier,
            analysis_progress_controller,
            pool: thread::Pool::new(),
            project_diagnostics: ProjectDiagnostics::new(),
            worker_handles: Vec::new(),
        };

        let parallelism = this.pool.parallelism();

        let thread = thread::Builder::new(ThreadPriority::Worker)
            .name("cairo-ls:diagnostics-controller".into())
            .spawn(move || this.event_loop())
            .expect("failed to spawn diagnostics controller thread");

        (thread, parallelism)
    }

    /// Runs diagnostics controller's event loop.
    fn event_loop(&mut self) {
        while let Some(state_snapshots) = self.receiver.wait() {
            assert!(self.worker_handles.is_empty());
            self.analysis_progress_controller.try_start_analysis();

            if let Err(err) = catch_unwind(AssertUnwindSafe(|| {
                self.diagnostics_controller_tick(state_snapshots);
            })) {
                if let Ok(err) = cancelled_anyhow(err, "diagnostics refreshing has been cancelled")
                {
                    trace!("{err:?}");
                } else {
                    error!("caught panic while refreshing diagnostics");
                }
            }

            self.join_and_clear_workers();
            self.analysis_progress_controller.try_stop_analysis();
        }
    }

    /// Runs a single tick of the diagnostics controller's event loop.
    #[tracing::instrument(skip_all)]
    fn diagnostics_controller_tick(&mut self, state_snapshots: StateSnapshots) {
        let (state, primary_snapshots, secondary_snapshots) = state_snapshots.split();

        let primary_set = find_primary_files(&state.db, &state.open_files);
        let primary: Vec<_> = primary_set.iter().copied().collect();
        self.spawn_refresh_workers(&primary, primary_snapshots);

        let secondary = find_secondary_files(&state.db, &primary_set);
        self.spawn_refresh_workers(&secondary, secondary_snapshots);

        let files_to_preserve: HashSet<Url> = primary
            .into_iter()
            .chain(secondary)
            .flat_map(|file| state.db.url_for_file(file))
            .collect();

        self.spawn_worker(move |project_diagnostics, notifier| {
            clear_old_diagnostics(files_to_preserve, project_diagnostics, notifier);
        });
    }

    /// Shortcut for spawning a worker task which does the boilerplate around cloning state parts
    /// and catching panics.
    fn spawn_worker(&mut self, f: impl FnOnce(ProjectDiagnostics, Notifier) + Send + 'static) {
        let project_diagnostics = self.project_diagnostics.clone();
        let notifier = self.notifier.clone();
        let worker_fn = move || f(project_diagnostics, notifier);
        let worker_handle = self.pool.spawn_with_tracking(ThreadPriority::Worker, move || {
            if let Err(err) = catch_unwind(AssertUnwindSafe(worker_fn)) {
                if let Ok(err) = cancelled_anyhow(err, "diagnostics worker has been cancelled") {
                    trace!("{err:?}");
                } else {
                    error!("caught panic in diagnostics worker");
                }
            }
        });
        self.worker_handles.push(worker_handle);
    }

    /// Makes batches out of `files` and spawns workers to run [`refresh_diagnostics`] on them.
    fn spawn_refresh_workers(&mut self, files: &[FileId], state_snapshots: Vec<StateSnapshot>) {
        let files_batches = batches(files, self.pool.parallelism());
        assert_eq!(files_batches.len(), state_snapshots.len());
        for (batch, state) in zip(files_batches, state_snapshots) {
            self.spawn_worker(move |project_diagnostics, notifier| {
                refresh_diagnostics(
                    &state.db,
                    batch,
                    state.config.trace_macro_diagnostics,
                    project_diagnostics,
                    notifier,
                );
            });
        }
    }

    fn join_and_clear_workers(&mut self) {
        for handle in self.worker_handles.iter() {
            handle.join();
        }
        self.worker_handles.clear();
    }
}

/// Holds multiple snapshots of the state.
///
/// It is not possible to clone Salsa snapshots nor share one between threads,
/// thus we explicitly create separate snapshots for all threads involved in advance.
struct StateSnapshots(Vec<StateSnapshot>);

impl StateSnapshots {
    /// Takes as many snapshots of `state` as specified in `props` and creates new
    /// [`StateSnapshots`].
    fn new(state: &State, props: &StateSnapshotsProps) -> StateSnapshots {
        StateSnapshots(
            iter::from_fn(|| Some(state.snapshot()))
                .take(props.parallelism.get() * 2 + 1)
                .collect(),
        )
    }

    /// Splits this collection into a tuple of control snapshot and primary and secondary snapshots
    /// sets.
    fn split(self) -> (StateSnapshot, Vec<StateSnapshot>, Vec<StateSnapshot>) {
        let Self(mut snapshots) = self;
        let control = snapshots.pop().unwrap();
        assert_eq!(snapshots.len() % 2, 0);
        let secondary = snapshots.split_off(snapshots.len() / 2);
        (control, snapshots, secondary)
    }
}

/// Stores necessary properties for creating [`StateSnapshots`].
struct StateSnapshotsProps {
    /// Parallelism of the diagnostics worker pool.
    parallelism: NonZero<usize>,
}
