#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_utils::Intern;
use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString, Url,
};
use serde::de::{Error as DeError, Unexpected};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::Utf8Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScarbCheckDiagnostic {
    pub severity: ScarbCheckDiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
    pub file: PathBuf,
    pub span: Utf8Span,
    pub related: Vec<ScarbCheckDiagnosticRelated>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScarbCheckDiagnosticSeverity {
    Error,
    Warning,
    Other(String),
}

impl<'de> Deserialize<'de> for ScarbCheckDiagnosticSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let severity = String::deserialize(deserializer)?;

        Ok(match severity.as_str() {
            "error" => Self::Error,
            "warning" => Self::Warning,
            _ => Self::Other(severity),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ScarbCheckDiagnosticRelated {
    pub message: String,
    pub file: PathBuf,
    #[serde(deserialize_with = "deserialize_utf8_span")]
    pub span: Utf8Span,
}

impl TryFrom<&Value> for ScarbCheckDiagnostic {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if !is_diagnostic_kind(value) {
            return Err(());
        }

        Ok(Self {
            severity: serde_json::from_value(value.get("severity").cloned().ok_or(())?)
                .map_err(|_| ())?,
            message: diagnostic_message(value).ok_or(())?,
            code: diagnostic_code(value),
            file: diagnostic_path(value).ok_or(())?,
            span: diagnostic_span(value).ok_or(())?,
            related: diagnostic_related(value).unwrap_or_default(),
        })
    }
}

pub fn collect_scarb_check_diagnostics(stdout: &str) -> Vec<ScarbCheckDiagnostic> {
    ndjson_values(stdout).filter_map(|value| ScarbCheckDiagnostic::try_from(&value).ok()).collect()
}

pub fn scarb_check_diagnostics_to_diagnostics(
    db: &AnalysisDatabase,
    diagnostics: Vec<ScarbCheckDiagnostic>,
) -> HashMap<Url, Vec<Diagnostic>> {
    let mut diagnostics_by_file: HashMap<Url, Vec<Diagnostic>> = HashMap::new();

    for diagnostic in diagnostics
        .into_iter()
        .filter_map(|diagnostic| scarb_check_diagnostic_to_lsp(db, diagnostic))
    {
        let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
        if !entry.contains(&diagnostic.diagnostic) {
            entry.push(diagnostic.diagnostic);
        }
    }

    diagnostics_by_file
}

fn is_diagnostic_kind(value: &Value) -> bool {
    value.get("type").and_then(Value::as_str).is_some_and(|value| value == "diagnostic")
}

fn diagnostic_message(value: &Value) -> Option<String> {
    value.get("message").and_then(Value::as_str).map(str::to_string)
}

fn diagnostic_code(value: &Value) -> Option<String> {
    value.get("code").and_then(Value::as_str).map(str::to_string)
}

fn diagnostic_path(value: &Value) -> Option<PathBuf> {
    value.get("file").and_then(Value::as_str).map(PathBuf::from)
}

fn diagnostic_span(value: &Value) -> Option<Utf8Span> {
    let span = value.get("span")?;
    let start = span.get("start")?.as_u64()?;
    let end = span.get("end")?.as_u64()?;

    Some(Utf8Span::new(start as usize, end as usize))
}

fn diagnostic_related(value: &Value) -> Option<Vec<ScarbCheckDiagnosticRelated>> {
    serde_json::from_value(value.get("related")?.clone()).ok()
}

fn deserialize_utf8_span<'de, D>(deserializer: D) -> Result<Utf8Span, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    let start = value
        .get("start")
        .and_then(Value::as_u64)
        .ok_or_else(|| D::Error::invalid_type(Unexpected::Other("missing span start"), &"u64"))?;
    let end = value
        .get("end")
        .and_then(Value::as_u64)
        .ok_or_else(|| D::Error::invalid_type(Unexpected::Other("missing span end"), &"u64"))?;

    Ok(Utf8Span::new(start as usize, end as usize))
}

fn ndjson_values(stdout: &str) -> impl Iterator<Item = Value> + '_ {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
}

pub fn workspace_root_for_check(manifest_path: &Path) -> Option<&Path> {
    (manifest_path.file_name()? == "Scarb.toml").then(|| manifest_path.parent()).flatten()
}

struct LspScarbDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

fn scarb_check_diagnostic_to_lsp(
    db: &AnalysisDatabase,
    diagnostic: ScarbCheckDiagnostic,
) -> Option<LspScarbDiagnostic> {
    let uri = Url::from_file_path(&diagnostic.file).ok()?;

    Some(LspScarbDiagnostic {
        uri,
        diagnostic: Diagnostic {
            range: diagnostic_range(db, &diagnostic.file, diagnostic.span),
            severity: Some(match diagnostic.severity {
                ScarbCheckDiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                ScarbCheckDiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
                ScarbCheckDiagnosticSeverity::Other(_) => DiagnosticSeverity::INFORMATION,
            }),
            code: diagnostic.code.map(NumberOrString::String),
            source: Some("scarb".to_string()),
            message: diagnostic.message,
            related_information: diagnostic_related_information(db, diagnostic.related),
            ..Diagnostic::default()
        },
    })
}

fn diagnostic_range(db: &AnalysisDatabase, path: &Path, span: Utf8Span) -> lsp_types::Range {
    let file = FileLongId::OnDisk(path.to_path_buf()).intern(db);
    span.to_lsp_range(db, file).unwrap_or_default()
}

fn diagnostic_related_information(
    db: &AnalysisDatabase,
    related: Vec<ScarbCheckDiagnosticRelated>,
) -> Option<Vec<DiagnosticRelatedInformation>> {
    let related_information: Vec<_> = related
        .into_iter()
        .filter_map(|related| {
            let uri = Url::from_file_path(&related.file).ok()?;
            Some(DiagnosticRelatedInformation {
                location: Location {
                    uri,
                    range: diagnostic_range(db, &related.file, related.span),
                },
                message: related.message,
            })
        })
        .collect();

    (!related_information.is_empty()).then_some(related_information)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    use indoc::indoc;
    use lsp_types::{
        Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, NumberOrString,
        Position, Range, Url,
    };
    use tempfile::tempdir;

    use super::{
        ScarbCheckDiagnostic, ScarbCheckDiagnosticRelated, ScarbCheckDiagnosticSeverity,
        collect_scarb_check_diagnostics, scarb_check_diagnostics_to_diagnostics,
    };
    use crate::lang::db::AnalysisDatabase;
    use crate::lang::lsp::Utf8Span;

    #[test]
    fn collects_only_structured_diagnostic_messages() {
        let diagnostics = collect_scarb_check_diagnostics(indoc! {r#"
            {"status":"checking","message":"hello v0.1.0 ([..]Scarb.toml)"}
            {"type":"diagnostic","severity":"error","message":"Skipped tokens.","code":"E1000","file":"/tmp/lib.cairo","span":{"start":13,"end":13}}
            {"type":"diagnostic","severity":"warning","message":"Unused variable.","code":"E0001","file":"/tmp/lib.cairo","span":{"start":17,"end":18},"related":[{"message":"diagnostic originates in generated code","file":"/tmp/generated.cairo","span":{"start":1,"end":4}}]}
            {"type":"error","message":"could not check `hello` due to 1 previous error"}
        "#});

        assert_eq!(
            diagnostics,
            vec![
                ScarbCheckDiagnostic {
                    severity: ScarbCheckDiagnosticSeverity::Error,
                    message: "Skipped tokens.".to_string(),
                    code: Some("E1000".to_string()),
                    file: PathBuf::from("/tmp/lib.cairo"),
                    span: Utf8Span::new(13, 13),
                    related: Vec::new(),
                },
                ScarbCheckDiagnostic {
                    severity: ScarbCheckDiagnosticSeverity::Warning,
                    message: "Unused variable.".to_string(),
                    code: Some("E0001".to_string()),
                    file: PathBuf::from("/tmp/lib.cairo"),
                    span: Utf8Span::new(17, 18),
                    related: vec![ScarbCheckDiagnosticRelated {
                        message: "diagnostic originates in generated code".to_string(),
                        file: PathBuf::from("/tmp/generated.cairo"),
                        span: Utf8Span::new(1, 4),
                    }],
                },
            ]
        );
    }

    #[test]
    fn converts_structured_check_diagnostics_to_lsp() {
        let workspace = tempdir().expect("failed to create temporary directory");
        let file = workspace.path().join("src").join("lib.cairo");
        let related_file = workspace.path().join("src").join("generated.cairo");
        fs::create_dir_all(file.parent().expect("file has parent")).expect("failed to create dir");
        fs::write(&file, "fn main() {\n    let x = 1;\n}\n").expect("failed to write file");
        fs::write(&related_file, "abcd\n").expect("failed to write related file");

        let diagnostics = vec![ScarbCheckDiagnostic {
            severity: ScarbCheckDiagnosticSeverity::Warning,
            message: "Unused variable.".to_string(),
            code: Some("E0001".to_string()),
            file: file.clone(),
            span: Utf8Span::new(20, 21),
            related: vec![ScarbCheckDiagnosticRelated {
                message: "diagnostic originates in generated code".to_string(),
                file: related_file.clone(),
                span: Utf8Span::new(1, 4),
            }],
        }];

        let db = AnalysisDatabase::new();
        let mapped = scarb_check_diagnostics_to_diagnostics(&db, diagnostics);

        assert_eq!(
            mapped,
            HashMap::from([(
                Url::from_file_path(&file).expect("valid file url"),
                vec![Diagnostic {
                    range: Range {
                        start: Position { line: 1, character: 8 },
                        end: Position { line: 1, character: 9 },
                    },
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("E0001".to_string())),
                    source: Some("scarb".to_string()),
                    message: "Unused variable.".to_string(),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: Url::from_file_path(&related_file).expect("valid file url"),
                            range: Range {
                                start: Position { line: 0, character: 1 },
                                end: Position { line: 0, character: 4 },
                            },
                        },
                        message: "diagnostic originates in generated code".to_string(),
                    }]),
                    ..Diagnostic::default()
                }]
            )])
        );
    }
}
