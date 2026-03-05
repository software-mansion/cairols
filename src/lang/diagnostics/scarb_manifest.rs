use std::collections::HashMap;
use std::path::Path;

use cairo_lang_filesystem::ids::FileId;
use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::{MetadataCommand, MetadataCommandError};
use serde::Deserialize;
use serde_json::Value;

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
    let diagnostics = diagnostics_from_metadata_error(metadata_result.err(), &root_url);
    if diagnostics.is_empty() {
        return Some((root_url, diagnostics_by_file));
    }

    for diagnostic in diagnostics {
        let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
        if !entry.contains(&diagnostic.diagnostic) {
            entry.push(diagnostic.diagnostic);
        }
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

fn diagnostics_from_metadata_error(
    error: Option<MetadataCommandError>,
    root_url: &Url,
) -> Vec<LspScarbDiagnostic> {
    let Some(error) = error else {
        return Vec::new();
    };

    match error {
        MetadataCommandError::ScarbError { stdout, .. } => {
            let diagnostics = extract_manifest_diagnostics_from_ndjson(&stdout, root_url);
            if !diagnostics.is_empty() {
                return diagnostics;
            }

            first_ndjson_error_message(&stdout)
                .or_else(|| non_empty_trimmed(&stdout))
                .map(|message| vec![build_diagnostic(root_url.clone(), message)])
                .unwrap_or_default()
        }
        MetadataCommandError::NotFound { stdout } => non_empty_trimmed(&stdout)
            .map(|message| vec![build_diagnostic(root_url.clone(), message)])
            .unwrap_or_else(|| {
                vec![build_diagnostic(
                    root_url.clone(),
                    "`scarb metadata` command did not produce metadata".to_string(),
                )]
            }),
        other => vec![build_diagnostic(root_url.clone(), other.to_string())],
    }
}

fn extract_manifest_diagnostics_from_ndjson(
    stdout: &str,
    root_url: &Url,
) -> Vec<LspScarbDiagnostic> {
    ndjson_values(stdout)
        .filter(|value| is_manifest_diagnostic_kind(value))
        .filter_map(|value| {
            let message = value.get("message").and_then(Value::as_str)?.trim().to_string();
            if message.is_empty() {
                return None;
            }

            let uri = diagnostic_uri(&value, root_url).unwrap_or_else(|| root_url.clone());
            Some(build_diagnostic(uri, message))
        })
        .collect()
}

fn is_manifest_diagnostic_kind(value: &Value) -> bool {
    fn is_manifest_kind(kind: &str) -> bool {
        let normalized = kind
            .chars()
            .filter(|char| char.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>();
        matches!(normalized.as_str(), "manifest" | "manifestdiagnostic" | "manifestdiagnostics")
    }

    value.get("kind").and_then(Value::as_str).is_some_and(is_manifest_kind)
        || value.get("type").and_then(Value::as_str).is_some_and(is_manifest_kind)
}

fn diagnostic_uri(value: &Value, root_url: &Url) -> Option<Url> {
    if let Some(uri) = value.get("uri").and_then(Value::as_str)
        && let Ok(uri) = Url::parse(uri)
    {
        return Some(uri);
    }

    for key in ["file", "path", "manifest_path"] {
        if let Some(path) = value.get(key).and_then(Value::as_str)
            && let Ok(uri) = Url::from_file_path(path)
        {
            return Some(uri);
        }
    }

    Some(root_url.clone())
}

fn build_diagnostic(uri: Url, message: String) -> LspScarbDiagnostic {
    LspScarbDiagnostic {
        uri,
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

#[derive(Deserialize)]
struct ScarbJsonMessage {
    #[serde(rename = "type")]
    message_type: Option<String>,
    message: Option<String>,
}

fn first_ndjson_error_message(stdout: &str) -> Option<String> {
    for value in ndjson_values(stdout) {
        let Ok(message) = serde_json::from_value::<ScarbJsonMessage>(value) else { continue };
        if message.message_type.as_deref() == Some("error")
            && let Some(message) = message.message
            && !message.trim().is_empty()
        {
            return Some(message);
        }
    }

    None
}

fn ndjson_values(stdout: &str) -> impl Iterator<Item = Value> + '_ {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
}

fn non_empty_trimmed(text: &str) -> Option<String> {
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
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

    #[test]
    fn extracts_all_manifest_diagnostics_from_ndjson() {
        let root_url = Url::parse("file:///tmp/Scarb.toml").unwrap();
        let stdout = indoc! {r#"
            {"type":"diagnostic","kind":"manifest-diagnostics","message":"first manifest issue"}
            {"type":"diagnostic","kind":"manifest","message":"second manifest issue"}
            {"type":"error","message":"generic failure"}
        "#};

        let diagnostics = diagnostics_from_metadata_error(
            Some(MetadataCommandError::ScarbError {
                stdout: stdout.to_string(),
                stderr: String::new(),
            }),
            &root_url,
        );

        let messages: Vec<_> =
            diagnostics.iter().map(|diag| diag.diagnostic.message.as_str()).collect();
        assert_eq!(messages, vec!["first manifest issue", "second manifest issue"]);
    }

    #[test]
    fn falls_back_to_error_when_no_manifest_diagnostics_found() {
        let root_url = Url::parse("file:///tmp/Scarb.toml").unwrap();
        let stdout = r#"{"type":"error","message":"fallback error"}"#;

        let diagnostics = diagnostics_from_metadata_error(
            Some(MetadataCommandError::ScarbError {
                stdout: stdout.to_string(),
                stderr: String::new(),
            }),
            &root_url,
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
