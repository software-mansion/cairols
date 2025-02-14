use cairo_lang_defs::ids::LanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::items::visibility::peek_visible_in;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;

/// Discovers struct members missing in the constructor call and returns completions containing
/// their names with type hints.
pub fn struct_constructor_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    constructor: ast::ExprStructCtorCall,
) -> Option<Vec<CompletionItem>> {
    let module_id = ctx.module_id;
    let lookup_item_id = ctx.lookup_item_id?;
    let function_id = lookup_item_id.function_with_body()?;

    let already_present_members = constructor
        .arguments(db)
        .arguments(db)
        .elements(db)
        .into_iter()
        .filter_map(|member| match member {
            ast::StructArg::StructArgSingle(struct_arg_single) => {
                Some(struct_arg_single.identifier(db).token(db).as_syntax_node().get_text(db))
            }
            // although tail covers all remaining unspecified members, we still want to show them in
            // completion.
            ast::StructArg::StructArgTail(_) => None,
        })
        .collect::<Vec<_>>();

    let constructor_expr_id =
        db.lookup_expr_by_ptr(function_id, constructor.stable_ptr().into()).ok()?;

    let semantic_expr = db.expr_semantic(function_id, constructor_expr_id);

    let cairo_lang_semantic::Expr::StructCtor(constructor_semantic_expr) = semantic_expr else {
        return None;
    };

    let struct_parent_module_id =
        constructor_semantic_expr.concrete_struct_id.struct_id(db).parent_module(db);

    let struct_members =
        db.concrete_struct_members(constructor_semantic_expr.concrete_struct_id).ok()?;

    let completions = struct_members
        .iter()
        .filter_map(|(name, data)| {
            let name = name.to_string();

            let visible = peek_visible_in(db, data.visibility, struct_parent_module_id, module_id);

            if !visible || already_present_members.contains(&name) {
                None
            } else {
                Some(CompletionItem {
                    label: name,
                    detail: Some(data.ty.format(db)),
                    kind: Some(CompletionItemKind::VALUE),
                    ..Default::default()
                })
            }
        })
        .collect::<Vec<_>>();

    if completions.is_empty() { None } else { Some(completions) }
}
