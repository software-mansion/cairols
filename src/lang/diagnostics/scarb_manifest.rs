use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use cairo_lang_filesystem::ids::FileId;
use lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};
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
    scarb_path: &Path,
) -> Option<(Url, HashMap<Url, Vec<Diagnostic>>)> {
    let root_url = db.url_for_file(root_on_disk_file)?;

    let output = Command::new(scarb_path)
        .arg("metadata")
        .arg("--manifest-path")
        .arg(manifest_path)
        .arg("--format-version")
        .arg("1")
        .arg("--json")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .ok()?;

    let mut diagnostics_collector = ManifestDiagnosticsCollector::new(root_url.clone());
    let mut has_manifest_errors = false;

    for line in BufReader::new(output.stdout.as_slice()).lines().map_while(Result::ok) {
        if let Some(diagnostic) = parse_metadata_diagnostic_line(&line, manifest_path) {
            has_manifest_errors = true;
            diagnostics_collector.push_lsp_diagnostic(diagnostic.uri, diagnostic.diagnostic);
        }
    }

    if !output.status.success() && !has_manifest_errors {
        diagnostics_collector.push_lsp_diagnostic(
            root_url.clone(),
            Diagnostic {
                range: Range::default(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("scarb-metadata".into())),
                code_description: None,
                source: Some("scarb".to_string()),
                message: "`scarb metadata` failed. Check if your project builds correctly via `scarb build`.".to_string(),
                related_information: None,
                tags: None,
                data: None,
            },
        );
    }

    Some((root_url, diagnostics_collector.into_inner()))
}

struct ParsedMetadataDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

fn parse_metadata_diagnostic_line(
    line: &str,
    fallback_manifest_path: &Path,
) -> Option<ParsedMetadataDiagnostic> {
    let message = serde_json::from_str::<MetadataMessage>(line).ok()?;
    if message.kind != "error" {
        return None;
    }

    let source_path = message.path.or_else(|| Some(fallback_manifest_path.to_path_buf()))?;
    let uri = Url::from_file_path(source_path).ok()?;

    let range = message
        .span
        .map(|span| Range {
            start: Position { line: span.start.line, character: span.start.col },
            end: Position { line: span.end.line, character: span.end.col },
        })
        .unwrap_or_default();

    Some(ParsedMetadataDiagnostic {
        uri,
        diagnostic: Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: message.code.map(NumberOrString::String),
            code_description: None,
            source: Some("scarb".to_string()),
            message: message.message,
            related_information: None,
            tags: None,
            data: None,
        },
    })
}

#[derive(Deserialize)]
struct MetadataMessage {
    #[serde(rename = "type")]
    kind: String,
    message: String,
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    path: Option<PathBuf>,
    #[serde(default)]
    span: Option<MetadataSpan>,
}

#[derive(Deserialize)]
struct MetadataSpan {
    start: MetadataPosition,
    end: MetadataPosition,
}

#[derive(Deserialize)]
struct MetadataPosition {
    line: u32,
    col: u32,
}

/// Aggregates diagnostics from Scarb manifest validation.
struct ManifestDiagnosticsCollector {
    diagnostics_by_file: HashMap<Url, Vec<Diagnostic>>,
}

impl ManifestDiagnosticsCollector {
    fn new(root_url: Url) -> Self {
        Self { diagnostics_by_file: HashMap::from([(root_url, Vec::new())]) }
    }

    fn push_lsp_diagnostic(&mut self, uri: Url, diagnostic: Diagnostic) {
        let entry = self.diagnostics_by_file.entry(uri).or_default();
        if !entry.contains(&diagnostic) {
            entry.push(diagnostic);
        }
    }

    fn into_inner(self) -> HashMap<Url, Vec<Diagnostic>> {
        self.diagnostics_by_file
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use indoc::formatdoc;
    use lsp_types::Url;
    use tempfile::tempdir;

    use super::collect_scarb_manifest_diagnostics;
    use crate::lang::{db::AnalysisDatabase, lsp::LsProtoGroup};

    #[test]
    fn collects_diagnostics_from_ndjson_stdout() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"type":"error","message":"bad manifest","path":"'$1'"}'
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
        assert!(diagnostics_by_file[&uri].iter().any(|diag| diag.message == "bad manifest"));
    }

    #[test]
    fn ignores_non_error_ndjson_messages() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"type":"warning","message":"ignore me","path":"'$1'"}'
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
    fn emits_fallback_error_when_metadata_fails_without_ndjson_errors() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(&path, "[package]\nname = \"pkg\"\nversion = \"0.1.0\"\n").unwrap();
        let scarb = fake_scarb(
            dir.path(),
            r#"echo '{"type":"warning","message":"still not an error"}'
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
                .any(|diag| diag.message.contains("`scarb metadata` failed"))
        );
    }

    fn fake_scarb(dir: &Path, script_body: &str) -> std::path::PathBuf {
        let path = dir.join("fake-scarb");
        fs::write(
            &path,
            formatdoc! {
                r#"
                #!/usr/bin/env bash
                set -euo pipefail
                if [ "$1" != "metadata" ]; then
                  exit 2
                fi
                shift
                while [ "$1" != "--manifest-path" ]; do shift; done
                shift
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
