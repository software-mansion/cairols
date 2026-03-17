use std::fmt::{self, Display};

use indoc::indoc;
use lsp_types::notification::DidChangeWatchedFiles;
use lsp_types::{
    ClientCapabilities, DidChangeWatchedFilesClientCapabilities, DidChangeWatchedFilesParams,
    FileChangeType, FileEvent, WorkspaceClientCapabilities,
};
use serde::Serialize;

use crate::support::diagnostics::{
    DiagnosticAndRelatedInfo, DiagnosticsWithUrl, get_related_diagnostic_code,
};
use crate::support::normalize::{normalize, normalize_diagnostics};
use crate::support::{MockClient, sandbox};

#[derive(Debug, Serialize)]
struct DiagnosticsReport {
    diagnostics: Vec<DiagnosticsWithUrl>,
}

#[derive(Debug, Serialize)]
struct DiagnosticsChangeReport {
    before_changes: DiagnosticsReport,
    after_changes: DiagnosticsReport,
}

impl Display for DiagnosticsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = serde_yaml::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&output)
    }
}

impl Display for DiagnosticsChangeReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = serde_yaml::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&output)
    }
}

#[test]
fn invalid_manifest_reports_diagnostics() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = 1
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => "fn main() {}\n",
        }
    };

    ls.open_and_wait_for_project_update("src/lib.cairo");

    insta::assert_snapshot!(diagnostics_report(&mut ls, &["Scarb.toml"]));
}

#[test]
fn workspace_manifest_diagnostics_from_member_manifest() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "root_ws"
                version = "0.1.0"
                edition = "2025_12"

                [workspace]
                members = ["members/member_a", "members/member_b"]
            "#},
            "members/member_a/Scarb.toml" => indoc! {r#"
                [package]
                name = "member_a"
                version = "0.1.0"
                edition = "2025_12"

                [[target.starknet-contract]]
                name = "dup_contract"
            "#},
            "members/member_a/src/lib.cairo" => "fn main() {}\n",
            "members/member_b/Scarb.toml" => indoc! {r#"
                [package]
                name = "member_b"
                version = "0.1.0"
                edition = "2025_12"

                [[target.starknet-contract]]
                name = "dup_contract"
            "#},
            "members/member_b/src/lib.cairo" => "fn main() {}\n",
        }
    };

    ls.open_and_wait_for_project_update("members/member_a/src/lib.cairo");

    insta::assert_snapshot!(diagnostics_report(
        &mut ls,
        &["Scarb.toml", "members/member_a/Scarb.toml", "members/member_b/Scarb.toml"]
    ));
}

#[test]
fn path_dependency_manifest_diagnostics_from_workspace_member() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [workspace]
                members = ["app"]
            "#},
            "app/Scarb.toml" => indoc! {r#"
                [package]
                name = "app"
                version = "0.1.0"
                edition = "2025_12"

                [dependencies]
                dep = { path = "../dep" }
            "#},
            "app/src/lib.cairo" => "fn main() {}\n",
            "dep/Scarb.toml" => indoc! {r#"
                [package]
                name = 1
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "dep/src/lib.cairo" => "fn main() {}\n",
        }
    };

    ls.open_and_wait_for_project_update("Scarb.toml");
    ls.open_and_wait_for_project_update("app/Scarb.toml");
    ls.open_and_wait_for_project_update("dep/Scarb.toml");

    insta::assert_snapshot!(diagnostics_report(
        &mut ls,
        &["Scarb.toml", "app/Scarb.toml", "dep/Scarb.toml"]
    ));
}

#[test]
fn manifest_edit_introduces_workspace_error() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "root_ws"
                version = "0.1.0"
                edition = "2025_12"

                [workspace]
                members = ["members/member_a", "members/member_b"]
            "#},
            "members/member_a/Scarb.toml" => indoc! {r#"
                [package]
                name = "member_a"
                version = "0.1.0"
                edition = "2025_12"

                [[target.starknet-contract]]
                name = "contract_a"
            "#},
            "members/member_a/src/lib.cairo" => "fn main() {}\n",
            "members/member_b/Scarb.toml" => indoc! {r#"
                [package]
                name = "member_b"
                version = "0.1.0"
                edition = "2025_12"

                [[target.starknet-contract]]
                name = "contract_b"
            "#},
            "members/member_b/src/lib.cairo" => "fn main() {}\n",
        }
        client_capabilities = caps;
    };

    ls.open_and_wait_for_project_update("members/member_a/src/lib.cairo");

    ls.edit_file(
        "members/member_b/Scarb.toml",
        indoc! {r#"
            [package]
            name = "member_b"
            version = "0.1.0"
            edition = "2025_12"

            [[target.starknet-contract]]
            name = "contract_a"
        "#},
    );
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: ls.doc_id("members/member_b/Scarb.toml").uri,
            typ: FileChangeType::CHANGED,
        }],
    });
    ls.wait_for_project_update();

    insta::assert_snapshot!(diagnostics_report(
        &mut ls,
        &["Scarb.toml", "members/member_a/Scarb.toml", "members/member_b/Scarb.toml"]
    ));
}

#[test]
fn fixing_workspace_error_clears_member_manifest_diagnostics() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "root_ws"
                version = "0.1.0"
                edition = "2025_12"

                [workspace]
                members = ["members/member_a", "members/member_b"]
            "#},
            "members/member_a/Scarb.toml" => indoc! {r#"
                [package]
                name = "member_a"
                version = "0.1.0"
                edition = "2025_12"

                [[target.starknet-contract]]
                name = "contract_a"
            "#},
            "members/member_a/src/lib.cairo" => "fn main() {}\n",
            "members/member_b/Scarb.toml" => indoc! {r#"
                [package]
                name = "member_b"
                version = "0.1.0"
                edition = "2025_12"

                [[target.starknet-contract]]
                name = "contract_b"
            "#},
            "members/member_b/src/lib.cairo" => "fn main() {}\n",
        }
        client_capabilities = caps;
    };

    ls.open_and_wait_for_project_update("members/member_a/src/lib.cairo");

    ls.edit_file(
        "members/member_b/Scarb.toml",
        indoc! {r#"
            [package]
            name = "member_b"
            version = "0.1.0"
            edition = "2025_12"

            [[target.starknet-contract]]
            name = "contract_a"
        "#},
    );
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: ls.doc_id("members/member_b/Scarb.toml").uri,
            typ: FileChangeType::CHANGED,
        }],
    });
    ls.wait_for_project_update();

    let before_changes = diagnostics_report(
        &mut ls,
        &["Scarb.toml", "members/member_a/Scarb.toml", "members/member_b/Scarb.toml"],
    );

    ls.edit_file(
        "members/member_b/Scarb.toml",
        indoc! {r#"
            [package]
            name = "member_b"
            version = "0.1.0"
            edition = "2025_12"

            [[target.starknet-contract]]
            name = "contract_b"
        "#},
    );
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent {
            uri: ls.doc_id("members/member_b/Scarb.toml").uri,
            typ: FileChangeType::CHANGED,
        }],
    });
    ls.wait_for_project_update();

    let after_changes = diagnostics_report(
        &mut ls,
        &["Scarb.toml", "members/member_a/Scarb.toml", "members/member_b/Scarb.toml"],
    );

    insta::assert_snapshot!(DiagnosticsChangeReport { before_changes, after_changes });
}

#[test]
fn watched_manifest_fix_clears_manifest_diagnostics() {
    let invalid_manifest = indoc! {r#"
        [package]
        name = "test_package"
        version = "0.1.0"
        edition = "2025_12"

        [dependencies]
        dep = 1
    "#};
    let fixed_manifest = indoc! {r#"
        [package]
        name = "test_package"
        version = "0.1.0"
        edition = "2025_12"

        [dependencies]
        # dep = 1
    "#};

    let mut ls = sandbox! {
        files {
            "Scarb.toml" => invalid_manifest,
            "src/lib.cairo" => "fn main() {}\n",
        }
        client_capabilities = caps;
    };

    ls.open_and_wait_for_project_update("src/lib.cairo");
    let before_changes = diagnostics_report(&mut ls, &["Scarb.toml"]);

    ls.edit_file("Scarb.toml", fixed_manifest);
    ls.send_notification::<DidChangeWatchedFiles>(DidChangeWatchedFilesParams {
        changes: vec![FileEvent { uri: ls.doc_id("Scarb.toml").uri, typ: FileChangeType::CHANGED }],
    });
    ls.wait_for_project_update();

    let after_changes = diagnostics_report(&mut ls, &["Scarb.toml"]);

    insta::assert_snapshot!(DiagnosticsChangeReport { before_changes, after_changes });
}

fn diagnostics_report(ls: &mut MockClient, paths: &[&str]) -> DiagnosticsReport {
    let diagnostics =
        paths.iter().map(|path| (ls.doc_id(path).uri, ls.get_diagnostics_for_file(path)));
    let normalized = normalize_diagnostics(ls, diagnostics);
    let mut report = Vec::new();

    for (original_url, normalized_url, diagnostics) in normalized {
        if diagnostics.is_empty() {
            continue;
        }

        let mut mapped_diagnostics = Vec::new();
        for mut diagnostic in diagnostics {
            diagnostic.message = normalize(&*ls, &diagnostic.message);
            let related_code = get_related_diagnostic_code(ls, &diagnostic, &original_url);
            mapped_diagnostics.push(DiagnosticAndRelatedInfo { related_code, diagnostic });
        }

        report.push(DiagnosticsWithUrl { url: normalized_url, diagnostics: mapped_diagnostics });
    }

    DiagnosticsReport { diagnostics: report }
}

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
