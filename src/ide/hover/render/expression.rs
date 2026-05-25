use cairo_lang_defs::ids::{LookupItemId, ModuleItemId};
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::{ConcreteTypeId, TypeId, TypeLongId};
use cairo_lang_syntax::node::ast::{Expr, ExprInlineMacro};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use indoc::formatdoc;
use itertools::Itertools;

use crate::ide::format::types::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Finds the innermost expression containing the given node and renders its inferred type as a
/// hover popup. Returns the hover content string and the expression's syntax node (used for
/// determining the highlight range).
#[tracing::instrument(level = "trace", skip_all)]
pub fn type_info<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> Option<(String, SyntaxNode<'db>)> {
    let importables = db.visible_importables_from_module(db.find_module_containing_node(node)?)?;

    let (expr_node, type_id) = node
        .ancestors_with_self(db)
        // Walk up to and including any enclosing inline macro. For nodes already in an expansion
        // file there is no ExprInlineMacro ancestor; for nodes in the source file inside a macro
        // argument token tree this prevents surfacing types of unrelated outer expressions.
        .take_while_inclusive(|n| ExprInlineMacro::cast(db, *n).is_none())
        .filter_map(|n| Expr::cast(db, n))
        .find_map(|expr| {
            let node = expr.as_syntax_node();
            find_expr_type(db, node).map(|ty| (node, ty))
        })?;

    if matches!(type_id.long(db), TypeLongId::Missing(_)) {
        return None;
    }

    let ty = format_hover_type(db, type_id, &importables);
    let content = formatdoc!(
        "
        ```cairo
        {ty}
        ```
        "
    );

    Some((content, expr_node))
}

fn format_hover_type<'db>(
    db: &'db AnalysisDatabase,
    ty: TypeId<'db>,
    importables: &cairo_lang_utils::ordered_hash_map::OrderedHashMap<
        cairo_lang_defs::ids::ImportableId<'db>,
        String,
    >,
) -> String {
    if let TypeLongId::Concrete(ConcreteTypeId::Struct(concrete_struct_id)) = ty.long(db)
        && let Some(signature) = db.get_item_signature(DocumentableItemId::LookupItem(
            LookupItemId::ModuleItem(ModuleItemId::Struct(concrete_struct_id.struct_id(db))),
        ))
    {
        return signature;
    }

    format_type(db, ty, importables, None)
}

fn find_expr_type<'db>(db: &'db AnalysisDatabase, node: SyntaxNode<'db>) -> Option<TypeId<'db>> {
    let function_id = db
        .collect_lookup_items_with_parent_files(node)
        .and_then(|nodes| nodes.iter().find_map(|id| id.function_with_body()))?;
    let expr = Expr::from_syntax_node(db, node);
    db.lookup_expr_by_ptr(function_id, expr.stable_ptr(db))
        .ok()
        .map(|expr_id| db.expr_semantic(function_id, expr_id).ty())
}
