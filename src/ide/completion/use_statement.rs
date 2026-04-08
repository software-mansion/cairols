use std::collections::HashSet;

use cairo_lang_defs::ids::ImportableId;
use cairo_lang_semantic::items::us::get_use_path_segments;
use cairo_lang_syntax::node::ast::{ItemUse, UsePathLeaf, UsePathMulti, UsePathSingle};
use cairo_lang_syntax::node::kind::SyntaxKind::{
    UsePathLeaf as UsePathLeafKind, UsePathMulti as UsePathMultiKind,
    UsePathSingle as UsePathSingleKind, UsePathStar,
};
use cairo_lang_syntax::node::{
    Token, TypedSyntaxNode,
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
        .ancestor_of_kinds(db, &[UsePathSingleKind, UsePathLeafKind, UsePathMultiKind, UsePathStar])
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
        .map(|items| {
            let excluded = already_imported_in_multi(db, ctx);
            items.into_iter().filter(|item| !excluded.contains(&item.item.label)).collect()
        })
}

/// Collects the names of items already listed in the enclosing [`UsePathMulti`],
/// excluding the leaf node currently being typed (so partial completions still work).
fn already_imported_in_multi<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> HashSet<String> {
    let Some(multi) = ctx.node.ancestor_of_type::<UsePathMulti>(db) else {
        return HashSet::new();
    };
    let current_leaf_ptr =
        ctx.node.ancestor_of_type::<UsePathLeaf>(db).map(|leaf| leaf.stable_ptr(db));
    multi
        .use_paths(db)
        .elements(db)
        .filter_map(|use_path| {
            let UsePath::Leaf(leaf) = use_path else { return None };
            if Some(leaf.stable_ptr(db)) == current_leaf_ptr {
                return None;
            }
            let PathSegment::Simple(simple) = leaf.ident(db) else { return None };
            Some(simple.ident(db).token(db).text(db).to_string(db))
        })
        .collect()
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
