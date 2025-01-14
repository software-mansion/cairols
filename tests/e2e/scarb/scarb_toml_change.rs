use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_language_server::lsp;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{
    ClientCapabilities, DidChangeWatchedFilesClientCapabilities, DidChangeWatchedFilesParams,
    FileChangeType, FileEvent, WorkspaceClientCapabilities,
};
use similar::TextDiff;

use crate::support::normalize::normalize;
use crate::support::sandbox;

cairo_lang_test_utils::test_file_test!(
    removing_member,
    "tests/test_data/scarb",
    {
        removing_member: "removing_member.txt"
    },
    test_removing_member
);

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default),
        workspace: base.workspace.or_else(Default::default).map(|it| WorkspaceClientCapabilities {
            did_change_watched_files: Some(DidChangeWatchedFilesClientCapabilities {
                dynamic_registration: None,
                relative_pattern_support: None,
            }),
            ..it
        }),
        ..base
    }
}

fn test_removing_member(
    inputs: &OrderedHashMap<String, String>,
    _args: &OrderedHashMap<String, String>,
) -> TestRunnerResult {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => &inputs["Scarb.toml"],
            "a/Scarb.toml" => &inputs["a/Scarb.toml"],
            "a/src/lib.cairo" => "",
            "b/Scarb.toml" => &inputs["b/Scarb.toml"],
            "b/src/lib.cairo" => "",
        }
        client_capabilities = caps;
    };

    assert!(ls.open_and_wait_for_diagnostics("a/src/lib.cairo").diagnostics.is_empty());
    // Check if opening `a` triggers calculating diagnostics for `b`.
    assert!(ls.wait_for_diagnostics("b/src/lib.cairo").diagnostics.is_empty());

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());
    let analyzed_crates = normalize(&ls, analyzed_crates);

    ls.edit_file("Scarb.toml", &inputs["Scarb.toml with removed member"]);
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id("Scarb.toml").uri, typ: FileChangeType::CHANGED }],
    });
    ls.wait_for_project_update();

    let analyzed_crates_after_member_removal = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());
    let analyzed_crates_after_member_removal = normalize(&ls, analyzed_crates_after_member_removal);

    let analyzed_crates_diff =
        TextDiff::from_lines(&analyzed_crates, &analyzed_crates_after_member_removal)
            .unified_diff()
            .context_radius(15)
            .to_string();

    TestRunnerResult::success(OrderedHashMap::from([
        ("Analyzed crates".to_string(), analyzed_crates),
        ("Analyzed crates diff after member removal".to_string(), analyzed_crates_diff),
    ]))
}
