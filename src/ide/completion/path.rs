use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::NamedLanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{ExprPath, PathSegment};
use cairo_lang_syntax::node::kind::SyntaxKind;
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionItemKind};

use super::helpers::completion_kind::{
    importable_completion_kind, resolved_generic_item_completion_kind,
};
use crate::ide::completion::CompletionItemHashable;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::importer::new_import_edit;
use crate::lang::text_matching::text_matches;
use crate::lang::visibility::peek_visible_in_with_edition;

/// Treats provided path as suffix, proposing elements that can prefix this path.
pub fn path_suffix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Vec<CompletionItem> {
    let (importables, typed_text) = if ctx.node.ancestor_of_kind(db, SyntaxKind::Attribute).is_none()
        // Enum patterns are handled in a separate function.
        && ctx.node.ancestor_of_kind(db, SyntaxKind::PatternEnum).is_none()
        && let Some(importables) = db.visible_importables_from_module(ctx.module_file_id)
        && let Some(typed_text_segments) = ctx
            .node
            .ancestor_of_type::<ExprPath>(db)
            .map(|path| path.segments(db).elements(db).collect_vec())
        && !typed_text_segments.is_empty()
    {
        (importables, typed_text_segments)
    } else {
        return Default::default();
    };

    let mut typed_text = typed_text
        .into_iter()
        .map(|segment| segment.as_syntax_node())
        // Allow proposing items in the middle of the existing path by filtering out the nodes which lie after the cursor:
        .filter(|segment_node| segment_node.offset(db) <= ctx.node.offset(db))
        .map(|segment_node| segment_node.get_text_without_trivia(db))
        .collect::<Vec<_>>();

    // After `::`, we want to propose all importables available at the preceding path.
    let last_typed_segment = if ctx.node.kind(db) == SyntaxKind::TerminalColonColon {
        ""
    } else {
        // Otherwise, the last segment is a partial identifier we want to complete using fuzzy search.
        typed_text.pop().expect("typed path should not be empty")
    };

    importables
        .iter()
        .filter_map(|(importable, path_str)| {
            let mut path_segments: Vec<_> = path_str.split("::").collect();

            let is_not_in_scope = path_segments.len() != 1;

            let last_segment = path_segments.pop().expect("path to import should not be empty");

            let mut last_poped = None;

            let previous_segment_matches = typed_text.iter().rev().all(|typed_segment| {
                last_poped = path_segments.pop();

                Some(*typed_segment) == last_poped
            });

            // Import path and typed path must have single overlapping element.
            // use foo::bar;
            //          bar::baz(12345);
            // If path was *not* empty we should *not* add use statement at all.
            if !path_segments.is_empty() {
                path_segments.push(last_poped.unwrap_or(last_segment));
            }

            if !previous_segment_matches || !text_matches(last_segment, last_typed_segment) {
                return None;
            }

            let additional_text_edits = if is_not_in_scope
                && !path_segments.is_empty()
                && let Some(import_edit) = new_import_edit(db, ctx, path_segments.join("::"))
            {
                Some(vec![import_edit])
            } else {
                None
            };

            Some(CompletionItem {
                label: last_segment.to_string(),
                kind: Some(importable_completion_kind(*importable)),
                additional_text_edits,
                ..CompletionItem::default()
            })
        })
        .unique_by(|completion| CompletionItemHashable(completion.clone()))
        .collect()
}

/// Treats provided path as prefix, proposing elements that should go next.
pub fn path_prefix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    segments: Vec<PathSegment<'db>>,
) -> Option<Vec<CompletionItem>> {
    let mut resolver = ctx.resolver(db);

    let mut diagnostics = SemanticDiagnostics::default();
    let item = resolver
        .resolve_concrete_path(&mut diagnostics, segments, NotFoundItemType::Identifier)
        .ok()?;

    Some(match item {
        ResolvedConcreteItem::Module(module_id) => db
            .module_items(module_id)
            .ok()?
            .iter()
            .filter_map(|item| {
                let resolved_item = ResolvedGenericItem::from_module_item(db, *item).ok()?;
                let item_info =
                    db.module_item_info_by_name(module_id, item.name(db).into()).ok()??;

                peek_visible_in_with_edition(
                    db,
                    item_info.visibility,
                    module_id,
                    ctx.module_file_id,
                )
                .then(|| CompletionItem {
                    label: item.name(db).to_string(),
                    kind: Some(resolved_generic_item_completion_kind(resolved_item)),
                    ..CompletionItem::default()
                })
            })
            .collect(),
        ResolvedConcreteItem::Trait(item) | ResolvedConcreteItem::SelfTrait(item) => db
            .trait_functions(item.trait_id(db))
            .unwrap_or_default()
            .iter()
            .map(|(name, _)| CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                ..CompletionItem::default()
            })
            .collect(),
        ResolvedConcreteItem::Impl(item) => item
            .concrete_trait(db)
            .map(|trait_id| {
                db.trait_functions(trait_id.trait_id(db))
                    .unwrap_or_default()
                    .iter()
                    .map(|(name, _)| CompletionItem {
                        label: name.to_string(),
                        kind: Some(CompletionItemKind::FUNCTION),
                        ..CompletionItem::default()
                    })
                    .collect()
            })
            .unwrap_or_default(),
        ResolvedConcreteItem::Type(ty) => match ty.long(db) {
            TypeLongId::Concrete(ConcreteTypeId::Enum(enum_id)) => db
                .enum_variants(enum_id.enum_id(db))
                .unwrap_or_default()
                .iter()
                .map(|(name, _)| CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::ENUM_MEMBER),
                    ..CompletionItem::default()
                })
                .collect(),
            _ => vec![],
        },
        _ => vec![],
    })
}
