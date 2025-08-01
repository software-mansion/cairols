use cairo_language_server::testing::SemanticTokenKind;
use lsp_types::{Position, Range, lsp_request};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2023_11;
use crate::support::cursor::render_text_with_annotations;
use crate::support::sandbox;

mod complex;

fn semantic_tokens(code: &str) -> String {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2023_11,
            "src/lib.cairo" => code,
        }
        client_capabilities = caps;
    };

    ls.open_all_and_wait_for_diagnostics_generation();

    let res = ls
        .send_request::<lsp_request!("textDocument/semanticTokens/full")>(
            lsp_types::SemanticTokensParams {
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
                text_document: ls.doc_id("src/lib.cairo"),
            },
        )
        .unwrap();
    let lsp_types::SemanticTokensResult::Tokens(tokens) = res else {
        panic!("expected full tokens")
    };

    let mut line = 0;
    let mut character = 0;

    let legend = SemanticTokenKind::legend();

    let tokens: Vec<_> = tokens
        .data
        .into_iter()
        .map(|token| {
            // Reset on new line.
            if token.delta_line != 0 {
                character = 0;
            }

            line += token.delta_line;
            character += token.delta_start;

            let start = Position { character, line };
            let end = Position { character: start.character + token.length, ..start };

            let token_type = legend[token.token_type as usize].as_str().to_string();

            (Range { start, end }, Some(token_type))
        })
        .collect();

    render_text_with_annotations(code, "token", &tokens)
}

fn caps(base: lsp_types::ClientCapabilities) -> lsp_types::ClientCapabilities {
    lsp_types::ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            lsp_types::TextDocumentClientCapabilities {
                semantic_tokens: Some(lsp_types::SemanticTokensClientCapabilities {
                    dynamic_registration: Some(false),
                    requests: lsp_types::SemanticTokensClientCapabilitiesRequests {
                        full: Some(lsp_types::SemanticTokensFullOptions::Bool(true)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                ..it
            }
        }),
        ..base
    }
}
