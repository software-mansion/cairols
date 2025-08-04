use std::fmt::Display;

use lsp_types::{
    ClientCapabilities, CodeLensParams, DynamicRegistrationClientCapabilities,
    ExecuteCommandParams, TextDocumentClientCapabilities, Url,
};
use serde::Serialize;

use crate::support::cursor::Cursor;
use crate::support::{cursors, sandbox};
use cairo_language_server::lsp::ext::{ExecuteInTerminal, ExecuteInTerminalParams};
use indoc::indoc;
use lsp_types::request::{CodeLensRequest, ExecuteCommand};
use serde_json::{Value, json};
use std::path::PathBuf;

mod both_runners;
mod cairo_test;
mod custom;
mod no_runners;
mod other_file;
mod snforge;

fn test_code_lens_snforge(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        indoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            snforge_std = "0.38.3"

            [tool.scarb]
            allow-prebuilt-plugins = ["snforge_std"]
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_cairo_test(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        indoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            cairo_test = "2.9.0"
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_both_runners(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        indoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            cairo_test = "2.9.0"
            snforge_std = "0.38.3"

            [tool.scarb]
            allow-prebuilt-plugins = ["snforge_std"]
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_no_runner(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        indoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2024_07"
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_custom_runner(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        indoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2024_07"
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true,
                "runTestCommand": "some random template ->{{TEST_PATH}}<- string",
                "testRunner": "custom"
            }
        }),
    )
}

fn test_code_lens(cairo_code: &str, scarb_toml: &str, config: Value) -> Report {
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "Scarb.toml" => scarb_toml,
            "src/lib.cairo" => cairo.clone(),
            "src/foo.cairo" => r#"
                #[test]
                fn test_from_other_file() {}
            "#,
            "src/bar.cairo" => r#"
                fn no_test_from_other_file() {}
            "#,
            "src/baz.cairo" => r#"
                mod wrapper {
                    #[test]
                    fn nested_test_in_other_file() {}
                }
            "#,
        }
        client_capabilities = caps;
        workspace_configuration = config;
    };

    let position = match cursors.assert_single() {
        Cursor::Caret(position) => position,
        Cursor::Selection(range) => range.start,
    };

    ls.open_and_wait_for_diagnostics_generation("src/lib.cairo");

    let lenses = ls.send_request::<CodeLensRequest>(CodeLensParams {
        text_document: ls.doc_id("src/lib.cairo"),
        partial_result_params: Default::default(),
        work_done_progress_params: Default::default(),
    });

    let execute_in_terminal = lenses.as_ref().and_then(|lenses| {
        let code_lens =
            lenses.iter().find(|code_lens| code_lens.range.start.line == position.line)?;

        let command = code_lens.command.clone().unwrap();

        ls.send_request::<ExecuteCommand>(ExecuteCommandParams {
            command: command.command,
            arguments: command.arguments.clone().unwrap().clone(),
            work_done_progress_params: Default::default(),
        });

        let ExecuteInTerminalParams { command, cwd } =
            ls.wait_for_notification::<ExecuteInTerminal>(|_| true);

        Some(ShellCommand { command, cwd: ls.fixture.file_relative_path(cwd) })
    });

    Report {
        lenses: lenses.map(|lenses| {
            let mut lenses: Vec<_> = lenses
                .into_iter()
                .map(|code_lens| {
                    let command = code_lens.command.unwrap();
                    let args = command.arguments.unwrap();

                    assert_eq!(command.command, "cairo.executeCodeLens");

                    CodeLensReport {
                        line: code_lens.range.start.line,
                        command: command.title,
                        index: args[0].as_u64().unwrap(),
                        file_path: ls
                            .fixture
                            .url_path(&Url::parse(args[1].as_str().unwrap()).unwrap())
                            .unwrap(),
                    }
                })
                .collect();

            lenses.sort_by_key(|lens| lens.line);

            lenses
        }),
        execute_in_terminal,
    }
}

#[derive(Serialize)]
struct Report {
    #[serde(skip_serializing_if = "Option::is_none")]
    lenses: Option<Vec<CodeLensReport>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    execute_in_terminal: Option<ShellCommand>,
}

#[derive(Serialize)]
struct CodeLensReport {
    line: u32,
    command: String,
    index: u64,
    file_path: PathBuf,
}

#[derive(Serialize)]
struct ShellCommand {
    command: String,
    cwd: PathBuf,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringifed = toml::to_string_pretty(self).map_err(|_| std::fmt::Error)?;

        f.write_str(&stringifed)
    }
}

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                code_lens: Some(DynamicRegistrationClientCapabilities {
                    dynamic_registration: Some(false),
                }),
                ..it
            }
        }),
        experimental: Some(json!({ "cairo": { "executeInTerminal": {} } })),
        ..base
    }
}
