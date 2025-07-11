use std::collections::HashSet;
use std::path::PathBuf;

pub use self::crate_data::{Crate, extract_custom_file_stems};
pub use self::model::ConfigsRegistry;
pub use self::project_manifest_path::*;
use crate::ide::code_lens::FileChange;
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::lsp::ext::CorelibVersionMismatch;
use crate::project::crate_data::CrateInfo;
use crate::project::model::ProjectModel;
use crate::project::scarb::extract_crates;
use crate::project::unmanaged_core_crate::try_to_init_unmanaged_core_if_not_present;
use crate::server::client::Notifier;
use crate::server::is_cairo_file_path;
use crate::server::schedule::thread;
use crate::server::schedule::thread::{JoinHandle, ThreadPriority};
use crate::state::{Snapshot, State};
use crate::toolchain::scarb::ScarbToolchain;
use anyhow::Context;
use cairo_lang_compiler::db::validate_corelib;
use cairo_lang_compiler::project::{setup_project, update_crate_roots_from_project_config};
use cairo_lang_filesystem::db::{CrateIdentifier, FilesGroup, FilesGroupEx};
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_project::ProjectConfig;
use crossbeam::channel::{Receiver, Sender};
use lsp_types::notification::ShowMessage;
use lsp_types::{MessageType, ShowMessageParams};
use salsa::ParallelDatabase;
use tracing::{debug, error, warn};

pub mod builtin_plugins;
mod crate_data;
mod model;
mod project_manifest_path;
mod scarb;
mod unmanaged_core_crate;

pub struct ProjectController {
    model: ProjectModel,
    scarb_toolchain: ScarbToolchain,
    // NOTE: Member order matters here.
    //   The request sender MUST be dropped before controller's thread join handle.
    //   Otherwise, the controller thread will never stop, and the controller's
    //   JoinHandle drop will cause deadlock by waiting for the thread to join.
    requests_sender: Sender<ProjectUpdateRequest>,
    response_receiver: Receiver<ProjectUpdate>,
    _thread: JoinHandle,
}

impl ProjectController {
    /// Initializes [`ProjectController`] by spawning a background thread to handle extracting
    /// project updates for files and initializing channels needed for communication with the
    /// thread.
    ///
    /// The background thread is responsible for fetching changes to the project model: check
    /// [`ProjectControllerThread::send_project_update_for_file`] for more information.
    pub fn initialize(scarb_toolchain: ScarbToolchain, notifier: Notifier) -> Self {
        let (requests_sender, requests_receiver) = crossbeam::channel::unbounded();
        let (response_sender, response_receiver) = crossbeam::channel::unbounded();

        let thread = ProjectControllerThread::spawn(
            requests_receiver,
            response_sender,
            scarb_toolchain.clone(),
            notifier,
        );

        ProjectController {
            model: ProjectModel::new(scarb_toolchain.clone()),
            scarb_toolchain,
            requests_sender,
            response_receiver,
            _thread: thread,
        }
    }

    pub fn configs_registry(&self) -> Snapshot<ConfigsRegistry> {
        self.model.configs_registry()
    }

    pub fn response_receiver(&self) -> Receiver<ProjectUpdate> {
        self.response_receiver.clone()
    }

    pub fn request_updating_project_for_file(&self, file_path: PathBuf) {
        // Skip updating the project model for dependencies from Scarb cache.
        // It is extremely likely that this is not the project that a user wants to work on and
        // opening it was a result of `goto` to dependency.
        if let Some(path) = self.scarb_toolchain.cache_path() {
            if file_path.starts_with(path) {
                return;
            }
        }

        self.send_request(ProjectUpdateRequest {
            file_path,
            loaded_manifests: self.model.loaded_manifests(),
        })
    }

    pub fn clear_loaded_workspaces(&mut self) {
        self.model.clear_loaded_workspaces();
    }

    /// Handles project update by applying necessary changes to the database.
    ///
    /// The project update is sent from [`ProjectControllerThread::send_project_update_for_file`]
    /// and received in the main [`event loop`](crate::Backend::event_loop).
    #[tracing::instrument(skip_all, fields(project_update))]
    pub fn handle_update(state: &mut State, notifier: Notifier, project_update: ProjectUpdate) {
        let db = &mut state.db;
        match project_update {
            ProjectUpdate::Scarb { crates, workspace_dir } => {
                debug!("updating crate roots from scarb metadata: {crates:#?}");

                state.project_controller.model.load_workspace(
                    db,
                    crates,
                    workspace_dir,
                    &state.proc_macro_controller,
                    state.config.enable_linter,
                );
            }
            ProjectUpdate::ScarbMetadataFailed => {
                // Try to set up a corelib at least if it is not in the db already.
                try_to_init_unmanaged_core_if_not_present(
                    db,
                    &state.config,
                    &state.scarb_toolchain,
                );
            }
            ProjectUpdate::CairoProjectToml(maybe_project_config) => {
                if let Some(project_config) = *maybe_project_config {
                    update_crate_roots_from_project_config(db, &project_config);

                    // Make sure cfg(test) is not set if the core crate comes from the Scarb cache.
                    if contains_core_from_scarb_cache(&project_config, &state.scarb_toolchain) {
                        let core_id = CrateId::core(db);

                        let mut core_settings = db.crate_config(core_id).unwrap();
                        core_settings.settings.cfg_set = Some(
                            core_settings
                                .settings
                                .cfg_set
                                .unwrap_or_default()
                                .union(&AnalysisDatabase::initial_cfg_set_for_deps()),
                        );
                        db.set_crate_config(core_id, Some(core_settings));
                    }
                }

                try_to_init_unmanaged_core_if_not_present(
                    db,
                    &state.config,
                    &state.scarb_toolchain,
                );
            }
            ProjectUpdate::NoConfig(file_path) => {
                try_to_init_unmanaged_core_if_not_present(
                    db,
                    &state.config,
                    &state.scarb_toolchain,
                );

                if let Err(err) = setup_project(&mut *db, &file_path) {
                    error!(
                        "error loading file {} as a single crate: {err}",
                        file_path.to_string_lossy()
                    );
                }
            }
        }

        // Drop mut ref so we can obtain snapshot.
        let _ = db;

        // Manifest may changed, update for open files
        state.code_lens_controller.on_did_change(
            state.db.snapshot(),
            state.config.clone(),
            state
                .open_files
                .iter()
                .filter(|&url| is_cairo_file_path(url))
                .cloned()
                .map(|file| FileChange { url: file, was_deleted: false }),
        );

        if let Err(result) = validate_corelib(&state.db) {
            notifier.notify::<CorelibVersionMismatch>(result.to_string());
        }

        #[cfg(feature = "testing")]
        notifier.notify::<crate::lsp::ext::testing::ProjectUpdatingFinished>(());
    }

    pub fn migrate_crates_to_new_db(
        &self,
        new_db: &mut AnalysisDatabase,
        proc_macro_controller: &ProcMacroClientController,
        enable_linter: bool,
    ) {
        self.model.apply_changes_to_db(new_db, proc_macro_controller, enable_linter);
    }

    /// Sends an action request to the background thread.
    fn send_request(&self, request: ProjectUpdateRequest) {
        self.requests_sender
            .send(request)
            .expect("project controller thread should not have panicked or dropped the receiver")
    }
}

/// Intermediate struct used to communicate what changes to the project model should be applied.
/// Associated with [`ProjectManifestPath`] (or its absence) that was detected for a given file.
pub enum ProjectUpdate {
    Scarb { crates: Vec<CrateInfo>, workspace_dir: PathBuf },
    ScarbMetadataFailed,
    CairoProjectToml(Box<Option<ProjectConfig>>),
    NoConfig(PathBuf),
}

/// Stores entire state of project controller thread.
struct ProjectControllerThread {
    requests_receiver: Receiver<ProjectUpdateRequest>,
    response_sender: Sender<ProjectUpdate>,
    scarb_toolchain: ScarbToolchain,
    notifier: Notifier,
}

impl ProjectControllerThread {
    /// Spawns a new project controller thread and returns a handle to it.
    fn spawn(
        requests_receiver: Receiver<ProjectUpdateRequest>,
        response_sender: Sender<ProjectUpdate>,
        scarb_toolchain: ScarbToolchain,
        notifier: Notifier,
    ) -> JoinHandle {
        let this = Self { requests_receiver, response_sender, scarb_toolchain, notifier };

        thread::Builder::new(ThreadPriority::Worker)
            .name("cairo-ls:project-controller".into())
            .spawn(move || this.event_loop())
            .expect("failed to spawn project controller thread")
    }

    /// Runs project controller's event loop.
    fn event_loop(mut self) {
        while let Ok(request) = self.requests_receiver.recv() {
            let project_update = self.fetch_project_update_for_file(request);
            if let Some(project_update) = project_update {
                self.send_project_update(project_update);
            }
        }
    }

    /// Tries to fetch changes to the project model that are necessary for the file analysis.
    ///
    /// NOTE: this function is potentially expensive as it may call `scarb metadata`.
    /// It is meant to be run only in the background thread.
    #[tracing::instrument(skip_all)]
    fn fetch_project_update_for_file(
        &mut self,
        project_update_request: ProjectUpdateRequest,
    ) -> Option<ProjectUpdate> {
        let project_update = match ProjectManifestPath::discover(
            &project_update_request.file_path,
            &self.notifier,
        ) {
            Some(ProjectManifestPath::Scarb(manifest_path)) => {
                if project_update_request.loaded_manifests.contains(&manifest_path) {
                    debug!("scarb project is already loaded: {}", manifest_path.display());
                    return None;
                }

                let metadata = self
                    .scarb_toolchain
                    .metadata(&manifest_path)
                    .with_context(|| {
                        format!("failed to refresh scarb workspace: {}", manifest_path.display())
                    })
                    .inspect_err(|err| {
                        error!("{err:?}");
                    })
                    .ok();

                metadata
                    .map(|metadata| ProjectUpdate::Scarb {
                        crates: extract_crates(&metadata),
                        workspace_dir: metadata.workspace.root.into_std_path_buf(),
                    })
                    .unwrap_or(ProjectUpdate::ScarbMetadataFailed)
            }

            Some(ProjectManifestPath::CairoProject(config_path)) => {
                // The base path of ProjectConfig must be absolute to ensure that all paths in Salsa
                // DB will also be absolute.
                assert!(config_path.is_absolute());

                let maybe_project_config = ProjectConfig::from_file(&config_path)
                    .inspect_err(|err| {
                        let config_path_lossy = config_path.to_string_lossy();
                        error!("parsing {config_path_lossy} failed: {err:?}");
                        self.notifier.notify::<ShowMessage>(ShowMessageParams {
                            typ: MessageType::ERROR,
                            message: format!(
                                "Failed to parse: {config_path_lossy}. Project analysis will not \
                                 be available.",
                            ),
                        });
                    })
                    .ok();
                ProjectUpdate::CairoProjectToml(Box::new(maybe_project_config))
            }

            None => ProjectUpdate::NoConfig(project_update_request.file_path),
        };

        Some(project_update)
    }

    ///  Sends [`ProjectUpdate`] to the main [`event loop`](crate::Backend::event_loop).
    fn send_project_update(&self, project_update: ProjectUpdate) {
        self.response_sender
            .send(project_update)
            .expect("the receiver was expected to exist in the main event loop");
    }
}

struct ProjectUpdateRequest {
    file_path: PathBuf,
    loaded_manifests: Snapshot<HashSet<PathBuf>>,
}

fn contains_core_from_scarb_cache(
    project_config: &ProjectConfig,
    scarb_toolchain: &ScarbToolchain,
) -> bool {
    project_config
        .content
        .crate_roots
        .get(&CrateIdentifier::from("core"))
        .map(|p| project_config.absolute_crate_root(p))
        .is_some_and(|core_root| {
            scarb_toolchain
                .cache_path()
                .is_some_and(|scarb_cache_path| core_root.starts_with(scarb_cache_path))
        })
}
