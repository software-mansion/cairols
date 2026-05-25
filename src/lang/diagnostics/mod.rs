use std::collections::{HashMap, HashSet};
use std::num::NonZero;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use cairo_lang_filesystem::ids::FileId;
use crossbeam::channel::{Receiver, Sender};
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{Diagnostic, DiagnosticSeverity, PublishDiagnosticsParams, Url};
use tracing::{error, trace};

use self::project_diagnostics::ProjectDiagnostics;
use self::refresh::{clear_old_diagnostics, refresh_diagnostics};
use crate::config::Config;
use crate::ide::analysis_progress::AnalysisProgressController;
use crate::lang::db::AnalysisDatabase;
use crate::lang::diagnostics::file_batches::{batches, find_primary_files, find_secondary_files};
use crate::lang::lsp::LsProtoGroup;
use crate::project::ConfigsRegistry;
use crate::project::{CrateInfo, ScarbMetadataMessage, scarb_metadata_messages_to_diagnostics};
use crate::server::client::Notifier;
use crate::server::panic::cancelled_anyhow;
use crate::server::schedule::thread::task_progress_monitor::{
    TaskHandle, TaskResult, task_progress_monitor,
};
use crate::server::schedule::thread::{self, JoinHandle, ThreadPriority};
use crate::server::trigger;
use crate::state::{Snapshot, State};
use crate::toolchain::scarb::ScarbToolchain;

mod file_batches;
mod file_diagnostics;
mod lsp;
mod project_diagnostics;
mod refresh;

type ScarbManifestDiagnostics = HashMap<Url, HashMap<Url, Vec<Diagnostic>>>;

/// Schedules refreshing of diagnostics in a background thread.
///
/// This structure *owns* the worker thread and is responsible for its lifecycle.
/// Dropping it will ask the worker to stop and synchronously wait for it to finish.
pub struct DiagnosticsController {
    // NOTE: Member order matters here.
    //   The trigger MUST be dropped before worker's join handle.
    //   Otherwise, the controller thread will never be requested to stop, and the controller's
    //   JoinHandle will never terminate.
    trigger: trigger::Sender<DiagnosticsRefreshInput>,
    runs: Arc<DiagnosticsRuns>,
    generate_code_complete_receiver: Receiver<()>,
    scarb_manifest_diagnostics: ScarbManifestDiagnostics,
    _thread: JoinHandle,
}

impl DiagnosticsController {
    /// Creates a new diagnostics controller.
    pub fn new(
        notifier: Notifier,
        analysis_progress_tracker: AnalysisProgressController,
        _scarb_toolchain: ScarbToolchain,
    ) -> Self {
        let (generate_code_complete_sender, generate_code_complete_receiver) =
            crossbeam::channel::bounded(1);
        let (trigger, receiver) = trigger::trigger();
        let runs = Arc::new(DiagnosticsRuns::default());
        let (thread, _) = DiagnosticsControllerThread::spawn(
            receiver,
            generate_code_complete_sender,
            notifier,
            analysis_progress_tracker,
        );
        Self {
            trigger,
            runs,
            generate_code_complete_receiver,
            scarb_manifest_diagnostics: Default::default(),
            _thread: thread,
        }
    }

    pub fn generate_code_complete_receiver(&self) -> Receiver<()> {
        self.generate_code_complete_receiver.clone()
    }

    /// Schedules diagnostics refreshing on a fresh diagnostics-only database.
    pub fn refresh(&self, state: &State) {
        let run = self.runs.start_new();

        let Ok(db) = catch_unwind(AssertUnwindSafe(|| {
            crate::lang::db::migrate_to_fresh_database(
                &state.db,
                &state.open_files,
                &state.project_controller,
                &state.proc_macro_controller,
            )
        })) else {
            error!("caught panic when preparing diagnostics db");
            return;
        };

        self.trigger.activate(DiagnosticsRefreshInput {
            db,
            scarb_toolchain: state.scarb_toolchain.clone(),
            open_files: state.open_files.snapshot(),
            config: state.config.snapshot(),
            configs_registry: state.project_controller.configs_registry(),
            run,
        });
    }

    pub fn publish_scarb_manifest_diagnostics(
        &mut self,
        root_manifest_path: &Path,
        diagnostics: Vec<ScarbMetadataMessage>,
        manifest_diagnostic_severity: DiagnosticSeverity,
        db: &AnalysisDatabase,
        notifier: &Notifier,
    ) {
        let Some(root_manifest_url) = Url::from_file_path(root_manifest_path).ok() else {
            return;
        };
        let diagnostics = scarb_metadata_messages_to_diagnostics(
            db,
            diagnostics,
            root_manifest_path,
            manifest_diagnostic_severity,
        )
        .unwrap_or_default();
        let diags_to_send = self.update_scarb_manifest_diagnostics(root_manifest_url, diagnostics);
        for (url, diagnostics) in diags_to_send {
            notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
                uri: url,
                diagnostics,
                version: None,
            });
        }
    }

    pub fn clear_scarb_manifest_diagnostics(&mut self, crates: &[CrateInfo], notifier: &Notifier) {
        let manifest_paths = crates
            .iter()
            .filter(|crate_info| crate_info.is_member)
            .map(|crate_info| crate_info.manifest_path.as_path());

        for manifest_path in manifest_paths {
            self.clear_scarb_manifest_diagnostics_for_path(manifest_path, notifier);
        }
    }

    fn update_scarb_manifest_diagnostics(
        &mut self,
        root_manifest_url: Url,
        new_diagnostics: HashMap<Url, Vec<Diagnostic>>,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        let old_diagnostics = self
            .scarb_manifest_diagnostics
            .insert(root_manifest_url, new_diagnostics.clone())
            .unwrap_or_default();

        if old_diagnostics == new_diagnostics {
            return HashMap::new();
        }

        let mut diagnostics_to_send = HashMap::new();

        for location_url in old_diagnostics.keys() {
            if !new_diagnostics.contains_key(location_url) {
                diagnostics_to_send.insert(location_url.clone(), Vec::new());
            }
        }

        for (location_url, diagnostics) in new_diagnostics {
            if old_diagnostics.get(&location_url) != Some(&diagnostics) {
                diagnostics_to_send.insert(location_url, diagnostics);
            }
        }

        diagnostics_to_send
    }

    fn clear_scarb_manifest_diagnostics_for_path(
        &mut self,
        manifest_path: &Path,
        notifier: &Notifier,
    ) {
        let Some(manifest_url) = Url::from_file_path(manifest_path).ok() else {
            return;
        };
        let Some(old_diagnostics) = self.scarb_manifest_diagnostics.remove(&manifest_url) else {
            return;
        };

        for url in old_diagnostics.into_keys() {
            notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
                uri: url,
                diagnostics: Vec::new(),
                version: None,
            });
        }
    }
}

/// Shared coordinator for diagnostics refresh runs.
#[derive(Default)]
struct DiagnosticsRuns {
    latest_generation: AtomicU64,
    active_diagnostics_db: Mutex<Option<AnalysisDatabase>>,
}

impl DiagnosticsRuns {
    /// Starts a new run and cancels the previously active disposable db, if any.
    fn start_new(self: &Arc<Self>) -> DiagnosticsRun {
        let generation = self.latest_generation.fetch_add(1, Ordering::AcqRel) + 1;
        self.cancel_active_db();
        DiagnosticsRun { generation, runs: self.clone() }
    }

    fn cancel_active_db(&self) {
        let Some(mut db) = self
            .active_diagnostics_db
            .lock()
            .expect("active diagnostics db mutex should never be poisoned")
            .as_ref()
            .cloned()
        else {
            return;
        };

        db.cancel_all();
    }
}

/// Token for one diagnostics refresh run.
///
/// Pass clones of this token to worker tasks instead of raw generation/db state.
#[derive(Clone)]
struct DiagnosticsRun {
    generation: u64,
    runs: Arc<DiagnosticsRuns>,
}

impl DiagnosticsRun {
    /// True if no newer refresh has been scheduled.
    fn is_current(&self) -> bool {
        self.runs.latest_generation.load(Ordering::Acquire) == self.generation
    }

    /// Registers this run's disposable db for cancellation by a future refresh.
    fn set_active_db(&self, db: AnalysisDatabase) {
        *self
            .runs
            .active_diagnostics_db
            .lock()
            .expect("active diagnostics db mutex should never be poisoned") = Some(db);
    }

    /// Clears this run's disposable db handle after completion.
    fn clear_active_db(&self) {
        *self
            .runs
            .active_diagnostics_db
            .lock()
            .expect("active diagnostics db mutex should never be poisoned") = None;
    }
}

#[derive(Clone)]
struct DiagnosticsRefreshInput {
    db: AnalysisDatabase,
    scarb_toolchain: ScarbToolchain,
    open_files: Snapshot<HashSet<Url>>,
    config: Snapshot<Config>,
    configs_registry: Snapshot<ConfigsRegistry>,
    run: DiagnosticsRun,
}

/// Stores entire state of diagnostics controller's worker thread.
struct DiagnosticsControllerThread {
    receiver: trigger::Receiver<DiagnosticsRefreshInput>,
    generate_code_complete_sender: Sender<()>,
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
        receiver: trigger::Receiver<DiagnosticsRefreshInput>,
        generate_code_complete_sender: Sender<()>,
        notifier: Notifier,
        analysis_progress_controller: AnalysisProgressController,
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
        while let Some(input) = self.receiver.wait() {
            assert!(self.worker_handles.is_empty());
            self.analysis_progress_controller.diagnostic_start();
            input.run.set_active_db(input.db.clone());

            let mut controller_cancelled = false;
            if let Err(err) = catch_unwind(AssertUnwindSafe(|| {
                self.diagnostics_controller_tick(&input);
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
            input.run.clear_active_db();

            self.analysis_progress_controller.diagnostic_end(diagnostics_cancelled);
        }
    }

    /// Runs a single tick of the diagnostics controller's event loop.
    #[tracing::instrument(skip_all)]
    fn diagnostics_controller_tick(&mut self, input: &DiagnosticsRefreshInput) {
        let primary_set = find_primary_files(&input.db, &input.open_files);
        let secondary = find_secondary_files(&input.db, &primary_set);
        // Event meaning that all generate_code() calls from this tick were called.
        // This is true because `find_primary_files`/`find_secondary_files` calls `db.file_modules()` and it does all generate_code() calls.
        let _ = self.generate_code_complete_sender.send(());

        let primary: Vec<_> = primary_set.iter().copied().collect();
        self.spawn_refresh_workers(&primary, input);
        self.spawn_refresh_workers(&secondary, input);

        let files_to_preserve: HashSet<Url> = primary
            .into_iter()
            .chain(secondary)
            .flat_map(|file| input.db.url_for_file(file))
            .collect();

        let run = input.run.clone();
        self.spawn_worker(move |project_diagnostics, notifier| {
            clear_old_diagnostics(files_to_preserve, project_diagnostics, notifier, run);
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
    fn spawn_refresh_workers<'db>(
        &mut self,
        files: &[FileId<'db>],
        input: &DiagnosticsRefreshInput,
    ) {
        // TODO(#869)
        let files: &[FileId<'static>] = unsafe { std::mem::transmute(files) };
        let files_batches =
            batches(files, self.pool.parallelism()).into_iter().filter(|v| !v.is_empty());

        for batch in files_batches {
            let db = input.db.clone();
            let config = input.config.clone();
            let configs_registry = input.configs_registry.clone();
            let scarb_toolchain = input.scarb_toolchain.clone();
            let run = input.run.clone();
            self.spawn_worker(move |project_diagnostics, notifier| {
                refresh_diagnostics(
                    &db,
                    &config,
                    &configs_registry,
                    batch,
                    project_diagnostics,
                    notifier,
                    scarb_toolchain,
                    run,
                );
            });
        }
    }

    fn join_and_clear_workers(&mut self) -> Vec<TaskResult> {
        self.worker_handles.drain(..).map(|handle| handle.join()).collect()
    }
}
