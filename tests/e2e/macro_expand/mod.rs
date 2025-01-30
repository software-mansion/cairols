use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_language_server::lsp::ext::ExpandMacro;
use lsp_types::{TextDocumentIdentifier, TextDocumentPositionParams};
use serde::Serialize;

use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::peek_caret;
use crate::support::data::data;
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

    let mut ordered_hash_map: OrderedHashMap<String, Vec<String>> = OrderedHashMap::default();
    cursors.carets().into_iter().for_each(|position| {
        let source_context = peek_caret(&cairo, position);

        let macro_expansion = ls.send_request::<ExpandMacro>(TextDocumentPositionParams {
            position,
            text_document: TextDocumentIdentifier { uri: ls.doc_id("src/lib.cairo").uri },
        });

        let expansion =
            macro_expansion.unwrap_or_else(|| String::from("No expansion information.\n"));

        ordered_hash_map.entry(expansion).or_default().push(source_context);
    });

    for (snap_seq, (expansion, source_contexts)) in ordered_hash_map.into_iter().enumerate() {
        let source_contexts = source_contexts.join("\n");
        let report = MacroExpandReport { source_contexts, expansion };

        insta::assert_toml_snapshot!(format!("{case_name}#snap_{snap_seq}"), report);
    }
}

#[derive(Serialize)]
struct MacroExpandReport {
    source_contexts: String,
    expansion: String,
}
