use attribute::{attribute_completions, derive_completions};
use cairo_lang_syntax::node::ast::{
    self, Attribute, BinaryOperator, ExprBinary, ExprPath, ExprStructCtorCall, ItemModule,
    TerminalIdentifier, UsePathLeaf, UsePathSingle,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{Terminal, TypedSyntaxNode};
use colon_colon::{expr_path, use_statement};
use completions::{dot_completions, struct_constructor_completions};
use if_chain::if_chain;
use lsp_types::{CompletionParams, CompletionResponse, CompletionTriggerKind};
use mod_item::mod_completions;

use self::completions::generic_completions;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use crate::lang::syntax::SyntaxNodeExt;

mod attribute;
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

    // Check if cursor is on attribute name. `#[my_a<cursor>ttr(arg1, args2: 1234)]`
    if_chain!(
        if let Some(node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath);
        if let Some(attr) = node.parent_of_type::<Attribute>(db);
        if let Some(attr_completions) = attribute_completions(db, attr);

        then {
            completions.extend(attr_completions);
        }
    );

    // Check if cursor is on `#[derive(Arg1, Ar<cursor>)]` arguments list.
    if_chain!(
        if let Some(path_node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath);
        if let Some(node) = path_node.parent_of_kind(db, SyntaxKind::ArgClauseUnnamed);
        if let Some(attr) = node.ancestor_of_type::<Attribute>(db);
        if let Some(derive_completions) = derive_completions(db, &path_node.get_text(db), attr);

        then {
            completions.extend(derive_completions);
        }
    );

    // Check if cursor is on `#[derive(Arg1, <cursor>)]` arguments list.
    if_chain!(
        if node.ancestor_of_kind(db, SyntaxKind::Arg).is_none();
        if let Some(attr) = node.ancestor_of_type::<Attribute>(db);
        if let Some(derive_completions) = derive_completions(db, "", attr);

        then {
            completions.extend(derive_completions);
        }
    );

    if_chain!(
        if let Some(ident) = TerminalIdentifier::cast(db, node.clone());
        if let Some(module_item) = node.parent_of_type::<ItemModule>(db);
        // We are in nested mod, we should not show completions for file modules.
        if module_item.as_syntax_node().ancestor_of_kind(db, SyntaxKind::ItemModule).is_none();
        if let Some(mod_names_completions) = mod_completions(db, module_item, file_id, &ident.text(db));

        then {
            completions.extend(mod_names_completions);
        }
    );

    if_chain!(
        // if there is no name `mod <cursor>` we will be on `mod`.
        if node.kind(db) == SyntaxKind::TerminalModule;
        if let Some(module_item) = node.parent_of_type::<ItemModule>(db);
        // We are in nested mod, we should not show completions for file modules.
        if module_item.as_syntax_node().ancestor_of_kind(db, SyntaxKind::ItemModule).is_none();
        // use "" as typed text in this case.
        if let Some(mod_names_completions) = mod_completions(db, module_item, file_id, "");

        then {
            completions.extend(mod_names_completions);
        }
    );

    if completions.is_empty()
        && trigger_kind == CompletionTriggerKind::INVOKED
        && !was_single
        // Another quickfix, `generic_completions` must be split into smaller parts to fit into system.
        && node.ancestor_of_kind(db, SyntaxKind::ExprStructCtorCall).is_none()
    {
        let result = generic_completions(db, module_file_id, lookup_items);

        completions.extend(result);
    }

    Some(CompletionResponse::Array(completions))
}
