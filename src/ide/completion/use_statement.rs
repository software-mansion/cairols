use cairo_lang_defs::ids::ImportableId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_syntax::node::ast::{UsePathLeaf, UsePathSingle};
use cairo_lang_syntax::node::{
    Token,
    ast::{PathSegment, UsePath},
};
use lsp_types::CompletionItem;

use super::{helpers::completion_kind::importable_completion_kind, path::path_prefix_completions};
use crate::lang::db::AnalysisDatabase;
use crate::lang::{analysis_context::AnalysisContext, text_matching::text_matches};

pub fn use_statement(
    db: &AnalysisDatabase,
    use_path_single: UsePathSingle,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    get_use_path_segments(db, UsePath::Single(use_path_single))
        .ok()
        .and_then(|segments| path_prefix_completions(db, ctx, segments))
}

/// Invariant: `use_path_leaf` is the first and only one element of the use path.
/// Therefore, it is not a descendant of [`UsePathSingle`].
pub fn use_statement_first_segment(
    db: &AnalysisDatabase,
    use_path_leaf: UsePathLeaf,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    get_use_path_segments(db, UsePath::Leaf(use_path_leaf)).ok().and_then(|mut segments| {
        let typed = segments.pop()?;

        // Should be always true if invariant is not violated.
        if segments.is_empty() {
            if let PathSegment::Simple(typed) = typed {
                first_segment(db, &typed.ident(db).token(db).text(db), ctx)
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn first_segment(
    db: &AnalysisDatabase,
    typed: &str,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    let importables = db.visible_importables_from_module(ctx.module_file_id)?;

    Some(
        importables
            .iter()
            .filter_map(|(importable, path)| {
                match importable {
                    // Other items can not be non-last segment, ignore them as it makes no sense to import them.
                    ImportableId::Submodule(_) | ImportableId::Crate(_) | ImportableId::Enum(_)
                    // Take only if this item can be used wihtout prefix path
                        if path.split("::").count() == 1 && text_matches(path, typed) =>
                    {
                        Some(CompletionItem {
                            label: path.clone(),
                            kind: Some(importable_completion_kind(*importable)),
                            ..CompletionItem::default()
                        })
                    }
                    _ => None,
                }
            })
            .collect(),
    )
}
