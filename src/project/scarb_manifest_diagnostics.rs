use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::{Metadata, MetadataCommand, MetadataCommandError};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScarbMetadataMessage {
    // Represents an "error" kind message from the metadata command
    MetadataError(MetadataError),
    // Represents a "manifest_diagnostic" kind message from the metadata command
    MetadataDiagnostic(MetadataDiagnostic),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataDiagnostic {
    pub path: PathBuf,
    pub message: String,
}

impl ScarbMetadataMessage {
    fn into_lsp(self, root_manifest_path: &Path) -> Option<LspScarbDiagnostic> {
        match self {
            ScarbMetadataMessage::MetadataError(message) => message.into_lsp(root_manifest_path),
            ScarbMetadataMessage::MetadataDiagnostic(message) => message.into_lsp(),
        }
    }
}

fn diagnostics_to_display(all_messages: Vec<ScarbMetadataMessage>) -> Vec<ScarbMetadataMessage> {
    let metadata_diagnostics: Vec<ScarbMetadataMessage> = all_messages
        .iter()
        .filter(|&message| matches!(message, ScarbMetadataMessage::MetadataDiagnostic(_)))
        .cloned()
        .collect();

    if !metadata_diagnostics.is_empty() { metadata_diagnostics } else { all_messages }
}

pub fn scarb_metadata_messages_contain_only_errors(messages: &[ScarbMetadataMessage]) -> bool {
    !messages.is_empty()
        && messages.iter().all(|message| matches!(message, ScarbMetadataMessage::MetadataError(_)))
}

pub fn scarb_metadata_messages_to_diagnostics(
    messages: Vec<ScarbMetadataMessage>,
    root_manifest_path: &Path,
) -> Option<HashMap<Url, Vec<Diagnostic>>> {
    let root_manifest_url = Url::from_file_path(root_manifest_path).ok()?;
    let mut diagnostics_by_file = HashMap::from([(root_manifest_url, Vec::new())]);

    for diagnostic in diagnostics_to_display(messages)
        .into_iter()
        .filter_map(|message| message.into_lsp(root_manifest_path))
    {
        let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
        if !entry.contains(&diagnostic.diagnostic) {
            entry.push(diagnostic.diagnostic);
        }
    }

    Some(diagnostics_by_file)
}

impl MetadataError {
    fn into_lsp(self, root_manifest_path: &Path) -> Option<LspScarbDiagnostic> {
        Url::from_file_path(root_manifest_path)
            .ok()
            .map(|uri| LspScarbDiagnostic { uri, diagnostic: build_diagnostic(self.message) })
    }
}

impl MetadataDiagnostic {
    fn into_lsp(self) -> Option<LspScarbDiagnostic> {
        Url::from_file_path(self.path)
            .ok()
            .map(|uri| LspScarbDiagnostic { uri, diagnostic: build_diagnostic(self.message) })
    }
}

impl TryFrom<&Value> for ScarbMetadataMessage {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if is_error_kind(value) {
            let message = diagnostic_message(value).ok_or(())?;
            if message.is_empty() {
                return Err(());
            }

            return Ok(ScarbMetadataMessage::MetadataError(MetadataError { message }));
        }

        if is_manifest_diagnostic_kind(value) {
            let message = diagnostic_message(value).ok_or(())?;
            if message.is_empty() {
                return Err(());
            }

            return Ok(ScarbMetadataMessage::MetadataDiagnostic(MetadataDiagnostic {
                path: diagnostic_path(value).ok_or(())?,
                message,
            }));
        }

        Err(())
    }
}

/// Collects diagnostics for a `Scarb.toml` file by re-running metadata validation.
pub fn collect_scarb_manifest_diagnostics(
    manifest_path: &Path,
    scarb_path: &Path,
) -> Option<Vec<ScarbMetadataMessage>> {
    let metadata_result = run_metadata_validation(manifest_path, scarb_path);
    if metadata_result.is_ok() {
        return None;
    }

    Some(metadata_messages_from_error(metadata_result.unwrap_err()))
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

fn metadata_messages_from_error(error: MetadataCommandError) -> Vec<ScarbMetadataMessage> {
    match error {
        MetadataCommandError::ScarbError { stdout, .. } => metadata_messages_from_ndjson(&stdout),
        MetadataCommandError::NotFound { stdout } => {
            vec![ScarbMetadataMessage::MetadataError(MetadataError { message: stdout })]
        }
        other => {
            vec![ScarbMetadataMessage::MetadataError(MetadataError { message: other.to_string() })]
        }
    }
}

fn metadata_messages_from_ndjson(stdout: &str) -> Vec<ScarbMetadataMessage> {
    ndjson_values(stdout).filter_map(|value| ScarbMetadataMessage::try_from(&value).ok()).collect()
}

fn is_error_kind(value: &Value) -> bool {
    value.get("type").and_then(Value::as_str).is_some_and(|value: &str| value == "error")
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
    value.get("file").and_then(Value::as_str).map(PathBuf::from)
}

fn build_diagnostic(message: String) -> Diagnostic {
    Diagnostic {
        range: Range::default(),
        severity: Some(DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("scarb".to_string()),
        message,
        related_information: None,
        tags: None,
        data: None,
    }
}

fn ndjson_values(stdout: &str) -> impl Iterator<Item = Value> + '_ {
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
    use lsp_types::DiagnosticSeverity;
    use tempfile::tempdir;

    use super::{
        MetadataDiagnostic, MetadataError, ScarbMetadataMessage,
        collect_scarb_manifest_diagnostics, metadata_messages_from_ndjson,
        scarb_metadata_messages_contain_only_errors, scarb_metadata_messages_to_diagnostics,
    };

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

        let Some(messages) = collect_scarb_manifest_diagnostics(&path, &scarb_path()) else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics_by_file = scarb_metadata_messages_to_diagnostics(messages, &path).unwrap();
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

        let Some(messages) = collect_scarb_manifest_diagnostics(
            &root.join("members/member_a/Scarb.toml"),
            &scarb_path(),
        ) else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics_by_file = scarb_metadata_messages_to_diagnostics(
            messages,
            &root.join("members/member_a/Scarb.toml"),
        )
        .unwrap();

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

        let Some(messages) = collect_scarb_manifest_diagnostics(&path, &scarb_path()) else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics_by_file = scarb_metadata_messages_to_diagnostics(messages, &path).unwrap();
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

        let diagnostics_by_file = collect_scarb_manifest_diagnostics(&path, &scarb_path());
        assert!(diagnostics_by_file.is_none());
    }

    #[test]
    fn extracts_all_metadata_messages_from_ndjson() {
        let stdout = indoc! {r#"
            {"type":"diagnostic","kind":"manifest_diagnostic","file":"/tmp/Scarb.toml","message":"first manifest issue"}
            {"type":"diagnostic","kind":"manifest_diagnostic","file":"/tmp/Scarb.toml","message":"second manifest issue"}
            {"type":"error","message":"generic failure"}
        "#};

        let diagnostics = metadata_messages_from_ndjson(stdout);

        assert_eq!(
            diagnostics,
            vec![
                ScarbMetadataMessage::MetadataDiagnostic(MetadataDiagnostic {
                    path: PathBuf::from("/tmp/Scarb.toml"),
                    message: "first manifest issue".to_string(),
                }),
                ScarbMetadataMessage::MetadataDiagnostic(MetadataDiagnostic {
                    path: PathBuf::from("/tmp/Scarb.toml"),
                    message: "second manifest issue".to_string(),
                }),
                ScarbMetadataMessage::MetadataError(MetadataError {
                    message: "generic failure".to_string(),
                }),
            ]
        );
    }

    #[test]
    fn extracts_metadata_error_when_no_manifest_diagnostics_found() {
        let stdout = r#"{"type":"error","message":"fallback error"}"#;

        let diagnostics = metadata_messages_from_ndjson(stdout);

        assert_eq!(
            diagnostics,
            vec![ScarbMetadataMessage::MetadataError(MetadataError {
                message: "fallback error".to_string(),
            })]
        );
    }

    #[test]
    fn notification_should_only_show_for_error_only_output() {
        assert!(scarb_metadata_messages_contain_only_errors(&[
            ScarbMetadataMessage::MetadataError(MetadataError { message: "error".to_string() },)
        ]));

        assert!(!scarb_metadata_messages_contain_only_errors(&[
            ScarbMetadataMessage::MetadataDiagnostic(MetadataDiagnostic {
                path: PathBuf::from("/tmp/Scarb.toml"),
                message: "manifest issue".to_string(),
            }),
            ScarbMetadataMessage::MetadataError(MetadataError { message: "error".to_string() }),
        ]));

        assert!(!scarb_metadata_messages_contain_only_errors(&[]));
    }

    #[test]
    fn diagnostics_hide_metadata_errors_when_manifest_diagnostics_exist() {
        let path = PathBuf::from("/tmp/Scarb.toml");
        let diagnostics_by_file = scarb_metadata_messages_to_diagnostics(
            vec![
                ScarbMetadataMessage::MetadataDiagnostic(MetadataDiagnostic {
                    path: path.clone(),
                    message: "manifest issue".to_string(),
                }),
                ScarbMetadataMessage::MetadataError(MetadataError {
                    message: "generic failure".to_string(),
                }),
            ],
            &path,
        )
        .unwrap();

        let diagnostics: Vec<_> = diagnostics_by_file.values().flatten().collect();
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].message, "manifest issue");
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
