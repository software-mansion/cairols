use cairo_lang_semantic::items::function_with_body::FunctionWithBodySemantic;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::Token;
use cairo_lang_syntax::node::ast::PathSegment;
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionItemKind};

use crate::ide::completion::expr::selector::expr_selector;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::text_matching::text_matches;

pub fn params_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    let (params, typed_text) = if let Some(path) = expr_selector(db, &ctx.node)
        && dot_expr_rhs(db, &ctx.node, was_node_corrected).is_none()
        && let [PathSegment::Simple(segment)] =
            path.segments(db).elements(db).take(2).collect_vec().as_slice()
        && let Some(lookup_item_id) = ctx.lookup_item_id
        && let Some(function_id) = lookup_item_id.function_with_body()
        && let Ok(signature) = db.function_with_body_signature(function_id)
    {
        (signature.params, segment.ident(db).token(db).text(db))
    } else {
        Default::default()
    };

    params
        .into_iter()
        .filter(|param| text_matches(&*param.name, typed_text))
        .map(|param| CompletionItemOrderable {
            item: CompletionItem {
                label: param.name.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                ..CompletionItem::default()
            },
            relevance: Some(CompletionRelevance::Highest),
        })
        .collect()
}
