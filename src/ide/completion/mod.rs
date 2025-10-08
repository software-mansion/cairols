use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use attribute::attribute_completions;
use attribute::derive::derive_completions;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use function::params::params_completions;
use function::variables::variables_completions;
use lsp_types::{CompletionParams, CompletionResponse, CompletionTriggerKind};
use path::path_suffix_completions;
use pattern::{enum_pattern_completions, struct_pattern_completions};
use self_completions::self_completions;
use struct_constructor::struct_constructor_completions;

use self::dot_completions::dot_completions;
use crate::ide::completion::expr::macro_call::{
    expr_inline_macro_completions, top_level_inline_macro_completions,
};
use crate::ide::completion::helpers::item::{
    CompletionItemHashable, CompletionItemOrderable, CompletionRelevance,
};
use crate::ide::completion::mod_item::mod_completions;
use crate::ide::completion::use_statement::use_completions;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};

mod attribute;
mod dot_completions;
mod expr;
mod function;
mod helpers;
mod mod_item;
mod path;
mod pattern;
mod self_completions;
mod struct_constructor;
mod use_statement;

/// Compute completion items at a given cursor position.
pub fn complete(params: CompletionParams, db: &AnalysisDatabase) -> Option<CompletionResponse> {
    let text_document_position = params.text_document_position;
    let file_id = db.file_for_url(&text_document_position.text_document.uri)?;
    let mut position = text_document_position.position;
    let base_position_node = db.find_syntax_node_at_position(file_id, position.to_cairo())?;

    // Try searching one character to the left to account for cases where caret is at the end
    // of a text e.g. `my::path::to::ite<caret>`.
    position.character = position.character.saturating_sub(1);
    let mut node = db.find_syntax_node_at_position(file_id, position.to_cairo())?;
    let was_node_corrected = base_position_node == node;

    // There is no completions for these.
    if matches!(node.kind(db), SyntaxKind::TokenSkipped | SyntaxKind::TokenSingleLineComment) {
        return None;
    }

    if matches!(
        node.kind(db),
        SyntaxKind::TokenSingleLineDocComment | SyntaxKind::TokenSingleLineInnerComment
    ) {
        // TODO(#290) doc completions.
        return None;
    }

    // In case we are on eof go back to last non-trivia non-missing node.
    if node.kind(db) == SyntaxKind::SyntaxFile
        || node.ancestor_of_kind(db, SyntaxKind::TerminalEndOfFile).is_some()
            && node.ancestor_of_kind(db, SyntaxKind::TriviumSkippedNode).is_none()
    {
        let syntax = db.file_module_syntax(file_id).to_option()?;

        let last_item = syntax.items(db).elements(db).last()?.as_syntax_node();

        node = find_last_meaning_node(db, last_item);
    }

    // Skip trivia.
    while ast::Trivium::is_variant(node.kind(db))
        || node.kind(db) == SyntaxKind::Trivia
        || node.kind(db).is_token()
    {
        node = node.parent(db).unwrap_or(node);
    }

    let trigger_kind =
        params.context.map(|it| it.trigger_kind).unwrap_or(CompletionTriggerKind::INVOKED);

    let deduplicated_items: Vec<_> = db
        .get_node_resultants(node)?
        .iter()
        .filter_map(|resultant| complete_ex(*resultant, trigger_kind, was_node_corrected, db))
        .flatten()
        .map(CompletionItemHashable)
        .collect::<OrderedHashSet<_>>()
        .into_iter()
        .map(|item| item.0)
        .collect();

    // Need to also deduplicate items with different relevance and leave the one with the highest relevance.
    let mut result = unique_completion_items_with_highest_relevance(deduplicated_items);
    result.sort_by(|a, b| {
        b.relevance.cmp(&a.relevance).then_with(|| compare_items_by_label_and_detail(a, b))
    });

    // Set the sort text as it's used to sort the items on the client side.
    // We want to keep the order the same way we have it here.
    for (index, item) in result.iter_mut().enumerate() {
        item.item.sort_text = Some(index.to_string());
    }

    Some(CompletionResponse::Array(result.into_iter().map(|item| item.item).collect()))
}

fn complete_ex<'db>(
    node: SyntaxNode<'db>,
    trigger_kind: CompletionTriggerKind,
    was_node_corrected: bool,
    db: &'db AnalysisDatabase,
) -> Option<Vec<CompletionItemOrderable>> {
    let ctx = AnalysisContext::from_node(db, node)?;
    let crate_id = ctx.module_file_id.0.owning_crate(db);

    let mut completions = vec![];

    completions.extend(dot_completions(db, &ctx, was_node_corrected));
    completions.extend(struct_constructor_completions(db, &ctx));
    completions.extend(use_completions(db, &ctx));
    completions.extend(self_completions(db, &ctx));
    completions.extend(attribute_completions(db, node, crate_id));
    completions.extend(derive_completions(db, node, crate_id));
    completions.extend(mod_completions(db, node));
    completions.extend(params_completions(db, &ctx, was_node_corrected));
    completions.extend(variables_completions(db, &ctx, was_node_corrected));
    completions.extend(struct_pattern_completions(db, &ctx));
    completions.extend(enum_pattern_completions(db, &ctx));
    completions.extend(expr_inline_macro_completions(db, &ctx, was_node_corrected));
    completions.extend(top_level_inline_macro_completions(db, &ctx, was_node_corrected));

    if trigger_kind == CompletionTriggerKind::INVOKED {
        completions.extend(path_suffix_completions(db, &ctx, was_node_corrected))
    }

    Some(completions)
}

fn find_last_meaning_node<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> SyntaxNode<'db> {
    for child in node.get_children(db).iter().rev() {
        if child.kind(db) == SyntaxKind::Trivia {
            continue;
        }

        if child
            .get_children(db)
            .iter()
            .find(|grand_child| grand_child.kind(db) != SyntaxKind::Trivia)
            .is_some_and(|grand_child| grand_child.kind(db) == SyntaxKind::TokenMissing)
        {
            continue;
        }

        return find_last_meaning_node(db, *child);
    }

    node
}

/// Given a list of completion items, returns a list with unique items keeping the one with the highest relevance.
fn unique_completion_items_with_highest_relevance(
    relevance_items: Vec<CompletionItemOrderable>,
) -> Vec<CompletionItemOrderable> {
    let mut unique_items: HashMap<String, CompletionItemOrderable> = HashMap::new();

    for relevance_item in relevance_items {
        let key =
            serde_json::to_string(&relevance_item.item).expect("serialization should not fail");
        match unique_items.entry(key) {
            Entry::Occupied(mut occupied) => {
                if relevance_item.relevance > occupied.get().relevance {
                    *occupied.get_mut() = relevance_item;
                }
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(relevance_item);
            }
        };
    }

    unique_items.into_values().collect()
}

fn compare_items_by_label_and_detail(
    a: &CompletionItemOrderable,
    b: &CompletionItemOrderable,
) -> Ordering {
    a.item
        .label
        .cmp(&b.item.label)
        .then_with(|| {
            let a_description = a.item.label_details.clone().unwrap_or_default().description;
            let b_description = b.item.label_details.clone().unwrap_or_default().description;
            a_description.cmp(&b_description)
        })
        .then_with(|| {
            let a_description = a.item.detail.clone();
            let b_description = b.item.detail.clone();
            a_description.cmp(&b_description)
        })
}
