use cairo_language_server::testing::SemanticTokenKind;
use lsp_types::{
    ClientCapabilities, Position, Range, SemanticTokens, SemanticTokensClientCapabilities,
    SemanticTokensClientCapabilitiesRequests, SemanticTokensFullOptions, SemanticTokensParams,
    SemanticTokensResult, TextDocumentClientCapabilities, lsp_request,
};

use crate::support::MockClient;
use crate::support::cursor::{Cursors, render_text_with_annotations};
use crate::support::transform::Transformer;

mod complex;
mod proc_macros;

impl Transformer for SemanticTokens {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        ClientCapabilities {
            text_document: base.text_document.or_else(Default::default).map(|it| {
                TextDocumentClientCapabilities {
                    semantic_tokens: Some(SemanticTokensClientCapabilities {
                        dynamic_registration: Some(false),
                        requests: SemanticTokensClientCapabilitiesRequests {
                            full: Some(SemanticTokensFullOptions::Bool(true)),
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

    fn transform(
        mut ls: MockClient,
        _cursors: Cursors,
        _config: Option<serde_json::Value>,
    ) -> String {
        let code = ls.fixture.read_file("src/lib.cairo");

        let res = ls
            .send_request::<lsp_request!("textDocument/semanticTokens/full")>(
                SemanticTokensParams {
                    work_done_progress_params: Default::default(),
                    partial_result_params: Default::default(),
                    text_document: ls.doc_id("src/lib.cairo"),
                },
            )
            .unwrap();
        let SemanticTokensResult::Tokens(tokens) = res else { panic!("expected full tokens") };

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

        render_text_with_annotations(&code, "token", &tokens)
    }
}
