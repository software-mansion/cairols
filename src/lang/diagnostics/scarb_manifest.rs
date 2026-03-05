use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cairo_lang_filesystem::ids::FileId;
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use scarb_metadata::{MetadataCommand, MetadataCommandError};

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
    let Some(diagnostic) =
        diagnostic_from_metadata_error(metadata_result.err(), manifest_path, &root_url)
    else {
        return Some((root_url, diagnostics_by_file));
    };

    let entry = diagnostics_by_file.entry(diagnostic.uri).or_default();
    if !entry.contains(&diagnostic.diagnostic) {
        entry.push(diagnostic.diagnostic);
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

    command.manifest_path(manifest_path).no_deps().exec().map(|_| ())
}

struct LspScarbDiagnostic {
    uri: Url,
    diagnostic: Diagnostic,
}

fn diagnostic_from_metadata_error(
    error: Option<MetadataCommandError>,
    fallback_manifest_path: &Path,
    root_url: &Url,
) -> Option<LspScarbDiagnostic> {
    let error = error?;

    let parsed = match error {
        MetadataCommandError::ScarbError { stdout, stderr } => {
            parse_scarb_command_output(&stdout, &stderr, fallback_manifest_path)
        }
        MetadataCommandError::NotFound { stdout } => ParsedScarbDiagnostic {
            file: Some(fallback_manifest_path.to_path_buf()),
            line: None,
            column: None,
            message: if stdout.trim().is_empty() {
                "`scarb metadata` command did not produce metadata".to_string()
            } else {
                stdout.trim().to_string()
            },
        },
        other => ParsedScarbDiagnostic {
            file: Some(fallback_manifest_path.to_path_buf()),
            line: None,
            column: None,
            message: other.to_string(),
        },
    };

    let uri = parsed
        .file
        .and_then(|path| uri_for_manifest_path(path, fallback_manifest_path, root_url))
        .unwrap_or_else(|| root_url.clone());

    Some(LspScarbDiagnostic {
        uri,
        diagnostic: Diagnostic {
            range: lsp_range(parsed.line, parsed.column),
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("scarb".to_string()),
            message: parsed.message,
            related_information: None,
            tags: None,
            data: None,
        },
    })
}

fn uri_for_manifest_path(
    path: PathBuf,
    fallback_manifest_path: &Path,
    root_url: &Url,
) -> Option<Url> {
    if paths_equivalent(path.as_path(), fallback_manifest_path) {
        return Some(root_url.clone());
    }

    Url::from_file_path(path).ok()
}

fn paths_equivalent(path: &Path, other: &Path) -> bool {
    if path == other {
        return true;
    }

    match (std::fs::canonicalize(path), std::fs::canonicalize(other)) {
        (Ok(path), Ok(other)) => path == other,
        _ => false,
    }
}

struct ParsedScarbDiagnostic {
    file: Option<PathBuf>,
    line: Option<usize>,
    column: Option<usize>,
    message: String,
}

fn parse_scarb_command_output(
    stdout: &str,
    stderr: &str,
    fallback_manifest_path: &Path,
) -> ParsedScarbDiagnostic {
    let text = if stdout.trim().is_empty() { stderr } else { stdout };

    let mut parsed = ParsedScarbDiagnostic {
        file: Some(fallback_manifest_path.to_path_buf()),
        line: None,
        column: None,
        message: "failed to resolve Scarb metadata".to_string(),
    };

    let mut fallback_error_message: Option<String> = None;
    let mut message_candidates = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(path) = trimmed.strip_prefix("error: failed to parse manifest at:") {
            let path = path.trim();
            if !path.is_empty() {
                parsed.file = Some(PathBuf::from(path));
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("--> ") {
            let mut parts = rest.rsplitn(3, ':');
            let column = parts.next().and_then(|value| value.trim().parse::<usize>().ok());
            let line = parts.next().and_then(|value| value.trim().parse::<usize>().ok());
            let path = parts.next().map(str::trim);

            if let (Some(line), Some(column)) = (line, column) {
                parsed.line = Some(line);
                parsed.column = Some(column);
            }

            if let Some(path) = path
                && !path.is_empty()
            {
                parsed.file = Some(PathBuf::from(path));
            }
            continue;
        }

        if let Some((line, column)) = parse_toml_line_column(trimmed) {
            parsed.line = Some(line);
            parsed.column = Some(column);
            continue;
        }

        if trimmed == "Caused by:"
            || trimmed.starts_with("help:")
            || is_source_snippet_line(trimmed)
        {
            continue;
        }

        if let Some(message) = trimmed.strip_prefix("error:") {
            let message = message.trim();
            if message.starts_with("failed to parse manifest at:") {
                if fallback_error_message.is_none() {
                    fallback_error_message = Some(message.to_string());
                }
            } else if !message.is_empty() {
                message_candidates.push(message.to_string());
            }
            continue;
        }

        message_candidates.push(trimmed.to_string());
    }

    parsed.message = message_candidates.pop().or(fallback_error_message).unwrap_or(parsed.message);

    parsed
}

fn parse_toml_line_column(line: &str) -> Option<(usize, usize)> {
    const PREFIX: &str = "TOML parse error at line ";
    let suffix = line.strip_prefix(PREFIX)?;
    let (line, column) = suffix.split_once(", column ")?;
    Some((line.trim().parse().ok()?, column.trim().parse().ok()?))
}

fn is_source_snippet_line(line: &str) -> bool {
    if line.starts_with('|') {
        return true;
    }

    let Some((left, _)) = line.split_once('|') else {
        return false;
    };

    !left.trim().is_empty() && left.trim().chars().all(|char| char.is_ascii_digit())
}

fn lsp_range(line: Option<usize>, column: Option<usize>) -> Range {
    let Some(line) = line else {
        return Range::default();
    };

    let line = line.saturating_sub(1) as u32;
    let column = column.unwrap_or(1).saturating_sub(1) as u32;

    Range {
        start: Position { line, character: column },
        end: Position { line, character: column.saturating_add(1) },
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use indoc::indoc;
    use lsp_types::{DiagnosticSeverity, Url};
    use tempfile::tempdir;

    use super::collect_scarb_manifest_diagnostics;
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
