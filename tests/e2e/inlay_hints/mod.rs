use itertools::Itertools;
use lsp_types::{
    ClientCapabilities, InlayHintClientCapabilities, InlayHintLabel, InlayHintLabelPartTooltip,
    InlayHintParams, MarkupContent, TextDocumentClientCapabilities, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::index_in_text;
use crate::support::{cursors, sandbox};

mod variables;

fn inlay_hint(cairo_code: &str) -> String {
    let (mut cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => cairo.clone()
        }
        client_capabilities = caps;
    };

    ls.open_and_wait_for_diagnostics("src/lib.cairo");

    let inlay_hint_params = InlayHintParams {
        text_document: ls.doc_id("src/lib.cairo"),
        range: cursors.assert_single_selection(),
        work_done_progress_params: Default::default(),
    };
    let inlay_hints = ls.send_request::<lsp_request!("textDocument/inlayHint")>(inlay_hint_params);

    let Some(mut inlay_hints) = inlay_hints else {
        panic!("Inlay hint request failed.");
    };

    inlay_hints.sort_by_key(|a| a.position);
    inlay_hints.reverse();

    for hint in inlay_hints {
        let offset = index_in_text(&cairo, hint.position);

        let label = match hint.label {
            InlayHintLabel::String(label) => label,
            InlayHintLabel::LabelParts(parts) => parts
                .into_iter()
                .map(|part| {
                    let tooltip = part
                        .tooltip
                        .map(|tooltip| match tooltip {
                            InlayHintLabelPartTooltip::String(tooltip) => tooltip,
                            InlayHintLabelPartTooltip::MarkupContent(MarkupContent {
                                value,
                                ..
                            }) => value,
                        })
                        .map(|tooltip| format!(" tooltip={tooltip:?}"))
                        .unwrap_or_default();

                    format!("<hint{tooltip}>{}</hint>", part.value)
                })
                .join(""),
        };

        cairo.insert_str(offset, &label);
    }

    cairo
}

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                inlay_hint: Some(InlayHintClientCapabilities {
                    dynamic_registration: Some(false),
                    ..Default::default()
                }),
                ..it
            }
        }),
        ..base
    }
}
