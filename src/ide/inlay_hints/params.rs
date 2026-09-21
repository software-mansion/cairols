use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::functions::FunctionsSemantic;
use cairo_lang_semantic::resolve::ResolvedConcreteItem;
use cairo_lang_syntax::node::ast::{self, ArgClause, BinaryOperator, ExprBinary, ExprFunctionCall};
use cairo_lang_syntax::node::helpers::PathSegmentEx;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_language_common::CommonGroup;
use lsp_types::{InlayHint, InlayHintKind, InlayHintLabel};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::lsp::ToLsp;
use crate::lang::proc_macros::db::get_og_node;

pub fn param_inlay_hints<'db>(
    db: &'db AnalysisDatabase,
    file: FileId<'db>,
    call_syntax: ExprFunctionCall<'db>,
) -> Vec<InlayHint> {
    let Some(signature) = resolve_call_signature(db, &call_syntax) else {
        return vec![];
    };

    let syntax_args: Vec<_> = call_syntax.arguments(db).arguments(db).elements(db).collect();

    let call_node = call_syntax.as_syntax_node();

    let is_method_call = call_node
        .parent(db)
        .and_then(|parent| ExprBinary::cast(db, parent))
        .is_some_and(|binary| matches!(binary.op(db), BinaryOperator::Dot(_)));

    let params: Vec<_> = signature
        .params
        .iter()
        .filter(|p| !(is_method_call && p.name.to_string(db) == "self"))
        .collect();

    syntax_args
        .iter()
        .zip(params.iter())
        .filter_map(|(arg, param)| hint_for_arg(db, file, arg, &param.name.to_string(db)))
        .collect()
}

/// Resolves a syntax-level function call to its semantic [`Signature`].
fn resolve_call_signature<'db>(
    db: &'db AnalysisDatabase,
    call_syntax: &ExprFunctionCall<'db>,
) -> Option<cairo_lang_semantic::Signature<'db>> {
    let call_node = call_syntax.as_syntax_node();

    db.get_node_resultants(call_node)?
        .iter()
        .find_map(|resultant| {
            let resultant_call = ExprFunctionCall::cast(db, *resultant)?;
            let lookup_item = db.find_lookup_item(resultant_call.as_syntax_node())?;

            let last_segment = resultant_call.path(db).segments(db).elements(db).last()?;
            let indentifier_ptr = last_segment.identifier_ast(db).stable_ptr(db);

            match db.lookup_resolved_concrete_item_by_ptr(lookup_item, indentifier_ptr)? {
                ResolvedConcreteItem::Function(function_id) => {
                    db.concrete_function_signature(function_id).ok()
                }
                _ => None,
            }
        })
        .cloned()
}

/// Produces a parameter-name hint for a single argument, or [`None`] if skipped.
/// Skips named args and args with a name matching the param name (`x: x`).
fn hint_for_arg<'db>(
    db: &'db AnalysisDatabase,
    file: FileId<'db>,
    arg: &ast::Arg<'db>,
    param_name: &str,
) -> Option<InlayHint> {
    let ArgClause::Unnamed(unnamed) = arg.arg_clause(db) else {
        return None;
    };

    if should_skip_hint(db, &unnamed.value(db), param_name) {
        return None;
    }

    let og_node = get_og_node(db, arg.as_syntax_node())?;
    make_hint(db, file, og_node, param_name)
}

/// Skips hint if the arg name matches the param name (`x: x`).
fn should_skip_hint(db: &AnalysisDatabase, arg_expr: &ast::Expr, param_name: &str) -> bool {
    if let ast::Expr::Path(path) = arg_expr {
        let text = path.as_syntax_node().get_text_without_trivia(db).to_string(db);
        if text == param_name {
            return true;
        }
    }
    false
}

fn make_hint<'db>(
    db: &'db AnalysisDatabase,
    file: FileId<'db>,
    node: SyntaxNode<'db>,
    param_name: &str,
) -> Option<InlayHint> {
    Some(InlayHint {
        position: node.span_without_trivia(db).position_in_file(db, file)?.start.to_lsp(),
        label: InlayHintLabel::String(format!("{param_name}: ")),
        kind: Some(InlayHintKind::PARAMETER),
        text_edits: None,
        tooltip: None,
        padding_left: None,
        padding_right: None,
        data: None,
    })
}
