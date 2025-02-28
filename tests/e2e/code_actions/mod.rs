use lsp_types::{
    ClientCapabilities, CodeActionContext, CodeActionOrCommand, CodeActionParams,
    HoverClientCapabilities, MarkupKind, Range, TextDocumentClientCapabilities, lsp_request,
};

use crate::support::cairo_project_toml::{CAIRO_PROJECT_TOML, CAIRO_PROJECT_TOML_2024_07};
use crate::support::{cursors, sandbox};

mod create_module_file;
mod fill_struct_fields;
mod fill_trait_members;
mod macro_expand;
mod missing_import;
mod missing_trait;
mod remove_unused_variable;

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

fn quick_fix(cairo_code: &str) -> String {
    quick_fix_general(cairo_code, CAIRO_PROJECT_TOML_2024_07)
}

fn quick_fix_without_visibility_constraints(cairo_code: &str) -> String {
    quick_fix_general(cairo_code, CAIRO_PROJECT_TOML)
}

fn quick_fix_general(cairo_code: &str, manifest_content: &str) -> String {
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => manifest_content,
            "src/lib.cairo" => cairo.clone(),
        }
        client_capabilities = caps;
    };

    let diagnostics = ls.open_and_wait_for_diagnostics("src/lib.cairo");

    assert_eq!(cursors.carets().len(), 1);
    let position = cursors.carets()[0];

    let root_path = ls.fixture.root_path().to_string_lossy().to_string();

    let code_action_params = CodeActionParams {
        text_document: ls.doc_id("src/lib.cairo"),
        range: Range { start: position, end: position },
        context: CodeActionContext {
            diagnostics: diagnostics
                .into_iter()
                .filter(|diagnostic| {
                    diagnostic.range.start <= position && position <= diagnostic.range.end
                })
                .collect(),
            only: None,
            trigger_kind: None,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };
    let code_actions =
        ls.send_request::<lsp_request!("textDocument/codeAction")>(code_action_params);

    let Some(code_actions) = code_actions else {
        panic!("Code actions request failed.");
    };

    render_code_actions_or_commands(code_actions, &root_path)
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
