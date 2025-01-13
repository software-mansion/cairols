use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::Itertools;
use lsp_types::{
    ClientCapabilities, Location, ReferenceClientCapabilities, ReferenceContext, ReferenceParams,
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

    let mut outputs = OrderedHashMap::default();
    for (n, position) in cursors.carets().into_iter().enumerate() {
        let mut query = |include_declaration: bool| {
            let params = ReferenceParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: ls.doc_id("src/lib.cairo"),
                    position,
                },
                context: ReferenceContext { include_declaration },
                work_done_progress_params: Default::default(),
                partial_result_params: Default::default(),
            };
            let locations = ls.send_request::<lsp_request!("textDocument/references")>(params)?;
            Some(locations.into_iter().map(LocationForComparison).collect::<HashSet<_>>())
        };

        let mut report = String::new();
        report.push_str(&peek_caret(&cairo, position));

        match (query(false), query(true)) {
            (None, None) => report.push_str("none response"),
            (Some(_), None) => panic!(
                "references excluding declaration returned response, but including declaration \
                 did not"
            ),
            (None, Some(_)) => panic!(
                "references including declaration returned response, but excluding declaration \
                 did not"
            ),
            (Some(excluding_declaration), Some(including_declaration)) => {
                assert!(
                    excluding_declaration.is_subset(&including_declaration),
                    "include_declarations: true should return a superset of include_declarations: \
                     false"
                );

                let mut declarations: Vec<Location> = including_declaration
                    .difference(&excluding_declaration)
                    .sorted()
                    .map(|l| l.0.clone())
                    .collect();

                let mut usages = excluding_declaration
                    .intersection(&including_declaration)
                    .sorted()
                    .map(|l| l.0.clone())
                    .collect::<Vec<_>>();

                report.push_str("\nDECLARATIONS:\n");

                remove_core_references(&mut declarations, &mut report);

                for location in declarations {
                    report.push_str(&peek_selection(&cairo, &location.range));
                }

                report.push_str("\nUSAGES:\n");

                remove_core_references(&mut usages, &mut report);

                for location in usages {
                    report.push_str(&peek_selection(&cairo, &location.range));
                }
            }
        }

        outputs.insert(format!("References #{n}"), report);
    }
    TestRunnerResult::success(outputs)
}

fn remove_core_references(locations: &mut Vec<Location>, report: &mut String) {
    // Remove any references found in the core crate.
    // We do not want to test core crate contents here, but we want to note that they
    // exist.
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
}

#[derive(PartialEq, Eq)]
struct LocationForComparison(Location);

impl PartialOrd for LocationForComparison {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LocationForComparison {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        #[expect(clippy::needless_borrowed_reference)] // Clippy asks to write erroneous code.
        let key = |&Self(ref loc)| (loc.uri.as_str(), loc.range.start, loc.range.end);
        key(self).cmp(&key(other))
    }
}

impl Hash for LocationForComparison {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.uri.hash(state);
        self.0.range.start.line.hash(state);
        self.0.range.start.character.hash(state);
        self.0.range.end.line.hash(state);
        self.0.range.end.character.hash(state);
    }
}
