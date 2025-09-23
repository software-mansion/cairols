use cairo_lang_defs::ids::ImportableId;
use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::ast::{UsePathLeaf, UsePathSingle};
use cairo_lang_syntax::node::{
    Token,
    ast::{PathSegment, UsePath},
};
use lsp_types::CompletionItem;

use super::{helpers::completion_kind::importable_completion_kind, path::path_prefix_completions};
use crate::ide::completion::{CompletionItemOrderable, CompletionRelevance};
use crate::lang::db::AnalysisDatabase;
use crate::lang::{analysis_context::AnalysisContext, text_matching::text_matches};

pub fn use_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItemOrderable> {
    if let Some(single) = ctx.node.ancestor_of_type::<UsePathSingle>(db)
        && let Some(use_completions) = use_statement(db, single, ctx)
    {
        return use_completions;
    }

    // If we are on the first segment of use e.g. `use co<caret>`.
    if ctx.node.ancestor_of_type::<UsePathSingle>(db).is_none()
        && let Some(leaf) = ctx.node.ancestor_of_type::<UsePathLeaf>(db)
        && let Some(use_completions) = use_statement_first_segment(db, leaf, ctx)
    {
        return use_completions;
    }

    vec![]
}

pub fn use_statement<'db>(
    db: &'db AnalysisDatabase,
    use_path_single: UsePathSingle<'db>,
    ctx: &AnalysisContext<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    get_use_path_segments(db, UsePath::Single(use_path_single))
        .ok()
        .and_then(|segments| path_prefix_completions(db, ctx, segments.segments))
}

/// Invariant: `use_path_leaf` is the first and only one element of the use path.
/// Therefore, it is not a descendant of [`UsePathSingle`].
pub fn use_statement_first_segment<'db>(
    db: &'db AnalysisDatabase,
    use_path_leaf: UsePathLeaf<'db>,
    ctx: &AnalysisContext<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    get_use_path_segments(db, UsePath::Leaf(use_path_leaf)).ok().and_then(|mut segments| {
        let typed = segments.segments.pop()?;

        // Should be always true if invariant is not violated.
        if segments.segments.is_empty() {
            if let PathSegment::Simple(typed) = typed {
                first_segment(db, &typed.ident(db).token(db).text(db).to_string(db), ctx)
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn first_segment<'db>(
    db: &'db AnalysisDatabase,
    typed: &str,
    ctx: &AnalysisContext<'db>,
) -> Option<Vec<CompletionItemOrderable>> {
    let importables = db.visible_importables_from_module(ctx.module_file_id)?;

    Some(
        importables
            .iter()
            .filter_map(|(importable, path)| {
                let is_in_scope = path.split("::").count() == 1;
                match importable {
                    // Other items can not be non-last segment, ignore them as it makes no sense to import them.
                    ImportableId::Submodule(_) | ImportableId::Crate(_) | ImportableId::Enum(_)
                    // Take only if this item can be used wihtout prefix path
                        if is_in_scope && text_matches(path, typed) =>
                    {
                      Some(CompletionItemOrderable {
                          item: CompletionItem {
                              label: path.clone(),
                              kind: Some(importable_completion_kind(*importable)),
                              ..CompletionItem::default()
                          },
                          relevance: CompletionRelevance::High,
                      })
                    }
                    _ => None,
                }
            })
            .collect(),
    )
}
