use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_utils::Intern;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Range, Url};
use scarb_metadata::MetadataCommandError;
use serde_json::Value;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::Utf8Span;
use crate::toolchain::scarb::{
    SCARB_METADATA_CAIRO_VERSION_MISMATCH_MESSAGE, SCARB_METADATA_FAILED_MESSAGE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScarbMetadataMessage {
    // Represents an "error" kind message from the metadata command.
    MetadataError(String),
    // Represents a "manifest_diagnostic" kind message from the metadata command.
    MetadataDiagnostic {
        path: PathBuf,
        message: String,
        error_code: Option<String>,
        span: Option<Utf8Span>,
    },
}

fn diagnostics_to_display(all_messages: Vec<ScarbMetadataMessage>) -> Vec<ScarbMetadataMessage> {
    let metadata_diagnostics: Vec<ScarbMetadataMessage> = all_messages
        .iter()
        .filter(|&message| matches!(message, ScarbMetadataMessage::MetadataDiagnostic { .. }))
        .cloned()
        .collect();

    if !metadata_diagnostics.is_empty() { metadata_diagnostics } else { all_messages }
}

/// Picks the most specific user-facing message for a failed `scarb metadata` invocation.
///
/// When the failure is caused by a Cairo version requirement mismatch, returns a dedicated
/// message pointing the user at the root cause instead of the generic one.
pub fn scarb_metadata_failed_message(messages: &[ScarbMetadataMessage]) -> &'static str {
    if messages.iter().any(scarb_metadata_message_is_cairo_version_mismatch) {
        SCARB_METADATA_CAIRO_VERSION_MISMATCH_MESSAGE
    } else {
        SCARB_METADATA_FAILED_MESSAGE
    }
}

fn scarb_metadata_message_is_cairo_version_mismatch(message: &ScarbMetadataMessage) -> bool {
    match message {
        ScarbMetadataMessage::MetadataError(message) => message.contains("required Cairo version"),
        ScarbMetadataMessage::MetadataDiagnostic { .. } => false,
    }
}

pub fn scarb_metadata_messages_to_diagnostics(
    db: &AnalysisDatabase,
    messages: Vec<ScarbMetadataMessage>,
    root_manifest_path: &Path,
    manifest_diagnostic_severity: DiagnosticSeverity,
) -> Option<HashMap<Url, Vec<Diagnostic>>> {
    let root_manifest_url = Url::from_file_path(root_manifest_path).ok()?;
    let mut diagnostics_by_file = HashMap::from([(root_manifest_url, Vec::new())]);

    for diagnostic in diagnostics_to_display(messages).into_iter().filter_map(|message| {
        scarb_metadata_message_to_diagnostic(
            db,
            message,
            root_manifest_path,
            manifest_diagnostic_severity,
        )
    }) {
        let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
        if !entry.contains(&diagnostic.diagnostic) {
            entry.push(diagnostic.diagnostic);
        }
    }

    Some(diagnostics_by_file)
}

/// Collects diagnostics for a `Scarb.toml` file from a failed metadata invocation.
pub fn collect_scarb_manifest_diagnostics(
    error: MetadataCommandError,
) -> Vec<ScarbMetadataMessage> {
    match error {
        // Scarb metadata ran with `--json` emits diagnostics and errors in NDJSON format.
        MetadataCommandError::ScarbError { stdout, .. } => {
            let manifest_diagnostics = manifest_diagnostics_from_ndjson(&stdout);
            if !manifest_diagnostics.is_empty() {
                manifest_diagnostics
            } else {
                metadata_errors_from_ndjson(&stdout)
            }
        }
        MetadataCommandError::NotFound { stdout } => {
            vec![ScarbMetadataMessage::MetadataError(stdout)]
        }
        other => {
            vec![ScarbMetadataMessage::MetadataError(other.to_string())]
        }
    }
}

struct LspScarbDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

pub(crate) fn manifest_diagnostics_from_ndjson(stdout: &str) -> Vec<ScarbMetadataMessage> {
    ndjson_values(stdout).filter_map(|value| manifest_diagnostic_from_value(&value)).collect()
}

fn metadata_errors_from_ndjson(stdout: &str) -> Vec<ScarbMetadataMessage> {
    ndjson_values(stdout).filter_map(|value| metadata_error_from_value(&value)).collect()
}

fn manifest_diagnostic_from_value(value: &Value) -> Option<ScarbMetadataMessage> {
    if is_manifest_diagnostic_kind(value) {
        let message = diagnostic_message(value)?;
        if message.is_empty() {
            return None;
        }

        return Some(ScarbMetadataMessage::MetadataDiagnostic {
            path: diagnostic_path(value)?,
            message,
            error_code: diagnostic_error_code(value),
            span: diagnostic_span(value),
        });
    }

    None
}

fn metadata_error_from_value(value: &Value) -> Option<ScarbMetadataMessage> {
    if is_error_kind(value) {
        let message = diagnostic_message(value)?;
        if message.is_empty() {
            return None;
        }

        return Some(ScarbMetadataMessage::MetadataError(message));
    }

    None
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

fn diagnostic_error_code(value: &Value) -> Option<String> {
    value.get("error_code").and_then(Value::as_str).map(ToOwned::to_owned)
}

fn diagnostic_span(value: &Value) -> Option<Utf8Span> {
    let span = value.get("span")?;
    let start = span.get("start")?.as_u64()?;
    let end = span.get("end")?.as_u64()?;

    Some(Utf8Span::new(start as usize, end as usize))
}

fn scarb_metadata_message_to_diagnostic(
    db: &AnalysisDatabase,
    message: ScarbMetadataMessage,
    root_manifest_path: &Path,
    manifest_diagnostic_severity: DiagnosticSeverity,
) -> Option<LspScarbDiagnostic> {
    match message {
        ScarbMetadataMessage::MetadataError(message) => {
            Url::from_file_path(root_manifest_path).ok().map(|uri| LspScarbDiagnostic {
                uri,
                diagnostic: build_diagnostic(
                    message,
                    None,
                    Range::default(),
                    DiagnosticSeverity::ERROR,
                ),
            })
        }
        ScarbMetadataMessage::MetadataDiagnostic { path, message, error_code, span } => {
            let range = manifest_diagnostic_range(db, &path, span);
            Url::from_file_path(path).ok().map(|uri| LspScarbDiagnostic {
                uri,
                diagnostic: build_diagnostic(
                    message,
                    error_code,
                    range,
                    manifest_diagnostic_severity,
                ),
            })
        }
    }
}

fn manifest_diagnostic_range(
    db: &AnalysisDatabase,
    manifest_path: &Path,
    span: Option<Utf8Span>,
) -> Range {
    let file = FileLongId::OnDisk(manifest_path.to_path_buf()).intern(db);
    span.and_then(|span| span.to_lsp_range(db, file)).unwrap_or_default()
}

fn build_diagnostic(
    message: String,
    error_code: Option<String>,
    range: Range,
    severity: DiagnosticSeverity,
) -> Diagnostic {
    Diagnostic {
        range,
        severity: Some(severity),
        code: error_code.map(NumberOrString::String),
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
    use super::*;

    #[test]
    fn collect_scarb_manifest_warnings_marks_diagnostics_as_warnings() {
        let stdout = r#"{"kind":"manifest_diagnostic","message":"unknown manifest field `package.typo_field`","file":"/tmp/Scarb.toml","span":{"start":10,"end":20}}"#;

        let diagnostics = manifest_diagnostics_from_ndjson(stdout);

        assert_eq!(
            diagnostics,
            vec![ScarbMetadataMessage::MetadataDiagnostic {
                path: PathBuf::from("/tmp/Scarb.toml"),
                message: "unknown manifest field `package.typo_field`".to_string(),
                error_code: None,
                span: Some(Utf8Span::new(10, 20)),
            }]
        );
    }

    #[test]
    fn collect_scarb_manifest_diagnostics_marks_manifest_diagnostics_as_errors() {
        let stdout = r#"{"kind":"manifest_diagnostic","message":"profile name `test` is not allowed","file":"/tmp/Scarb.toml","span":{"start":4,"end":11}}"#;

        let diagnostics = collect_scarb_manifest_diagnostics(MetadataCommandError::ScarbError {
            stdout: stdout.to_string(),
            stderr: String::new(),
        });

        assert_eq!(
            diagnostics,
            vec![ScarbMetadataMessage::MetadataDiagnostic {
                path: PathBuf::from("/tmp/Scarb.toml"),
                message: "profile name `test` is not allowed".to_string(),
                error_code: None,
                span: Some(Utf8Span::new(4, 11)),
            }]
        );
    }

    #[test]
    fn cairo_version_mismatch_errors_get_dedicated_message() {
        let stdout = "{\"type\":\"error\",\"message\":\"the required Cairo version of package \
                      sphincs_plus is not compatible with current version\\nCairo version \
                      required: =2.1.0\\nCairo version of Scarb: 2.18.0\\n\"}\n{\"type\":\"error\",\
                      \"message\":\"the required Cairo version of each package must match the \
                      current Cairo version\\nhelp: pass `--ignore-cairo-version` to ignore Cairo \
                      version mismatch\"}";

        let messages = collect_scarb_manifest_diagnostics(MetadataCommandError::ScarbError {
            stdout: stdout.to_string(),
            stderr: String::new(),
        });

        assert_eq!(
            scarb_metadata_failed_message(&messages),
            SCARB_METADATA_CAIRO_VERSION_MISMATCH_MESSAGE
        );
    }

    #[test]
    fn generic_metadata_errors_get_generic_message() {
        let messages =
            vec![ScarbMetadataMessage::MetadataError("something else went wrong".to_string())];

        assert_eq!(scarb_metadata_failed_message(&messages), SCARB_METADATA_FAILED_MESSAGE);
    }

    #[test]
    fn located_manifest_diagnostics_get_generic_message() {
        let messages = vec![ScarbMetadataMessage::MetadataDiagnostic {
            path: PathBuf::from("/tmp/Scarb.toml"),
            message: "profile name `test` is not allowed".to_string(),
            error_code: None,
            span: Some(Utf8Span::new(4, 11)),
        }];

        assert_eq!(scarb_metadata_failed_message(&messages), SCARB_METADATA_FAILED_MESSAGE);
    }

    #[test]
    fn build_diagnostic_sets_lsp_code_from_manifest_error_code() {
        let diagnostic = build_diagnostic(
            "profile name `test` is not allowed".to_string(),
            Some("SE0003".to_string()),
            Range::default(),
            DiagnosticSeverity::ERROR,
        );

        assert_eq!(diagnostic.code, Some(NumberOrString::String("SE0003".to_string())));
    }
}
