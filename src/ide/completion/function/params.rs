use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use if_chain::if_chain;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::PathSegment;

pub fn params_completions(db: &AnalysisDatabase, ctx: &AnalysisContext<'_>) -> Vec<CompletionItem> {
    let (params, typed_text) = if_chain!(
        if let Some(path) = expr_selector(db, &ctx.node);
        if dot_expr_rhs(db, &ctx.node).is_none();
        if let [PathSegment::Simple(segment)] = path.elements(db).as_slice();

        if let Some(lookup_item_id) = ctx.lookup_item_id;
        if let Some(function_id) = lookup_item_id.function_with_body();
        if let Ok(signature) = db.function_with_body_signature(function_id);

        then {
            (signature.params, segment.ident(db).token(db).text(db))
        } else {
            Default::default()
        }
    );

    params
        .into_iter()
        .filter(|param| text_matches(&param.name, &typed_text))
        .map(|param| CompletionItem {
            label: param.name.clone().into(),
            kind: Some(CompletionItemKind::VARIABLE),
            ..CompletionItem::default()
        })
        .collect()
}
