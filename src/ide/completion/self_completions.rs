use cairo_lang_filesystem::ids::SmolStrId;
use cairo_lang_semantic::resolve::AsSegments;
use cairo_lang_syntax::node::TypedSyntaxNode;

use super::{expr::selector::expr_selector, path::path_prefix_completions};
use crate::{
    ide::completion::CompletionItemOrderable,
    lang::{analysis_context::AnalysisContext, db::AnalysisDatabase},
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
        result
    } else {
        Default::default()
    }
}
