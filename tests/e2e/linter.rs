use std::fmt::Display;

use serde::Serialize;
use serde_json::json;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::insta::test_transform;
use crate::support::sandbox;

#[test]
fn test_simple_lint() {
    test_transform!(
        test_linter_diagnostics,
        r#"
        fn foo() {
            let mut span = array![0x0].span();

            loop {
                match span.pop_front() {
                    Some(_) => {},
                    None => { break; },
                }
            }
        }
        "#,
        @r#"
        [[diagnostics]]
        severity = "Warning"
        message = "Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`"
        "#
    )
}

#[test]
fn test_two_simultaneous_lints() {
    test_transform!(
        test_linter_diagnostics,
        r#"
        fn foo() {
            let mut span = array![0x0].span();

            loop {
                match span.pop_front() {
                    Some(_) => {},
                    None => { break (); },
                }
            }
        }
        "#,
        @r#"
        [[diagnostics]]
        severity = "Warning"
        message = "Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them."

        [[diagnostics]]
        severity = "Warning"
        message = "Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`"
        "#
    )
}

#[derive(Serialize)]
struct Report {
    diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize)]
struct Diagnostic {
    severity: Option<String>,
    message: String,
}

impl From<lsp_types::Diagnostic> for Diagnostic {
    fn from(value: lsp_types::Diagnostic) -> Self {
        let severity = value.severity.map(|severity| format!("{severity:?}"));
        Self { severity, message: value.message }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match toml::to_string_pretty(self) {
            Ok(repr) => f.write_str(&repr),
            Err(_) => Err(std::fmt::Error),
        }
    }
}

/// Collects diagnostics emitted by the linter.
///
/// This function spawns a sandbox language server with the given code in the `src/lib.cairo` file.
/// The Cairo source code is expected to contain caret markers.
/// The function then requests quick fixes at each caret position and compares the result with the
/// expected quick fixes from the snapshot file.
fn test_linter_diagnostics(cairo_code: &str) -> Report {
    let mut ls = sandbox!(
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => cairo_code
        }
        workspace_configuration = json!({
            "cairo1": {
                "enableLinter": true
            }
        });
    );

    let diagnostics = ls
        .open_and_wait_for_diagnostics("src/lib.cairo")
        .into_iter()
        .map(Diagnostic::from)
        .collect();

    Report { diagnostics }
}
