use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use lsp_types::{
    ClientCapabilities, ReferenceClientCapabilities, ReferenceContext, ReferenceParams,
    TextDocumentClientCapabilities, TextDocumentPositionParams, lsp_request,
};

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::{peek_caret, peek_selection};
use crate::support::{cursors, sandbox};

cairo_lang_test_utils::test_file_test!(
    references,
    "tests/test_data/references",
    {
        enum_variants: "enum_variants.txt",
        enums: "enums.txt",
        fns: "fns.txt",
        inline_macros: "inline_macros.txt",
        methods: "methods.txt",
        struct_members: "struct_members.txt",
        structs: "structs.txt",
        traits: "traits.txt",
        variables: "variables.txt",
    },
    test_references
);

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

fn test_references(
    inputs: &OrderedHashMap<String, String>,
    args: &OrderedHashMap<String, String>,
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

    let mut outputs = OrderedHashMap::default();
    for (n, position) in cursors.carets().into_iter().enumerate() {
        let mut report = String::new();
        report.push_str(&peek_caret(&cairo, position));

        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: ls.doc_id("src/lib.cairo"),
                position,
            },
            context: ReferenceContext {
                include_declaration: args["include_declaration"] == "true",
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let response = ls.send_request::<lsp_request!("textDocument/references")>(params);

        if let Some(mut locations) = response {
            report.push_str("---\n");

            // LS does not guarantee any order of the results.
            locations
                .sort_by_key(|loc| (loc.uri.as_str().to_owned(), loc.range.start, loc.range.end));

            // Remove any references found in the core crate.
            // We do not want to test core crate contents here, but we want to note that they exist.
            let mut found_core_refs = false;
            locations.retain(|loc| {
                let path = loc.uri.path();
                if path.contains("/core/src/") || path.contains("/corelib/src/") {
                    found_core_refs = true;
                    false
                } else {
                    true
                }
            });
            if found_core_refs {
                report.push_str("found several references in the core crate\n");
            }

            for location in locations {
                report.push_str(&peek_selection(&cairo, &location.range));
            }
        } else {
            report.push_str("none response")
        }

        outputs.insert(format!("References #{n}"), report);
    }
    TestRunnerResult::success(outputs)
}
