use std::sync::Arc;

use cairo_lang_defs::ids::{GenericTypeId, ModuleId, ModuleItemId};
use cairo_lang_defs::ids::{ImportableId, LanguageElementId, NamedLanguageElementId};
use cairo_lang_filesystem::ids::{CrateLongId, SmolStrId};
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::items::constant::ConstantSemantic;
use cairo_lang_semantic::items::enm::EnumSemantic;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::items::module::ModuleSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{ConcreteTypeId, TypeLongId};
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{ExprPath, PathSegment};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_utils::OptionFrom;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::Itertools;
use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails};

use super::helpers::completion_kind::resolved_generic_item_completion_kind;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::helpers::formatting::{
    format_enum_variant, generate_abbreviated_signature,
};
use crate::ide::completion::helpers::item::{
    CompletionItemOrderable, ImportableCompletionItem, ImportableCompletionItemHashable,
    get_item_relevance,
};
use crate::ide::format::types::format_type;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::visibility::peek_visible_in_with_edition;

/// Treats provided path as suffix, proposing elements that can prefix this path.
pub fn path_suffix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &'db AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<ImportableCompletionItem<'db>> {
    let importables = if let Some(importables) =
        get_importables_for_path_suffix_completions(db, ctx, was_node_corrected)
    {
        importables
    } else {
        return Default::default();
    };

    let (typed_text, last_typed_segment) = match get_typed_text_and_last_segment(db, ctx) {
        (Some(typed_text), Some(last_typed_segment)) => (typed_text, last_typed_segment),
        _ => return Default::default(),
    };

    let current_crate = ctx.module_id.owning_crate(db);

    importables
        .iter()
        .filter_map(|(importable, path_str)| {
            ImportableCompletionItem::get_completion_item_for_importable(
                db,
                ctx,
                importable,
                current_crate,
                path_str,
                typed_text.clone(),
                last_typed_segment,
            )
        })
        .unique_by(|completion| ImportableCompletionItemHashable(completion.clone()))
        .collect()
}

/// Treats provided path as prefix, proposing elements that should go next.
pub fn path_prefix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    segments: Vec<PathSegment<'db>>,
) -> Option<Vec<CompletionItemOrderable>> {
    let is_current_scope = segments.len() == 1;
    let mut resolver = ctx.resolver(db);

    let mut diagnostics = SemanticDiagnostics::new(ctx.module_id);
    let item = resolver
        .resolve_concrete_path(&mut diagnostics, segments, NotFoundItemType::Identifier)
        .ok()?;

    let current_crate = ctx.module_id.owning_crate(db);

    Some(match item {
        ResolvedConcreteItem::Module(module_id) => module_id
            .module_data(db)
            .ok()?
            .items(db)
            .iter()
            .filter_map(|item| {
                let resolved_item = ResolvedGenericItem::from_module_item(db, *item).ok()?;
                let item_info = db.module_item_info_by_name(module_id, item.name(db)).ok()??;
                let crate_id = module_id.owning_crate(db);

                peek_visible_in_with_edition(db, item_info.visibility, module_id, ctx.module_id)
                    .then(|| CompletionItemOrderable {
                        item: CompletionItem {
                            label: item.name(db).to_string(db),
                            label_details: Some(CompletionItemLabelDetails {
                                description: module_item_completion_detail(
                                    db,
                                    *item,
                                    ctx.module_id,
                                ),
                                detail: None,
                            }),
                            kind: Some(resolved_generic_item_completion_kind(resolved_item)),
                            ..CompletionItem::default()
                        },
                        relevance: get_item_relevance(
                            is_current_scope,
                            crate_id == current_crate,
                            *crate_id.long(db) == CrateLongId::core(db),
                        ),
                    })
            })
            .collect(),
        ResolvedConcreteItem::Trait(item) | ResolvedConcreteItem::SelfTrait(item) => db
            .trait_functions(item.trait_id(db))
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|(name, trait_function_id)| {
                let signature = db
                    .trait_function_signature(*trait_function_id)
                    .map(|sig| {
                        generate_abbreviated_signature(
                            db,
                            sig,
                            Some(trait_function_id.trait_id(db)),
                        )
                    })
                    .ok();

                let crate_id = item.trait_id(db).parent_module(db).owning_crate(db);
                CompletionItemOrderable {
                    item: CompletionItem {
                        label: name.to_string(db),
                        label_details: Some(CompletionItemLabelDetails {
                            detail: None,
                            description: signature,
                        }),
                        kind: Some(CompletionItemKind::FUNCTION),
                        ..CompletionItem::default()
                    },
                    relevance: get_item_relevance(
                        is_current_scope,
                        crate_id == current_crate,
                        *crate_id.long(db) == CrateLongId::core(db),
                    ),
                }
            })
            .collect(),
        ResolvedConcreteItem::Impl(item) => item
            .concrete_trait(db)
            .map(|trait_id| {
                db.trait_functions(trait_id.trait_id(db))
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                    .map(|(name, trait_function_id)| {
                        let crate_id = trait_id.trait_id(db).parent_module(db).owning_crate(db);
                        let signature = db
                            .trait_function_signature(*trait_function_id)
                            .map(|sig| {
                                generate_abbreviated_signature(db, sig, Some(trait_id.trait_id(db)))
                            })
                            .ok();

                        CompletionItemOrderable {
                            item: CompletionItem {
                                label: name.to_string(db),
                                label_details: Some(CompletionItemLabelDetails {
                                    description: signature,
                                    detail: None,
                                }),
                                kind: Some(CompletionItemKind::FUNCTION),
                                ..CompletionItem::default()
                            },
                            relevance: get_item_relevance(
                                is_current_scope,
                                crate_id == current_crate,
                                *crate_id.long(db) == CrateLongId::core(db),
                            ),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default(),
        ResolvedConcreteItem::Type(ty) => match ty.long(db) {
            TypeLongId::Concrete(ConcreteTypeId::Enum(enum_id)) => db
                .enum_variants(enum_id.enum_id(db))
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|(name, variant_id)| {
                    let formatted_enum_variant =
                        format_enum_variant(db, &enum_id.enum_id(db), variant_id);

                    let crate_id = enum_id.enum_id(db).parent_module(db).owning_crate(db);
                    CompletionItemOrderable {
                        item: CompletionItem {
                            label: name.to_string(db),
                            label_details: Some(CompletionItemLabelDetails {
                                description: formatted_enum_variant,
                                detail: None,
                            }),
                            kind: Some(CompletionItemKind::ENUM_MEMBER),
                            ..CompletionItem::default()
                        },
                        relevance: get_item_relevance(
                            is_current_scope,
                            crate_id == current_crate,
                            *crate_id.long(db) == CrateLongId::core(db),
                        ),
                    }
                })
                .collect(),
            _ => vec![],
        },
        _ => vec![],
    })
}

fn get_importables_for_path_suffix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Option<Arc<OrderedHashMap<ImportableId<'db>, String>>> {
    if ctx.node.ancestor_of_kind(db, SyntaxKind::Attribute).is_none()
        && dot_expr_rhs(db, &ctx.node, was_node_corrected).is_none()
        && ctx.node.ancestor_of_kind(db, SyntaxKind::PatternEnum).is_none()
    {
        db.visible_importables_from_module(ctx.module_id)
    } else {
        None
    }
}

fn get_typed_text_and_last_segment<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> (Option<Vec<SmolStrId<'db>>>, Option<SmolStrId<'db>>) {
    let typed_text = match ctx
        .node
        .ancestor_of_type::<ExprPath>(db)
        .map(|path| path.segments(db).elements(db).collect_vec())
    {
        Some(segments) if !segments.is_empty() => segments,
        _ => return (None, None),
    };

    let mut typed_text_as_smol_str = typed_text
        .into_iter()
        .map(|segment| segment.as_syntax_node())
        // Allow proposing items in the middle of the existing path by filtering out the nodes which lie after the cursor:
        .filter_map(|segment_node| {
            if segment_node.offset(db) <= ctx.node.offset(db) {
                Some(segment_node.get_text_without_trivia(db))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // After `::`, we want to propose all importables available at the preceding path.
    let last_typed_segment: SmolStrId<'db> = if ctx.node.kind(db) == SyntaxKind::TerminalColonColon
    {
        SmolStrId::from(db, "")
    } else {
        // Otherwise, the last segment is a partial identifier we want to complete using fuzzy search.
        typed_text_as_smol_str.pop().expect("typed path should not be empty")
    };

    (Some(typed_text_as_smol_str), Some(last_typed_segment))
}

fn module_item_completion_detail<'db>(
    db: &'db AnalysisDatabase,
    item: ModuleItemId<'db>,
    ctx_module_id: ModuleId<'db>,
) -> Option<String> {
    if let ModuleItemId::Constant(constant_id) = item
        && let Ok(constant) = db.constant_semantic_data(constant_id)
        && let Some(importables) = db.visible_importables_from_module(ctx_module_id)
    {
        Some(format_type(db, constant.ty(), &importables, None))
    } else if let Some(generic_type_id) = GenericTypeId::option_from(item) {
        Some(generic_type_id.format(db))
    } else if let Some(generic_function_id) = GenericFunctionId::option_from(item)
        && let Ok(signature) = generic_function_id.generic_signature(db)
    {
        let abbreviated_signature = generate_abbreviated_signature(db, signature, None);
        Some(abbreviated_signature)
    } else {
        None
    }
}
