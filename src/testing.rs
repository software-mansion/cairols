use std::path::PathBuf;

use anyhow::Result;
use lsp_server::{Message, Notification, Request, RequestId, Response};
use lsp_types::{ConfigurationParams, notification, request};
use serde_json::Value;

use crate::Backend;
pub use crate::ide::semantic_highlighting::token_kind::SemanticTokenKind;
use crate::server::connection::ConnectionInitializer;
use crate::server::schedule::thread::JoinHandle;

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

/// A minimal shared LSP client connection used by both bench and e2e test clients.
///
/// Provides typed message sending and workspace configuration helpers.
/// The receive loop is intentionally left to the caller: bench and e2e test clients have
/// different message handling requirements (blocking vs. timeout recv, message tracing, etc.).
pub struct LspClientConnection {
    pub connection: lsp_server::Connection,
    req_id: i32,
}

impl LspClientConnection {
    pub fn new(connection: lsp_server::Connection) -> Self {
        LspClientConnection { connection, req_id: 0 }
    }

    /// Generates a new unique request ID.
    pub fn next_id(&mut self) -> RequestId {
        self.req_id += 1;
        RequestId::from(self.req_id)
    }

    /// Serializes and sends a typed LSP request. Returns the request ID.
    /// The caller is responsible for receiving the response.
    pub fn begin_request<R: request::Request>(&mut self, params: R::Params) -> RequestId {
        let id = self.next_id();
        let params = serde_json::to_value(params).expect("failed to serialize request params");
        self.connection
            .sender
            .send(Message::Request(Request::new(id.clone(), R::METHOD.to_string(), params)))
            .expect("failed to send request");
        id
    }

    /// Serializes and sends a typed LSP notification.
    pub fn send_notification<N: notification::Notification>(&self, params: N::Params) {
        let params = serde_json::to_value(params).expect("failed to serialize notification params");
        self.connection
            .sender
            .send(Message::Notification(Notification::new(N::METHOD.to_string(), params)))
            .expect("failed to send notification");
    }

    /// Sends a response to a server-initiated request.
    pub fn send_response(&self, id: RequestId, result: Value) {
        self.connection
            .sender
            .send(Message::Response(Response { id, result: Some(result), error: None }))
            .expect("failed to send response");
    }

    /// Computes the response to a `workspace/configuration` request by performing
    /// dot-separated path lookup into `workspace_configuration`.
    pub fn compute_workspace_configuration(
        workspace_configuration: &Value,
        params: ConfigurationParams,
    ) -> Vec<Value> {
        params
            .items
            .iter()
            .map(|item| match &item.section {
                Some(section) => section
                    .split('.')
                    .try_fold(workspace_configuration, |config, key| config.get(key))
                    .cloned()
                    .unwrap_or(Value::Null),
                None => workspace_configuration.clone(),
            })
            .collect()
    }
}
