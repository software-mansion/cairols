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

impl Display for DiagnosticsReport {
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
