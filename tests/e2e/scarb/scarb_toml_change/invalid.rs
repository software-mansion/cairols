use cairo_language_server::lsp;
use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{DidChangeWatchedFilesParams, FileChangeType, FileEvent};

use super::caps;
use crate::support::sandbox;

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

    assert!(ls.open_and_wait_for_diagnostics("src/lib.cairo").is_empty());

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    ls.edit_file(
        "Scarb.toml",
        indoc! {r#"
            [package]
            version = "0.1.0"
            edition = "2024_07"
        "#},
    );

    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id("Scarb.toml").uri, typ: FileChangeType::CHANGED }],
    });
    ls.wait_for_project_update();

    let analyzed_crates_after_failed_metadata = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    assert_eq!(analyzed_crates, analyzed_crates_after_failed_metadata);
}
