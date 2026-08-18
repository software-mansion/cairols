use std::fmt::Display;

use indoc::indoc;
use lsp_types::request::Completion;
use lsp_types::{
    CompletionContext, CompletionParams, CompletionTriggerKind, TextDocumentPositionParams,
    lsp_request,
};
use serde::Serialize;

use crate::support::cursor::{Cursors, peek_caret};
use crate::support::fixture::Fixture;
use crate::support::transform::Transformer;
use crate::support::{MockClient, fixture};

mod attribute;
mod doc_links;
mod dot;
mod macros;
mod methods_text_edits;
mod mod_file;
mod order;
mod path;
mod patterns;
mod structs;
mod traits;
mod untyped;
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
            edition = "2025_12"
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
            edition = "2025_12"
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

fn sorting_dep_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc!(r#"
            [crate_roots]
            hello = "src"
            alexandria_sorting = "sorting/src"
            alexandria_data_structures = "data_structures/src"

            [config.override.hello]
            edition = "2025_12"
            [config.override.alexandria_sorting]
            edition = "2023_11"
            [config.override.alexandria_data_structures]
            edition = "2023_11"

            [config.override.hello.dependencies]
            alexandria_sorting = { discriminator = "alexandria_sorting" }

            [config.override.alexandria_sorting.dependencies]
            alexandria_data_structures = { discriminator = "alexandria_data_structures" }
        "#),
        "data_structures/src/lib.cairo" => indoc!("
            pub mod vec;
        "),
        "data_structures/src/vec.cairo" => indoc!("
            pub struct Felt252Vec<T> {
                pub items: Felt252Dict<Nullable<T>>,
                pub len: usize,
            }
        "),
        "sorting/src/lib.cairo" => indoc!("
            pub mod interface;
            pub mod merge_sort;

            pub use interface::{Sortable, SortableVec};
            pub use merge_sort::MergeSort;
        "),
        "sorting/src/interface.cairo" => indoc!("
            use alexandria_data_structures::vec::Felt252Vec;

            pub trait Sortable {
                fn sort<T, +Copy<T>, +Drop<T>, +PartialOrd<T>>(array: Span<T>) -> Array<T>;
            }

            pub trait SortableVec {
                fn sort<T, +Copy<T>, +Drop<T>, +PartialOrd<T>, +Felt252DictValue<T>>(
                    array: Felt252Vec<T>,
                ) -> Felt252Vec<T>;
            }
        "),
        "sorting/src/merge_sort.cairo" => indoc!("
            use super::Sortable;

            pub impl MergeSort of Sortable {
                fn sort<T, +Copy<T>, +Drop<T>, +PartialOrd<T>>(mut array: Span<T>) -> Array<T> {
                    let len = array.len();
                    if len == 0 {
                        return array![];
                    }
                    if len == 1 {
                        return array![*array[0]];
                    }
                    let middle = len / 2;
                    let left_arr = array.slice(0, middle);
                    let right_arr = array.slice(middle, len - middle);
                    let sorted_left = Self::sort(left_arr);
                    let sorted_right = Self::sort(right_arr);
                    let mut result_arr = array![];
                    merge_recursive(sorted_left, sorted_right, ref result_arr, 0, 0);
                    result_arr
                }
            }

            fn merge_recursive<T, +Copy<T>, +Drop<T>, +PartialOrd<T>>(
                mut left_arr: Array<T>,
                mut right_arr: Array<T>,
                ref result_arr: Array<T>,
                left_arr_ix: usize,
                right_arr_ix: usize,
            ) {
                if result_arr.len() == left_arr.len() + right_arr.len() {
                    return;
                }
                if left_arr_ix == left_arr.len() {
                    result_arr.append(*right_arr[right_arr_ix]);
                    return merge_recursive(left_arr, right_arr, ref result_arr, left_arr_ix, right_arr_ix + 1);
                }
                if right_arr_ix == right_arr.len() {
                    result_arr.append(*left_arr[left_arr_ix]);
                    return merge_recursive(left_arr, right_arr, ref result_arr, left_arr_ix + 1, right_arr_ix);
                }
                if *left_arr[left_arr_ix] < *right_arr[right_arr_ix] {
                    result_arr.append(*left_arr[left_arr_ix]);
                    merge_recursive(left_arr, right_arr, ref result_arr, left_arr_ix + 1, right_arr_ix)
                } else {
                    result_arr.append(*right_arr[right_arr_ix]);
                    merge_recursive(left_arr, right_arr, ref result_arr, left_arr_ix, right_arr_ix + 1)
                }
            }
        ")
    }
}

fn transform(ls: MockClient, cursors: Cursors, main_file: &str) -> String {
    transform_with_context(ls, cursors, main_file, None)
}

fn transform_triggered_by_char(
    ls: MockClient,
    cursors: Cursors,
    main_file: &str,
    trigger_char: char,
) -> String {
    transform_with_context(
        ls,
        cursors,
        main_file,
        Some(CompletionContext {
            trigger_kind: CompletionTriggerKind::TRIGGER_CHARACTER,
            trigger_character: Some(trigger_char.to_string()),
        }),
    )
}

fn transform_with_context(
    mut ls: MockClient,
    cursors: Cursors,
    main_file: &str,
    context: Option<CompletionContext>,
) -> String {
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
        context,
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
                completion_label_path: completion.label_details.clone().unwrap_or_default().detail,
                completion_label_type_info: completion
                    .label_details
                    .unwrap_or_default()
                    .description,
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
    completion_label_type_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insert_text: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    text_edits: Vec<String>,
}
