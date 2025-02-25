use std::collections::HashSet;
use std::default::Default;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use lsp_types::{ClientCapabilities, Url};
use salsa::ParallelDatabase;

use crate::config::Config;
use crate::ide::analysis_progress::{AnalysisProgressController, ProcMacroServerTracker};
use crate::lang::db::{AnalysisDatabase, AnalysisDatabaseSwapper};
use crate::lang::diagnostics::DiagnosticsController;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::project::{ManifestRegistry, ProjectController};
use crate::server::client::Client;
use crate::server::connection::ClientSender;
use crate::toolchain::scarb::ScarbToolchain;

/// State of the LS.
pub struct State {
    pub db: DbBox<AnalysisDatabase>,
    pub open_files: Owned<HashSet<Url>>,
    pub config: Owned<Config>,
    pub client_capabilities: Owned<ClientCapabilities>,
    pub scarb_toolchain: ScarbToolchain,
    pub db_swapper: AnalysisDatabaseSwapper,
    pub diagnostics_controller: DiagnosticsController,
    pub proc_macro_controller: ProcMacroClientController,
    pub project_controller: ProjectController,
    pub analysis_progress_controller: AnalysisProgressController,
}

impl State {
    pub fn new(
        sender: ClientSender,
        client_capabilities: ClientCapabilities,
        preinitialized_database: Option<DbBox<AnalysisDatabase>>,
    ) -> Self {
        let notifier = Client::new(sender).notifier();
        let scarb_toolchain = ScarbToolchain::new(notifier.clone());

        let proc_macro_request_tracker = ProcMacroServerTracker::new();

        let analysis_progress_controller =
            AnalysisProgressController::new(notifier.clone(), proc_macro_request_tracker.clone());
        let proc_macro_controller = ProcMacroClientController::new(
            scarb_toolchain.clone(),
            notifier.clone(),
            proc_macro_request_tracker,
        );

        let diagnostics_controller = DiagnosticsController::new(
            notifier.clone(),
            analysis_progress_controller.clone(),
            scarb_toolchain.clone(),
        );

        Self {
            db: preinitialized_database.unwrap_or_default(),
            open_files: Default::default(),
            config: Default::default(),
            client_capabilities: Owned::new(client_capabilities.into()),
            scarb_toolchain: scarb_toolchain.clone(),
            db_swapper: AnalysisDatabaseSwapper::new(),
            diagnostics_controller,
            analysis_progress_controller,
            proc_macro_controller,
            project_controller: ProjectController::initialize(scarb_toolchain, notifier),
        }
    }

    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            db: self.db.snapshot(),
            scarb_toolchain: self.scarb_toolchain.clone(),
            open_files: self.open_files.snapshot(),
            config: self.config.snapshot(),
            loaded_scarb_manifests: self.project_controller.manifests_registry(),
        }
    }
}

/// Readonly snapshot of the LS state.
pub struct StateSnapshot {
    pub db: salsa::Snapshot<AnalysisDatabase>,
    pub scarb_toolchain: ScarbToolchain,
    pub open_files: Snapshot<HashSet<Url>>,
    pub config: Snapshot<Config>,
    pub loaded_scarb_manifests: Snapshot<ManifestRegistry>,
}

impl std::panic::UnwindSafe for StateSnapshot {}

/// A [`Box`]-like structure that holds state objects that can be shared among E2E test runs.
///
/// This is a conditionally compiled alias to a concrete implementation.
/// It is required for such implementations to implement `Deref<T>`, `DerefMut<T>` and `Default`.
#[cfg(feature = "testing")]
pub type DbBox<T> = crate::testing::MaybeShared<T>;
#[cfg(not(feature = "testing"))]
pub type DbBox<T> = self::not_testing::TransparentDbBox<T>;

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

    /// Creates a snapshot of the value's current state.
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

#[cfg(not(feature = "testing"))]
mod not_testing {
    use std::ops::{Deref, DerefMut};

    /// An implementation of [`super::DbBox`] contract that is transparent regarding memory layout.
    ///
    /// This is used in production code.
    #[repr(transparent)]
    pub struct TransparentDbBox<T>(T);

    impl<T> Deref for TransparentDbBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for TransparentDbBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T: Default> Default for TransparentDbBox<T> {
        fn default() -> Self {
            TransparentDbBox(T::default())
        }
    }
}
