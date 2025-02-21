use std::fmt::Display;

use lsp_types::{
    ClientCapabilities, Hover, HoverClientCapabilities, HoverContents, HoverParams, MarkupContent,
    MarkupKind, TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};
use serde::Serialize;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2023_11;
use crate::support::cursor::{peek_caret, peek_selection};
use crate::support::{cursors, sandbox};

mod basic;
mod literals;
mod missing_module;
mod partial;
mod paths;
mod structs;
mod traits;
mod variables;

/// Perform hover test.
///
/// This function spawns a sandbox language server with the given code in the `src/lib.cairo` file.
/// The Cairo source code is expected to contain caret markers.
/// The function then requests hover information at each caret position and compares the result with
/// the expected hover information from the snapshot file.
fn test_hover(cairo_code: &str) -> Report {
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2023_11,
            "src/lib.cairo" => cairo.clone(),
        }
        client_capabilities = caps;
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    assert_eq!(cursors.carets().len(), 1);
    let position = cursors.carets()[0];

    let hover = ls.send_request::<lsp_request!("textDocument/hover")>(HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: ls.doc_id("src/lib.cairo"),
            position,
        },
        work_done_progress_params: Default::default(),
    });

    Report {
        source_context: peek_caret(&cairo, position),
        highlight: hover.as_ref().and_then(|h| h.range).map(|range| peek_selection(&cairo, &range)),
        popover: hover.map(render),
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

fn caps(base: ClientCapabilities) -> ClientCapabilities {
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
