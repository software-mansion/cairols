use std::fmt::Display;
use std::path::PathBuf;

use cairo_language_server::lsp::ext::{
    ExecuteInTerminal, ExecuteInTerminalParams, LaunchDebugger, LaunchDebuggerParams,
};
use indoc::{formatdoc, indoc};
use lsp_types::notification::ShowMessage;
use lsp_types::request::{CodeLensRequest, ExecuteCommand};
use lsp_types::{
    ClientCapabilities, CodeLensParams, DynamicRegistrationClientCapabilities,
    ExecuteCommandParams, ShowMessageParams, TextDocumentClientCapabilities, Url,
};
use serde::Serialize;
use serde_json::{Value, json};

use crate::support::cursor::Cursor;
use crate::support::{cursors, sandbox};

mod both_runners;
mod cairo_test;
mod custom;
mod executable;
mod no_runners;
mod other_file;
mod proc_macro;
mod snforge;

fn test_code_lens_scarb_execute(args: (&str, &str)) -> Report {
    let (cairo_code, scarb_toml) = args;
    test_code_lens(
        cairo_code,
        scarb_toml,
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_snforge(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        indoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            snforge_std = "0.50.0"

            [tool.scarb]
            allow-prebuilt-plugins = ["snforge_std"]

            [cairo]
            add-functions-debug-info = true
            skip-optimizations = true
            unstable-add-statements-code-locations-debug-info = true
            add-statements-functions-debug-info = true
            add-types-debug-info = true
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_snforge_with_macros(cairo_code: &str) -> Report {
    test_code_lens(
        cairo_code,
        &formatdoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            snforge_std = "0.50.0"
            cairols_test_macros_v2 = {{ path = "{}" }}

            [tool.scarb]
            allow-prebuilt-plugins = ["snforge_std"]

            [cairo]
            add-functions-debug-info = true
            skip-optimizations = true
            unstable-add-statements-code-locations-debug-info = true
            add-statements-functions-debug-info = true
            add-types-debug-info = true
            "#,
            crate::macros::SCARB_TEST_MACROS_V2_PACKAGE.display()
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
    )
}

fn test_code_lens_snforge_wrong_debug_config(cairo_code: &str) -> Report {
    test_code_lens_impl(
        cairo_code,
        &formatdoc!(
            r#"
            [package]
            name = "hello"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            snforge_std = "0.50.0"

            [tool.scarb]
            allow-prebuilt-plugins = ["snforge_std"]

            [cairo]
            add-functions-debug-info = true
            skip-optimizations = false
            add-statements-code-locations-debug-info = true
            add-statements-functions-debug-info = true
            "#
        ),
        json!({
            "cairo1": {
                "enableProcMacros": true
            }
        }),
        true,
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
            edition = "2025_12"

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
            edition = "2025_12"

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
            edition = "2025_12"
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
            edition = "2025_12"
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
    test_code_lens_impl(cairo_code, scarb_toml, config, false)
}

fn test_code_lens_impl(
    cairo_code: &str,
    scarb_toml: &str,
    config: Value,
    expect_wrong_compiler_config_for_debug: bool,
) -> Report {
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

    let mut execute_in_terminal: Vec<ShellCommand> = vec![];
    let mut show_messages: Vec<ShowMessageReport> = vec![];

    if let Some(lenses) = &lenses {
        for code_lens in lenses.iter().filter(|l| l.range.start.line == position.line) {
            let command = code_lens.command.clone().unwrap();

            ls.send_request::<ExecuteCommand>(ExecuteCommandParams {
                command: command.command,
                arguments: command.arguments.unwrap().clone(),
                work_done_progress_params: Default::default(),
            });

            let is_debug_lens = command.title.contains("Debug");

            if is_debug_lens && expect_wrong_compiler_config_for_debug {
                let ShowMessageParams { typ, message } =
                    ls.wait_for_notification::<ShowMessage>(|_| true);
                show_messages.push(ShowMessageReport { typ: format!("{typ:?}"), message });
            } else {
                let (command, cwd) = if is_debug_lens {
                    let LaunchDebuggerParams { command, cwd, test_name: _ } =
                        ls.wait_for_notification::<LaunchDebugger>(|_| true);
                    (command, cwd)
                } else {
                    let ExecuteInTerminalParams { command, cwd } =
                        ls.wait_for_notification::<ExecuteInTerminal>(|_| true);
                    (command, cwd)
                };
                execute_in_terminal
                    .push(ShellCommand { command, cwd: ls.fixture.file_relative_path(cwd) });
            }
        }
    }

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
                        file_path: ls
                            .fixture
                            .url_path(&Url::parse(args[0].as_str().unwrap()).unwrap())
                            .unwrap(),
                        index: args[1].as_u64().unwrap() as u32,
                    }
                })
                .collect();

            lenses.sort_by_key(|lens| lens.line);

            lenses
        }),
        show_messages,
        execute_in_terminal,
    }
}

#[derive(Serialize)]
struct Report {
    #[serde(skip_serializing_if = "Option::is_none")]
    lenses: Option<Vec<CodeLensReport>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    show_messages: Vec<ShowMessageReport>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    execute_in_terminal: Vec<ShellCommand>,
}

#[derive(Serialize)]
struct CodeLensReport {
    line: u32,
    command: String,
    file_path: PathBuf,
    index: u32,
}

#[derive(Serialize)]
struct ShowMessageReport {
    typ: String,
    message: String,
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
        experimental: Some(json!({ "cairo": { "executeInTerminal": {}, "launchDebugger": {} } })),
        ..base
    }
}
