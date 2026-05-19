use std::fmt::{self, Display};
use std::thread;
use std::time::Duration;

use indoc::indoc;
use serde::Serialize;

use crate::support::diagnostics::{
    DiagnosticAndRelatedInfo, DiagnosticsWithUrl, get_related_diagnostic_code,
};
use crate::support::normalize::{normalize, normalize_diagnostics};
use crate::support::{MockClient, sandbox};

#[derive(Debug, Serialize)]
struct DiagnosticsReport {
    diagnostics: Vec<DiagnosticsWithUrl>,
}

#[derive(Debug, Serialize)]
struct DiagnosticsLifecycleReport {
    before_save: DiagnosticsReport,
    after_unsaved_edit: DiagnosticsReport,
    after_save: DiagnosticsReport,
}

impl Display for DiagnosticsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = serde_yaml::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&output)
    }
}

impl Display for DiagnosticsLifecycleReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = serde_yaml::to_string(self).map_err(|_| fmt::Error)?;
        f.write_str(&output)
    }
}

#[test]
fn syntax_error_reports_scarb_check_diagnostics() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "syntax_error"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => indoc! {r#"
                fn main() {
                    let x =
                }
            "#},
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    insta::assert_snapshot!(diagnostics_report(&mut ls, &["src/lib.cairo"]));
}

#[test]
fn semantic_diagnostics_refresh_only_after_save() {
    let broken = indoc! {r#"
        fn main() {
            let x: felt252 = 1;
            let _y: u32 = x;
        }
    "#};
    let fixed = indoc! {r#"
        fn main() {
            let _x: felt252 = 1;
            let _y: u32 = 1;
        }
    "#};

    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! {r#"
                [package]
                name = "semantic_error"
                version = "0.1.0"
                edition = "2025_12"
            "#},
            "src/lib.cairo" => broken,
        }
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");
    let before_save = diagnostics_report(&mut ls, &["src/lib.cairo"]);

    ls.change_file("src/lib.cairo", fixed);
    thread::sleep(Duration::from_millis(500));
    let after_unsaved_edit = diagnostics_report(&mut ls, &["src/lib.cairo"]);

    ls.edit_file("src/lib.cairo", fixed);
    ls.save_file("src/lib.cairo");
    ls.wait_for_analysis();
    let after_save = diagnostics_report(&mut ls, &["src/lib.cairo"]);

    insta::assert_snapshot!(DiagnosticsLifecycleReport {
        before_save,
        after_unsaved_edit,
        after_save,
    });
}

fn diagnostics_report(ls: &mut MockClient, paths: &[&str]) -> DiagnosticsReport {
    let diagnostics =
        paths.iter().map(|path| (ls.doc_id(path).uri, ls.get_diagnostics_for_file(path)));
    let normalized = normalize_diagnostics(ls, diagnostics);
    let mut report = Vec::new();

    for (original_url, normalized_url, diagnostics) in normalized {
        if diagnostics.is_empty() {
            continue;
        }

        let mut mapped_diagnostics = Vec::new();
        for mut diagnostic in diagnostics {
            diagnostic.message = normalize(&*ls, &diagnostic.message);
            let related_code = get_related_diagnostic_code(ls, &diagnostic, &original_url);
            mapped_diagnostics.push(DiagnosticAndRelatedInfo { related_code, diagnostic });
        }

        report.push(DiagnosticsWithUrl { url: normalized_url, diagnostics: mapped_diagnostics });
    }

    DiagnosticsReport { diagnostics: report }
}
