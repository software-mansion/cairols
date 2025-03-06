use cairo_lang_defs::db::DefsGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::expr::selector::expr_selector;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::PathSegment;

pub fn macro_call_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Vec<CompletionItem> {
    if_chain!(
        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if db.function_body(function_id).is_ok();

        if let Some(path) = expr_selector(db, &ctx.node);
        // Currently inline macros can not be imported/exported
        if let [PathSegment::Simple(path_segment)] = path.elements(db).as_slice();

        then {
            let crate_id = ctx.module_id.owning_crate(db);

            let inline_plugins = db.crate_inline_macro_plugins(crate_id);

            let typed = path_segment.ident(db).token(db).text(db).to_string();

            inline_plugins
                .iter()
                .filter(|(name,_)| name.starts_with(&typed))
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
