use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_syntax::node::{
    Token,
    ast::{PathSegment, UsePath},
};
use lsp_types::CompletionItem;

use super::{helpers::completion_kind::importable_completion_kind, path::path_prefix_completions};
use crate::lang::db::AnalysisDatabase;
use crate::lang::{analysis_context::AnalysisContext, text_matching::text_matches};
use cairo_lang_defs::ids::{FileIndex, ImportableId, ModuleFileId};
use cairo_lang_semantic::db::SemanticGroup;

pub fn use_statement(
    db: &AnalysisDatabase,
    use_path: UsePath,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    get_use_path_segments(db, use_path.clone()).ok().and_then(|mut segments| {
        let mut typed = None;
        if let UsePath::Leaf(_) = use_path {
            typed = segments.pop();
        };

        if segments.is_empty() {
            let typed = typed?;

            if let PathSegment::Simple(typed) = typed {
                first_segment(db, &typed.ident(db).token(db).text(db), ctx)
            } else {
                None
            }
        } else {
            path_prefix_completions(db, ctx, segments)
        }
    })
}

fn first_segment(
    db: &AnalysisDatabase,
    typed: &str,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    let importables =
        db.visible_importables_from_module(ModuleFileId(ctx.module_id, FileIndex(0)))?;

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
