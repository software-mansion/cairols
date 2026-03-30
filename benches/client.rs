//! Shared LSP bench client used by all benchmark binaries.

#![allow(dead_code)]

use std::path::PathBuf;

use cairo_language_server::testing::BackendForTesting;
use lsp_server::{Message, Notification, Request, RequestId, Response};
use lsp_types::request::Request as _;
use lsp_types::{notification, request, *};
use serde_json::{Value, json};
use tempfile::TempDir;

/// A minimal LSP client for benchmarking.
///
/// Wraps an in-process `lsp_server::Connection` and handles the LSP protocol details
/// (initialize handshake, `workspace/configuration` responses, etc.) so benchmark
/// functions can focus on the operation being measured.
pub struct BenchClient {
    connection: lsp_server::Connection,
    req_id: i32,
    pub main_file_uri: Url,
    // Kept alive to prevent the temp directory from being deleted.
    _dir: TempDir,
}

impl BenchClient {
    /// Spins up a language server with `cairo_code` as `src/lib.cairo` and waits until the
    /// project finishes loading. Returns a client ready to send LSP requests.
    pub fn new(cairo_code: &str) -> Self {
        let dir = TempDir::new().unwrap();
        let root = dir.path();

        std::fs::write(
            root.join("cairo_project.toml"),
            "[crate_roots]\nhello = \"src\"\n\n[config.override.hello]\nedition = \"2025_12\"\n",
        )
        .unwrap();
        std::fs::create_dir(root.join("src")).unwrap();

        let main_path = root.join("src/lib.cairo");
        std::fs::write(&main_path, cairo_code).unwrap();
        let main_file_uri = Url::from_file_path(&main_path).unwrap();

        let (init, connection) = BackendForTesting::new();
        let cwd: PathBuf = root.into();
        let cwd_clone = cwd.clone();
        std::thread::spawn(move || init(cwd_clone).run_for_tests().unwrap());

        let mut client = BenchClient { connection, req_id: 0, main_file_uri, _dir: dir };

        client.initialize(&cwd);
        client.open_file(cairo_code);
        client.wait_for_project_update();
        client
    }

    fn next_id(&mut self) -> RequestId {
        self.req_id += 1;
        RequestId::from(self.req_id)
    }

    /// Performs the `initialize` / `initialized` handshake.
    fn initialize(&mut self, root: &PathBuf) {
        let root_uri = Url::from_directory_path(root).unwrap();

        self.send_request::<request::Initialize>(InitializeParams {
            process_id: Some(std::process::id()),
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: root_uri,
                name: "hello".to_string(),
            }]),
            capabilities: ClientCapabilities {
                workspace: Some(WorkspaceClientCapabilities {
                    configuration: Some(true),
                    ..Default::default()
                }),
                text_document: Some(TextDocumentClientCapabilities {
                    inlay_hint: Some(InlayHintClientCapabilities {
                        dynamic_registration: Some(false),
                        resolve_support: None,
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..InitializeParams::default()
        });

        // Acknowledge the `client/registerCapability` request the server sends before
        // `initialized` can be processed.
        self.send_notification::<notification::Initialized>(InitializedParams {});
    }

    /// Sends a `textDocument/didOpen` notification for the main file.
    fn open_file(&mut self, content: &str) {
        self.send_notification::<notification::DidOpenTextDocument>(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: self.main_file_uri.clone(),
                language_id: "cairo".to_string(),
                version: 0,
                text: content.to_string(),
            },
        });
    }

    /// Drains incoming messages until `cairo/projectUpdatingFinished` is received.
    fn wait_for_project_update(&mut self) {
        loop {
            match self.connection.receiver.recv().unwrap() {
                Message::Notification(n) if n.method == "cairo/projectUpdatingFinished" => break,
                Message::Request(req) => self.handle_server_request(&req),
                _ => {}
            }
        }
    }

    /// Responds to server-initiated requests (workspace/configuration, registerCapability, …).
    fn handle_server_request(&mut self, req: &Request) {
        match req.method.as_str() {
            request::WorkspaceConfiguration::METHOD => {
                let params: ConfigurationParams =
                    serde_json::from_value(req.params.clone()).unwrap();
                let result: Vec<Value> = params
                    .items
                    .iter()
                    .map(|item| match item.section.as_deref() {
                        Some("cairo1") => json!({
                            "enableProcMacros": false,
                            "enableLinter": false,
                        }),
                        _ => json!(null),
                    })
                    .collect();
                self.connection
                    .sender
                    .send(Message::Response(Response {
                        id: req.id.clone(),
                        result: Some(serde_json::to_value(result).unwrap()),
                        error: None,
                    }))
                    .unwrap();
            }
            request::RegisterCapability::METHOD => {
                self.connection
                    .sender
                    .send(Message::Response(Response {
                        id: req.id.clone(),
                        result: Some(json!(null)),
                        error: None,
                    }))
                    .unwrap();
            }
            _ => {}
        }
    }

    /// Sends a typed LSP request and returns the typed result.
    fn send_request<R: request::Request>(&mut self, params: R::Params) -> R::Result {
        let id = self.next_id();
        self.connection
            .sender
            .send(Message::Request(Request::new(
                id.clone(),
                R::METHOD.to_string(),
                serde_json::to_value(params).unwrap(),
            )))
            .unwrap();
        let value = self.recv_until_response(id).unwrap_or(Value::Null);
        serde_json::from_value(value).unwrap()
    }

    /// Sends a typed LSP notification.
    fn send_notification<N: notification::Notification>(&mut self, params: N::Params) {
        self.connection
            .sender
            .send(Message::Notification(Notification::new(
                N::METHOD.to_string(),
                serde_json::to_value(params).unwrap(),
            )))
            .unwrap();
    }

    /// Receives messages, dispatching server requests, until a `Response` with `id` arrives.
    fn recv_until_response(&mut self, id: RequestId) -> Option<Value> {
        loop {
            match self.connection.receiver.recv().unwrap() {
                Message::Response(resp) if resp.id == id => return resp.result,
                Message::Request(req) => self.handle_server_request(&req),
                _ => {}
            }
        }
    }

    pub fn completion(&mut self, line: u32, character: u32) -> Option<CompletionResponse> {
        self.send_request::<request::Completion>(CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        })
    }

    pub fn hover(&mut self, line: u32, character: u32) -> Option<Hover> {
        self.send_request::<request::HoverRequest>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
        })
    }

    pub fn goto_definition(&mut self, line: u32, character: u32) -> Option<GotoDefinitionResponse> {
        self.send_request::<request::GotoDefinition>(GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    pub fn references(&mut self, line: u32, character: u32) -> Option<Vec<Location>> {
        self.send_request::<request::References>(ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            context: ReferenceContext { include_declaration: true },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    pub fn document_highlight(
        &mut self,
        line: u32,
        character: u32,
    ) -> Option<Vec<DocumentHighlight>> {
        self.send_request::<request::DocumentHighlightRequest>(DocumentHighlightParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    pub fn formatting(&mut self) -> Option<Vec<TextEdit>> {
        self.send_request::<request::Formatting>(DocumentFormattingParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            options: FormattingOptions::default(),
            work_done_progress_params: Default::default(),
        })
    }

    pub fn semantic_tokens(&mut self) -> Option<SemanticTokensResult> {
        self.send_request::<request::SemanticTokensFullRequest>(SemanticTokensParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }

    pub fn inlay_hints(&mut self, range: Range) -> Option<Vec<InlayHint>> {
        self.send_request::<request::InlayHintRequest>(InlayHintParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            range,
            work_done_progress_params: Default::default(),
        })
    }

    pub fn code_actions(&mut self, line: u32, character: u32) -> Option<CodeActionResponse> {
        let pos = Position { line, character };
        self.send_request::<request::CodeActionRequest>(CodeActionParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            range: Range { start: pos, end: pos },
            context: CodeActionContext { diagnostics: vec![], only: None, trigger_kind: None },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        })
    }
}
