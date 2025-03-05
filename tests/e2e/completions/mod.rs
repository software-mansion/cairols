use std::fmt::Display;

use lsp_types::{CompletionParams, TextDocumentPositionParams, lsp_request};
use serde::Serialize;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::peek_caret;
use crate::support::{MockClient, cursors, sandbox};

mod attribute;
mod methods_text_edits;
mod mod_file;
mod module_items;
mod path;
mod structs;
mod traits;

/// Perform completions text edits test. Notice that the test shows many possible completions,
/// however in practice only those who have the same prefix as the existing code are shown.
///
/// This function spawns a sandbox language server with the given code in the `src/lib.cairo` file.
/// The Cairo source code is expected to contain caret markers.
/// The function then requests quick fixes at each caret position and compares the result with the
/// expected quick fixes from the snapshot file.
fn test_completions_text_edits(cairo_code: &str) -> Report {
    test_completions_text_edits_inner(cairo_code, "src/lib.cairo", |cairo| {
        sandbox! {
            files {
                "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
                "src/lib.cairo" => cairo,
            }
        }
    })
}

fn test_completions_text_edits_inner(
    cairo_code: &str,
    file: &str,
    ls: impl FnOnce(&str) -> MockClient,
) -> Report {
    let (cairo, cursors) = cursors(cairo_code);

    let mut ls = ls(&cairo);

    ls.open_all_cairo_files_and_wait_for_project_update();

    assert_eq!(cursors.carets().len(), 1);
    let position = cursors.carets()[0];

    let caret = peek_caret(&cairo, position);

    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: ls.doc_id(file),
            position,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    };

    let caret_completions =
        ls.send_request::<lsp_request!("textDocument/completion")>(completion_params);

    let mut completion_items = caret_completions
        .map(|completions| match completions {
            lsp_types::CompletionResponse::Array(items) => items,
            lsp_types::CompletionResponse::List(list) => list.items,
        })
        .unwrap_or_default();

    completion_items.sort_by_key(|x| x.label.clone());

    Report {
        caret,
        completions: completion_items
            .into_iter()
            .map(|completion| Completions {
                completion_label: completion.label,
                detail: completion.detail,
                insert_text: completion.insert_text,
                text_edits: completion
                    .additional_text_edits
                    .unwrap_or_default()
                    .into_iter()
                    .map(|edit| edit.new_text)
                    .collect(),
            })
            .collect(),
    }
}

#[derive(Serialize)]
struct Report {
    caret: String,
    completions: Vec<Completions>,
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringifed = toml::to_string_pretty(self).map_err(|_| std::fmt::Error)?;

        f.write_str(&stringifed)
    }
}

#[derive(Serialize)]
struct Completions {
    completion_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_text: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    text_edits: Vec<String>,
}
