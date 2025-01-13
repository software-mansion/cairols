use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use lsp_types::{
    ClientCapabilities, GotoCapability, GotoDefinitionParams, GotoDefinitionResponse,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::{peek_caret, peek_selection};
use crate::support::{cursors, sandbox};

cairo_lang_test_utils::test_file_test!(
    goto_definition,
    "tests/test_data/goto",
    {
        enum_variants: "enum_variants.txt",
        inline_macros: "inline_macros.txt",
        items: "items.txt",
        modules: "modules.txt",
        struct_members: "struct_members.txt",
        variables: "variables.txt",
    },
    test_goto_definition
);

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

fn test_goto_definition(
    inputs: &OrderedHashMap<String, String>,
    _args: &OrderedHashMap<String, String>,
) -> TestRunnerResult {
    let (cairo, cursors) = cursors(&inputs["cairo_code"]);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => cairo.clone(),
        }
        client_capabilities = caps;
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    let mut goto_definitions = OrderedHashMap::default();

    for (n, position) in cursors.carets().into_iter().enumerate() {
        let mut report = String::new();

        report.push_str(&peek_caret(&cairo, position));
        let code_action_params = GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: ls.doc_id("src/lib.cairo"),
                position,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let response =
            ls.send_request::<lsp_request!("textDocument/definition")>(code_action_params);

        match response {
            Some(GotoDefinitionResponse::Scalar(location)) => {
                report.push_str("---\n");
                report.push_str(&peek_selection(&cairo, &location.range));
            }
            Some(GotoDefinitionResponse::Array(locations)) => {
                for location in locations {
                    report.push_str("---\n");
                    report.push_str(&peek_selection(&cairo, &location.range));
                }
            }
            Some(GotoDefinitionResponse::Link(_)) => {
                panic!("unexpected GotoDefinitionResponse::Link");
            }
            None => {
                report.push_str("None");
            }
        }
        goto_definitions.insert(format!("Goto definition #{}", n), report);
    }

    TestRunnerResult::success(goto_definitions)
}
