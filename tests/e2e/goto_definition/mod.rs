use lsp_types::{
    ClientCapabilities, GotoCapability, GotoDefinitionParams, GotoDefinitionResponse,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::render_selections;
use crate::support::{cursors, sandbox};

mod enums;
mod fns;
mod macros;
mod paths;
mod structs;
mod traits;
mod vars;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                definition: Some(GotoCapability {
                    dynamic_registration: Some(false),
                    link_support: None,
                }),
                ..it
            }
        }),
        ..base
    }
}

fn goto_definition(cairo_code: &str) -> String {
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

    let code_action_params = GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: ls.doc_id("src/lib.cairo"),
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };
    let response = ls.send_request::<lsp_request!("textDocument/definition")>(code_action_params);

    let ranges = match response {
        Some(GotoDefinitionResponse::Scalar(location)) => {
            vec![location.range]
        }
        Some(GotoDefinitionResponse::Array(locations)) => {
            locations.into_iter().map(|location| location.range).collect()
        }
        Some(GotoDefinitionResponse::Link(_)) => {
            panic!("unexpected GotoDefinitionResponse::Link");
        }
        None => {
            return "none response".into();
        }
    };

    render_selections(&cairo, &ranges)
}
