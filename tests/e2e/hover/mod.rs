use std::fmt::Display;

use lsp_types::{
    ClientCapabilities, Hover, HoverClientCapabilities, HoverContents, HoverParams, MarkupContent,
    MarkupKind, TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};
use serde::Serialize;

use crate::support::MockClient;
use crate::support::cursor::{Cursors, peek_caret, peek_selection};
use crate::support::transform::Transformer;

mod basic;
mod consts;
mod function_call;
mod literals;
mod missing_module;
mod partial;
mod paths;
mod structs;
mod traits;
mod types;
mod variables;

impl Transformer for Hover {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        ClientCapabilities {
            text_document: base.text_document.or_else(Default::default).map(|it| {
                TextDocumentClientCapabilities {
                    hover: Some(HoverClientCapabilities {
                        dynamic_registration: Some(false),
                        content_format: Some(vec![MarkupKind::Markdown, MarkupKind::PlainText]),
                    }),
                    ..it
                }
            }),
            ..base
        }
    }

    fn transform(mut ls: MockClient, cursors: Cursors) -> String {
        let cairo = ls.fixture.read_file("src/lib.cairo");
        let position = cursors.assert_single_caret();

        let hover = ls.send_request::<lsp_request!("textDocument/hover")>(HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: ls.doc_id("src/lib.cairo"),
                position,
            },
            work_done_progress_params: Default::default(),
        });

        Report {
            source_context: peek_caret(&cairo, position),
            highlight: hover
                .as_ref()
                .and_then(|h| h.range)
                .map(|range| peek_selection(&cairo, &range)),
            popover: hover.map(render),
        }
        .to_string()
    }
}

#[derive(Serialize)]
struct Report {
    source_context: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    popover: Option<String>,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringifed = toml::to_string_pretty(self).map_err(|_| std::fmt::Error)?;

        f.write_str(&stringifed)
    }
}

/// Render a hover response to a Markdown string that resembles what would be shown in a hover popup
/// in the text editor.
fn render(h: Hover) -> String {
    match h.contents {
        HoverContents::Markup(MarkupContent { value, .. }) => value,
        contents => {
            panic!("LS returned deprecated MarkedString-based hover contents: {:#?}", contents);
        }
    }
}
