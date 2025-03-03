use lsp_types::{
    ClientCapabilities, DocumentHighlightParams, ReferenceClientCapabilities,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::render_selections_with_attrs;
use crate::support::{cursors, sandbox};

mod highlight;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                references: Some(ReferenceClientCapabilities { dynamic_registration: Some(false) }),
                ..it
            }
        }),
        ..base
    }
}

fn document_highlight(cairo_code: &str) -> String {
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => cairo.clone(),
        }
        client_capabilities = caps;
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    assert_eq!(cursors.carets().len(), 1);
    let position = cursors.carets()[0];

    let params = DocumentHighlightParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: ls.doc_id("src/lib.cairo"),
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };

    let highlights =
        ls.send_request::<lsp_request!("textDocument/documentHighlight")>(params).unwrap();

    let ranges = highlights.into_iter().map(|loc| (loc.range, None)).collect::<Vec<_>>();

    render_selections_with_attrs(&cairo, &ranges)
}
