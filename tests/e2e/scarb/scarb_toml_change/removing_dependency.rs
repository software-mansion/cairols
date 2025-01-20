use cairo_language_server::lsp;
use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{DidChangeWatchedFilesParams, FileChangeType, FileEvent};
use similar::TextDiff;

use super::{AnalyzedCratesResult, caps};
use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn test_removing_dependency() {
    let mut ls = sandbox! {
        files {
            "a/Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                b = { path = "../b" }
            "#),
            "a/src/lib.cairo" => "",
            "b/Scarb.toml" => indoc!(r#"
                [package]
                name = "b"
                version = "0.1.0"
                edition = "2024_07"
            "#),
            "b/src/lib.cairo" => "",
        }
        client_capabilities = caps;
    };

    assert!(ls.open_and_wait_for_diagnostics("a/src/lib.cairo").diagnostics.is_empty());
    // Check if opening `a` triggers calculating diagnostics for `b`.
    assert!(ls.wait_for_diagnostics("b/src/lib.cairo").diagnostics.is_empty());

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());
    let analyzed_crates = normalize(&ls, analyzed_crates);

    ls.edit_file(
        "a/Scarb.toml",
        indoc!(
            r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"
            "#,
        ),
    );
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
    let analyzed_crates_after_dep_removal = normalize(&ls, analyzed_crates_after_dep_removal);

    let analyzed_crates_diff =
        TextDiff::from_lines(&analyzed_crates, &analyzed_crates_after_dep_removal)
            .unified_diff()
            .context_radius(5)
            .to_string();

    insta::assert_toml_snapshot!(AnalyzedCratesResult { analyzed_crates, analyzed_crates_diff })
}
