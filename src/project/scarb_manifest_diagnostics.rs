use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::MetadataCommandError;
use serde::Deserialize;
use serde_json::Value;

/// Collects diagnostics for `Scarb.toml`s from a failed metadata invocation.
pub fn collect_scarb_manifests_diagnostics(
    error: MetadataCommandError,
    root_manifest_path: &Path,
) -> HashMap<Url, Vec<Diagnostic>> {
    match error {
        MetadataCommandError::ScarbError { stdout, .. } => {
            let diagnostics = extract_manifest_diagnostics_from_ndjson(&stdout);
            if diagnostics.is_empty() {
                // Fallback to an error message (should always be present) if no manifest diagnostics found.
                first_ndjson_error_message(&stdout)
                    .map(|message| vec![build_single_file_diagnostic(root_manifest_path, message)])
                    .unwrap_or_default();
            }

            diagnostics
        }
        MetadataCommandError::NotFound { stdout } => {
            HashMap::from_iter(vec![build_single_file_diagnostic(root_manifest_path, stdout)])
        }
        other => HashMap::from_iter(vec![build_single_file_diagnostic(
            root_manifest_path,
            other.to_string(),
        )]),
    }
}

fn extract_manifest_diagnostics_from_ndjson(stdout: &str) -> HashMap<Url, Vec<Diagnostic>> {
    ndjson_values(stdout)
        .filter(is_manifest_diagnostic_kind)
        .filter_map(|value| {
            let message = diagnostic_message(&value)?;
            if message.is_empty() {
                return None;
            }

            Some(build_single_file_diagnostic(diagnostic_path(&value)?, message))
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

fn build_single_file_diagnostic(
    manifest_path: impl AsRef<Path>,
    message: String,
) -> (Url, Vec<Diagnostic>) {
    (
        Url::from_file_path(manifest_path).unwrap(),
        vec![Diagnostic {
            range: Range::default(),
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("scarb".to_string()),
            message,
            related_information: None,
            tags: None,
            data: None,
        }],
    )
}

#[derive(Deserialize)]
struct ScarbJsonErrorMessage {
    #[serde(rename = "type")]
    message_type: Option<String>,
    message: Option<String>,
}

// Grabs error message from scarb - which should always be present in case of failure (exactly one)
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
