use std::ops::Not;

use cairo_lang_defs::ids::{LookupItemId, ModuleFileId};
use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_semantic::resolve::AsSegments;
use cairo_lang_syntax::node::ast::{ExprPath, UsePath};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::Upcast;
use lsp_types::CompletionItem;

use super::completions::colon_colon_completions;
use crate::lang::db::AnalysisDatabase;
use crate::lang::syntax::SyntaxNodeExt;

pub fn use_statement(
    db: &AnalysisDatabase,
    use_path: UsePath,
    module_file_id: ModuleFileId,
    lookup_items: Vec<LookupItemId>,
) -> Option<Vec<CompletionItem>> {
    get_use_path_segments(db.upcast(), use_path.clone()).ok().and_then(|mut segments| {
        if let UsePath::Leaf(_) = use_path {
            segments.pop();
        }

        segments
            .is_empty()
            .not()
            .then(|| colon_colon_completions(db, module_file_id, lookup_items.clone(), segments))
            .flatten()
    })
}

pub fn expr_path(
    db: &AnalysisDatabase,
    node: &SyntaxNode,
    expr: ExprPath,
    module_file_id: ModuleFileId,
    lookup_items: Vec<LookupItemId>,
) -> Option<Vec<CompletionItem>> {
    let first_segment = expr.elements(db).into_iter().next()?;
    let first_segment = first_segment.as_syntax_node();

    node.is_descendant(&first_segment)
        .not()
        .then(|| {
            let mut segments = expr.to_segments(db);

            if expr.has_tail(db) {
                segments.pop();
            }

            colon_colon_completions(db, module_file_id, lookup_items.clone(), segments)
        })
        .flatten()
}
