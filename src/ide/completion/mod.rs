use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_semantic::resolve::AsSegments;
use cairo_lang_syntax::node::ast::{BinaryOperator, ExprBinary, ExprStructCtorCall, PathSegment};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::Upcast;
use completions::{attribute_completions, struct_constructor_completions};
use if_chain::if_chain;
use lsp_types::{CompletionParams, CompletionResponse, CompletionTriggerKind};
use mod_item::mod_completions;
use tracing::debug;

use self::completions::{colon_colon_completions, dot_completions, generic_completions};
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use crate::lang::syntax::SyntaxNodeExt;

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

    let old_way_completions = match completion_kind(db, node.clone()) {
        CompletionKind::ColonColon(segments) if !segments.is_empty() => {
            colon_colon_completions(db, module_file_id, lookup_items, segments)
        }
        _ if trigger_kind == CompletionTriggerKind::INVOKED => {
            let result = attribute_completions(db, node.clone())
                .or_else(|| mod_completions(db, node, file_id))
                .unwrap_or_else(|| generic_completions(db, module_file_id, lookup_items));

            Some(result)
        }
        _ => None,
    }?;

    if completions.is_empty() {
        completions.extend(old_way_completions);
    }

    Some(CompletionResponse::Array(completions))
}

enum CompletionKind {
    ColonColon(Vec<PathSegment>),
}

fn completion_kind(db: &AnalysisDatabase, node: SyntaxNode) -> CompletionKind {
    debug!("node.kind: {:#?}", node.kind(db));
    match node.kind(db) {
        SyntaxKind::TerminalColonColon => {
            let parent = node.parent().unwrap();
            debug!("parent.kind: {:#?}", parent.kind(db));
            if parent.kind(db) == SyntaxKind::ExprPath {
                return completion_kind_from_path_node(db, parent);
            }
            let grandparent = parent.parent().unwrap();
            debug!("grandparent.kind: {:#?}", grandparent.kind(db));
            if grandparent.kind(db) == SyntaxKind::ExprPath {
                return completion_kind_from_path_node(db, grandparent);
            }
            let (use_ast, should_pop) = if parent.kind(db) == SyntaxKind::UsePathLeaf {
                (ast::UsePath::Leaf(ast::UsePathLeaf::from_syntax_node(db, parent)), true)
            } else if grandparent.kind(db) == SyntaxKind::UsePathLeaf {
                (ast::UsePath::Leaf(ast::UsePathLeaf::from_syntax_node(db, grandparent)), true)
            } else if parent.kind(db) == SyntaxKind::UsePathSingle {
                (ast::UsePath::Single(ast::UsePathSingle::from_syntax_node(db, parent)), false)
            } else if grandparent.kind(db) == SyntaxKind::UsePathSingle {
                (ast::UsePath::Single(ast::UsePathSingle::from_syntax_node(db, grandparent)), false)
            } else {
                debug!("Generic");
                return CompletionKind::ColonColon(vec![]);
            };
            let Ok(mut segments) = get_use_path_segments(db.upcast(), use_ast) else {
                debug!("Generic");
                return CompletionKind::ColonColon(vec![]);
            };
            if should_pop {
                segments.pop();
            }
            debug!("ColonColon");
            return CompletionKind::ColonColon(segments);
        }
        SyntaxKind::TerminalIdentifier => {
            let parent = node.parent().unwrap();
            debug!("parent.kind: {:#?}", parent.kind(db));
            let grandparent = parent.parent().unwrap();
            debug!("grandparent.kind: {:#?}", grandparent.kind(db));
            if grandparent.kind(db) == SyntaxKind::ExprPath
                && db.get_children(grandparent.clone())[0].stable_ptr() != parent.stable_ptr()
            {
                // Not the first segment.
                debug!("Not first segment");
                return completion_kind_from_path_node(db, grandparent);
            }
            if grandparent.kind(db) == SyntaxKind::UsePathLeaf {
                let use_ast = ast::UsePathLeaf::from_syntax_node(db, grandparent);
                let Ok(mut segments) =
                    get_use_path_segments(db.upcast(), ast::UsePath::Leaf(use_ast))
                else {
                    debug!("Generic");
                    return CompletionKind::ColonColon(vec![]);
                };
                segments.pop();
                debug!("ColonColon");
                return CompletionKind::ColonColon(segments);
            }
        }
        _ => (),
    }
    debug!("Generic");
    CompletionKind::ColonColon(vec![])
}

fn completion_kind_from_path_node(db: &AnalysisDatabase, parent: SyntaxNode) -> CompletionKind {
    debug!("completion_kind_from_path_node: {}", parent.clone().get_text_without_trivia(db));
    let expr = ast::ExprPath::from_syntax_node(db, parent);
    debug!("has_tail: {}", expr.has_tail(db));
    let mut segments = expr.to_segments(db);
    if expr.has_tail(db) {
        segments.pop();
    }
    CompletionKind::ColonColon(segments)
}
