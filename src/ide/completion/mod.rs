use std::hash::Hash;

use attribute::attribute_completions;
use attribute::derive::derive_completions;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use expr::macro_call::macro_call_completions;
use function::params::params_completions;
use function::variables::variables_completions;
use helpers::binary_expr::dot_rhs::dot_expr_rhs;
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse, CompletionTriggerKind};
use path::path_suffix_completions;
use pattern::{enum_pattern_completions, struct_pattern_completions};
use self_completions::self_completions;
use struct_constructor::struct_constructor_completions;

use self::dot_completions::dot_completions;
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
    if matches!(
        node.kind(db),
        SyntaxKind::TokenSkipped
            | SyntaxKind::TriviumSkippedNode
            | SyntaxKind::TokenSingleLineComment
    ) {
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

    let result: Vec<_> = db
        .get_node_resultants(node)?
        .into_iter()
        .filter_map(|resultant| complete_ex(resultant, trigger_kind, was_node_corrected, db))
        .flatten()
        .map(CompletionItemHashable)
        .collect::<OrderedHashSet<_>>()
        .into_iter()
        .map(|item| item.0)
        .collect();

    Some(CompletionResponse::Array(result))
}

fn complete_ex<'db>(
    node: SyntaxNode<'db>,
    trigger_kind: CompletionTriggerKind,
    was_node_corrected: bool,
    db: &'db AnalysisDatabase,
) -> Option<Vec<CompletionItem>> {
    let ctx = AnalysisContext::from_node(db, node)?;
    let crate_id = ctx.module_file_id.0.owning_crate(db);
    let file_id = node.stable_ptr(db).file_id(db);

    let mut completions = vec![];

    completions.extend(dot_completions(db, &ctx, node, was_node_corrected));
    completions.extend(struct_constructor_completions(db, &ctx, node));
    completions.extend(use_completions(db, node, &ctx));
    completions.extend(self_completions(db, &ctx));
    completions.extend(attribute_completions(db, node, crate_id));
    completions.extend(derive_completions(db, node, crate_id));
    completions.extend(mod_completions(db, node, file_id));
    completions.extend(params_completions(db, &ctx, was_node_corrected));
    completions.extend(variables_completions(db, &ctx, was_node_corrected));
    completions.extend(struct_pattern_completions(db, &ctx));
    completions.extend(enum_pattern_completions(db, &ctx));

    if dot_expr_rhs(db, &node, was_node_corrected).is_none() {
        completions.extend(macro_call_completions(db, &ctx));

        if trigger_kind == CompletionTriggerKind::INVOKED {
            completions.extend(path_suffix_completions(db, &ctx))
        }
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

#[derive(PartialEq)]
pub struct CompletionItemHashable(CompletionItem);

impl Eq for CompletionItemHashable {}

impl Hash for CompletionItemHashable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        serde_json::to_string(&self.0).expect("serialization should not fail").hash(state);
    }
}
