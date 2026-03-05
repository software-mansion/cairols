use std::collections::HashMap;
use std::path::Path;

use cairo_lang_filesystem::ids::FileId;
use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::{MetadataCommand, MetadataCommandError};
use serde::Deserialize;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

/// Collects diagnostics for a `Scarb.toml` file.
///
/// Caller must ensure `root_on_disk_file` points to a Scarb manifest and provide its on-disk path.
pub fn collect_scarb_manifest_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    root_on_disk_file: FileId<'db>,
    manifest_path: &'db Path,
    scarb_path: Option<&Path>,
) -> Option<(Url, HashMap<Url, Vec<Diagnostic>>)> {
    let root_url = db.url_for_file(root_on_disk_file)?;
    let mut diagnostics_by_file = HashMap::from([(root_url.clone(), Vec::new())]);

    let metadata_result = run_metadata_validation(manifest_path, scarb_path);
    let Some(diagnostic) = diagnostic_from_metadata_error(metadata_result.err(), &root_url) else {
        return Some((root_url, diagnostics_by_file));
    };

    let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
    if !entry.contains(&diagnostic.diagnostic) {
        entry.push(diagnostic.diagnostic);
    }

    Some((root_url, diagnostics_by_file))
}

fn run_metadata_validation(
    manifest_path: &Path,
    scarb_path: Option<&Path>,
) -> Result<(), MetadataCommandError> {
    let mut command = MetadataCommand::new();
    if let Some(scarb_path) = scarb_path {
        command.scarb_path(scarb_path);
    }

    command.manifest_path(manifest_path).no_deps().json().exec().map(|_| ())
}

struct LspScarbDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

fn diagnostic_from_metadata_error(
    error: Option<MetadataCommandError>,
    root_url: &Url,
) -> Option<LspScarbDiagnostic> {
    let error = error?;
    let message = message_from_metadata_error(error);

    Some(LspScarbDiagnostic {
        uri: root_url.clone(),
        diagnostic: Diagnostic {
            range: Range::default(),
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("scarb".to_string()),
            message,
            related_information: None,
            tags: None,
            data: None,
        },
    })
}

fn message_from_metadata_error(error: MetadataCommandError) -> String {
    match error {
        MetadataCommandError::ScarbError { stdout, .. } => {
            first_ndjson_error_message(&stdout).unwrap_or_else(|| stdout.trim().to_string())
        }
        MetadataCommandError::NotFound { stdout } => {
            if stdout.trim().is_empty() {
                "`scarb metadata` command did not produce metadata".to_string()
            } else {
                stdout.trim().to_string()
            }
        }
        other => other.to_string(),
    }
}

#[derive(Deserialize)]
struct ScarbJsonMessage {
    #[serde(rename = "type")]
    kind: Option<String>,
    message: Option<String>,
}

fn first_ndjson_error_message(stdout: &str) -> Option<String> {
    for line in stdout.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let Ok(message) = serde_json::from_str::<ScarbJsonMessage>(line) else {
            continue;
        };

        if message.kind.as_deref() == Some("error")
            && let Some(message) = message.message
            && !message.trim().is_empty()
        {
            return Some(message);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use indoc::indoc;
    use lsp_types::{DiagnosticSeverity, Url};
    use tempfile::tempdir;

    use super::collect_scarb_manifest_diagnostics;
    use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};

    #[test]
    fn collects_diagnostics_for_scarb_manifest() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(
            &path,
            indoc! {r#"
                [package]
                name = 1
                version = "0.1.0"
            "#},
        )
        .unwrap();

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, Some(&scarb_path()))
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert_eq!(root_url, uri);

        let diagnostics = diagnostics_by_file.get(&uri).cloned().unwrap_or_default();
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(
            |diag| !diag.message.is_empty() && diag.severity == Some(DiagnosticSeverity::ERROR)
        ));
    }

    #[test]
    fn collects_workspace_level_diagnostics_from_member_manifest() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        write_file(
            &root.join("Scarb.toml"),
            indoc! {r#"
                [package]
                name = "root_ws"
                version = "0.1.0"
                edition = "2024_07"

                [workspace]
                members = ["members/member_a", "members/member_b"]
            "#},
        );
        write_file(
            &root.join("members/member_a/Scarb.toml"),
            indoc! {r#"
                [package]
                name = "member_a"
                version = "0.1.0"
                edition = "2024_07"

                [[target.starknet-contract]]
                name = "dup_contract"
            "#},
        );
        write_file(
            &root.join("members/member_b/Scarb.toml"),
            indoc! {r#"
                [package]
                name = "member_b"
                version = "0.1.0"
                edition = "2024_07"

                [[target.starknet-contract]]
                name = "dup_contract"
            "#},
        );
        write_file(&root.join("members/member_a/src/lib.cairo"), "fn main() {}\n");
        write_file(&root.join("members/member_b/src/lib.cairo"), "fn main() {}\n");

        let db = AnalysisDatabase::new();
        let member_uri = Url::from_file_path(root.join("members/member_a/Scarb.toml")).unwrap();
        let file_id = db.file_for_url(&member_uri).unwrap();

        let member_manifest_path = root.join("members/member_a/Scarb.toml");
        let Some((_root_url, diagnostics_by_file)) = collect_scarb_manifest_diagnostics(
            &db,
            file_id,
            &member_manifest_path,
            Some(&scarb_path()),
        ) else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert!(diagnostics_by_file.values().flatten().any(|diag| {
            diag.message.contains("workspace contains duplicate target definitions")
        }));
    }

    #[test]
    fn avoids_duplicate_workspace_errors_when_manifest_validation_already_failed() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(
            &path,
            indoc! {r#"
                [workspace]
                members = []

                [dependencies]
                foo = "1.0.0"
            "#},
        )
        .unwrap();

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, Some(&scarb_path()))
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics = diagnostics_by_file.get(&uri).cloned().unwrap_or_default();
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn returns_no_diagnostics_for_valid_manifest() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(
            &path,
            indoc! {r#"
                [package]
                name = "manifest_diagnostics_ws"
                version = "0.1.0"
                edition = "2024_07"
            "#},
        )
        .unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(dir.path().join("src/lib.cairo"), "fn main() {}\n").unwrap();

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, Some(&scarb_path()))
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert!(diagnostics_by_file.get(&uri).is_some_and(|diagnostics| diagnostics.is_empty()));
    }

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    fn scarb_path() -> PathBuf {
        which::which("scarb").expect("running tests requires a `scarb` binary available in `PATH`")
    }
}
