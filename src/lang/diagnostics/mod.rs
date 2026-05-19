use std::collections::HashMap;
use std::num::NonZero;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use lsp_types::notification::PublishDiagnostics;
use lsp_types::{Diagnostic, DiagnosticSeverity, PublishDiagnosticsParams, Url};
use tracing::{error, trace};

use self::file_batches::{batches, find_primary_files, find_secondary_files};
use self::project_diagnostics::WindowDiagnostics;
use self::refresh::{clear_old_plugin_diagnostics, refresh_plugin_diagnostics};
use crate::ide::analysis_progress::AnalysisProgressController;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::project::{
    CrateInfo, ScarbMetadataMessage, scarb_check_diagnostics_to_diagnostics,
    scarb_metadata_messages_to_diagnostics,
};
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

type ScarbManifestDiagnostics = HashMap<Url, HashMap<Url, Vec<Diagnostic>>>;
const SCARB_CHECK_REFRESH_DEBOUNCE: Duration = Duration::from_millis(150);

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
    refresh_revision: Arc<AtomicU64>,
    scarb_manifest_diagnostics: ScarbManifestDiagnostics,
    _thread: JoinHandle,
}

impl DiagnosticsController {
    /// Creates a new diagnostics controller.
    pub fn new(
        notifier: Notifier,
        analysis_progress_tracker: AnalysisProgressController,
        scarb_toolchain: ScarbToolchain,
    ) -> Self {
        let (trigger, receiver) = trigger::trigger();
        let refresh_revision = Arc::new(AtomicU64::new(0));
        let (thread, _) = DiagnosticsControllerThread::spawn(
            receiver,
            notifier,
            analysis_progress_tracker,
            scarb_toolchain,
            refresh_revision.clone(),
        );
        Self {
            trigger,
            refresh_revision,
            scarb_manifest_diagnostics: Default::default(),
            _thread: thread,
        }
    }

    /// Schedules diagnostics refreshing on snapshot(s) of the current state.
    pub fn refresh(&self, state: &State) {
        self.refresh_revision.fetch_add(1, Ordering::Relaxed);
        self.trigger.activate(state.snapshot());
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

/// Stores entire state of diagnostics controller's worker thread.
struct DiagnosticsControllerThread {
    receiver: trigger::Receiver<StateSnapshot>,
    notifier: Notifier,
    pool: thread::Pool,
    window_diagnostics: WindowDiagnostics,
    analysis_progress_controller: AnalysisProgressController,
    worker_handles: Vec<TaskHandle>,
    scarb_toolchain: ScarbToolchain,
    refresh_revision: Arc<AtomicU64>,
}

impl DiagnosticsControllerThread {
    /// Spawns a new diagnostics controller worker thread
    /// and returns a handle to it and the amount of parallelism it provides.
    fn spawn(
        receiver: trigger::Receiver<StateSnapshot>,
        notifier: Notifier,
        analysis_progress_controller: AnalysisProgressController,
        scarb_toolchain: ScarbToolchain,
        refresh_revision: Arc<AtomicU64>,
    ) -> (JoinHandle, NonZero<usize>) {
        let mut this = Self {
            receiver,
            notifier,
            analysis_progress_controller,
            // Above 4 threads we start losing performance
            // due to salsa locking and context switching.
            pool: thread::Pool::new(4, "diagnostic-worker"),
            window_diagnostics: WindowDiagnostics::new(),
            worker_handles: Vec::new(),
            scarb_toolchain,
            refresh_revision,
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
        while let Some(state) = self.next_debounced_state() {
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

    fn next_debounced_state(&self) -> Option<StateSnapshot> {
        let mut state = self.receiver.wait()?;

        loop {
            std::thread::sleep(SCARB_CHECK_REFRESH_DEBOUNCE);
            let Some(newer_state) = self.receiver.try_wait() else {
                return Some(state);
            };
            state = newer_state;
        }
    }

    /// Runs a single tick of the diagnostics controller's event loop.
    #[tracing::instrument(skip_all)]
    fn diagnostics_controller_tick(&mut self, state: &StateSnapshot) {
        let scheduled_revision = self.refresh_revision.load(Ordering::Relaxed);
        let primary_set = find_primary_files(&state.db, &state.open_files);
        let secondary = find_secondary_files(&state.db, &primary_set);
        let workspace_manifests: Vec<_> =
            state.loaded_workspace_manifests.iter().cloned().collect();

        let primary: Vec<_> = primary_set.iter().copied().collect();
        self.spawn_plugin_diagnostics_workers(&primary, state);
        self.spawn_plugin_diagnostics_workers(&secondary, state);
        self.spawn_scarb_check_workers(&workspace_manifests, state, scheduled_revision);

        let files_to_preserve: std::collections::HashSet<Url> = primary
            .into_iter()
            .chain(secondary)
            .flat_map(|file| state.db.url_for_file(file))
            .collect();

        let refresh_revision = self.refresh_revision.clone();
        self.spawn_worker(move |window_diagnostics, notifier| {
            if refresh_revision.load(Ordering::Relaxed) != scheduled_revision {
                trace!(scheduled_revision, "discarding stale diagnostics cleanup");
                return;
            }
            clear_old_plugin_diagnostics(files_to_preserve, window_diagnostics, notifier);
        });
    }

    /// Shortcut for spawning a worker task which does the boilerplate around cloning state parts
    /// and catching panics.
    fn spawn_worker(&mut self, f: impl FnOnce(WindowDiagnostics, Notifier) + Send + 'static) {
        let window_diagnostics = self.window_diagnostics.clone();
        let notifier = self.notifier.clone();
        let worker_fn = move || f(window_diagnostics, notifier);
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

    /// Spawns one worker per loaded Scarb workspace to run `scarb check`.
    fn spawn_scarb_check_workers(
        &mut self,
        workspace_manifests: &[PathBuf],
        state: &StateSnapshot,
        scheduled_revision: u64,
    ) {
        for workspace_manifest_path in workspace_manifests.iter().cloned() {
            let scarb_toolchain = self.scarb_toolchain.clone();
            let state = state.clone();
            let refresh_revision = self.refresh_revision.clone();
            self.spawn_worker(move |window_diagnostics, notifier| {
                refresh_scarb_check_diagnostics(
                    &state.db,
                    workspace_manifest_path,
                    scheduled_revision,
                    refresh_revision,
                    window_diagnostics,
                    notifier,
                    scarb_toolchain,
                );
            });
        }
    }

    fn spawn_plugin_diagnostics_workers<'db>(
        &mut self,
        files: &[cairo_lang_filesystem::ids::FileId<'db>],
        state: &StateSnapshot,
    ) {
        let files: &[cairo_lang_filesystem::ids::FileId<'static>] =
            unsafe { std::mem::transmute(files) };
        let files_batches =
            batches(files, self.pool.parallelism()).into_iter().filter(|batch| !batch.is_empty());

        for batch in files_batches {
            let scarb_toolchain = self.scarb_toolchain.clone();
            let state = state.clone();
            self.spawn_worker(move |window_diagnostics, notifier| {
                refresh_plugin_diagnostics(
                    &state.db,
                    &state.config,
                    &state.configs_registry,
                    batch,
                    window_diagnostics,
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

#[tracing::instrument(skip_all, fields(workspace_manifest = %workspace_manifest_path.display()))]
fn refresh_scarb_check_diagnostics(
    db: &AnalysisDatabase,
    workspace_manifest_path: PathBuf,
    scheduled_revision: u64,
    refresh_revision: Arc<AtomicU64>,
    window_diagnostics: WindowDiagnostics,
    notifier: Notifier,
    scarb_toolchain: ScarbToolchain,
) {
    let Ok(diagnostics) = scarb_toolchain.check_diagnostics(&workspace_manifest_path) else {
        return;
    };
    let Ok(workspace_manifest_url) = Url::from_file_path(&workspace_manifest_path) else {
        return;
    };

    if refresh_revision.load(Ordering::Relaxed) != scheduled_revision {
        trace!(
            workspace_manifest = %workspace_manifest_path.display(),
            scheduled_revision,
            "discarding stale scarb check diagnostics"
        );
        return;
    }

    let diags_to_send = window_diagnostics.update_workspace(
        workspace_manifest_url,
        scarb_check_diagnostics_to_diagnostics(db, diagnostics),
    );

    for (url, diagnostics) in diags_to_send {
        notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
            uri: url,
            diagnostics,
            version: None,
        });
    }
}
