use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{DidChangeWatchedFilesParams, FileChangeType, FileEvent};

use super::caps;
use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn newly_created_integration_test_file_reloads_project_model() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "test_package"
                version = "0.1.0"
                edition = "2025_12"
            "#),
            "src/lib.cairo" => "",
        }
        client_capabilities = caps;
    };

    let analyzed_crates = ls.open_and_wait_for_project_update("src/lib.cairo");
    let analyzed_crates = normalize(&ls, analyzed_crates);
    assert!(!analyzed_crates.contains("[ROOT]/tests/new_test.cairo"), "{analyzed_crates}");

    ls.edit_file(
        "tests/new_test.cairo",
        indoc! {r#"
            #[test]
            fn new_test() {}
        "#},
    );
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: ls.doc_id("tests/new_test.cairo").uri,
            typ: FileChangeType::CREATED,
        }],
    });

    let analyzed_crates_after_test_creation = ls.wait_for_project_update();
    let analyzed_crates_after_test_creation = normalize(&ls, analyzed_crates_after_test_creation);
    assert!(
        analyzed_crates_after_test_creation.contains("[ROOT]/tests/new_test.cairo"),
        "{analyzed_crates_after_test_creation}"
    );
}
