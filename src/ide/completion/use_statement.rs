use std::ops::Not;

use cairo_lang_defs::ids::{LookupItemId, ModuleFileId};
use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_syntax::node::ast::UsePath;
use cairo_lang_utils::Upcast;
use lsp_types::CompletionItem;

use super::path::colon_colon_completions;
use crate::lang::db::AnalysisDatabase;

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
