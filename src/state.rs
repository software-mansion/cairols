use std::collections::HashSet;
use std::default::Default;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;

use lsp_types::{ClientCapabilities, Url};
use salsa::ParallelDatabase;

use crate::config::Config;
use crate::ide::analysis_progress::AnalysisProgressController;
use crate::ide::code_lens::CodeLensController;
use crate::lang::db::{AnalysisDatabase, AnalysisDatabaseSwapper};
use crate::lang::diagnostics::DiagnosticsController;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::project::{ConfigsRegistry, ProjectController};
use crate::server::client::Client;
use crate::server::connection::ClientSender;
use crate::toolchain::scarb::ScarbToolchain;

/// State of Language server.
pub struct State {
    pub db: AnalysisDatabase,
    pub open_files: Owned<HashSet<Url>>,
    pub config: Owned<Config>,
    pub client_capabilities: Owned<ClientCapabilities>,
    pub scarb_toolchain: ScarbToolchain,
    pub db_swapper: AnalysisDatabaseSwapper,
    pub diagnostics_controller: DiagnosticsController,
    pub proc_macro_controller: ProcMacroClientController,
    pub project_controller: ProjectController,
    pub analysis_progress_controller: AnalysisProgressController,
    pub code_lens_controller: CodeLensController,
}

impl State {
    pub fn new(
        sender: ClientSender,
        client_capabilities: ClientCapabilities,
        cwd: PathBuf,
    ) -> Self {
        let notifier = Client::new(sender).notifier();
        let scarb_toolchain = ScarbToolchain::new(notifier.clone());

        let analysis_progress_controller = AnalysisProgressController::new(notifier.clone());

        let diagnostics_controller = DiagnosticsController::new(
            notifier.clone(),
            analysis_progress_controller.clone(),
            scarb_toolchain.clone(),
        );

        let proc_macro_controller = ProcMacroClientController::new(
            scarb_toolchain.clone(),
            notifier.clone(),
            analysis_progress_controller.server_tracker(),
            cwd,
            diagnostics_controller.generate_code_complete_receiver(),
        );

        Self {
            db: AnalysisDatabase::new(),
            open_files: Default::default(),
            config: Default::default(),
            client_capabilities: Owned::new(client_capabilities.into()),
            scarb_toolchain: scarb_toolchain.clone(),
            db_swapper: AnalysisDatabaseSwapper::new(),
            diagnostics_controller,
            analysis_progress_controller,
            proc_macro_controller,
            project_controller: ProjectController::initialize(scarb_toolchain, notifier),
            code_lens_controller: CodeLensController::new(),
        }
    }

    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            db: self.db.snapshot(),
            scarb_toolchain: self.scarb_toolchain.clone(),
            open_files: self.open_files.snapshot(),
            config: self.config.snapshot(),
            client_capabilities: self.client_capabilities.snapshot(),
            configs_registry: self.project_controller.configs_registry(),
            code_lens_controller: self.code_lens_controller.clone(),
        }
    }
}

/// Readonly snapshot of Language server state.
pub struct StateSnapshot {
    pub db: salsa::Snapshot<AnalysisDatabase>,
    pub scarb_toolchain: ScarbToolchain,
    pub open_files: Snapshot<HashSet<Url>>,
    pub config: Snapshot<Config>,
    pub client_capabilities: Snapshot<ClientCapabilities>,
    pub configs_registry: Snapshot<ConfigsRegistry>,
    pub code_lens_controller: CodeLensController,
}

impl std::panic::UnwindSafe for StateSnapshot {}

/// Represents owned value that can be mutated.
/// Allows creating snapshot from self.
#[derive(Debug, Default)]
pub struct Owned<T: ?Sized>(Arc<T>);

/// Readonly snapshot of [`Owned`] value.
#[derive(Debug, Default, Clone)]
pub struct Snapshot<T: ?Sized>(Arc<T>);

impl<T: ?Sized> Owned<T> {
    pub fn new(inner: Arc<T>) -> Self {
        Self(inner)
    }

    /// Creates a snapshot of value's current state.
    pub fn snapshot(&self) -> Snapshot<T> {
        Snapshot(self.0.clone())
    }
}

impl<T: ?Sized> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl<T: ?Sized> Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
