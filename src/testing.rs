use crate::Backend;
use crate::lang::db::AnalysisDatabase;
use crate::server::connection::ConnectionInitializer;
use crate::server::schedule::thread::JoinHandle;
use crate::state::DbBox;
use anyhow::Result;
use parking_lot::{ArcMutexGuard, Mutex, RawMutex};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::thread;

/// Special object to run the language server in end-to-end tests.
pub struct BackendForTesting(Backend);

impl BackendForTesting {
    pub fn new(
        preinitialized_database: Option<DbBox<AnalysisDatabase>>,
    ) -> (Box<dyn FnOnce() -> BackendForTesting + Send>, lsp_server::Connection) {
        let (connection_initializer, client) = ConnectionInitializer::memory();

        let init = Box::new(|| {
            BackendForTesting(
                Backend::initialize(connection_initializer, preinitialized_database).unwrap(),
            )
        });

        (init, client)
    }

    pub fn run(self) -> Result<JoinHandle<Result<()>>> {
        self.0.run()
    }
}

/// Holds either an owned or shared instance of `T`.
///
/// Shared `T` instances hold a mutex guard protecting concurrent access.
/// There can be only one instance of [`MaybeShared`] of the given `T` object at the time.
/// This is important when sharing `T` among tests: a test function will wait for the previous
/// test's share to be dropped before taking its own instance.
/// If tests are leaving unterminated threads that hold shares, this can hang the entire test suite!
///
/// This type follows the [`DbBox`] contract.
pub enum MaybeShared<T: 'static> {
    Owned(T),
    Shared(ArcMutexGuard<RawMutex, T>),
}

impl<T: 'static> MaybeShared<T> {
    /// Creates a new [`MaybeShared`] instance from a thread-local.
    pub fn share_thread_local(local_key: &'static thread::LocalKey<Shareable<T>>) -> Self {
        let guard = local_key.with(Arc::clone).lock_arc();
        Self::Shared(guard)
    }
}

impl<T: 'static> Deref for MaybeShared<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(it) => it,
            Self::Shared(it) => it,
        }
    }
}

impl<T: 'static> DerefMut for MaybeShared<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Owned(it) => it,
            Self::Shared(it) => it,
        }
    }
}

impl<T: Default> Default for MaybeShared<T> {
    fn default() -> Self {
        Self::Owned(Default::default())
    }
}

pub type Shareable<T> = Arc<Mutex<T>>;
