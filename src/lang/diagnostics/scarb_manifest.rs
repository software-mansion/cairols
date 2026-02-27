use std::collections::HashMap;

use cairo_lang_filesystem::ids::FileId;
use camino::Utf8Path;
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range, Url};
use scarb_manifest_validator::{
    ManifestDiagnostic as ScarbManifestDiagnostic, validate_manifest, validate_workspace,
};

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;

/// Collects diagnostics for a `Scarb.toml` file.
///
/// Caller must ensure `root_on_disk_file` points to a Scarb manifest and provide its on-disk path.
pub fn collect_scarb_manifest_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    root_on_disk_file: FileId<'db>,
    manifest_path: &'db std::path::Path,
) -> Option<(Url, HashMap<Url, Vec<Diagnostic>>)> {
    let root_url = db.url_for_file(root_on_disk_file)?;
    let Some(manifest_path) = Utf8Path::from_path(manifest_path) else {
        return Some((root_url.clone(), HashMap::from([(root_url, Vec::new())])));
    };

    let result = validate_manifest(manifest_path);
    let has_manifest_errors = !result.is_valid();

    let mut diagnostics_collector = ManifestDiagnosticsCollector::new(root_url.clone());
    for diagnostic in result.diagnostics {
        diagnostics_collector.push_manifest_diagnostic(diagnostic);
    }

    // Workspace-level validation captures runtime/business-rule checks from Scarb workspace loading.
    // Also skip when manifest-level validation already failed, to avoid duplicate/cascading errors.
    if !has_manifest_errors {
        let workspace_result = validate_workspace(manifest_path);
        for diagnostic in workspace_result.diagnostics {
            diagnostics_collector.push_manifest_diagnostic(diagnostic);
        }
    }

    Some((root_url, diagnostics_collector.into_inner()))
}

/// Aggregates diagnostics from Scarb manifest validation.
struct ManifestDiagnosticsCollector {
    root_url: Url,
    diagnostics_by_file: HashMap<Url, Vec<Diagnostic>>,
}

impl ManifestDiagnosticsCollector {
    fn new(root_url: Url) -> Self {
        Self { diagnostics_by_file: HashMap::from([(root_url.clone(), Vec::new())]), root_url }
    }

    fn push_manifest_diagnostic(&mut self, diagnostic: ScarbManifestDiagnostic) {
        let uri = Url::from_file_path(diagnostic.file.as_std_path())
            .ok()
            .unwrap_or_else(|| self.root_url.clone());
        let diagnostic = self.to_lsp_diagnostic(diagnostic);
        self.push_lsp_diagnostic(uri, diagnostic);
    }

    fn to_lsp_diagnostic(&self, diagnostic: ScarbManifestDiagnostic) -> Diagnostic {
        let range = diagnostic
            .span
            .map(|span| Range {
                start: Position { line: span.start, character: span.start },
                end: Position { line: span.end, character: span.end },
            })
            .unwrap_or_default();

        Diagnostic {
            range,
            severity: Some(DiagnosticSeverity::ERROR),
            code: None,
            code_description: None,
            source: Some("scarb".to_string()),
            message: diagnostic.message,
            related_information: None,
            tags: None,
            data: None,
        }
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
    use std::path::Path;

    use indoc::indoc;
    use lsp_types::Url;
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
            collect_scarb_manifest_diagnostics(&db, file_id, &path)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        assert_eq!(root_url, uri);
        assert!(diagnostics_by_file[&uri].iter().any(|diag| !diag.message.is_empty()));
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
                edition = "2025_12"

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
                edition = "2025_12"

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
                edition = "2025_12"

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
        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &member_manifest_path)
        else {
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
            collect_scarb_manifest_diagnostics(&db, file_id, &path)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let diagnostics = diagnostics_by_file.get(&uri).cloned().unwrap_or_default();
        let duplicate_message_count = diagnostics
            .iter()
            .filter(|diag| {
                diag.message.contains(
                    "this virtual manifest specifies a [dependencies] section, which is not \
                     allowed",
                )
            })
            .count();

        assert_eq!(duplicate_message_count, 1);
    }

    #[test]
    fn validates_profile_specific_manifest_rules_for_declared_profiles() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("Scarb.toml");
        fs::write(
            &path,
            indoc! {r#"
                [package]
                name = "manifest_diagnostics_ws"
                version = "0.1.0"
                edition = "2025_12"

                [profile.some-profile]

                [profile.custom]
                inherits = "some-profile"
            "#},
        )
        .unwrap();
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(dir.path().join("src/lib.cairo"), "fn main() {}\n").unwrap();

        let db = AnalysisDatabase::new();
        let uri = Url::from_file_path(&path).unwrap();
        let file_id = db.file_for_url(&uri).unwrap();

        let Some((_root_url, diagnostics_by_file)) =
            collect_scarb_manifest_diagnostics(&db, file_id, &path)
        else {
            panic!("Scarb manifest diagnostics were not collected");
        };

        let matching_count = diagnostics_by_file
            .values()
            .flatten()
            .filter(|diag| {
                diag.message.contains("profile can inherit from `dev` or `release` only")
            })
            .count();
        assert_eq!(matching_count, 1);
    }

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }
}
