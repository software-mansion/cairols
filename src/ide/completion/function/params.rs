use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;

pub fn params_completions(db: &AnalysisDatabase, ctx: &AnalysisContext<'_>) -> Vec<CompletionItem> {
    let params = if_chain!(
        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if let Ok(signature) = db.function_with_body_signature(function_id);

        then {
            signature.params
        } else {
            Default::default()
        }
    );

    params
        .into_iter()
        .map(|param| CompletionItem {
            label: param.name.clone().into(),
            kind: Some(CompletionItemKind::VARIABLE),
            ..CompletionItem::default()
        })
        .collect()
}
