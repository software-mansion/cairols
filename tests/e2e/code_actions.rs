use cairo_lang_test_utils::parse_test_file::TestRunnerResult;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use lsp_types::{
    ClientCapabilities, CodeActionContext, CodeActionOrCommand, CodeActionParams,
    HoverClientCapabilities, MarkupKind, Range, TextDocumentClientCapabilities, lsp_request,
};

use crate::support::cursor::peek_caret;
use crate::support::{cursors, sandbox};

cairo_lang_test_utils::test_file_test!(
    quick_fix,
    "tests/test_data/code_actions",
    {
        missing_trait: "missing_trait.txt",
        macro_expand: "macro_expand.txt",
        fill_struct_fields: "fill_struct_fields.txt",
        fill_trait_members: "fill_trait_members.txt",
        create_module_file: "create_module_file.txt",
    },
    test_quick_fix
);

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default).map(|it| {
            TextDocumentClientCapabilities {
                hover: Some(HoverClientCapabilities {
                    dynamic_registration: Some(false),
                    content_format: Some(vec![MarkupKind::Markdown, MarkupKind::PlainText]),
                }),
                ..it
            }
        }),
        ..base
    }
}

/// Perform quick fix test.
///
/// This function spawns a sandbox language server with the given code in the `src/lib.cairo` file.
/// The Cairo source code is expected to contain caret markers.
/// The function then requests quick fixes at each caret position and compares the result with the
/// expected quick fixes from the snapshot file.
fn test_quick_fix(
    inputs: &OrderedHashMap<String, String>,
    _args: &OrderedHashMap<String, String>,
) -> TestRunnerResult {
    let (cairo, cursors) = cursors(&inputs["cairo_code"]);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => inputs["cairo_project.toml"].clone(),
            "src/lib.cairo" => cairo.clone(),
        }
        client_capabilities = caps;
    };

    let diagnostics = ls.open_and_wait_for_diagnostics("src/lib.cairo");

    let mut actions = OrderedHashMap::default();
    let root_path = ls.as_ref().root_path().to_string_lossy().to_string();

    for (n, position) in cursors.carets().into_iter().enumerate() {
        let mut report = String::new();

        report.push_str(&peek_caret(&cairo, position));
        let code_action_params = CodeActionParams {
            text_document: ls.doc_id("src/lib.cairo"),
            range: Range { start: position, end: position },
            context: CodeActionContext {
                diagnostics: diagnostics.diagnostics.clone(),
                only: None,
                trigger_kind: None,
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
        };
        let code_actions =
            ls.send_request::<lsp_request!("textDocument/codeAction")>(code_action_params);
        if let Some(code_actions) = code_actions {
            report.push_str(&render_code_actions_or_commands(code_actions, &root_path));
        } else {
            panic!("Code actions request failed.");
        }
        actions.insert(format!("Code action #{}", n), report);
    }

    TestRunnerResult::success(actions)
}

fn render_code_actions_or_commands(
    code_actions_or_commands: Vec<CodeActionOrCommand>,
    root_path: &str,
) -> String {
    if code_actions_or_commands.is_empty() {
        return "No code actions.\n".to_string();
    }
    let mut result = String::new();
    for code_action_or_command in code_actions_or_commands {
        result.push_str(&render_code_action_or_command(&code_action_or_command, root_path));
    }
    result
}

fn render_code_action_or_command(
    code_action_or_command: &CodeActionOrCommand,
    root_path: &str,
) -> String {
    let mut result = String::new();
    match code_action_or_command {
        CodeActionOrCommand::Command(_) => todo!("Not implemented yet"),
        CodeActionOrCommand::CodeAction(code_action) => {
            result.push_str(&format!("Title: {}\n", code_action.title));

            if let Some(document_changes) =
                code_action.edit.as_ref().and_then(|edit| edit.document_changes.as_ref())
            {
                result.push_str(&format!(
                    "Document changes json: {}\n",
                    serde_json::to_string_pretty(&document_changes).unwrap().replace(root_path, "")
                ));
            }

            let Some(changes) = code_action.edit.as_ref().and_then(|edit| edit.changes.as_ref())
            else {
                return result;
            };
            for edits in changes.values() {
                for edit in edits {
                    result.push_str(&format!(
                        "Add new text: \"{}\"\nAt: {:?}\n",
                        edit.new_text, edit.range
                    ));
                }
            }
        }
    }
    result
}
