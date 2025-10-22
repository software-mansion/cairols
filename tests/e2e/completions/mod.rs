use std::fmt::Display;

use indoc::indoc;
use lsp_types::request::Completion;
use lsp_types::{CompletionParams, TextDocumentPositionParams, lsp_request};
use serde::Serialize;

use crate::support::cursor::{Cursors, peek_caret};
use crate::support::fixture::Fixture;
use crate::support::transform::Transformer;
use crate::support::{MockClient, fixture};

mod attribute;
mod dot;
mod macros;
mod methods_text_edits;
mod mod_file;
mod order;
mod path;
mod patterns;
mod structs;
mod traits;
mod uses;
mod vars_and_params;

impl Transformer for Completion {
    fn capabilities(base: lsp_types::ClientCapabilities) -> lsp_types::ClientCapabilities {
        base
    }

    fn transform(ls: MockClient, cursors: Cursors, _config: Option<serde_json::Value>) -> String {
        transform(ls, cursors, Self::main_file())
    }
}

fn completion_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc!(r#"
            [crate_roots]
            hello = "src"
            dep = "dep"

            [config.override.hello]
            edition = "2024_07"
            [config.override.dep]
            edition = "2023_10" # Edition with visibility ignores

            [config.override.hello.dependencies]
            dep = { discriminator = "dep" }
        "#),
        "dep/lib.cairo" => indoc!("
            pub struct Foo {
                a: felt252
                pub b: felt252
            }
        ")
    }
}

fn completion_fixture_with_pub_dep_items() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc!(r#"
            [crate_roots]
            hello = "src"
            dep = "dep"

            [config.override.hello]
            edition = "2024_07"
            [config.override.dep]
            edition = "2023_10" # Edition with visibility ignores

            [config.override.hello.dependencies]
            dep = { discriminator = "dep" }
        "#),
        "dep/lib.cairo" => indoc!("
            pub trait AddAssign {
                fn add_assign() -> felt252;
            }

            pub trait ResultTraitCustom {}
        ")
    }
}

fn transform(mut ls: MockClient, cursors: Cursors, main_file: &str) -> String {
    let cairo = ls.fixture.read_file(main_file);
    let position = cursors.assert_single_caret();

    let caret = peek_caret(&cairo, position);

    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: ls.doc_id(main_file),
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

    // This ensures that tests return the same order that will be presented on the client side.
    // Refer to [lsp_types::CompletionItem::sort_text] for more details.
    completion_items.sort_by(|a, b| match (&a.sort_text, &b.sort_text) {
        (Some(a_sort), Some(b_sort)) => {
            let ord = a_sort.cmp(b_sort);
            if ord == std::cmp::Ordering::Equal { a.label.cmp(&b.label) } else { ord }
        }
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.label.cmp(&b.label),
    });

    Report {
        caret,
        completions: completion_items
            .into_iter()
            .map(|completion| Completions {
                completion_label: completion.label,
                completion_label_path: completion.label_details.unwrap_or_default().description,
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
    .to_string()
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
    completion_label_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_text: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    text_edits: Vec<String>,
}
