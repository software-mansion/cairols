use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cairo_lang_filesystem::ids::FileId;
use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::{Metadata, MetadataCommand, MetadataCommandError};
use serde::Deserialize;
use serde_json::Value;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

/// Collects diagnostics for a `Scarb.toml` file.
pub fn collect_scarb_manifest_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    manifest_file: FileId<'db>,
    scarb_path: &Path,
) -> Option<HashMap<Url, Vec<Diagnostic>>> {
    let root_manifest_url: Url = db.url_for_file(manifest_file)?;
    let root_manifest_path_buf: PathBuf = root_manifest_url.to_file_path().ok()?;
    let root_manifest_path: &Path = root_manifest_path_buf.as_path();
    let mut diagnostics_by_file = HashMap::from([(root_manifest_url.clone(), Vec::new())]);

    let metadata_result = run_metadata_validation(root_manifest_path, scarb_path);
    if metadata_result.is_ok() {
        return None;
    }

    let diagnostics =
        diagnostics_from_metadata_error(metadata_result.unwrap_err(), root_manifest_path);
    if diagnostics.is_empty() {
        return Some(diagnostics_by_file);
    }

    for diagnostic in diagnostics {
        let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
        if !entry.contains(&diagnostic.diagnostic) {
            entry.push(diagnostic.diagnostic);
        }
    }

    Some(diagnostics_by_file)
}

fn run_metadata_validation(
    manifest_path: &Path,
    scarb_path: &Path,
) -> Result<Metadata, MetadataCommandError> {
    let mut command = MetadataCommand::new();
    command.scarb_path(scarb_path);
    command.manifest_path(manifest_path).no_deps().json().exec()
}

struct LspScarbDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

fn diagnostics_from_metadata_error(
    error: MetadataCommandError,
    root_manifest_path: &Path,
) -> Vec<LspScarbDiagnostic> {
    match error {
        MetadataCommandError::ScarbError { stdout, .. } => {
            let diagnostics = extract_manifest_diagnostics_from_ndjson(&stdout);
            if !diagnostics.is_empty() {
                return diagnostics;
            }

            // Fallback to an error message (should always be present) if no manifest diagnostics found.
            first_ndjson_error_message(&stdout)
                .map(|message| vec![build_diagnostic(root_manifest_path, message)])
                .unwrap_or_default()
        }
        MetadataCommandError::NotFound { stdout } => {
            vec![build_diagnostic(root_manifest_path, stdout)]
        }
        other => vec![build_diagnostic(root_manifest_path, other.to_string())],
    }
}

fn extract_manifest_diagnostics_from_ndjson(stdout: &str) -> Vec<LspScarbDiagnostic> {
    ndjson_values(stdout)
        .filter(is_manifest_diagnostic_kind)
        .filter_map(|value| {
            let message = diagnostic_message(&value)?;
            if message.is_empty() {
                return None;
            }

            Some(build_diagnostic(diagnostic_path(&value)?, message))
        })
        .collect()
}

fn is_manifest_diagnostic_kind(value: &Value) -> bool {
    value
        .get("kind")
        .and_then(Value::as_str)
        .is_some_and(|value: &str| value == "manifest_diagnostic")
}

fn diagnostic_message(value: &Value) -> Option<String> {
    let msg = value.get("message").and_then(Value::as_str)?;
    Some(msg.to_string())
}

fn diagnostic_path(value: &Value) -> Option<PathBuf> {
    if let Some(path_str) = value.get("file").and_then(Value::as_str) {
        return Some(PathBuf::from(path_str));
    }

    None
}

fn build_diagnostic(manifest_path: impl AsRef<Path>, message: String) -> LspScarbDiagnostic {
    LspScarbDiagnostic {
        uri: Url::from_file_path(manifest_path).unwrap(),
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
    }
}

// This is misaligned right now in scarb (manifest diagnostics have a different format)
#[derive(Deserialize)]
struct ScarbJsonErrorMessage {
    #[serde(rename = "type")]
    message_type: Option<String>,
    message: Option<String>,
}

fn first_ndjson_error_message(stdout: &str) -> Option<String> {
    for value in ndjson_values(stdout) {
        let Ok(message) = serde_json::from_value::<ScarbJsonErrorMessage>(value) else { continue };
        if message.message_type.as_deref() == Some("error")
            && let Some(message) = message.message
        {
            return Some(message);
        }
    }

    None
}

fn ndjson_values(stdout: &str) -> impl Iterator<Item = Value> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use indoc::indoc;
    use lsp_types::{DiagnosticSeverity, Url};
    use scarb_metadata::MetadataCommandError;
    use tempfile::tempdir;

    use super::{collect_scarb_manifest_diagnostics, diagnostics_from_metadata_error};
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

        let Some(diagnostics_by_file) =
            collect_scarb_manifest_diagnostics(&db, file_id, &scarb_path())
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics: Vec<_> = diagnostics_by_file.values().flatten().collect();
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

        let Some(diagnostics_by_file) =
            collect_scarb_manifest_diagnostics(&db, file_id, &scarb_path())
        else {
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

        let Some(diagnostics_by_file) =
            collect_scarb_manifest_diagnostics(&db, file_id, &scarb_path())
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics_count = diagnostics_by_file.values().flatten().count();
        assert_eq!(diagnostics_count, 1);
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

        let diagnostics_by_file = collect_scarb_manifest_diagnostics(&db, file_id, &scarb_path());
        assert!(diagnostics_by_file.is_none());
    }

    #[test]
    fn extracts_all_manifest_diagnostics_from_ndjson() {
        let root_path = PathBuf::from("/tmp/Scarb.toml");
        let stdout = indoc! {r#"
            {"type":"diagnostic","kind":"manifest_diagnostic","file":"/tmp/Scarb.toml","message":"first manifest issue"}
            {"type":"diagnostic","kind":"manifest_diagnostic","file":"/tmp/Scarb.toml","message":"second manifest issue"}
            {"type":"error","message":"generic failure"}
        "#};

        let diagnostics = diagnostics_from_metadata_error(
            MetadataCommandError::ScarbError { stdout: stdout.to_string(), stderr: String::new() },
            &root_path,
        );

        let messages: Vec<_> =
            diagnostics.iter().map(|diag| diag.diagnostic.message.as_str()).collect();
        assert_eq!(messages, vec!["first manifest issue", "second manifest issue"]);
    }

    #[test]
    fn falls_back_to_error_when_no_manifest_diagnostics_found() {
        let root_path = PathBuf::from("/tmp/Scarb.toml");
        let stdout = r#"{"type":"error","message":"fallback error"}"#;

        let diagnostics = diagnostics_from_metadata_error(
            MetadataCommandError::ScarbError { stdout: stdout.to_string(), stderr: String::new() },
            &root_path,
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].diagnostic.message, "fallback error");
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
