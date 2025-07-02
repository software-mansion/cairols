use lsp_types::{
    ClientCapabilities, DocumentHighlight, DocumentHighlightParams, ReferenceClientCapabilities,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::cursor::render_selections_with_attrs;
use crate::support::transform::Transformer;

mod highlight;

impl Transformer for DocumentHighlight {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        ClientCapabilities {
            text_document: base.text_document.or_else(Default::default).map(|it| {
                TextDocumentClientCapabilities {
                    references: Some(ReferenceClientCapabilities {
                        dynamic_registration: Some(false),
                    }),
                    ..it
                }
            }),
            ..base
        }
    }

    fn transform(
        mut ls: crate::support::MockClient,
        cursors: crate::support::cursor::Cursors,
        _config: Option<serde_json::Value>,
    ) -> String {
        let cairo = ls.fixture.read_file("src/lib.cairo");
        let position = cursors.assert_single_caret();

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
}
