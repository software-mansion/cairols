use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_language_server::lsp;
use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{
    ClientCapabilities, DidChangeWatchedFilesClientCapabilities, DidChangeWatchedFilesParams,
    FileChangeType, FileEvent, WorkspaceClientCapabilities,
};

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

cairo_lang_test_utils::test_file_test!(
    removing_dependency,
    "tests/test_data/scarb",
    {
        removing_dependency: "removing_dependency.txt"
    },
    test_removing_dependency
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

    ls.edit_file("Scarb.toml", indoc! {r#"
        [workspace]
        members = [
            "a",
        ]
    "#
    });
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id("Scarb.toml").uri, typ: FileChangeType::CHANGED }],
    });
    ls.wait_for_project_update();

    let analyzed_crates_after_member_removal = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    TestRunnerResult::success(OrderedHashMap::from([
        ("Analyzed crates".to_string(), normalize(&ls, analyzed_crates)),
        (
            "Analyzed crates after member removal".to_string(),
            normalize(&ls, analyzed_crates_after_member_removal),
        ),
    ]))
}

fn test_removing_dependency(
    inputs: &OrderedHashMap<String, String>,
    _args: &OrderedHashMap<String, String>,
) -> TestRunnerResult {
    let mut ls = sandbox! {
        files {
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

    ls.edit_file("a/Scarb.toml", indoc! {r#"
        [package]
        name = "a"
        version = "0.1.0"
        edition = "2024_07"
    "#
    });

    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: ls.doc_id("a/Scarb.toml").uri,
            typ: FileChangeType::CHANGED,
        }],
    });
    ls.wait_for_project_update();

    // FIXME(#90): `b` should disappear from the project model - `CrateId` representing `b`
    //  should be removed from db.crate_configs().
    let analyzed_crates_after_dep_removal = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    TestRunnerResult::success(OrderedHashMap::from([
        ("Analyzed crates".to_string(), normalize(&ls, analyzed_crates)),
        (
            "Analyzed crates after dependency removal".to_string(),
            normalize(&ls, analyzed_crates_after_dep_removal),
        ),
    ]))
}

#[test]
fn test_invalid_scarb_toml_change() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! { r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"
            "#},
            "src/lib.cairo" => "",
        }
        client_capabilities = caps;
    };

    assert!(ls.open_and_wait_for_diagnostics("src/lib.cairo").diagnostics.is_empty());

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    ls.edit_file("Scarb.toml", indoc! {r#"
        [package]
        version = "0.1.0"
        edition = "2024_07"
    "#
    });

    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id("Scarb.toml").uri, typ: FileChangeType::CHANGED }],
    });
    ls.wait_for_project_update();

    let analyzed_crates_after_failed_metadata = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    pretty_assertions::assert_eq!(analyzed_crates, analyzed_crates_after_failed_metadata);
}
