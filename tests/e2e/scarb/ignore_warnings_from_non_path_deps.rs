use indoc::indoc;
use lsp_types::{
    ClientCapabilities, GotoCapability, GotoDefinitionParams, GotoDefinitionResponse,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};
use serde_json::json;

use crate::support::{cursors, sandbox};

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

#[ignore = "Uncomment after Scarb bump in CI"]
#[test]
fn test_ignore_warnings_from_non_path_deps() {
    let cairo_code = indoc! {r#"
        use snforge_std::byte_array::byte_array_a<caret>s_felt_array;

        fn func() {
            byte_array_as_felt_array(@"abc");
        }
    "#};
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"

                [dev-dependencies]
                snforge_std = "0.37.0"  # This version contains lint errors.
            "#),
            "src/lib.cairo" => cairo,
        }
        client_capabilities = caps;
        workspace_configuration = json!({
            "cairo1": {
                "enableLinter": true,
            }
        });
    };

    // Test for lack of diagnostics in the entire project even though the dep contains lint errors.
    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo")
        .into_iter()
        .for_each(|(url, diags)| assert!(diags.is_empty(), "{url} → {diags:#?}"));

    // Goto is the easiest way to get an absolute path to a file from a dependency.
    let url_to_dependency_file_with_lint_errors = {
        let code_action_params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: ls.doc_id("src/lib.cairo"),
                position: cursors.carets()[0],
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let response =
            ls.send_request::<lsp_request!("textDocument/definition")>(code_action_params).unwrap();

        match response {
            GotoDefinitionResponse::Scalar(location) => location.uri,
            _ => panic!("Unexpected goto response variant"),
        }
    };

    // Test if no diagnostics appear even though we opened a dep file that contains lint errors.
    assert!(
        ls.open_and_wait_for_diagnostics_generation(
            url_to_dependency_file_with_lint_errors.to_file_path().unwrap()
        )
        .get(&url_to_dependency_file_with_lint_errors)
        .unwrap()
        .is_empty()
    );
}
