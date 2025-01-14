use cairo_language_server::lsp::ext::ExpandMacro;
use itertools::Itertools;
use lsp_types::{TextDocumentIdentifier, TextDocumentPositionParams};
use serde::Serialize;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::peek_caret;
use crate::support::data::data;
use crate::support::quick_hash::quick_hash;
use crate::support::{cursors, sandbox};

#[test]
fn test_macro_expand_attribute() {
    check(data!("attribute.cairo.template"), "attribute")
}

#[test]
fn test_macro_expand_simple_inline() {
    check(data!("simple_inline.cairo.template"), "simple_inline")
}

#[test]
fn test_macro_expand_empty() {
    check(data!("empty.cairo.template"), "empty")
}

#[test]
fn test_macro_expand_derive() {
    check(data!("derive.cairo.template"), "derive")
}

fn check(code: String, case_name: &str) {
    let (cairo, cursors) = cursors(&code);

    let mut ls = sandbox! {
        files {
            "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
            "src/lib.cairo" => cairo.clone(),
        }
    };

    ls.open_all_cairo_files_and_wait_for_project_update();

    for (expansion, source_contexts) in cursors
        .carets()
        .into_iter()
        .map(|position| {
            let source_context = peek_caret(&cairo, position);

            let macro_expansion = ls.send_request::<ExpandMacro>(TextDocumentPositionParams {
                position,
                text_document: TextDocumentIdentifier { uri: ls.doc_id("src/lib.cairo").uri },
            });

            let expansion =
                macro_expansion.unwrap_or_else(|| String::from("No expansion information.\n"));

            (expansion, source_context)
        })
        .into_group_map()
    {
        let hash = quick_hash(&expansion);
        let source_contexts = source_contexts.join("\n");
        let report = MacroExpandReport { source_contexts, expansion };

        insta::assert_toml_snapshot!(format!("{case_name}#{hash:x}"), report);
    }
}

#[derive(Serialize)]
struct MacroExpandReport {
    source_contexts: String,
    expansion: String,
}
