use cairo_lang_defs::db::DefsGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::ast::UsePathSingle;
use cairo_lang_syntax::node::kind::SyntaxKind;

pub fn macro_call_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    if_chain!(
        // TODO remove this check when we have expression completions.
        if ctx.node.ancestor_of_type::<UsePathSingle>(db).is_none()
        && ctx.node.ancestor_of_kind(db, SyntaxKind::ExprStructCtorCall).is_none()
        // Currently inline macros can not be imported/exported
        && ctx.node.ancestor_of_kind(db, SyntaxKind::ExprPath).is_none();

        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if db.function_body(function_id).is_ok();

        then {
            let crate_id = ctx.module_id.owning_crate(db);

            let inline_plugins = db.crate_inline_macro_plugins(crate_id);

            inline_plugins
                .iter()
                .map(|(plugin_name, _)| CompletionItem {
                    label: format!("{}!", plugin_name),
                    kind: Some(CompletionItemKind::FUNCTION),
                    ..CompletionItem::default()
                })
                .collect()
        } else {
            Default::default()
        }
    )
}
