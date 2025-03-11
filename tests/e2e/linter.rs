use std::fmt::Display;

use indoc::indoc;
use serde::Serialize;
use serde_json::json;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::fixture::{Fixture, fixture};
use crate::support::sandbox;

#[test]
fn test_simple_lint() {
    let report = test_linter_diagnostics(fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => indoc!(r#"
            fn foo() {
                let mut span = array![0x0].span();

                loop {
                    match span.pop_front() {
                        Some(_) => {},
                        None => { break; },
                    }
                }
            }
        "#)
    });

    insta::assert_toml_snapshot!(
        report,
        @r"
        [[diagnostics]]
        severity = 'Warning'
        message = 'Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`'
        "
    );
}

#[test]
fn test_two_simultaneous_lints() {
    let report = test_linter_diagnostics(fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => r#"
        fn foo() {
            let mut span = array![0x0].span();

            loop {
                match span.pop_front() {
                    Some(_) => {},
                    None => { break (); },
                }
            }
        }
        "#
    });

    insta::assert_toml_snapshot!(
        report,
        @r"
        [[diagnostics]]
        severity = 'Warning'
        message = 'Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.'

        [[diagnostics]]
        severity = 'Warning'
        message = 'Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`'
        "
    );
}

#[test]
fn test_linter_with_starknet_analyzer_plugins() {
    let report = test_linter_diagnostics(fixture! {
        "Scarb.toml" => indoc!(r#"
            [package]
            name = "test_package"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            starknet = "2.10.0"
        "#),
        "src/lib.cairo" => indoc!(r#"
            //! > cairo_code
            #[starknet::contract]
            mod test_contract {
                #[storage]
                struct Storage {}

                #[external(v0)]
                fn foo() {
                    loop {
                        break ();
                    }
                }
            }
        "#)
    });

    insta::assert_toml_snapshot!(
        report,
        @r"
        [[diagnostics]]
        severity = 'Error'
        message = 'Plugin diagnostic: The first parameter of an entry point must be `self`.'

        [[diagnostics]]
        severity = 'Warning'
        message = 'Plugin diagnostic: Failed to generate ABI: Entrypoints must have a self first param.'

        [[diagnostics]]
        severity = 'Warning'
        message = 'Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.'
        "
    );
}

#[test]
fn allow_lint_doesnt_generate_diagnostics_with_linter_off() {
    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => indoc!(r#"
                #[allow(break_unit)]
                fn uwu() {}
            "#),
        }
        workspace_configuration = json!({
            "cairo1": {
                "enableLinter": false
            }
        });
    };

    assert!(ls.open_and_wait_for_diagnostics("src/lib.cairo").is_empty());
}

#[test]
fn allow_lint_doesnt_generate_diagnostics_for_scarb_package_with_linter_off() {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"
            "#),
            "src/lib.cairo" => indoc!(r#"
                #[allow(break_unit)]
                fn uwu() {}
            "#),
        }
        workspace_configuration = json!({
            "cairo1": {
                "enableLinter": false
            }
        });
    };

    assert!(ls.open_and_wait_for_diagnostics("src/lib.cairo").is_empty());
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
/// This function spawns a sandbox language server with the given fixture.
/// The Cairo source code is expected to contain caret markers.
/// The function then requests quick fixes at each caret position and compares the result with the
/// expected quick fixes from the snapshot file.
fn test_linter_diagnostics(fixture: Fixture) -> Report {
    let mut ls = sandbox!(
        fixture = fixture;
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
