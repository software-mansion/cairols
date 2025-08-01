use std::hash::Hash;

use attribute::attribute_completions;
use attribute::derive::derive_completions;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::{
    self, Attribute, ExprStructCtorCall, ItemModule, TerminalIdentifier, UsePathLeaf, UsePathSingle,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use expr::macro_call::macro_call_completions;
use function::params::params_completions;
use function::variables::variables_completions;
use helpers::binary_expr::dot_rhs::dot_expr_rhs;
use lsp_types::{CompletionItem, CompletionParams, CompletionResponse, CompletionTriggerKind};
use mod_item::mod_completions;
use path::path_suffix_completions;
use pattern::{enum_pattern_completions, struct_pattern_completions};
use self_completions::self_completions;
use struct_constructor::struct_constructor_completions;

use self::dot_completions::dot_completions;
use crate::ide::completion::use_statement::{use_statement, use_statement_first_segment};
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
    position.character = position.character.saturating_sub(1);

    let mut node = db.find_syntax_node_at_position(file_id, position.to_cairo())?;

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
        .filter_map(|resultant| complete_ex(resultant, trigger_kind, db))
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
    db: &'db AnalysisDatabase,
) -> Option<Vec<CompletionItem>> {
    let ctx = AnalysisContext::from_node(db, node)?;
    let crate_id = ctx.module_file_id.0.owning_crate(db);
    let file_id = node.stable_ptr(db).file_id(db);

    let mut completions = vec![];

    let dot_binary_expression = dot_expr_rhs(db, &node);
    let is_dot_expression = dot_binary_expression.is_some();

    if let Some(binary_expression) = dot_binary_expression
        && let Some(dot_completions) = dot_completions(db, &ctx, binary_expression)
    {
        completions.extend(dot_completions);
    }

    if let Some(constructor) = node.ancestor_of_type::<ExprStructCtorCall>(db)
        && let Some(struct_completions) = struct_constructor_completions(db, &ctx, constructor)
    {
        completions.extend(struct_completions);
    }

    if let Some(single) = node.ancestor_of_type::<UsePathSingle>(db)
        && let Some(use_completions) = use_statement(db, single, &ctx)
    {
        completions.extend(use_completions);
    }

    // If we are on the first segment of use e.g. `use co<caret>`.

    if node.ancestor_of_type::<UsePathSingle>(db).is_none()
        && let Some(leaf) = node.ancestor_of_type::<UsePathLeaf>(db)
        && let Some(use_completions) = use_statement_first_segment(db, leaf, &ctx)
    {
        completions.extend(use_completions);
    }

    completions.extend(self_completions(db, &ctx));

    // Check if cursor is on attribute name. `#[my_a<cursor>ttr(arg1, args2: 1234)]`

    if let Some(node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(attr) = node.parent_of_type::<Attribute>(db)
        && let Some(attr_completions) = attribute_completions(db, attr, crate_id)
    {
        completions.extend(attr_completions);
    }

    // Check if cursor is on `#[derive(Arg1, Ar<cursor>)]` arguments list.

    if let Some(path_node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(node) = path_node.parent_of_kind(db, SyntaxKind::ArgClauseUnnamed)
        && let Some(attr) = node.ancestor_of_type::<Attribute>(db)
        && let Some(derive_completions) =
            derive_completions(db, path_node.get_text(db), attr, crate_id)
    {
        completions.extend(derive_completions);
    }

    // Check if cursor is on `#[derive(Arg1, <cursor>)]` arguments list.

    if node.ancestor_of_kind(db, SyntaxKind::Arg).is_none()
        && let Some(attr) = node.ancestor_of_type::<Attribute>(db)
        && let Some(derive_completions) = derive_completions(db, "", attr, crate_id)
    {
        completions.extend(derive_completions);
    }

    if let Some(ident) = TerminalIdentifier::cast(db, node)
        && let Some(module_item) = node.parent_of_type::<ItemModule>(db)
        // We are in nested mod, we should not show completions for file modules.
        && module_item.as_syntax_node().ancestor_of_kind(db, SyntaxKind::ItemModule).is_none()
        && let Some(mod_names_completions) =
            mod_completions(db, module_item, file_id, ident.text(db))
    {
        completions.extend(mod_names_completions);
    }

    // if there is no name `mod <cursor>` we will be on `mod`.
    if node.kind(db) == SyntaxKind::TerminalModule
        && let Some(module_item) = node.parent_of_type::<ItemModule>(db)
        // We are in nested mod, we should not show completions for file modules.
        && module_item.as_syntax_node().ancestor_of_kind(db, SyntaxKind::ItemModule).is_none()
        // use "" as typed text in this case.
        && let Some(mod_names_completions) = mod_completions(db, module_item, file_id, "")
    {
        completions.extend(mod_names_completions);
    }

    completions.extend(params_completions(db, &ctx));
    completions.extend(variables_completions(db, &ctx));
    completions.extend(struct_pattern_completions(db, &ctx));
    completions.extend(enum_pattern_completions(db, &ctx));

    if !is_dot_expression {
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

        if let Some(grand_child) = child
            .get_children(db)
            .iter()
            .find(|grand_child| grand_child.kind(db) != SyntaxKind::Trivia)
        {
            if grand_child.kind(db) == SyntaxKind::TokenMissing {
                continue;
            }
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
