use cairo_lang_filesystem::ids::SmolStrId;
use cairo_lang_semantic::resolve::AsSegments;
use cairo_lang_syntax::node::TypedSyntaxNode;

use super::{
    expr::selector::expr_selector,
    path::{path_prefix_completions, resolve_typed_segments},
};
use crate::{
    ide::completion::CompletionItemOrderable,
    lang::{analysis_context::AnalysisContext, db::AnalysisDatabase, text_matching::text_matches},
};

pub fn self_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItemOrderable> {
    if let Some(expr) = expr_selector(db, &ctx.node)
        && let mut segments = expr.to_segments(db)
        && let _ = {
            // If there is tail (ie. some::path::) last segment will be of type missing, remove it.
            if expr.segments(db).has_tail(db) {
                segments.pop();
            }
        }
        && let Some(first_segment) = segments.first()
        && first_segment.as_syntax_node().get_text_without_trivia(db) == SmolStrId::from(db, "Self")
        && let Some(result) = path_prefix_completions(db, ctx, segments)
    {
        let typed = resolve_typed_segments(db, ctx)
            .map(|(_, last_typed_segment)| last_typed_segment.to_string(db))
            .unwrap_or_default();
        result.into_iter().filter(|item| text_matches(&item.item.label, &typed)).collect()
    } else {
        Default::default()
    }
}
