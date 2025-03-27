use std::collections::HashSet;
use std::path::PathBuf;

use crate::lang::db::AnalysisDatabase;
use crate::lsp::ext::CorelibVersionMismatch;
use crate::project::crate_data::CrateInfo;
use crate::project::model::ProjectModel;
use crate::project::scarb::extract_crates;
use crate::project::unmanaged_core_crate::try_to_init_unmanaged_core_if_not_present;
use crate::server::client::Notifier;
use crate::server::schedule::thread;
use crate::server::schedule::thread::{JoinHandle, ThreadPriority};
use crate::state::{Snapshot, State};
use crate::toolchain::scarb::ScarbToolchain;
use anyhow::Context;
use cairo_lang_compiler::db::validate_corelib;
use cairo_lang_compiler::project::{setup_project, update_crate_roots_from_project_config};
use cairo_lang_project::ProjectConfig;
use crossbeam::channel::{Receiver, Sender};
use lsp_types::notification::ShowMessage;
use lsp_types::{MessageType, ShowMessageParams};
use tracing::{debug, error, warn};

pub use self::crate_data::Crate;
pub use self::model::ConfigsRegistry;
pub use self::project_manifest_path::*;

pub mod builtin_plugins;
mod crate_data;
mod model;
mod project_manifest_path;
mod scarb;
mod unmanaged_core_crate;

pub struct ProjectController {
    model: ProjectModel,
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
            scarb_toolchain,
            notifier,
        );

        ProjectController {
            requests_sender,
            response_receiver,
            model: Default::default(),
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
        self.send_request(ProjectUpdateRequest {
            file_path,
            loaded_manifests: self.model.loaded_manifests(),
        })
    }

    pub fn clear_loaded_workspaces(&mut self, db: &mut AnalysisDatabase) {
        self.model.clear_loaded_workspaces(db);
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
                try_to_init_unmanaged_core_if_not_present(
                    db,
                    &state.config,
                    &state.scarb_toolchain,
                );

                if let Some(project_config) = maybe_project_config {
                    update_crate_roots_from_project_config(db, &project_config);
                }
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

        if let Err(result) = validate_corelib(db) {
            notifier.notify::<CorelibVersionMismatch>(result.to_string());
        }

        #[cfg(feature = "testing")]
        notifier.notify::<crate::lsp::ext::testing::ProjectUpdatingFinished>(());
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
    CairoProjectToml(Option<ProjectConfig>),
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
                ProjectUpdate::CairoProjectToml(maybe_project_config)
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
