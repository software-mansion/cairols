use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_language_server::lsp;
use lsp_types::NumberOrString;

use crate::support::normalize::normalize;
use crate::support::sandbox;

cairo_lang_test_utils::test_file_test!(
    simple_deps,
    "tests/test_data/scarb",
    {
        simple_deps: "simple_deps.txt"
    },
    test_simple_deps
);

fn test_simple_deps(
    inputs: &OrderedHashMap<String, String>,
    _args: &OrderedHashMap<String, String>,
) -> TestRunnerResult {
    let mut ls = sandbox! {
        files {
            "a/Scarb.toml" => &inputs["a/Scarb.toml"],
            "a/src/lib.cairo" => &inputs["a/src/lib.cairo"],
            "b/Scarb.toml" => &inputs["b/Scarb.toml"],
            "b/src/lib.cairo" => &inputs["b/src/lib.cairo"],
        }
    };

    assert!(ls.open_and_wait_for_diagnostics("a/src/lib.cairo").diagnostics.is_empty());
    // Check if opening `a` triggers calculating diagnostics for `b`.
    let diagnostics_from_b = ls.wait_for_diagnostics("b/src/lib.cairo");
    assert_eq!(diagnostics_from_b.diagnostics.len(), 1);
    assert_eq!(
        diagnostics_from_b.diagnostics[0].code,
        Some(NumberOrString::String("E0005".to_string()))
    );

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    TestRunnerResult::success(OrderedHashMap::from([(
        "Analyzed crates".to_string(),
        normalize(&ls, analyzed_crates),
    )]))
}
