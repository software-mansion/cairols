use indoc::indoc;
use lsp_types::{
    CompletionParams, GotoDefinitionParams, GotoDefinitionResponse, HoverParams,
    TextDocumentPositionParams, lsp_request,
};

use crate::support::cursor::cursors;
use crate::support::sandbox;

#[test]
fn lsp_features_work_in_integration_test_files() {
    let (test_file, cursors) = cursors(indoc! {r#"
        use hello::helper;

        #[test]
        fn probe() {
            let value = hel<caret>per(1);
            val<caret>ue;
        }
    "#});
    let helper_call = cursors.caret(0);
    let local_value = cursors.caret(1);

    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "hello"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => indoc! {r#"
                pub fn helper(value: felt252) -> felt252 {
                    value
                }
            "#},
            "tests/test.cairo" => test_file,
        }
    };

    ls.open_all_and_wait_for_diagnostics_generation();

    let test_doc = ls.doc_id("tests/test.cairo");
    let analyzed_crates = ls.send_request::<cairo_language_server::lsp::ext::ViewAnalyzedCrates>(());

    let hover = ls.send_request::<lsp_request!("textDocument/hover")>(HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: test_doc.clone(),
            position: helper_call,
        },
        work_done_progress_params: Default::default(),
    });
    let goto = ls.send_request::<lsp_request!("textDocument/definition")>(GotoDefinitionParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: test_doc.clone(),
            position: helper_call,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    });

    let completions = ls.send_request::<lsp_request!("textDocument/completion")>(CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: test_doc,
            position: local_value,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    });
    let completion_items = completions
        .map(|completions| match completions {
            lsp_types::CompletionResponse::Array(items) => items,
            lsp_types::CompletionResponse::List(list) => list.items,
        })
        .unwrap_or_default();
    let has_local_completion = completion_items.iter().any(|item| item.label == "value");

    assert!(
        hover.is_some()
            && matches!(
                goto,
                Some(GotoDefinitionResponse::Scalar(_)) | Some(GotoDefinitionResponse::Array(_))
            )
            && has_local_completion,
        "integration test file LSP results:\nanalyzed_crates={analyzed_crates}\nhover={hover:#?}\ngoto={goto:#?}\ncompletion_items={completion_items:#?}"
    );
}
