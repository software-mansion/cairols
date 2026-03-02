use std::collections::HashSet;
use std::num::NonZero;
use std::panic::{AssertUnwindSafe, catch_unwind};

use cairo_lang_filesystem::ids::FileId;
use crossbeam::channel::{Receiver, Sender};
use lsp_types::Url;
use tracing::{error, trace};

use self::project_diagnostics::ProjectDiagnostics;
use self::refresh::{clear_old_diagnostics, refresh_diagnostics};
use crate::ide::analysis_progress::AnalysisProgressController;
use crate::lang::diagnostics::file_batches::{batches, find_primary_files, find_secondary_files};
use crate::lang::lsp::LsProtoGroup;
use crate::server::client::Notifier;
use crate::server::panic::cancelled_anyhow;
use crate::server::schedule::thread::task_progress_monitor::{
    TaskHandle, TaskResult, task_progress_monitor,
};
use crate::server::schedule::thread::{self, JoinHandle, ThreadPriority};
use crate::server::trigger;
use crate::state::{State, StateSnapshot};
use crate::toolchain::scarb::ScarbToolchain;

mod file_batches;
mod file_diagnostics;
mod lsp;
mod project_diagnostics;
mod refresh;
mod scarb_manifest;

/// Schedules refreshing of diagnostics in a background thread.
///
/// This structure *owns* the worker thread and is responsible for its lifecycle.
/// Dropping it will ask the worker to stop and synchronously wait for it to finish.
pub struct DiagnosticsController {
    // NOTE: Member order matters here.
    //   The trigger MUST be dropped before worker's join handle.
    //   Otherwise, the controller thread will never be requested to stop, and the controller's
    //   JoinHandle will never terminate.
    trigger: trigger::Sender<StateSnapshot>,
    generate_code_complete_receiver: Receiver<()>,
    _thread: JoinHandle,
}

impl DiagnosticsController {
    /// Creates a new diagnostics controller.
    pub fn new(
        notifier: Notifier,
        analysis_progress_tracker: AnalysisProgressController,
        scarb_toolchain: ScarbToolchain,
    ) -> Self {
        let (generate_code_complete_sender, generate_code_complete_receiver) =
            crossbeam::channel::bounded(1);
        let (trigger, receiver) = trigger::trigger();
        let (thread, _) = DiagnosticsControllerThread::spawn(
            receiver,
            generate_code_complete_sender,
            notifier,
            analysis_progress_tracker,
            scarb_toolchain,
        );
        Self { trigger, generate_code_complete_receiver, _thread: thread }
    }

    pub fn generate_code_complete_receiver(&self) -> Receiver<()> {
        self.generate_code_complete_receiver.clone()
    }

    /// Schedules diagnostics refreshing on snapshot(s) of the current state.
    pub fn refresh(&self, state: &State) {
        self.trigger.activate(state.snapshot());
    }
}

/// Stores entire state of diagnostics controller's worker thread.
struct DiagnosticsControllerThread {
    receiver: trigger::Receiver<StateSnapshot>,
    generate_code_complete_sender: Sender<()>,
    notifier: Notifier,
    pool: thread::Pool,
    project_diagnostics: ProjectDiagnostics,
    analysis_progress_controller: AnalysisProgressController,
    worker_handles: Vec<TaskHandle>,
    scarb_toolchain: ScarbToolchain,
}

impl DiagnosticsControllerThread {
    /// Spawns a new diagnostics controller worker thread
    /// and returns a handle to it and the amount of parallelism it provides.
    fn spawn(
        receiver: trigger::Receiver<StateSnapshot>,
        generate_code_complete_sender: Sender<()>,
        notifier: Notifier,
        analysis_progress_controller: AnalysisProgressController,
        scarb_toolchain: ScarbToolchain,
    ) -> (JoinHandle, NonZero<usize>) {
        let mut this = Self {
            receiver,
            generate_code_complete_sender,
            notifier,
            analysis_progress_controller,
            // Above 4 threads we start losing performance
            // due to salsa locking and context switching.
            pool: thread::Pool::new(4, "diagnostic-worker"),
            project_diagnostics: ProjectDiagnostics::new(),
            worker_handles: Vec::new(),
            scarb_toolchain,
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
        while let Some(state) = self.receiver.wait() {
            assert!(self.worker_handles.is_empty());
            self.analysis_progress_controller.diagnostic_start();

            let mut controller_cancelled = false;
            if let Err(err) = catch_unwind(AssertUnwindSafe(|| {
                self.diagnostics_controller_tick(&state);
            })) {
                if let Ok(err) = cancelled_anyhow(err, "diagnostics refreshing has been cancelled")
                {
                    trace!("{err:?}");
                    controller_cancelled = true;
                } else {
                    error!("caught panic while refreshing diagnostics");
                }
            }

            let diagnostics_results = self.join_and_clear_workers();
            let diagnostics_cancelled =
                controller_cancelled || diagnostics_results.contains(&TaskResult::Cancelled);

            self.analysis_progress_controller.diagnostic_end(diagnostics_cancelled);
        }
    }

    /// Runs a single tick of the diagnostics controller's event loop.
    #[tracing::instrument(skip_all)]
    fn diagnostics_controller_tick(&mut self, state: &StateSnapshot) {
        let primary_set = find_primary_files(&state.db, &state.open_files);
        let secondary = find_secondary_files(&state.db, &primary_set);
        // Event meaning that all generate_code() calls from this tick were called.
        // This is true because `find_primary_files`/`find_secondary_files` calls `db.file_modules()` and it does all generate_code() calls.
        let _ = self.generate_code_complete_sender.send(());

        let primary: Vec<_> = primary_set.iter().copied().collect();
        self.spawn_refresh_workers(&primary, state);
        self.spawn_refresh_workers(&secondary, state);

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
        let (tracker, handle) = task_progress_monitor();
        self.pool.spawn(ThreadPriority::Worker, move || {
            if let Err(err) = catch_unwind(AssertUnwindSafe(worker_fn)) {
                if let Ok(err) = cancelled_anyhow(err, "diagnostics worker has been cancelled") {
                    tracker.signal_finish(TaskResult::Cancelled);
                    trace!("{err:?}");
                } else {
                    // Does not matter for us if the task was finished or panicked.
                    tracker.signal_finish(TaskResult::Done);
                    error!("caught panic in diagnostics worker");
                }
            } else {
                tracker.signal_finish(TaskResult::Done);
            }
        });
        self.worker_handles.push(handle);
    }

    /// Makes batches out of `files` and spawns workers to run [`refresh_diagnostics`] on them.
    fn spawn_refresh_workers<'db>(&mut self, files: &[FileId<'db>], state: &StateSnapshot) {
        // TODO(#869)
        let files: &[FileId<'static>] = unsafe { std::mem::transmute(files) };
        let files_batches =
            batches(files, self.pool.parallelism()).into_iter().filter(|v| !v.is_empty());

        for batch in files_batches {
            let scarb_toolchain = self.scarb_toolchain.clone();
            let state = state.clone();
            self.spawn_worker(move |project_diagnostics, notifier| {
                refresh_diagnostics(
                    &state.db,
                    &state.config,
                    &state.configs_registry,
                    batch,
                    project_diagnostics,
                    notifier,
                    scarb_toolchain,
                );
            });
        }
    }

    fn join_and_clear_workers(&mut self) -> Vec<TaskResult> {
        self.worker_handles.drain(..).map(|handle| handle.join()).collect()
    }
}
