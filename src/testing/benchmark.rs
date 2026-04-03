use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Duration, Instant};

use lsp_server::{Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument,
    Notification as LspNotification,
};
use lsp_types::request::{
    Completion, GotoDefinition, HoverRequest, References, RegisterCapability, Request as LspRequest,
    SemanticTokensFullRequest,
};
use lsp_types::{
    ClientCapabilities, CompletionClientCapabilities, CompletionParams, Diagnostic,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    HoverClientCapabilities, HoverParams, Position, PublishDiagnosticsParams,
    ReferenceClientCapabilities, ReferenceContext, ReferenceParams,
    SemanticTokensClientCapabilities, SemanticTokensClientCapabilitiesRequests,
    SemanticTokensFullOptions, SemanticTokensParams, SemanticTokensResult,
    TextDocumentClientCapabilities, TextDocumentContentChangeEvent, TextDocumentIdentifier,
    TextDocumentItem, TextDocumentPositionParams, Url, WorkspaceClientCapabilities,
    WorkspaceFolder, lsp_notification, lsp_request,
};
use serde_json::{Value, json};

use crate::lsp::ext::testing::ProjectUpdatingFinished;
use crate::lsp::ext::testing_requests::{
    DatabaseSwapped, DatabaseSwappedParams, DumpBenchmarkSnapshot, DumpBenchmarkSnapshotParams,
    DumpBenchmarkSnapshotResponse, ForceDatabaseSwap, ForceDatabaseSwapResponse,
};
use crate::lsp::ext::{
    ServerStatus, ServerStatusEvent, ServerStatusParams, ShowMemoryUsage,
    ShowMemoryUsageResponse,
};
use crate::testing::BackendForTesting;

#[derive(Default)]
struct RequestIdGenerator {
    next_id: i32,
}

impl RequestIdGenerator {
    fn next(&mut self) -> RequestId {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id.into()
    }
}

pub struct BenchmarkClient {
    project_root: PathBuf,
    client: lsp_server::Connection,
    req_id: RequestIdGenerator,
    trace: Vec<Message>,
    workspace_configuration: Value,
    diagnostics: HashMap<Url, Vec<Diagnostic>>,
    document_versions: HashMap<Url, i32>,
    project_loaded: bool,
}

impl BenchmarkClient {
    pub fn start(project_root: PathBuf, workspace_configuration: Option<Value>) -> Self {
        let (init, client) = BackendForTesting::new();
        let cwd = project_root.clone();
        std::thread::spawn(|| init(cwd).run_for_tests());

        let mut this = Self {
            project_root,
            client,
            req_id: RequestIdGenerator::default(),
            trace: Vec::new(),
            workspace_configuration: workspace_configuration.unwrap_or_else(default_workspace_configuration),
            diagnostics: HashMap::new(),
            document_versions: HashMap::new(),
            project_loaded: false,
        };
        this.initialize();
        this
    }

    fn initialize(&mut self) {
        self.send_request::<lsp_request!("initialize")>(lsp_types::InitializeParams {
            process_id: Some(process::id()),
            capabilities: benchmark_capabilities(),
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: Url::from_directory_path(&self.project_root).expect("project root must be a directory"),
                name: self
                    .project_root
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("workspace")
                    .to_string(),
            }]),
            client_info: Some(lsp_types::ClientInfo {
                name: "cairols-bench".to_string(),
                version: Some("1.0.0".to_string()),
            }),
            locale: Some("en".to_string()),
            ..lsp_types::InitializeParams::default()
        });

        self.send_notification::<lsp_notification!("initialized")>(lsp_types::InitializedParams {});
    }

    pub fn send_request<R: lsp_types::request::Request>(&mut self, params: R::Params) -> R::Result {
        let params = serde_json::to_value(params).expect("failed to serialize request params");
        let result = self.send_request_untyped(R::METHOD, params);
        serde_json::from_value(result).expect("failed to parse request response")
    }

    pub fn send_request_untyped(&mut self, method: &'static str, params: Value) -> Value {
        const SERVER_CANCELLED: i32 = -32802;

        for attempt in 0..=2 {
            let id = self.req_id.next();
            let message =
                Message::Request(Request::new(id.clone(), method.to_string(), params.clone()));
            self.client.sender.send(message.clone()).expect("failed to send request");

            while let Some(response_message) =
                self.recv().unwrap_or_else(|err| panic!("{err}: {message:?}"))
            {
                if let Message::Response(response) = response_message {
                    assert_eq!(response.id, id, "request id mismatch");
                    match (response.result, response.error) {
                        (Some(result), None) => return result,
                        (_, Some(error)) if error.code == SERVER_CANCELLED && attempt < 2 => {
                            eprintln!(
                                "    request {method} cancelled by server on attempt {}, retrying after analysis settles",
                                attempt + 1
                            );
                            self.wait_for_post_cancel_recovery();
                            break;
                        }
                        (_, Some(error)) => panic!("error response: {error:?}"),
                        _ => panic!("invalid response without result or error"),
                    }
                }
            }

            if attempt == 2 {
                panic!("no response for request after retries: {message:?}");
            }
        }

        unreachable!("request retry loop must either return or panic")
    }

    pub fn send_notification<N: lsp_types::notification::Notification>(&mut self, params: N::Params) {
        let params = serde_json::to_value(params).expect("failed to serialize notification params");
        self.send_notification_untyped(N::METHOD, params);
    }

    pub fn send_notification_untyped(&mut self, method: &'static str, params: Value) {
        self.client
            .sender
            .send(Message::Notification(Notification::new(method.to_string(), params)))
            .expect("failed to send notification");
    }

    pub fn open(&mut self, path: impl AsRef<Path>) {
        let path = absolutize(&self.project_root, path.as_ref());
        let url = Url::from_file_path(&path).expect("path must be valid file path");
        let language_id =
            path.extension().and_then(|ext| ext.to_str()).unwrap_or_default().to_string();
        let text = std::fs::read_to_string(&path).expect("failed to read benchmark file");
        self.document_versions.insert(url.clone(), 0);
        self.send_notification::<DidOpenTextDocument>(DidOpenTextDocumentParams {
            text_document: TextDocumentItem { uri: url, language_id, version: 0, text },
        });
    }

    pub fn change(&mut self, path: impl AsRef<Path>, text: String) {
        let path = absolutize(&self.project_root, path.as_ref());
        let url = Url::from_file_path(&path).expect("path must be valid file path");
        let version = {
            let version = self.document_versions.entry(url.clone()).or_default();
            *version += 1;
            *version
        };
        self.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
            text_document: lsp_types::VersionedTextDocumentIdentifier {
                uri: url,
                version,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text,
            }],
        });
    }

    pub fn save(&mut self, path: impl AsRef<Path>) {
        let path = absolutize(&self.project_root, path.as_ref());
        let url = Url::from_file_path(path).expect("path must be valid file path");
        self.send_notification::<DidSaveTextDocument>(DidSaveTextDocumentParams {
            text_document: TextDocumentIdentifier { uri: url },
            text: None,
        });
    }

    pub fn close(&mut self, path: impl AsRef<Path>) {
        let path = absolutize(&self.project_root, path.as_ref());
        let url = Url::from_file_path(path).expect("path must be valid file path");
        self.send_notification::<DidCloseTextDocument>(lsp_types::DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri: url.clone() },
        });
        self.document_versions.remove(&url);
    }

    pub fn wait_for_project_update(&mut self) {
        eprintln!("    waiting for project update notification");
        self.wait_for_notification::<ProjectUpdatingFinished>(|_| true);
        eprintln!("    project update notification received");
        self.project_loaded = true;
    }

    pub fn wait_for_diagnostics_generation(&mut self) -> HashMap<Url, Vec<Diagnostic>> {
        eprintln!("    waiting for analysis finished");
        self.wait_for_analysis_finished();
        eprintln!("    analysis finished received, waiting for diagnostics quiescence");
        self.settle(Duration::from_millis(1_500));
        eprintln!("    diagnostics quiescence reached");
        self.diagnostics.clone()
    }

    pub fn memory_usage(&mut self) -> ShowMemoryUsageResponse {
        self.send_request::<ShowMemoryUsage>(())
    }

    pub fn dump_benchmark_snapshot(&mut self, label: impl Into<String>) -> DumpBenchmarkSnapshotResponse {
        self.send_request::<DumpBenchmarkSnapshot>(DumpBenchmarkSnapshotParams { label: label.into() })
    }

    pub fn force_database_swap(&mut self) -> ForceDatabaseSwapResponse {
        self.send_request::<ForceDatabaseSwap>(())
    }

    pub fn wait_for_database_swap(&mut self) -> DatabaseSwappedParams {
        eprintln!("    waiting for database swapped notification");
        let params = self.wait_for_notification::<DatabaseSwapped>(|_| true);
        eprintln!("    database swapped notification received: {}", params.reason);
        params
    }

    pub fn request_hover(
        &mut self,
        path: impl AsRef<Path>,
        position: Position,
    ) -> Option<lsp_types::Hover> {
        self.send_request::<HoverRequest>(HoverParams {
            text_document_position_params: self.text_document_position(path, position),
            work_done_progress_params: Default::default(),
        })
    }

    pub fn request_goto_definition(
        &mut self,
        path: impl AsRef<Path>,
        position: Position,
    ) -> Option<lsp_types::GotoDefinitionResponse> {
        self.send_request::<GotoDefinition>(lsp_types::GotoDefinitionParams {
            text_document_position_params: self.text_document_position(path, position),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    pub fn request_completion(
        &mut self,
        path: impl AsRef<Path>,
        position: Position,
    ) -> Option<lsp_types::CompletionResponse> {
        self.send_request::<Completion>(CompletionParams {
            text_document_position: self.text_document_position(path, position),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        })
    }

    pub fn request_references(
        &mut self,
        path: impl AsRef<Path>,
        position: Position,
    ) -> Option<Vec<lsp_types::Location>> {
        self.send_request::<References>(ReferenceParams {
            text_document_position: self.text_document_position(path, position),
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: ReferenceContext { include_declaration: true },
        })
    }

    pub fn request_semantic_tokens(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Option<SemanticTokensResult> {
        self.send_request::<SemanticTokensFullRequest>(SemanticTokensParams {
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            text_document: TextDocumentIdentifier {
                uri: Url::from_file_path(absolutize(&self.project_root, path.as_ref()))
                    .expect("path must be valid file path"),
            },
        })
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> String {
        let path = absolutize(&self.project_root, path.as_ref());
        std::fs::read_to_string(path).expect("failed to read benchmark file")
    }

    fn text_document_position(
        &self,
        path: impl AsRef<Path>,
        position: Position,
    ) -> TextDocumentPositionParams {
        let path = absolutize(&self.project_root, path.as_ref());
        TextDocumentPositionParams {
            text_document: TextDocumentIdentifier {
                uri: Url::from_file_path(path).expect("path must be valid file path"),
            },
            position,
        }
    }

    fn wait_for_notification<N>(&mut self, predicate: impl Fn(&N::Params) -> bool) -> N::Params
    where
        N: lsp_types::notification::Notification,
    {
        self.wait_for_message(|message| {
            let Message::Notification(notification) = message else { return None };
            if notification.method != N::METHOD {
                return None;
            }
            let params = serde_json::from_value(notification.params.clone())
                .expect("failed to parse notification params");
            predicate(&params).then_some(params)
        })
        .unwrap_or_else(|err| panic!("waiting for notification failed: {err}"))
    }

    fn wait_for_analysis_finished(&mut self) {
        eprintln!("      awaiting cairo/serverStatus AnalysisFinished");
        self.wait_for_notification::<ServerStatus>(|params| {
            params.event == ServerStatusEvent::AnalysisFinished
        });
        eprintln!("      received cairo/serverStatus AnalysisFinished");
    }

    fn wait_for_post_cancel_recovery(&mut self) {
        eprintln!("      waiting for post-cancel recovery");

        if self
            .wait_for_server_status_event(
                ServerStatusEvent::AnalysisFinished,
                Duration::from_secs(2),
            )
            .unwrap_or_else(|err| panic!("waiting for analysis recovery failed: {err}"))
        {
            eprintln!("      observed AnalysisFinished after cancellation");
            self.settle(Duration::from_millis(500));
            return;
        }

        if self
            .wait_for_server_status_event(
                ServerStatusEvent::AnalysisStarted,
                Duration::from_secs(2),
            )
            .unwrap_or_else(|err| panic!("waiting for analysis recovery failed: {err}"))
        {
            eprintln!("      observed AnalysisStarted after cancellation");

            if self
                .wait_for_server_status_event(
                    ServerStatusEvent::AnalysisFinished,
                    Duration::from_secs(180),
                )
                .unwrap_or_else(|err| panic!("waiting for analysis recovery failed: {err}"))
            {
                eprintln!("      observed AnalysisFinished after cancellation");
            } else {
                eprintln!(
                    "      no AnalysisFinished arrived after AnalysisStarted, proceeding after idle settle"
                );
            }
        } else {
            eprintln!("      no serverStatus recovery event observed, proceeding after idle settle");
        }

        self.settle(Duration::from_millis(500));
    }

    fn wait_for_server_status_event(
        &mut self,
        expected: ServerStatusEvent,
        timeout: Duration,
    ) -> Result<bool, String> {
        if self.try_take_server_status_event(&expected) {
            return Ok(true);
        }

        let deadline = Instant::now() + timeout;
        loop {
            let now = Instant::now();
            if now >= deadline {
                return Ok(false);
            }

            let remaining = deadline.saturating_duration_since(now);
            let poll_timeout = remaining.min(Duration::from_millis(250));
            match self.recv_with_timeout(poll_timeout) {
                Ok(Some(Message::Notification(Notification { method, params })))
                    if method == <ServerStatus as LspNotification>::METHOD =>
                {
                    let status: ServerStatusParams =
                        serde_json::from_value(params)
                            .expect("failed to parse serverStatus notification");
                    if status.event == expected {
                        self.trace.pop();
                        return Ok(true);
                    }
                }
                Ok(Some(_)) => continue,
                Ok(None) => return Ok(false),
                Err(err) if err == "timed out waiting for server message" => return Ok(false),
                Err(err) => return Err(err),
            }
        }
    }

    fn try_take_server_status_event(&mut self, expected: &ServerStatusEvent) -> bool {
        for (index, message) in self.trace.iter().enumerate() {
            let Message::Notification(Notification { method, params }) = message else {
                continue;
            };
            if method != <ServerStatus as LspNotification>::METHOD {
                continue;
            }
            let status: ServerStatusParams =
                serde_json::from_value(params.clone())
                    .expect("failed to parse serverStatus notification");
            if &status.event == expected {
                self.trace.remove(index);
                return true;
            }
        }

        false
    }

    fn wait_for_message<T>(
        &mut self,
        predicate: impl Fn(&Message) -> Option<T>,
    ) -> Result<T, String> {
        for (index, message) in self.trace.iter().enumerate() {
            if let Some(value) = predicate(message) {
                self.trace.remove(index);
                return Ok(value);
            }
        }

        loop {
            let message = self.recv()?;
            let Some(message) = message else { return Err("connection closed".to_string()) };
            if let Some(value) = predicate(&message) {
                self.trace.pop();
                return Ok(value);
            }
        }
    }

    fn recv(&mut self) -> Result<Option<Message>, String> {
        self.recv_with_timeout(Duration::from_secs(180))
    }

    fn recv_with_timeout(&mut self, timeout: Duration) -> Result<Option<Message>, String> {
        let message = match self.client.receiver.recv_timeout(timeout) {
            Ok(msg) => Some(msg),
            Err(crossbeam::channel::RecvTimeoutError::Disconnected) => None,
            Err(crossbeam::channel::RecvTimeoutError::Timeout) => {
                return Err("timed out waiting for server message".to_string());
            }
        };

        if let Some(message_ref) = &message {
            self.trace.push(message_ref.clone());

            if let Message::Request(request) = message_ref {
                self.auto_respond(request);
            }
            if let Message::Notification(Notification { method, params }) = message_ref
                && method == "textDocument/publishDiagnostics"
            {
                let params: PublishDiagnosticsParams = serde_json::from_value(params.clone())
                    .expect("failed to parse diagnostics notification");
                self.diagnostics.insert(params.uri, params.diagnostics);
            }
        }

        Ok(message)
    }

    fn settle(&mut self, idle_window: Duration) {
        let mut seen_messages = 0usize;
        loop {
            match self.recv_with_timeout(idle_window) {
                Ok(Some(message)) => {
                    seen_messages += 1;
                    if let Message::Notification(Notification { method, .. }) = &message {
                        eprintln!("      settle observed notification: {method}");
                    }
                    continue;
                }
                Ok(None) => {
                    eprintln!("      settle ended: connection closed after {seen_messages} messages");
                    return;
                }
                Err(err) if err == "timed out waiting for server message" => {
                    eprintln!("      settle ended: idle timeout after {seen_messages} messages");
                    return;
                }
                Err(err) => panic!("failed while waiting for benchmark quiescence: {err}"),
            }
        }
    }

    fn auto_respond(&mut self, request: &Request) {
        match request.method.as_str() {
            <lsp_request!("workspace/configuration")>::METHOD => {
                let id = request.id.clone();
                let params = serde_json::from_value(request.params.clone())
                    .expect("failed to parse workspace/configuration params");
                let result = serde_json::to_value(self.compute_workspace_configuration(params))
                    .expect("failed to serialize workspace/configuration response");
                self.client
                    .sender
                    .send(Message::Response(Response::new_ok(id, result)))
                    .expect("failed to send workspace/configuration response");
            }
            RegisterCapability::METHOD => {
                self.client
                    .sender
                    .send(Message::Response(Response::new_ok(request.id.clone(), Value::Null)))
                    .expect("failed to send registerCapability response");
            }
            method => panic!("unhandled server request during benchmark: {method}"),
        }
    }

    fn compute_workspace_configuration(
        &self,
        params: <lsp_request!("workspace/configuration") as LspRequest>::Params,
    ) -> Vec<Value> {
        params
            .items
            .iter()
            .map(|item| {
                item.section
                    .as_deref()
                    .and_then(|section| {
                        section
                            .split('.')
                            .try_fold(&self.workspace_configuration, |config, key| config.get(key))
                    })
                    .cloned()
                    .unwrap_or(Value::Null)
            })
            .collect()
    }
}

fn benchmark_capabilities() -> ClientCapabilities {
    ClientCapabilities {
        workspace: Some(WorkspaceClientCapabilities {
            configuration: Some(true),
            ..WorkspaceClientCapabilities::default()
        }),
        text_document: Some(TextDocumentClientCapabilities {
            hover: Some(HoverClientCapabilities::default()),
            completion: Some(CompletionClientCapabilities::default()),
            references: Some(ReferenceClientCapabilities::default()),
            definition: Some(lsp_types::GotoCapability::default()),
            semantic_tokens: Some(SemanticTokensClientCapabilities {
                requests: SemanticTokensClientCapabilitiesRequests {
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    ..Default::default()
                },
                ..Default::default()
            }),
            synchronization: Some(lsp_types::TextDocumentSyncClientCapabilities {
                did_save: Some(true),
                ..Default::default()
            }),
            ..TextDocumentClientCapabilities::default()
        }),
        ..ClientCapabilities::default()
    }
}

fn default_workspace_configuration() -> Value {
    json!({
        "cairo1": {
            "enableProcMacros": false,
            "enableLinter": false
        }
    })
}

fn absolutize(project_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        project_root.join(path)
    }
}
