use cairo_language_server::lsp;
use cairo_language_server::lsp::ext::testing_requests::{DatabaseSwapped, ForceDatabaseSwap};
use cairo_language_server::lsp::ext::{ProvideVirtualFile, ProvideVirtualFileRequest};
use indoc::indoc;
use lsp_types::HoverContents;
use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument,
};
use lsp_types::request::Formatting;
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, DocumentFormattingParams, ExecuteCommandParams, FormattingOptions,
    HoverParams, Position, PublishDiagnosticsParams, Range, SemanticTokensParams,
    SemanticTokensResult, TextDocumentContentChangeEvent, TextDocumentItem,
    TextDocumentPositionParams, VersionedTextDocumentIdentifier, lsp_notification, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML;
use crate::support::cursor::{cursors, text_chunk_at_range};
use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn cairo_projects() {
    let mut ls = sandbox! {
        files {
            "project1/cairo_project.toml" => indoc! {r#"
                [crate_roots]
                project1 = "src"
            "#},
            "project1/src/lib.cairo" => "fn main() {}",

            "project2/cairo_project.toml" => indoc! {r#"
                [crate_roots]
                project2 = "src"
            "#},
            "project2/src/lib.cairo" => "fn main() {}",

            "project2/subproject/cairo_project.toml" => indoc! {r#"
                [crate_roots]
                subproject = "src"
            "#},
            "project2/subproject/src/lib.cairo" => "fn main() {}"
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let output = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    insta::assert_snapshot!("view_analyzed_crates", normalize(&ls, output));
}

#[test]
fn test_reload() {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML,
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let expected = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    ls.send_request::<lsp_request!("workspace/executeCommand")>(ExecuteCommandParams {
        command: "cairo.reload".into(),
        ..Default::default()
    });
    let actual = ls.wait_for_project_update();

    assert_eq!(expected, actual);
}

#[test]
fn assert_macros_with_no_cairo_test() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2025_12"

                [dependencies]
                assert_macros = "2"
            "#),
            "src/lib.cairo" => "fn main() {}",
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let output = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    insta::assert_snapshot!("view_analyzed_crates_assert_macros", normalize(&ls, output));
}

#[test]
fn did_open_dirty_content_before_project_load_is_applied() {
    let clean = "fn main() {\n    let clean = 1;\n}\n";
    let dirty = "fn main( {\n    let dirty = 2;\n}\n";
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "open_dirty"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidOpenTextDocument>(DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: uri.clone(),
            language_id: "cairo".into(),
            version: 0,
            text: dirty.into(),
        },
    });

    let diagnostics = ls.wait_for_diagnostics_generation();
    let file_diagnostics = diagnostics.get(&uri).cloned().unwrap_or_default();
    assert!(
        !file_diagnostics.is_empty(),
        "expected diagnostics for dirty in-memory content, got none"
    );

    let provided =
        ls.send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: uri.clone() });
    assert_eq!(provided.content.as_deref(), Some(dirty));
}

#[test]
fn incremental_did_change_is_applied_and_survives_save() {
    let clean = "fn main() {\n    let clean = 1;\n}";
    let dirty = "fn main() {\n    let clean = ;\n}";
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "incremental_dirty"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position { line: 1, character: 16 },
                end: Position { line: 1, character: 17 },
            }),
            range_length: Some(1),
            text: String::new(),
        }],
    });
    ls.send_notification::<DidSaveTextDocument>(DidSaveTextDocumentParams {
        text_document: ls.doc_id("src/lib.cairo"),
        text: None,
    });

    let file_diagnostics = ls
        .wait_for_notification::<lsp_notification!("textDocument/publishDiagnostics")>(
            |params: &PublishDiagnosticsParams| params.uri == uri,
        )
        .diagnostics;
    assert!(
        !file_diagnostics.is_empty(),
        "expected diagnostics for incrementally changed content, got none"
    );

    let provided =
        ls.send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: uri.clone() });
    assert_eq!(provided.content.as_deref(), Some(dirty));
}

#[test]
fn did_save_keeps_analysis_on_open_document_override() {
    let clean = "fn main() {\n    let clean = 1;\n}";
    let dirty = "fn main() {\n    let broken = ;\n}\n";
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "save_dirty"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: dirty.into(),
        }],
    });
    ls.send_notification::<DidSaveTextDocument>(DidSaveTextDocumentParams {
        text_document: ls.doc_id("src/lib.cairo"),
        text: None,
    });

    let file_diagnostics = ls
        .wait_for_notification::<lsp_notification!("textDocument/publishDiagnostics")>(
            |params: &PublishDiagnosticsParams| params.uri == uri,
        )
        .diagnostics;
    assert!(!file_diagnostics.is_empty(), "expected diagnostics for dirty saved content, got none");

    let provided =
        ls.send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: uri.clone() });
    assert_eq!(provided.content.as_deref(), Some(dirty));
    assert_eq!(ls.fixture.read_file("src/lib.cairo"), clean);
}

#[test]
fn database_swap_keeps_analysis_on_open_document_override() {
    let clean = "fn main() {\n    let clean = 1;\n}";
    let dirty = "fn main() {\n    let broken = ;\n}\n";
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "swap_dirty"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: dirty.into(),
        }],
    });
    ls.wait_for_notification::<lsp_notification!("textDocument/publishDiagnostics")>(
        |params: &PublishDiagnosticsParams| params.uri == uri,
    );

    ls.send_request::<ForceDatabaseSwap>(());
    ls.wait_for_notification::<DatabaseSwapped>(|_| true);

    let file_diagnostics = ls
        .wait_for_notification::<lsp_notification!("textDocument/publishDiagnostics")>(
            |params: &PublishDiagnosticsParams| params.uri == uri,
        )
        .diagnostics;
    assert!(
        !file_diagnostics.is_empty(),
        "expected diagnostics for dirty content after database swap, got none"
    );

    let provided =
        ls.send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: uri.clone() });
    assert_eq!(provided.content.as_deref(), Some(dirty));
}

#[test]
fn formatting_uses_current_open_document_content() {
    let clean = "fn main() {\n    let clean = 1;\n}\n";
    let edited = "fn main() {\n    let clean = 2;\n}\n";
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "format_incremental"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: ls.doc_id("src/lib.cairo").uri,
            version: 1,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position { line: 1, character: 16 },
                end: Position { line: 1, character: 17 },
            }),
            range_length: Some(1),
            text: "2".into(),
        }],
    });

    let edits = ls.send_request::<Formatting>(DocumentFormattingParams {
        text_document: ls.doc_id("src/lib.cairo"),
        options: FormattingOptions { tab_size: 4, insert_spaces: true, ..Default::default() },
        work_done_progress_params: Default::default(),
    });

    let formatted = edits.expect("formatting should succeed");
    assert_eq!(formatted.len(), 1);
    assert!(formatted[0].new_text.contains("let clean = 2;"));
    assert_eq!(formatted[0].new_text, edited);
}

#[test]
fn hover_uses_current_open_document_spans() {
    let clean = "fn main() {\n    let x = 1;\n    x;\n}\n";
    let (dirty, markers) =
        cursors("fn main() {\n    let very_long_name = 1;\n    <caret>very_long_name;\n}\n");
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "hover_live_spans"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");
    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: dirty.clone(),
        }],
    });
    ls.wait_for_notification::<lsp_notification!("textDocument/publishDiagnostics")>(
        |params: &PublishDiagnosticsParams| params.uri == uri,
    );

    let hover = ls
        .send_request::<lsp_request!("textDocument/hover")>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: ls.doc_id("src/lib.cairo"),
                position: markers.assert_single_caret(),
            },
            work_done_progress_params: Default::default(),
        })
        .expect("hover should succeed");

    let HoverContents::Markup(ref contents) = hover.contents else {
        panic!("expected markdown hover contents");
    };
    assert!(
        contents.value.contains("very_long_name"),
        "expected hover to describe edited identifier, got: {hover:#?}"
    );
}

#[test]
fn semantic_tokens_use_current_open_document_spans() {
    let clean = "fn main() {\n    let x = 1;\n    x;\n}\n";
    let dirty = "fn main() {\n    let very_long_name = 1;\n    very_long_name;\n}\n".to_string();
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "semantic_live_spans"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => clean,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");
    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: dirty.clone(),
        }],
    });
    ls.wait_for_notification::<lsp_notification!("textDocument/publishDiagnostics")>(
        |params: &PublishDiagnosticsParams| params.uri == uri,
    );

    let semantic_tokens = ls
        .send_request::<lsp_request!("textDocument/semanticTokens/full")>(SemanticTokensParams {
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            text_document: ls.doc_id("src/lib.cairo"),
        })
        .expect("semantic tokens should succeed");

    let SemanticTokensResult::Tokens(tokens) = semantic_tokens else {
        panic!("expected full semantic tokens");
    };

    let ranges = semantic_token_ranges(tokens.data.into_iter().collect());
    assert!(
        ranges.iter().any(|range| text_chunk_at_range(dirty.clone(), *range) == "very_long_name"),
        "expected semantic tokens to align with edited identifier, got ranges: {ranges:#?}"
    );
}

#[test]
fn closing_reveals_on_disk_content_again() {
    let original = "fn main() {\n    let clean = 1;\n}\n";
    let dirty = "fn main() {\n    let dirty = 2;\n}\n";
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "close_dirty"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => original,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");
    let uri = ls.doc_id("src/lib.cairo").uri;
    ls.send_notification::<DidChangeTextDocument>(DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier { uri: uri.clone(), version: 1 },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: dirty.into(),
        }],
    });
    ls.send_notification::<DidCloseTextDocument>(DidCloseTextDocumentParams {
        text_document: ls.doc_id("src/lib.cairo"),
    });

    let provided =
        ls.send_request::<ProvideVirtualFile>(ProvideVirtualFileRequest { uri: uri.clone() });
    assert_eq!(provided.content.as_deref(), Some(ls.fixture.read_file("src/lib.cairo").as_str()));
}

fn semantic_token_ranges(tokens: Vec<lsp_types::SemanticToken>) -> Vec<Range> {
    let mut line = 0;
    let mut character = 0;

    tokens
        .into_iter()
        .map(|token| {
            if token.delta_line != 0 {
                character = 0;
            }

            line += token.delta_line;
            character += token.delta_start;

            let start = Position { line, character };
            let end = Position { line, character: character + token.length };
            Range { start, end }
        })
        .collect()
}
