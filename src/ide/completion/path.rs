use std::ops::Not;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{LookupItemId, ModuleFileId, NamedLanguageElementId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::items::visibility::peek_visible_in;
use cairo_lang_semantic::lookup_item::HasResolverData;
use cairo_lang_semantic::resolve::{
    AsSegments, ResolvedConcreteItem, ResolvedGenericItem, Resolver,
};
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_syntax::node::ast::{ExprPath, PathSegment};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::{LookupIntern, Upcast};
use lsp_types::{CompletionItem, CompletionItemKind};

use super::helpers::completion_kind::resolved_generic_item_completion_kind;
use crate::lang::db::AnalysisDatabase;
use crate::lang::syntax::SyntaxNodeExt;

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

pub fn colon_colon_completions(
    db: &AnalysisDatabase,
    module_file_id: ModuleFileId,
    lookup_items: Vec<LookupItemId>,
    segments: Vec<PathSegment>,
) -> Option<Vec<CompletionItem>> {
    // Get a resolver in the current context.
    let resolver_data = match lookup_items.into_iter().next() {
        Some(item) => {
            (*item.resolver_data(db).ok()?).clone_with_inference_id(db, InferenceId::NoContext)
        }
        None => Resolver::new(db, module_file_id, InferenceId::NoContext).data,
    };
    let mut resolver = Resolver::with_data(db, resolver_data);

    let mut diagnostics = SemanticDiagnostics::default();
    let item = resolver
        .resolve_concrete_path(&mut diagnostics, segments, NotFoundItemType::Identifier)
        .ok()?;

    let current_module_id = module_file_id.0;

    Some(match item {
        ResolvedConcreteItem::Module(module_id) => db
            .module_items(module_id)
            .ok()?
            .iter()
            .filter_map(|item| {
                let resolved_item = ResolvedGenericItem::from_module_item(db, *item).ok()?;
                let item_info = db.module_item_info_by_name(module_id, item.name(db)).ok()??;

                peek_visible_in(db, item_info.visibility, module_id, current_module_id).then(|| {
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
