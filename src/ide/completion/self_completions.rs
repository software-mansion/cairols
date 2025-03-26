use super::{expr::selector::expr_selector, path::path_prefix_completions};
use crate::lang::{analysis_context::AnalysisContext, db::AnalysisDatabase};
use cairo_lang_semantic::resolve::AsSegments;
use cairo_lang_syntax::node::TypedSyntaxNode;
use lsp_types::CompletionItem;

use if_chain::if_chain;

pub fn self_completions(db: &AnalysisDatabase, ctx: &AnalysisContext<'_>) -> Vec<CompletionItem> {
    if_chain!(
        if let Some(expr) = expr_selector(db, &ctx.node);
        let mut segments = expr.to_segments(db);
        let _ = {
            // If there is tail (ie. some::path::) last segment will be of type missing, remove it.
            if expr.has_tail(db) {
                segments.pop();
            }
        };
        if let Some(first_segment) = segments.first();
        if first_segment.as_syntax_node().get_text_without_trivia(db) == "Self";
        if let Some(result) = path_prefix_completions(db, ctx, segments);

        then {
            result
        } else {
            Default::default()
        }
    )
}
