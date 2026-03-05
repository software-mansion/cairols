use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextSpan, TextWidth};
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Range, Url};
use serde::Deserialize;

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::{LsProtoGroup, ToLsp};

/// Collects diagnostics for a `Scarb.toml` file.
///
/// Caller must ensure `root_on_disk_file` points to a Scarb manifest and provide its on-disk path.
pub fn collect_scarb_manifest_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    root_on_disk_file: FileId<'db>,
    manifest_path: &'db Path,
    scarb_path: &Path,
) -> Option<(Url, HashMap<Url, Vec<Diagnostic>>)> {
    let root_url = db.url_for_file(root_on_disk_file)?;

    let output = Command::new(scarb_path)
        .arg("--json")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("metadata")
        .arg("--format-version")
        .arg("1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .ok()?;

    let mut diagnostics_collector = ManifestDiagnosticsCollector::new(root_url.clone());
    let mut manifest_diagnostics = Vec::new();
    let mut metadata_errors = Vec::new();

    for line in BufReader::new(output.stdout.as_slice()).lines().map_while(Result::ok) {
        let Some(message) = serde_json::from_str::<MetadataMessage>(&line).ok() else {
            continue;
        };

        match message {
            message @ MetadataMessage::ManifestDiagnostic { .. } => {
                let Some(diagnostic) = message.into_diagnostic(db, manifest_path) else {
                    continue;
                };
                manifest_diagnostics.push(diagnostic);
            }
            message @ MetadataMessage::Error { .. } => {
                if let Some(diagnostic) = message.into_diagnostic(db, manifest_path) {
                    metadata_errors.push(diagnostic);
                }
            }
            MetadataMessage::Other => {}
        }
    }

    if !manifest_diagnostics.is_empty() {
        for diagnostic in manifest_diagnostics {
            diagnostics_collector.push_lsp_diagnostic(diagnostic);
        }
    } else if !output.status.success() {
        if metadata_errors.is_empty() {
            // If metadata failed without any diagnostics, emit stderr as a diagnostic.
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if let Some(diagnostic) = (!stderr.is_empty())
                .then_some(MetadataMessage::Error { message: stderr })
                .and_then(|message| message.into_diagnostic(db, manifest_path))
            {
                metadata_errors.push(diagnostic);
            }
        }

        for diagnostic in metadata_errors {
            diagnostics_collector.push_lsp_diagnostic(diagnostic);
        }
    }

    Some((root_url, diagnostics_collector.get_diagnostics_by_file()))
}

struct ScarbMetadataDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

#[derive(Deserialize)]
#[serde(tag = "kind")]
enum MetadataMessage {
    #[serde(rename = "manifest_diagnostic")]
    ManifestDiagnostic {
        message: String,
        #[serde(default)]
        code: Option<String>,
        #[serde(default)]
        file: Option<PathBuf>,
        #[serde(default)]
        span: Option<MetadataSpan>,
    },
    #[serde(rename = "error")]
    Error { message: String },
    #[serde(other)]
    Other,
}

#[derive(Clone, Copy, Deserialize)]
struct MetadataSpan {
    start: u32,
    end: u32,
}

impl MetadataMessage {
    fn into_diagnostic(
        self,
        db: &AnalysisDatabase,
        fallback_manifest_path: &Path,
    ) -> Option<ScarbMetadataDiagnostic> {
        match self {
            Self::ManifestDiagnostic { message, code, file, span } => {
                let source_path = file.or_else(|| Some(fallback_manifest_path.to_path_buf()))?;
                let uri = Url::from_file_path(source_path).ok()?;
                let range = span.and_then(|span| span.to_lsp_range(db, &uri)).unwrap_or_default();

                Some(ScarbMetadataDiagnostic {
                    uri,
                    diagnostic: Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: code.map(NumberOrString::String),
                        code_description: None,
                        source: Some("scarb".to_string()),
                        message,
                        related_information: None,
                        tags: None,
                        data: None,
                    },
                })
            }
            Self::Error { message } => Some(ScarbMetadataDiagnostic {
                uri: Url::from_file_path(fallback_manifest_path).ok()?,
                diagnostic: Diagnostic {
                    range: Range::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("scarb-metadata".into())),
                    code_description: None,
                    source: Some("scarb".to_string()),
                    message,
                    related_information: None,
                    tags: None,
                    data: None,
                },
            }),
            Self::Other => None,
        }
    }
}

impl MetadataSpan {
    fn to_lsp_range(self, db: &AnalysisDatabase, uri: &Url) -> Option<Range> {
        let file_id = db.file_for_url(uri)?;
        let content = db.file_content(file_id)?;
        let start = TextOffset::START.add_width(TextWidth::at(content, self.start as usize));
        let end = TextOffset::START.add_width(TextWidth::at(content, self.end as usize));
        Some(TextSpan::new(start, end).position_in_file(db, file_id)?.to_lsp())
    }
}

/// Aggregates diagnostics from Scarb manifest validation.
struct ManifestDiagnosticsCollector {
    diagnostics_by_file: HashMap<Url, Vec<Diagnostic>>,
}

impl ManifestDiagnosticsCollector {
    fn new(root_url: Url) -> Self {
        Self { diagnostics_by_file: HashMap::from([(root_url, Vec::new())]) }
    }

    fn push_lsp_diagnostic(&mut self, diagnostic: ScarbMetadataDiagnostic) {
        let entry = self.diagnostics_by_file.entry(diagnostic.uri).or_default();
        if !entry.contains(&diagnostic.diagnostic) {
            entry.push(diagnostic.diagnostic);
        }
    }

    fn get_diagnostics_by_file(self) -> HashMap<Url, Vec<Diagnostic>> {
        self.diagnostics_by_file
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use indoc::formatdoc;
    use lsp_types::{Position, Url};
    use tempfile::tempdir;

    use super::collect_scarb_manifest_diagnostics;
    use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};

    #[test]
    fn collects_diagnostics_from_ndjson_stdout() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        let manifest = "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n";
        fs::write(&path, manifest).unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"kind":"manifest_diagnostic","message":"bad manifest","file":"'$1'","span":{"start":10,"end":14}}'
exit 1"#,
        );

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, &scarb)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert_eq!(root_url, uri);
        let diagnostic = diagnostics_by_file[&uri]
            .iter()
            .find(|diag| diag.message == "bad manifest")
            .expect("manifest diagnostic not found");
        assert_eq!(diagnostic.range.start, Position { line: 1, character: 0 });
        assert_eq!(diagnostic.range.end, Position { line: 1, character: 4 });
    }

    #[test]
    fn ignores_non_error_ndjson_messages() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"kind":"warning","message":"ignore me","path":"'$1'"}'
exit 0"#,
        );

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, &scarb)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert!(diagnostics_by_file[&uri].is_empty());
    }

    #[test]
    fn emits_error_message_from_ndjson_when_metadata_fails_without_manifest_diagnostics() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"kind":"warning","message":"still not an error"}'
echo '{"kind":"error","message":"real scarb metadata error"}'
exit 1"#,
        );

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, &scarb)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert!(
            diagnostics_by_file[&uri]
                .iter()
                .any(|diag| diag.message == "real scarb metadata error")
        );
    }

    #[test]
    fn emits_stderr_message_when_metadata_fails_without_ndjson_error() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo "stderr metadata failure" >&2
exit 1"#,
        );

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, &scarb)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert!(
            diagnostics_by_file[&uri].iter().any(|diag| diag.message == "stderr metadata failure")
        );
    }

    #[test]
    fn ignores_terminal_error_summary_when_manifest_diagnostic_is_present() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"kind":"manifest_diagnostic","message":"bad manifest","file":"'$1'","span":{"start":10,"end":14}}'
echo '{"kind":"error","message":"summary error"}'
exit 1"#,
        );

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path, &scarb)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert_eq!(diagnostics_by_file[&uri].len(), 1);
        assert_eq!(diagnostics_by_file[&uri][0].message, "bad manifest");
    }

    fn fake_scarb(dir: &Path, script_body: &str) -> std::path::PathBuf {
        let path = dir.join("fake-scarb");
        fs::write(
            &path,
            formatdoc! {
                r#"
                #!/usr/bin/env bash
                set -euo pipefail
                if [ "$1" != "--json" ]; then
                  exit 2
                fi
                shift
                if [ "$1" != "--manifest-path" ]; then
                  exit 2
                fi
                shift
                manifest_path="$1"
                shift
                if [ "$1" != "metadata" ]; then
                  exit 2
                fi
                shift
                if [ "$1" != "--format-version" ]; then
                  exit 2
                fi
                shift
                if [ "$1" != "1" ]; then
                  exit 2
                fi
                shift
                set -- "$manifest_path" "$@"
                {script_body}
                "#
            },
        )
        .unwrap();
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).unwrap();
        path
    }
}
