use cairo_lang_defs::ids::LanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::items::structure::StructSemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::ast::ExprStructCtorCall;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use cairo_lang_utils::Upcast;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::ide::format::types::format_type;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::visibility::peek_visible_in_with_edition;

/// Discovers struct members missing in the constructor call and returns completions containing
/// their names with type hints.
pub fn struct_constructor_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItemOrderable> {
    struct_constructor_completions_ex(db, ctx).unwrap_or_default()
}

fn struct_constructor_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    let constructor = ctx.node.ancestor_of_type::<ExprStructCtorCall>(db)?;
    let module_id = ctx.module_file_id;
    let lookup_item_id = ctx.lookup_item_id?;
    let function_id = lookup_item_id.function_with_body()?;
    let importables = db.visible_importables_from_module(ctx.module_file_id)?;

    let already_present_members = constructor
        .arguments(db)
        .arguments(db)
        .elements(db)
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
        db.lookup_expr_by_ptr(function_id, constructor.stable_ptr(db).into()).ok()?;

    let semantic_db: &dyn SemanticGroup = db.upcast();
    let semantic_expr = semantic_db.expr_semantic(function_id, constructor_expr_id);

    let cairo_lang_semantic::Expr::StructCtor(constructor_semantic_expr) = semantic_expr else {
        return None;
    };

    let struct_parent_module_id =
        constructor_semantic_expr.concrete_struct_id.struct_id(db).parent_module(db);

    let struct_members =
        db.concrete_struct_members(constructor_semantic_expr.concrete_struct_id).ok()?;

    // If any field is not visible this struct is unconstructable anyway, don't propose completions.
    if !struct_members.values().all(|data| {
        peek_visible_in_with_edition(db, data.visibility, struct_parent_module_id, module_id)
    }) {
        return None;
    }

    let completions = struct_members
        .iter()
        .filter_map(|(name, data)| {
            if already_present_members.contains(&&**name) {
                None
            } else {
                Some(CompletionItemOrderable {
                    item: CompletionItem {
                        label: name.to_string(),
                        detail: Some(format_type(db, data.ty, &importables)),
                        kind: Some(CompletionItemKind::VALUE),
                        ..Default::default()
                    },
                    relevance: CompletionRelevance::High,
                })
            }
        })
        .collect::<Vec<_>>();

    if completions.is_empty() { None } else { Some(completions) }
}
