//! Shared LSP bench client used by all benchmark binaries.

#![allow(dead_code)]

use std::path::PathBuf;

use cairo_language_server::testing::BackendForTesting;
use lsp_server::{Message, Notification, Request, RequestId, Response};
use lsp_types::*;
use serde_json::{Value, json};
use tempfile::TempDir;

const TOOL_VERSIONS: &str = include_str!("../.tool-versions");

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

        std::fs::write(root.join(".tool-versions"), TOOL_VERSIONS).unwrap();
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
        let id = self.next_id();

        let params = InitializeParams {
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
        };

        self.send_raw(id.clone(), "initialize", serde_json::to_value(params).unwrap());
        self.recv_until_response(id);

        // Acknowledge the `client/registerCapability` request the server sends before
        // `initialized` can be processed.
        self.connection
            .sender
            .send(Message::Notification(Notification::new("initialized".to_string(), json!({}))))
            .unwrap();
    }

    /// Sends a `textDocument/didOpen` notification for the main file.
    fn open_file(&mut self, content: &str) {
        let params = DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: self.main_file_uri.clone(),
                language_id: "cairo".to_string(),
                version: 0,
                text: content.to_string(),
            },
        };
        self.connection
            .sender
            .send(Message::Notification(Notification::new(
                "textDocument/didOpen".to_string(),
                serde_json::to_value(params).unwrap(),
            )))
            .unwrap();
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
            "workspace/configuration" => {
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
            "client/registerCapability" => {
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

    fn send_raw(&mut self, id: RequestId, method: &str, params: Value) {
        self.connection
            .sender
            .send(Message::Request(Request::new(id, method.to_string(), params)))
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
        let id = self.next_id();
        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        };
        self.send_raw(id.clone(), "textDocument/completion", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).map(|v| serde_json::from_value(v).unwrap())
    }

    pub fn hover(&mut self, line: u32, character: u32) -> Option<Hover> {
        let id = self.next_id();
        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
        };
        self.send_raw(id.clone(), "textDocument/hover", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn goto_definition(&mut self, line: u32, character: u32) -> Option<GotoDefinitionResponse> {
        let id = self.next_id();
        let params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(id.clone(), "textDocument/definition", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn references(&mut self, line: u32, character: u32) -> Option<Vec<Location>> {
        let id = self.next_id();
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            context: ReferenceContext { include_declaration: true },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(id.clone(), "textDocument/references", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn document_highlight(
        &mut self,
        line: u32,
        character: u32,
    ) -> Option<Vec<DocumentHighlight>> {
        let id = self.next_id();
        let params = DocumentHighlightParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/documentHighlight",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn formatting(&mut self) -> Option<Vec<TextEdit>> {
        let id = self.next_id();
        let params = DocumentFormattingParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            options: FormattingOptions::default(),
            work_done_progress_params: Default::default(),
        };
        self.send_raw(id.clone(), "textDocument/formatting", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn semantic_tokens(&mut self) -> Option<SemanticTokensResult> {
        let id = self.next_id();
        let params = SemanticTokensParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/semanticTokens/full",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn inlay_hints(&mut self, range: Range) -> Option<Vec<InlayHint>> {
        let id = self.next_id();
        let params = InlayHintParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            range,
            work_done_progress_params: Default::default(),
        };
        self.send_raw(id.clone(), "textDocument/inlayHint", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    pub fn code_actions(&mut self, line: u32, character: u32) -> Option<CodeActionResponse> {
        let id = self.next_id();
        let pos = Position { line, character };
        let params = CodeActionParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            range: Range { start: pos, end: pos },
            context: CodeActionContext { diagnostics: vec![], only: None, trigger_kind: None },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(id.clone(), "textDocument/codeAction", serde_json::to_value(params).unwrap());
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }
}
