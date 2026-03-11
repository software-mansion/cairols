use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use scarb_metadata::MetadataCommandError;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScarbMetadataMessage {
    // Represents an "error" kind message from the metadata command.
    MetadataError(String),
    // Represents a "manifest_diagnostic" kind message from the metadata command.
    MetadataDiagnostic { path: PathBuf, message: String },
}

fn diagnostics_to_display(all_messages: Vec<ScarbMetadataMessage>) -> Vec<ScarbMetadataMessage> {
    let metadata_diagnostics: Vec<ScarbMetadataMessage> = all_messages
        .iter()
        .filter(|&message| matches!(message, ScarbMetadataMessage::MetadataDiagnostic { .. }))
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

impl ScarbMetadataMessage {
    fn into_lsp(self, root_manifest_path: &Path) -> Option<LspScarbDiagnostic> {
        match self {
            ScarbMetadataMessage::MetadataError(message) => Url::from_file_path(root_manifest_path)
                .ok()
                .map(|uri| LspScarbDiagnostic { uri, diagnostic: build_diagnostic(message) }),
            ScarbMetadataMessage::MetadataDiagnostic { message, path } => Url::from_file_path(path)
                .ok()
                .map(|uri| LspScarbDiagnostic { uri, diagnostic: build_diagnostic(message) }),
        }
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

            return Ok(ScarbMetadataMessage::MetadataError(message));
        }

        if is_manifest_diagnostic_kind(value) {
            let message = diagnostic_message(value).ok_or(())?;
            if message.is_empty() {
                return Err(());
            }

            return Ok(ScarbMetadataMessage::MetadataDiagnostic {
                path: diagnostic_path(value).ok_or(())?,
                message,
            });
        }

        Err(())
    }
}

/// Collects diagnostics for a `Scarb.toml` file from a failed metadata invocation.
pub fn collect_scarb_manifest_diagnostics(
    error: MetadataCommandError,
) -> Vec<ScarbMetadataMessage> {
    metadata_messages_from_error(error)
}

struct LspScarbDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

fn metadata_messages_from_error(error: MetadataCommandError) -> Vec<ScarbMetadataMessage> {
    match error {
        MetadataCommandError::ScarbError { stdout, .. } => metadata_messages_from_ndjson(&stdout),
        MetadataCommandError::NotFound { stdout } => {
            vec![ScarbMetadataMessage::MetadataError(stdout)]
        }
        other => {
            vec![ScarbMetadataMessage::MetadataError(other.to_string())]
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
