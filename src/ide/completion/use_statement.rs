use std::ops::Not;

use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_syntax::node::ast::UsePath;
use cairo_lang_utils::Upcast;
use lsp_types::CompletionItem;

use super::path::path_prefix_completions;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;

pub fn use_statement(
    db: &AnalysisDatabase,
    use_path: UsePath,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    get_use_path_segments(db.upcast(), use_path.clone()).ok().and_then(|mut segments| {
        if let UsePath::Leaf(_) = use_path {
            segments.pop();
        }

        segments.is_empty().not().then(|| path_prefix_completions(db, ctx, segments)).flatten()
    })
}
