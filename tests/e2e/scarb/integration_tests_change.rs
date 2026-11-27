use std::fs;

use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{DidChangeWatchedFilesParams, FileChangeType, FileEvent};
use serde::Serialize;
use similar::TextDiff;

use super::scarb_toml_change::caps;
use crate::support::normalize::normalize;
use crate::support::{MockClient, sandbox};

#[derive(Serialize)]
struct IntegrationTestsChangeResult {
    analyzed_crates: String,
    diff_after_creating_test_file_without_lib: String,
    diff_after_creating_tests_lib: String,
    diff_after_deleting_tests_lib: String,
}

#[test]
fn integration_tests_structure_change_reloads_project_model() {
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

    // `tests/*.cairo` is created and there is no `tests/lib.cairo`.
    ls.edit_file("tests/a.cairo", "#[test]\nfn a() {}");
    let after_creating_test_file =
        notify_and_wait_for_project_update(&mut ls, "tests/a.cairo", FileChangeType::CREATED);

    // `tests/lib.cairo` is created.
    ls.edit_file("tests/lib.cairo", "mod a;");
    let after_creating_tests_lib =
        notify_and_wait_for_project_update(&mut ls, "tests/lib.cairo", FileChangeType::CREATED);

    // `tests/lib.cairo` is deleted.
    let tests_lib_url = ls.doc_id("tests/lib.cairo").uri;
    fs::remove_file(ls.fixture.file_absolute_path("tests/lib.cairo")).unwrap();
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: tests_lib_url, typ: FileChangeType::DELETED }],
    });
    let after_deleting_tests_lib = ls.wait_for_project_update();
    let after_deleting_tests_lib = normalize(&ls, after_deleting_tests_lib);

    insta::assert_toml_snapshot!(IntegrationTestsChangeResult {
        diff_after_creating_test_file_without_lib: diff(
            &analyzed_crates,
            &after_creating_test_file
        ),
        diff_after_creating_tests_lib: diff(&after_creating_test_file, &after_creating_tests_lib),
        diff_after_deleting_tests_lib: diff(&after_creating_tests_lib, &after_deleting_tests_lib),
        analyzed_crates,
    });
}

fn notify_and_wait_for_project_update(
    ls: &mut MockClient,
    path: &str,
    typ: FileChangeType,
) -> String {
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id(path).uri, typ }],
    });
    let analyzed_crates = ls.wait_for_project_update();
    normalize(ls, analyzed_crates)
}

fn diff(before: &str, after: &str) -> String {
    TextDiff::from_lines(before, after).unified_diff().context_radius(15).to_string()
}
