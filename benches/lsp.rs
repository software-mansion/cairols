//! End-to-end LSP benchmarks.
//!
//! Each benchmark spins up a real language server in a temp directory, performs the full LSP
//! initialize handshake, opens a Cairo source file, and waits for the project to load. Only then
//! does the timed loop begin, so setup cost is excluded from the measurements.
//!
//! Run with:
//!   cargo bench
//! or for a single group:
//!   cargo bench --bench lsp completions

use std::path::PathBuf;
use std::time::Duration;

use cairo_language_server::testing::BackendForTesting;
use criterion::{Criterion, criterion_group, criterion_main};
use lsp_server::{Message, Notification, Request, RequestId, Response};
use lsp_types::*;
use serde_json::{Value, json};
use tempfile::TempDir;

const TOOL_VERSIONS: &str = include_str!("../.tool-versions");

// ── Cairo fixtures ────────────────────────────────────────────────────────────

/// Simple file used for completion benchmarks.
const COMPLETION_CAIRO: &str = r#"
fn foo(value: u32) -> u32 {
    let doubled = value * 2;
    let result = doubled + value;
    result
}
"#;

/// File used for hover benchmarks — imports a stdlib function so hover hits real semantic data.
///
/// Line map (0-indexed):
///   0: ""
///   1: "use core::integer::u32_sqrt;"
///   2: ""
///   3: "fn compute(x: u32, y: u32) -> u32 {"
///   4: "    let sum = x + y;"
///   5: "    u32_sqrt(sum)"
///   6: "}"
const HOVER_CAIRO: &str = r#"
use core::integer::u32_sqrt;

fn compute(x: u32, y: u32) -> u32 {
    let sum = x + y;
    u32_sqrt(sum)
}
"#;

/// File used for navigation benchmarks (goto definition, find references, document highlight).
///
/// Line map (0-indexed):
///   0: ""
///   1: "use core::integer::u32_sqrt;"
///   2: ""
///   3: "fn helper(x: u32) -> u32 {"    ← `helper` defined at col 3
///   4: "    u32_sqrt(x)"                ← `u32_sqrt` called at col 4
///   5: "}"
///   6: ""
///   7: "fn compute(a: u32, b: u32) -> u32 {"
///   8: "    let x = helper(a);"         ← `helper` called at col 12
///   9: "    let y = helper(b);"
///  10: "    x + y"
///  11: "}"
const NAV_CAIRO: &str = r#"
use core::integer::u32_sqrt;

fn helper(x: u32) -> u32 {
    u32_sqrt(x)
}

fn compute(a: u32, b: u32) -> u32 {
    let x = helper(a);
    let y = helper(b);
    x + y
}
"#;

/// Poorly formatted Cairo for formatting benchmarks.
const FORMAT_CAIRO: &str = r#"
fn  add( a:u32,b:u32 )->u32{
    let   result=a+b;
    result
}
fn  mul( a:u32,b:u32 )->u32{
    let   result=a*b;
    result
}
"#;

/// File with an unused variable for code action benchmarks.
///
/// Line map (0-indexed):
///   0: ""
///   1: "fn foo(value: u32) -> u32 {"
///   2: "    let unused = 42_u32;"    ← `unused` at col 8
///   3: "    value"
///   4: "}"
const CODE_ACTION_CAIRO: &str = r#"
fn foo(value: u32) -> u32 {
    let unused = 42_u32;
    value
}
"#;

// ── BenchClient ──────────────────────────────────────────────────────────────

/// A minimal LSP client for benchmarking.
///
/// Wraps an in-process `lsp_server::Connection` and handles the LSP protocol details
/// (initialize handshake, `workspace/configuration` responses, etc.) so benchmark
/// functions can focus on the operation being measured.
struct BenchClient {
    connection: lsp_server::Connection,
    req_id: i32,
    main_file_uri: Url,
    // Kept alive to prevent the temp directory from being deleted.
    _dir: TempDir,
}

impl BenchClient {
    /// Spins up a language server with `cairo_code` as `src/lib.cairo` and waits until the
    /// project finishes loading. Returns a client ready to send LSP requests.
    fn new(cairo_code: &str) -> Self {
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

    fn completion(&mut self, line: u32, character: u32) -> Option<CompletionResponse> {
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
        self.send_raw(
            id.clone(),
            "textDocument/completion",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).map(|v| serde_json::from_value(v).unwrap())
    }

    fn hover(&mut self, line: u32, character: u32) -> Option<Hover> {
        let id = self.next_id();
        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/hover",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    fn goto_definition(&mut self, line: u32, character: u32) -> Option<GotoDefinitionResponse> {
        let id = self.next_id();
        let params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
                position: Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/definition",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    fn references(&mut self, line: u32, character: u32) -> Option<Vec<Location>> {
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
        self.send_raw(
            id.clone(),
            "textDocument/references",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    fn document_highlight(
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

    fn formatting(&mut self) -> Option<Vec<TextEdit>> {
        let id = self.next_id();
        let params = DocumentFormattingParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            options: FormattingOptions::default(),
            work_done_progress_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/formatting",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    fn semantic_tokens(&mut self) -> Option<SemanticTokensResult> {
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

    fn inlay_hints(&mut self, range: Range) -> Option<Vec<InlayHint>> {
        let id = self.next_id();
        let params = InlayHintParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            range,
            work_done_progress_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/inlayHint",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }

    fn code_actions(&mut self, line: u32, character: u32) -> Option<CodeActionResponse> {
        let id = self.next_id();
        let pos = Position { line, character };
        let params = CodeActionParams {
            text_document: TextDocumentIdentifier { uri: self.main_file_uri.clone() },
            range: Range { start: pos, end: pos },
            context: CodeActionContext {
                diagnostics: vec![],
                only: None,
                trigger_kind: None,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        self.send_raw(
            id.clone(),
            "textDocument/codeAction",
            serde_json::to_value(params).unwrap(),
        );
        self.recv_until_response(id).and_then(|v| serde_json::from_value(v).ok())
    }
}

// ── Benchmarks ───────────────────────────────────────────────────────────────

fn bench_completions(c: &mut Criterion) {
    let mut client = BenchClient::new(COMPLETION_CAIRO);
    let mut group = c.benchmark_group("completions");
    group.measurement_time(Duration::from_secs(10));

    // Cursor after a typed identifier ("res") — tests fuzzy-filtered completions.
    group.bench_function("typed_prefix", |b| {
        b.iter(|| client.completion(4, 10)) // line: "    result"
    });

    // Cursor on a blank line — tests the untyped/empty-trigger path (issue #957).
    group.bench_function("untyped", |b| {
        b.iter(|| client.completion(1, 0)) // line: ""  (blank after fn declaration)
    });

    group.finish();
}

fn bench_hover(c: &mut Criterion) {
    let mut client = BenchClient::new(HOVER_CAIRO);
    let mut group = c.benchmark_group("hover");
    group.measurement_time(Duration::from_secs(10));

    // Hover over the `u32_sqrt` function call — resolves to a stdlib function.
    group.bench_function("stdlib_function", |b| {
        b.iter(|| client.hover(5, 4)) // line: "    u32_sqrt(sum)"
    });

    // Hover over a local variable.
    group.bench_function("local_variable", |b| {
        b.iter(|| client.hover(4, 8)) // line: "    let sum = x + y;"
    });

    group.finish();
}

fn bench_goto_definition(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("goto_definition");
    group.measurement_time(Duration::from_secs(10));

    // Go to definition of a local function — resolves within the same file.
    group.bench_function("local_function", |b| {
        b.iter(|| client.goto_definition(8, 12)) // line: "    let x = helper(a);"
    });

    // Go to definition of a stdlib function — resolves to a virtual file.
    group.bench_function("stdlib_function", |b| {
        b.iter(|| client.goto_definition(4, 4)) // line: "    u32_sqrt(x)"
    });

    group.finish();
}

fn bench_references(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("references");
    group.measurement_time(Duration::from_secs(10));

    // Find all references of a local function called from two call sites.
    group.bench_function("local_function", |b| {
        b.iter(|| client.references(3, 3)) // line: "fn helper(x: u32) -> u32 {"
    });

    group.finish();
}

fn bench_document_highlight(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("document_highlight");
    group.measurement_time(Duration::from_secs(10));

    // Highlight all occurrences of a function with two call sites in the file.
    group.bench_function("local_function", |b| {
        b.iter(|| client.document_highlight(3, 3)) // line: "fn helper(x: u32) -> u32 {"
    });

    group.finish();
}

fn bench_formatting(c: &mut Criterion) {
    let mut client = BenchClient::new(FORMAT_CAIRO);
    let mut group = c.benchmark_group("formatting");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("file", |b| b.iter(|| client.formatting()));

    group.finish();
}

fn bench_semantic_tokens(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("semantic_tokens");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("full_file", |b| b.iter(|| client.semantic_tokens()));

    group.finish();
}

fn bench_inlay_hints(c: &mut Criterion) {
    let mut client = BenchClient::new(NAV_CAIRO);
    let mut group = c.benchmark_group("inlay_hints");
    group.measurement_time(Duration::from_secs(10));

    let whole_file =
        Range { start: Position { line: 0, character: 0 }, end: Position { line: 999, character: 0 } };
    group.bench_function("full_file", |b| b.iter(|| client.inlay_hints(whole_file)));

    group.finish();
}

fn bench_code_actions(c: &mut Criterion) {
    let mut client = BenchClient::new(CODE_ACTION_CAIRO);
    let mut group = c.benchmark_group("code_actions");
    group.measurement_time(Duration::from_secs(10));

    // Code actions on an unused variable — may suggest a rename-to-underscore fix.
    group.bench_function("unused_variable", |b| {
        b.iter(|| client.code_actions(2, 8)) // line: "    let unused = 42_u32;"
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_completions,
    bench_hover,
    bench_goto_definition,
    bench_references,
    bench_document_highlight,
    bench_formatting,
    bench_semantic_tokens,
    bench_inlay_hints,
    bench_code_actions,
);
criterion_main!(benches);
