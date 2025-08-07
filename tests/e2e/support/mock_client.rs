use std::collections::HashMap;
use std::ffi::OsStr;
use std::ops::ControlFlow;
use std::path::Path;
use std::time::Duration;
use std::{fmt, process};

use cairo_language_server::lsp::ext::ServerStatusEvent::{AnalysisFinished, AnalysisStarted};
use cairo_language_server::lsp::ext::ServerStatusParams;
use cairo_language_server::lsp::ext::testing::ProjectUpdatingFinished;
use cairo_language_server::testing::BackendForTesting;
use lsp_server::{Message, Notification, Request, Response};
use lsp_types::notification::PublishDiagnostics;
use lsp_types::request::{RegisterCapability, Request as LspRequest};
use lsp_types::{Diagnostic, PublishDiagnosticsParams, Url, lsp_notification, lsp_request};
use serde_json::Value;

use crate::support::fixture::Fixture;
use crate::support::jsonrpc::RequestIdGenerator;
use crate::support::mock_client::Action::{NoOp, RemoveFromTrace};

/// A mock language client implementation that facilitates end-to-end testing language servers.
///
/// ## Termination
///
/// The language server is terminated abruptly upon dropping of this struct.
/// The `shutdown` request and `exit` notifications are not sent at all.
/// Instead, the thread executing the server is being shut down and any running
/// blocking tasks are given a small period of time to complete.
pub struct MockClient {
    pub fixture: Fixture,
    // Keeps last diagnostics generation for each file
    diagnostics: HashMap<Url, Vec<Diagnostic>>,
    req_id: RequestIdGenerator,
    client: lsp_server::Connection,
    trace: Vec<Message>,
    workspace_configuration: Value,
    // Mapping between request name and request handler
    // This enforces only one request matcher per method at a time
    // (can be extended eventually to a custom matcher logic with param parsing etc.)
    expect_request_handlers: HashMap<String, ExpectRequestHandler>,
}

impl MockClient {
    /// Starts and initializes CairoLS in the context of a given fixture and given client
    /// capabilities.
    ///
    /// Upon completion of this function, the language server will be in the _initialized_ state
    /// (i.e., the `initialize` request and `initialized` notification both will be completed).
    #[must_use]
    pub fn start(
        mut fixture: Fixture,
        capabilities: lsp_types::ClientCapabilities,
        workspace_configuration: Value,
        cwd: Option<impl AsRef<Path>>,
    ) -> Self {
        let (init, client) = BackendForTesting::new();

        fixture.update_insta_settings();
        let root_path = fixture.root_path();
        let cwd = cwd.map(|cwd| root_path.join(cwd)).unwrap_or_else(|| root_path.join("./"));

        let mut this = Self {
            fixture,
            client,
            req_id: RequestIdGenerator::default(),
            trace: Vec::new(),
            workspace_configuration,
            expect_request_handlers: Default::default(),
            diagnostics: Default::default(),
        };

        std::thread::spawn(|| init(cwd).run_for_tests());

        this.initialize(capabilities);

        this
    }

    /// Performs the `initialize`/`initialized` handshake with the server synchronously.
    fn initialize(&mut self, capabilities: lsp_types::ClientCapabilities) {
        let workspace_folders = Some(vec![lsp_types::WorkspaceFolder {
            uri: self.fixture.root_url(),
            name: "hello_world".to_string(),
        }]);

        self.send_request::<lsp_request!("initialize")>(lsp_types::InitializeParams {
            process_id: Some(process::id()),
            capabilities,
            workspace_folders,
            client_info: Some(lsp_types::ClientInfo {
                name: "e2e".to_string(),
                version: Some("1.0.0".to_string()),
            }),
            locale: Some("en".to_string()),
            ..lsp_types::InitializeParams::default()
        });

        self.expect_request::<RegisterCapability>(|_req| {});

        self.send_notification::<lsp_notification!("initialized")>(lsp_types::InitializedParams {});
    }

    /// Sends a typed request to the server.
    pub fn send_request<R: lsp_types::request::Request>(&mut self, params: R::Params) -> R::Result {
        let params = serde_json::to_value(params).expect("failed to serialize request params");
        let result = self.send_request_untyped(R::METHOD, params);
        serde_json::from_value(result).expect("failed to parse response")
    }

    /// Sends an arbitrary request to the server.
    pub fn send_request_untyped(&mut self, method: &'static str, params: Value) -> Value {
        let id = self.req_id.next();
        let message = Message::Request(Request::new(id.clone(), method.to_owned(), params));

        self.client.sender.send(message.clone()).expect("failed to send request");

        while let Some(response_message) =
            self.recv().unwrap_or_else(|err| panic!("{err:?}: {message:?}"))
        {
            if let Message::Response(res) = response_message {
                let res_id = res.id;
                let result = res.result.ok_or_else(|| res.error.unwrap());

                assert_eq!(res_id, id);

                match result {
                    Ok(result) => return result,
                    Err(err) => panic!("error response: {err:#?}"),
                }
            }
        }

        panic!("no response for request: {message:?}")
    }

    /// Sends a typed notification to the server.
    pub fn send_notification<N: lsp_types::notification::Notification>(
        &mut self,
        params: N::Params,
    ) {
        let params = serde_json::to_value(params).expect("failed to serialize notification params");
        self.send_notification_untyped(N::METHOD, params)
    }

    /// Sends an arbitrary notification to the server.
    pub fn send_notification_untyped(&mut self, method: &'static str, params: Value) {
        let message = Message::Notification(Notification::new(method.to_string(), params));
        self.client.sender.send(message).expect("failed to send notification");
    }

    fn get_handler_for(&mut self, request_name: &String) -> Option<ExpectRequestHandler> {
        self.expect_request_handlers.remove(request_name)
    }
}

/// Introspection.
impl MockClient {
    /// Gets a list of messages received from the server.
    pub fn trace(&self) -> &[Message] {
        &self.trace
    }

    /// Gets state of diagnostics from previous round for the given file
    pub fn get_diagnostics_for_file(&self, path: impl AsRef<Path>) -> Vec<Diagnostic> {
        let url = self.fixture.file_url(&path);
        self.diagnostics.get(&url).cloned().unwrap_or_default()
    }
}

#[derive(Debug)]
enum RecvError {
    Timeout,
    NoMessage,
}

/// Receiving messages.
impl MockClient {
    /// Receives a message from the server.
    fn recv(&mut self) -> Result<Option<Message>, RecvError> {
        const TIMEOUT: Duration = Duration::from_secs(3 * 60);
        let message = match self.client.receiver.recv_timeout(TIMEOUT) {
            Ok(msg) => Some(msg),
            Err(crossbeam::channel::RecvTimeoutError::Disconnected) => None,
            Err(crossbeam::channel::RecvTimeoutError::Timeout) => return Err(RecvError::Timeout),
        };

        if let Some(message) = &message {
            self.trace.push(message.clone());

            if let Message::Request(request) = &message {
                if request.method == <lsp_request!("workspace/configuration")>::METHOD {
                    self.auto_respond_to_workspace_configuration_request(request);
                } else if let Some(handler) = self.get_handler_for(&request.method) {
                    let response = (handler.f)(request);
                    let message = Message::Response(response);
                    self.client.sender.send(message).expect("failed to send response");
                } else {
                    panic!("unhandled request {request:?}");
                }
            }
            if let Message::Notification(Notification { method, params }) = &message
                && method.as_str() == "textDocument/publishDiagnostics"
            {
                let params: PublishDiagnosticsParams = serde_json::from_value(params.clone())
                    .expect("failed to parse `textDocument/publishDiagnostics` params");
                self.diagnostics.insert(params.uri, params.diagnostics);
            }
        }

        Ok(message)
    }

    /// Looks for a message that satisfies the given predicate in message trace and removes it from
    /// it or waits for a new one.
    fn wait_for_message<T>(
        &mut self,
        predicate: impl Fn(&Message) -> Option<T>,
    ) -> Result<T, RecvError> {
        for (index, message) in self.trace.iter().enumerate() {
            if let Some(ret) = predicate(message) {
                self.trace.remove(index);
                return Ok(ret);
            }
        }

        loop {
            let message = self.recv()?.ok_or(RecvError::NoMessage)?;
            if let Some(ret) = predicate(&message) {
                self.trace.pop();
                return Ok(ret);
            }
        }
    }

    /// Looks for a client JSON-RPC request that satisfies the given predicate in message trace
    /// or waits for a new one.
    fn wait_for_rpc_notification<T>(
        &mut self,
        predicate: impl Fn(&lsp_server::Notification) -> Option<T>,
    ) -> T {
        self.wait_for_message(|message| {
            let Message::Notification(notification) = message else { return None };
            predicate(notification)
        })
        .unwrap_or_else(|err| panic!("waiting for request failed: {err:?}"))
    }

    /// Looks for a typed client notification that satisfies the given predicate in message trace
    /// or waits for a new one.
    pub fn wait_for_notification<N>(&mut self, predicate: impl Fn(&N::Params) -> bool) -> N::Params
    where
        N: lsp_types::notification::Notification,
    {
        self.wait_for_rpc_notification(|notification| {
            if notification.method != N::METHOD {
                return None;
            }
            let params = serde_json::from_value(notification.params.clone())
                .expect("failed to parse notification params");
            predicate(&params).then_some(params)
        })
    }

    fn wait_for_notification_sequence(
        &mut self,
        notification_matchers: Vec<Box<NotificationMatcher>>,
    ) {
        // Block which checks if the notification matches the next expected one in the sequence
        let try_advance_sequence = |message: &Message, current_seq: &mut usize| {
            let mut advanced = false;
            if *current_seq < notification_matchers.len()
                && let Message::Notification(notification) = message
                && notification_matchers[*current_seq](notification)
            {
                *current_seq += 1;
                advanced = true;
            }
            advanced
        };

        let mut seq: usize = 0;
        // Check for sequence in existing trace
        let mut message_matched = vec![];
        for message in self.trace.iter() {
            if try_advance_sequence(message, &mut seq) {
                message_matched.push(true);
            } else {
                message_matched.push(false);
            }
        }

        // Remove matched messages from the trace
        self.trace = self
            .trace
            .iter()
            .zip(message_matched.iter())
            .filter_map(|(msg, matched)| if *matched { None } else { Some(msg.clone()) })
            .collect();

        // Check if sequence wasn't whole in the trace
        if seq == notification_matchers.len() {
            return;
        }

        // Wait for next messages in sequence
        while let Some(message) = self.recv().expect("No message received") {
            if try_advance_sequence(&message, &mut seq) {
                self.trace.pop();
                if seq == notification_matchers.len() {
                    return;
                }
            }
        }
    }
}

type NotificationMatcher = dyn Fn(&Notification) -> bool;

/// Methods for handling interactive requests.
impl MockClient {
    /// Expect a specified request to be received from the server while processing the next client
    /// request.
    ///
    /// The handler is expected to return a response to the caught request.
    /// Handler can validate the request by asserting its parameters.
    /// Calls to this method can be stacked sequentially, to expect a sequence of requests being
    /// received from the server.
    pub fn expect_request<R>(&mut self, handler: impl FnOnce(&R::Params) -> R::Result + 'static)
    where
        R: lsp_types::request::Request,
    {
        self.expect_request_untyped(R::METHOD, move |req| {
            assert_eq!(req.method, R::METHOD);

            let id = req.id.clone();

            let params =
                serde_json::from_value(req.params.clone()).expect("failed to parse request params");
            let result = handler(&params);
            let result = serde_json::to_value(result).expect("failed to serialize response");

            lsp_server::Response::new_ok(id, result)
        })
    }

    /// Untyped version of [`MockClient::expect_request`].
    ///
    /// The `description` parameter is used in panic messages to tell that this handler did not
    /// fire. Usually it is enough to put request method name here.
    pub fn expect_request_untyped(
        &mut self,
        method: &'static str,
        handler: impl FnOnce(&Request) -> Response + 'static,
    ) {
        self.expect_request_handlers.insert(
            method.to_string(),
            ExpectRequestHandler { description: method, f: Box::new(handler) },
        );
    }
}

type StreamConsumer<Elements> = Box<dyn FnMut(Elements) -> ControlFlow<(), Action>>;
enum Action {
    RemoveFromTrace,
    NoOp,
}

/// Quality of life helpers for interacting with the server.
impl MockClient {
    /// Returns a `TextDocumentIdentifier` for the given file.
    pub fn doc_id(&self, path: impl AsRef<Path>) -> lsp_types::TextDocumentIdentifier {
        lsp_types::TextDocumentIdentifier { uri: self.fixture.file_url(path) }
    }

    /// Sends `textDocument/didOpen` notification to the server.
    pub fn open(&mut self, path: impl AsRef<Path>) {
        // Poor man's attempt at guessing the language ID
        // by assuming that file extension represents it.
        let language_id = self
            .fixture
            .file_absolute_path(&path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .to_string();

        self.send_notification::<lsp_notification!("textDocument/didOpen")>(
            lsp_types::DidOpenTextDocumentParams {
                text_document: lsp_types::TextDocumentItem {
                    uri: self.fixture.file_url(&path),
                    language_id,
                    version: 0,
                    text: self.fixture.read_file(&path),
                },
            },
        );
    }

    /// Overrides the file's content. The caller is responsible for sending appropriate notification
    /// to the server after calling this method, e.g. `DidChangeWatchedFiles`, as well as setting
    /// appropriate client capabilities at the beginning of the test.
    pub fn edit_file(&mut self, path: impl AsRef<Path>, contents: &str) {
        self.fixture.edit_file(path, contents);
    }

    /// Sends `textDocument/didOpen` notification to the server and
    /// waits for `cairo/projectUpdatingFinished` to be sent.
    pub fn open_and_wait_for_project_update(&mut self, path: impl AsRef<Path>) {
        self.open(path);
        self.wait_for_project_update();
    }

    /// Sends `textDocument/didOpen` notification to the server and then waits for
    /// `cairo/projectUpdatingFinished` and the next `textDocument/publishDiagnostics` for a given
    /// path.
    pub fn open_and_wait_for_diagnostics(&mut self, path: impl AsRef<Path>) -> Vec<Diagnostic> {
        let path = path.as_ref();
        let file_url = self.fixture.file_url(path);
        self.open(path);
        self.wait_for_notification_sequence(vec![
            Box::new(|notification: &Notification| {
                notification.method == "cairo/projectUpdatingFinished"
            }),
            Box::new(move |notification: &Notification| {
                if notification.method == "textDocument/publishDiagnostics" {
                    let params: PublishDiagnosticsParams =
                        serde_json::from_value(notification.params.clone()).unwrap();

                    return params.uri == file_url;
                }
                false
            }),
        ]);
        self.get_diagnostics_for_file(path)
    }

    /// Opens the file and waits until all procmacros, and related diagnostics get resolved within
    /// one generation span (until AnalysisStarted + AnalysisFinished get emitted via
    /// `cairo/serverStatus` notification).
    pub fn open_and_wait_for_diagnostics_generation(
        &mut self,
        path: impl AsRef<Path>,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        self.open(path);
        self.wait_for_diagnostics_generation()
    }

    /// Opens each `*.cairo` file in the test fixture and waits until all procmacros,
    /// and related diagnostics get resolved within one generation span (until
    /// AnalysisStarted + AnalysisFinished get emitted via `cairo/serverStatus` notification).
    pub fn open_all_and_wait_for_diagnostics_generation(
        &mut self,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        let cairo_files = self
            .fixture
            .files()
            .iter()
            .filter(|file| file.extension().is_some_and(|ext| ext == "cairo"))
            .cloned()
            .collect::<Vec<_>>();

        for file in &cairo_files {
            self.open(file);
        }

        self.wait_for_diagnostics_generation()
    }

    /// Waits until all procmacros, and related diagnostics get resolved within
    /// one generation span (until AnalysisStarted + AnalysisFinished get emitted via
    /// `cairo/serverStatus` notification).
    fn wait_for_diagnostics_generation(&mut self) -> HashMap<Url, Vec<Diagnostic>> {
        let mut in_progress_open = false;
        let mut project_updated = false;

        self.subscribe_notifications(Box::new(
            move |notification: Notification| match notification.method.as_str() {
                "cairo/serverStatus" => {
                    let params: ServerStatusParams =
                        serde_json::from_value(notification.params).unwrap();

                    match params.event {
                        AnalysisStarted => {
                            in_progress_open = true;
                            ControlFlow::Continue(RemoveFromTrace)
                        }
                        AnalysisFinished => {
                            if in_progress_open && project_updated {
                                ControlFlow::Break(())
                            } else {
                                ControlFlow::Continue(RemoveFromTrace)
                            }
                        }
                    }
                }
                "cairo/projectUpdatingFinished" => {
                    project_updated = true;
                    ControlFlow::Continue(RemoveFromTrace)
                }
                _ => ControlFlow::Continue(NoOp),
            },
        ));

        // It should contain all accumulated diagnostics up to the end of the generation.
        self.diagnostics.clone()
    }

    fn subscribe_notifications(&mut self, mut stream_consumer: StreamConsumer<Notification>) {
        let mut already_removed = 0;
        for (i, msg) in self.trace.clone().into_iter().enumerate() {
            if let Message::Notification(notification) = msg {
                match stream_consumer(notification) {
                    ControlFlow::Continue(action) => match action {
                        RemoveFromTrace => {
                            self.trace.remove(i - already_removed);
                            already_removed += 1;
                        }
                        NoOp => {}
                    },
                    ControlFlow::Break(()) => {
                        self.trace.remove(i - already_removed);
                        return;
                    }
                };
            }
        }

        while let Some(msg) = self.recv().expect("Cannot read from the server") {
            if let Message::Notification(notification) = msg {
                match stream_consumer(notification.clone()) {
                    ControlFlow::Continue(action) => match action {
                        RemoveFromTrace => {
                            self.trace.pop();
                        }
                        NoOp => {}
                    },
                    ControlFlow::Break(()) => {
                        self.trace.pop();
                        return;
                    }
                };
            }
        }
    }

    /// Waits for `textDocument/publishDiagnostics` notification for the given file.
    pub fn wait_for_diagnostics(&mut self, path: impl AsRef<Path>) -> Vec<Diagnostic> {
        let url = self.fixture.file_url(path);
        self.wait_for_notification::<PublishDiagnostics>(|params: &PublishDiagnosticsParams| {
            params.uri == url
        })
        .diagnostics
    }

    /// Waits for `cairo/projectUpdatingFinished` notification.
    pub fn wait_for_project_update(&mut self) {
        self.wait_for_notification::<ProjectUpdatingFinished>(|_| true);
    }

    /// Sends `textDocument/didChange` notification to the server for each `*.cairo` file in test
    /// fixture and then waits for all corresponding `cairo/projectUpdatingFinished` notifications.
    pub fn open_all_cairo_files_and_wait_for_project_update(&mut self) {
        let cairo_files = self
            .fixture
            .files()
            .iter()
            .filter(|file| file.extension().is_some_and(|ext| ext == "cairo"))
            .cloned()
            .collect::<Vec<_>>();

        for file in &cairo_files {
            self.open(file);
        }

        for _ in &cairo_files {
            self.wait_for_notification::<ProjectUpdatingFinished>(|_| true);
        }
    }
}

/// Handling workspace configuration workflow.
impl MockClient {
    /// Assuming `request` is a `workspace/configuration` request, computes and sends a response to
    /// it.
    fn auto_respond_to_workspace_configuration_request(&mut self, request: &lsp_server::Request) {
        assert_eq!(request.method, <lsp_request!("workspace/configuration") as LspRequest>::METHOD);

        let id = request.id.clone();

        let params = serde_json::from_value(request.params.clone())
            .expect("failed to parse `workspace/configuration` params");

        let result = self.compute_workspace_configuration(params);

        let result = serde_json::to_value(result)
            .expect("failed to serialize `workspace/configuration` response");

        let message = Message::Response(Response::new_ok(id, result));
        self.client
            .sender
            .send(message)
            .expect("failed to send `workspace/configuration` response");
    }

    /// Computes response to `workspace/configuration` request.
    fn compute_workspace_configuration(
        &self,
        params: <lsp_request!("workspace/configuration") as LspRequest>::Params,
    ) -> Vec<Value> {
        params
            .items
            .iter()
            .map(|item| {
                // NOTE: `scope_uri` is ignored.
                match &item.section {
                    Some(section) => {
                        // Items may ask for nested entries, with dot being the path separator.
                        section
                            .split('.')
                            .try_fold(&self.workspace_configuration, |config, key| config.get(key))
                            .cloned()
                            .unwrap_or(Value::Null)
                    }
                    None => self.workspace_configuration.clone(),
                }
            })
            .collect()
    }
}

impl AsRef<Fixture> for MockClient {
    fn as_ref(&self) -> &Fixture {
        &self.fixture
    }
}

/// A container for callbacks passed to [`MockClient::expect_request`] that also carries a text
/// telling what this callback expects.
///
/// The description is used in panic messages.
struct ExpectRequestHandler {
    description: &'static str,
    f: Box<dyn FnOnce(&lsp_server::Request) -> lsp_server::Response>,
}

impl fmt::Debug for ExpectRequestHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.description, f)
    }
}
