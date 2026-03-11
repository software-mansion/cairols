use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::{Metadata, MetadataCommand, MetadataCommandError};
use serde::Deserialize;
use serde_json::Value;

/// Collects diagnostics for a `Scarb.toml` file by re-running metadata validation.
pub fn collect_scarb_manifest_diagnostics(
    manifest_path: &Path,
    scarb_path: &Path,
) -> Option<HashMap<Url, Vec<Diagnostic>>> {
    let root_manifest_url = Url::from_file_path(manifest_path).ok()?;
    let mut diagnostics_by_file = HashMap::from([(root_manifest_url.clone(), Vec::new())]);

    let metadata_result = run_metadata_validation(manifest_path, scarb_path);
    if metadata_result.is_ok() {
        return None;
    }

    let diagnostics = diagnostics_from_metadata_error(metadata_result.unwrap_err(), manifest_path);
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
    value.get("file").and_then(Value::as_str).map(PathBuf::from)
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

fn ndjson_values(stdout: &str) -> impl Iterator<Item = Value> + '_ {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
}
