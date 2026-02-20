use cairo_lang_defs::ids::ImportableId;
use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_syntax::node::ast::{ItemUse, UsePathLeaf, UsePathSingle};
use cairo_lang_syntax::node::kind::SyntaxKind::{
    UsePathLeaf as UsePathLeafKind, UsePathMulti, UsePathSingle as UsePathSingleKind, UsePathStar,
};
use cairo_lang_syntax::node::{
    Token,
    ast::{PathSegment, UsePath},
};

use super::{helpers::item::first_segment_completion_candidates, path::path_prefix_completions};
use crate::ide::completion::CompletionItemOrderable;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;

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

    // If nothing was typed after use i.e. `use <caret>`.
    if ctx
        .node
        .ancestor_of_kinds(db, &[UsePathSingleKind, UsePathLeafKind, UsePathMulti, UsePathStar])
        .is_none()
        && ctx.node.ancestor_of_type::<ItemUse>(db).is_some()
        && let Some(use_completions) = first_segment(db, "", ctx)
    {
        return use_completions;
    }

    vec![]
}

fn use_statement<'db>(
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
fn use_statement_first_segment<'db>(
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
    Some(
        first_segment_completion_candidates(db, ctx, typed)
            .into_iter()
            .filter_map(|candidate| {
                match candidate.completion.importable_id {
                    // Other items can not be non-last segment, ignore them as it makes no sense to import them.
                    ImportableId::Submodule(_) | ImportableId::Crate(_) | ImportableId::Enum(_)
                    // Take only if this item can be used without prefix path.
                        =>
                    {
                        Some(candidate.into_path_completion())
                    }
                    _ => None,
                }
            })
            .collect(),
    )
}
