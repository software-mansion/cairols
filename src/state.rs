use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::config::Config;
use crate::lang::db::{AnalysisDatabase, AnalysisDatabaseSwapper};
use crate::lang::diagnostics::DiagnosticsController;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::project::ProjectController;
use crate::server::client::Client;
use crate::server::connection::ClientSender;
use crate::toolchain::scarb::ScarbToolchain;
use crate::Tricks;
use lsp_types::{ClientCapabilities, Url};
use salsa::ParallelDatabase;

/// State of Language server.
pub struct State {
    pub db: AnalysisDatabase,
    pub open_files: Owned<HashSet<Url>>,
    pub config: Owned<Config>,
    pub client_capabilities: Owned<ClientCapabilities>,
    pub scarb_toolchain: ScarbToolchain,
    pub db_swapper: AnalysisDatabaseSwapper,
    pub tricks: Owned<Tricks>,
    pub diagnostics_controller: DiagnosticsController,
    pub proc_macro_controller: ProcMacroClientController,
    pub project_controller: ProjectController,
}

impl State {
    pub fn new(
        sender: ClientSender,
        client_capabilities: ClientCapabilities,
        tricks: Tricks,
    ) -> Self {
        let notifier = Client::new(sender).notifier();
        let scarb_toolchain = ScarbToolchain::new(notifier.clone());
        let proc_macro_controller =
            ProcMacroClientController::new(scarb_toolchain.clone(), notifier.clone());

        Self {
            db: AnalysisDatabase::new(&tricks),
            open_files: Default::default(),
            config: Default::default(),
            client_capabilities: Owned::new(client_capabilities.into()),
            scarb_toolchain: scarb_toolchain.clone(),
            db_swapper: AnalysisDatabaseSwapper::new(),
            tricks: Owned::new(tricks.into()),
            diagnostics_controller: DiagnosticsController::new(notifier.clone()),
            proc_macro_controller,
            project_controller: ProjectController::initialize(scarb_toolchain, notifier),
        }
    }

    pub fn snapshot(&self) -> StateSnapshot {
        Beacon::wrap(SnapshotInternal {
            db: self.db.snapshot(),
            open_files: self.open_files.snapshot(),
            config: self.config.snapshot(),
        })
    }
}

pub struct Beacon<T> {
    value: T,
    drop_hook: Option<Box<dyn FnOnce() -> () + Send>>,
}

impl<T> Beacon<T>
where
    T: Send,
{
    // Constructor to wrap a value
    pub fn wrap(value: T) -> Self {
        Self { value, drop_hook: None }
    }

    // Set the drop hook
    pub fn on_drop<F>(&mut self, drop_hook: F)
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        self.drop_hook = Some(Box::new(drop_hook));
    }
}

impl<T> Drop for Beacon<T> {
    fn drop(&mut self) {
        // take the hook, replacing with None
        if let Some(hook) = self.drop_hook.take() {
            hook(); // call the hook
        }
    }
}

impl<T> Deref for Beacon<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub type StateSnapshot = Beacon<SnapshotInternal>;

/// Readonly snapshot of Language server state.
pub struct SnapshotInternal {
    pub db: salsa::Snapshot<AnalysisDatabase>,
    pub open_files: Snapshot<HashSet<Url>>,
    pub config: Snapshot<Config>,
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
