use crate::Backend;
use crate::server::connection::ConnectionInitializer;
use crate::server::schedule::thread::JoinHandle;
use anyhow::Result;

pub use crate::ide::semantic_highlighting::token_kind::SemanticTokenKind;
use std::path::PathBuf;

/// Special object to run the language server in end-to-end tests.
pub struct BackendForTesting(Backend);

impl BackendForTesting {
    pub fn new() -> (Box<dyn FnOnce(PathBuf) -> BackendForTesting + Send>, lsp_server::Connection) {
        let (connection_initializer, client) = ConnectionInitializer::memory();

        let init = Box::new(|cwd| {
            BackendForTesting(Backend::initialize(connection_initializer, cwd).unwrap())
        });

        (init, client)
    }

    pub fn run_for_tests(self) -> Result<JoinHandle<Result<()>>> {
        self.0.run()
    }
}
