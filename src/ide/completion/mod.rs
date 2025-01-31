use cairo_lang_syntax::node::ast::{
    BinaryOperator, ExprBinary, ExprPath, ExprStructCtorCall, UsePathLeaf, UsePathSingle,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use colon_colon::{expr_path, use_statement};
use completions::{attribute_completions, struct_constructor_completions};
use if_chain::if_chain;
use lsp_types::{CompletionParams, CompletionResponse, CompletionTriggerKind};
use mod_item::mod_completions;

use self::completions::{dot_completions, generic_completions};
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use crate::lang::syntax::SyntaxNodeExt;

mod colon_colon;
mod completions;
mod mod_item;

/// Compute completion items at a given cursor position.
pub fn complete(params: CompletionParams, db: &AnalysisDatabase) -> Option<CompletionResponse> {
    let text_document_position = params.text_document_position;
    let file_id = db.file_for_url(&text_document_position.text_document.uri)?;
    let mut position = text_document_position.position;
    position.character = position.character.saturating_sub(1);

    let mut node = db.find_syntax_node_at_position(file_id, position.to_cairo())?;
    let lookup_items = db.collect_lookup_items_stack(&node)?;
    let module_file_id = db.find_module_file_containing_node(&node)?;

    // Skip trivia.
    while ast::Trivium::is_variant(node.kind(db))
        || node.kind(db) == SyntaxKind::Trivia
        || node.kind(db).is_token()
    {
        node = node.parent().unwrap_or(node);
    }

    let trigger_kind =
        params.context.map(|it| it.trigger_kind).unwrap_or(CompletionTriggerKind::INVOKED);

    let mut completions = vec![];

    if_chain!(
        if let Some(constructor) = node.ancestor_of_type::<ExprStructCtorCall>(db);
        if let Some(struct_completions) = struct_constructor_completions(db, lookup_items.clone(), constructor);

        then {
            completions.extend(struct_completions);
        }
    );

    if_chain!(
        if let Some(binary_expression) = node.ancestor_of_type::<ExprBinary>(db);
        if let BinaryOperator::Dot(_) = binary_expression.op(db);
        if node.is_descendant(&binary_expression.rhs(db).as_syntax_node());
        if let Some(dot_completions) = dot_completions(db, file_id, lookup_items.clone(), binary_expression);

        then {
            completions.extend(dot_completions);
        }
    );

    // Temp fix, this is due to fact that some completions dont have match yet.
    let mut was_single = false;

    if_chain!(
        if let Some(leaf) = node.ancestor_of_type::<UsePathLeaf>(db);
        if let Some(use_completions) = use_statement(db, ast::UsePath::Leaf(leaf), module_file_id, lookup_items.clone());

        then {
            completions.extend(use_completions);
        }
    );

    if_chain!(
        if let Some(single) = node.ancestor_of_type::<UsePathSingle>(db);
        if {
            was_single = true;
            true
        };
        if let Some(use_completions) = use_statement(db, ast::UsePath::Single(single), module_file_id, lookup_items.clone());

        then {
            completions.extend(use_completions);
        }
    );

    if_chain!(
        if let Some(expr) = node.ancestor_of_type::<ExprPath>(db);
        if let Some(expr_path_completions) = expr_path(db, &node, expr, module_file_id, lookup_items.clone());

        then {
            completions.extend(expr_path_completions);
        }
    );

    if completions.is_empty() && trigger_kind == CompletionTriggerKind::INVOKED && !was_single {
        let result = attribute_completions(db, node.clone())
            .or_else(|| mod_completions(db, node, file_id))
            .unwrap_or_else(|| generic_completions(db, module_file_id, lookup_items));

        completions.extend(result);
    }

    Some(CompletionResponse::Array(completions))
}
