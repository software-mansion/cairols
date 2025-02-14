use std::ops::Not;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::NamedLanguageElementId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::items::visibility::peek_visible_in;
use cairo_lang_semantic::resolve::{AsSegments, ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{ExprPath, PathSegment};
use cairo_lang_utils::{LookupIntern, Upcast};
use lsp_types::{CompletionItem, CompletionItemKind};

use super::helpers::completion_kind::resolved_generic_item_completion_kind;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::syntax::SyntaxNodeExt;

pub fn expr_path(
    db: &AnalysisDatabase,
    expr: ExprPath,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<CompletionItem>> {
    let first_segment = expr.elements(db).into_iter().next()?;
    let first_segment = first_segment.as_syntax_node();

    ctx.node
        .is_descendant(&first_segment)
        .not()
        .then(|| {
            let mut segments = expr.to_segments(db);

            if expr.has_tail(db) {
                segments.pop();
            }

            colon_colon_completions(db, ctx, segments)
        })
        .flatten()
}

pub fn colon_colon_completions(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
    segments: Vec<PathSegment>,
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
                let item_info = db.module_item_info_by_name(module_id, item.name(db)).ok()??;

                peek_visible_in(db, item_info.visibility, module_id, ctx.module_id).then(|| {
                    CompletionItem {
                        label: item.name(db.upcast()).to_string(),
                        kind: Some(resolved_generic_item_completion_kind(resolved_item)),
                        ..CompletionItem::default()
                    }
                })
            })
            .collect(),
        ResolvedConcreteItem::Trait(item) => db
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
        ResolvedConcreteItem::Type(ty) => match ty.lookup_intern(db) {
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
