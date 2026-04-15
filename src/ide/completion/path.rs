use std::sync::Arc;

use cairo_lang_defs::ids::{GenericTypeId, ModuleId, ModuleItemId, TraitId};
use cairo_lang_defs::ids::{ImportableId, LanguageElementId, NamedLanguageElementId};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId, SmolStrId};
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnostics};
use cairo_lang_semantic::items::constant::ConstantSemantic;
use cairo_lang_semantic::items::enm::EnumSemantic;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::items::imp::ImplSemantic;
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
use lsp_types::{CompletionItem, CompletionItemKind, CompletionItemLabelDetails, InsertTextFormat};

use super::helpers::completion_kind::resolved_generic_item_completion_kind;
use crate::ide::completion::helpers::binary_expr::dot_rhs::dot_expr_rhs;
use crate::ide::completion::helpers::formatting::{
    format_enum_variant, generate_abbreviated_signature,
};
use crate::ide::completion::helpers::is_empty_body_context;
use crate::ide::completion::helpers::item::{
    CompletionItemOrderable, ImportableCompletionItem, ImportableCompletionItemHashable,
    get_item_relevance,
};
use crate::ide::completion::helpers::snippets::TypedSnippet;
use crate::ide::format::types::format_type;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::visibility::peek_visible_in_with_edition;

/// Proposes importable items whose full path ends with the typed text.
///
/// "Suffix" refers to the typed text being matched against the **end** of known importable paths:
/// e.g. typing `ArrayTrait` surfaces `core::array::ArrayTrait` as a candidate to import.
/// Used by [`macro_call`] for filtering completions by importable kind.
pub(crate) fn importable_path_suffix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &'db AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<ImportableCompletionItem<'db>> {
    let Some(importables) =
        get_importables_for_path_suffix_completions(db, ctx, was_node_corrected)
    else {
        return Default::default();
    };
    let Some((typed_segments, last_typed_segment)) =
        resolve_typed_segments(db, ctx)
    else {
        return Default::default();
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
                typed_segments.clone(),
                last_typed_segment,
            )
        })
        .unique_by(|completion| ImportableCompletionItemHashable(completion.clone()))
        .collect()
}

/// Proposes completions for an unresolved path expression. Combines two strategies:
///
/// - **Importable suffix match**: the typed text is matched against the end of known importable
///   paths, surfacing items that can be imported. e.g. typing `ArrayTrait` surfaces
///   `core::array::ArrayTrait`.
///
/// - **Trait/impl item expansion** (expression blocks only): when the cursor is directly after
///   `::`, e.g. `MyTrait::<caret>`, proposes bare `item_name` completions for all items of
///   traits/impls whose path matches the typed prefix.
pub fn path_suffix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &'db AnalysisContext<'db>,
    was_node_corrected: bool,
) -> Vec<CompletionItemOrderable> {
    let Some(importables) =
        get_importables_for_path_suffix_completions(db, ctx, was_node_corrected)
    else {
        return Default::default();
    };
    let Some((typed_segments, last_typed_segment)) = resolve_typed_segments(db, ctx) else {
        return Default::default();
    };
    let current_crate = ctx.module_id.owning_crate(db);

    let mut result: Vec<CompletionItemOrderable> = importables
        .iter()
        .filter_map(|(importable, path_str)| {
            ImportableCompletionItem::get_completion_item_for_importable(
                db,
                ctx,
                importable,
                current_crate,
                path_str,
                typed_segments.clone(),
                last_typed_segment,
            )
        })
        .unique_by(|completion| ImportableCompletionItemHashable(completion.clone()))
        .map(|item| item.item)
        .collect();

    // In expression blocks, when the cursor is directly after `::` with non-empty preceding
    // segments, also propose bare item names from traits/impls whose path matches.
    if ctx.node.ancestor_of_kind(db, SyntaxKind::ExprBlock).is_some()
        && !typed_segments.is_empty()
        && last_typed_segment.to_string(db).is_empty()
    {
        extend_with_trait_items_for_matching_paths(
            db,
            ctx,
            &importables,
            &typed_segments,
            current_crate,
            &mut result,
        );
    }

    result
}

/// Proposes completions for the next segment of a resolvable path.
///
/// "Prefix" means the typed path is successfully resolved as the beginning of a longer path,
/// and this function returns what can legally follow it. e.g. typing `core::array::` resolves
/// that module and proposes its members.
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
        ResolvedConcreteItem::Trait(item) | ResolvedConcreteItem::SelfTrait(item) => {
            let trait_id = item.trait_id(db);
            let crate_id = trait_id.parent_module(db).owning_crate(db);
            trait_items_prefix_completions(
                db,
                ctx,
                trait_id,
                is_current_scope,
                current_crate,
                crate_id,
            )
        }
        ResolvedConcreteItem::Impl(item) => item
            .concrete_trait(db)
            .map(|concrete_trait| {
                let trait_id = concrete_trait.trait_id(db);
                let crate_id = trait_id.parent_module(db).owning_crate(db);
                trait_items_prefix_completions(
                    db,
                    ctx,
                    trait_id,
                    is_current_scope,
                    current_crate,
                    crate_id,
                )
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

/// Returns the typed path segments and the last (potentially partial) segment, falling back to
/// empty values when the cursor is in an empty body context (no path typed yet).
///
/// Returns `None` when neither condition holds (i.e. there is no path at the cursor and the
/// context is not an empty body).
fn resolve_typed_segments<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
) -> Option<(Vec<SmolStrId<'db>>, SmolStrId<'db>)> {
    match get_typed_text_and_last_segment(db, ctx) {
        (Some(typed_segments), Some(last_typed_segment)) => Some((typed_segments, last_typed_segment)),
        _ if is_empty_body_context(db, &ctx.node) => Some((vec![], SmolStrId::from(db, ""))),
        _ => None,
    }
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

/// Generates prefix completion items for all items (functions, types, constants) of a trait.
fn trait_items_prefix_completions<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    trait_id: TraitId<'db>,
    is_current_scope: bool,
    current_crate: CrateId<'db>,
    crate_id: CrateId<'db>,
) -> Vec<CompletionItemOrderable> {
    let relevance = get_item_relevance(
        is_current_scope,
        crate_id == current_crate,
        *crate_id.long(db) == CrateLongId::core(db),
    );

    let functions = db.trait_functions(trait_id).cloned().unwrap_or_default().into_iter().map(
        |(name, trait_function_id)| {
            let name_str = name.to_string(db);
            let signature = db.trait_function_signature(trait_function_id).ok();
            let snippet = signature
                .map(|sig| TypedSnippet::function_call(db, &name_str, sig, Some(trait_id)));
            let label = if snippet.is_some() { format!("{}(...)", name_str) } else { name_str };
            CompletionItemOrderable {
                item: CompletionItem {
                    label,
                    insert_text: snippet.clone().map(|s| s.lsp_snippet),
                    insert_text_format: snippet.clone().map(|_| InsertTextFormat::SNIPPET),
                    label_details: Some(CompletionItemLabelDetails {
                        detail: None,
                        description: snippet.and_then(|s| s.type_hint),
                    }),
                    kind: Some(CompletionItemKind::FUNCTION),
                    ..CompletionItem::default()
                },
                relevance,
            }
        },
    );

    let types =
        db.trait_types(trait_id).cloned().unwrap_or_default().into_iter().map(|(name, _)| {
            CompletionItemOrderable {
                item: CompletionItem {
                    label: name.to_string(db),
                    label_details: Some(CompletionItemLabelDetails {
                        detail: None,
                        description: None,
                    }),
                    kind: Some(CompletionItemKind::CLASS),
                    ..CompletionItem::default()
                },
                relevance,
            }
        });

    let constants = db.trait_constants(trait_id).cloned().unwrap_or_default().into_iter().map(
        |(name, trait_constant_id)| {
            let description = db.trait_constant_type(trait_constant_id).ok().and_then(|ty| {
                db.visible_importables_from_module(ctx.module_id)
                    .map(|importables| format_type(db, ty, &importables, None))
            });
            CompletionItemOrderable {
                item: CompletionItem {
                    label: name.to_string(db),
                    label_details: Some(CompletionItemLabelDetails { detail: None, description }),
                    kind: Some(CompletionItemKind::CONSTANT),
                    ..CompletionItem::default()
                },
                relevance,
            }
        },
    );

    functions.chain(types).chain(constants).collect()
}

/// For each trait or impl in `importables` whose path ends with `typed_segments`, extends
/// `result` with completion items for all of that trait's items (functions, types, constants).
fn extend_with_trait_items_for_matching_paths<'db>(
    db: &'db AnalysisDatabase,
    ctx: &'db AnalysisContext<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
    typed_segments: &[SmolStrId<'db>],
    current_crate: CrateId<'db>,
    result: &mut Vec<CompletionItemOrderable>,
) {
    for (importable, item_path) in importables.iter() {
        let trait_id = match importable {
            ImportableId::Trait(trait_id) => *trait_id,
            ImportableId::Impl(impl_id) => {
                let Some(trait_id) = db.impl_def_trait(*impl_id).ok() else { continue };
                trait_id
            }
            _ => continue,
        };

        let path_segments: Vec<&str> = item_path.split("::").collect();

        // The typed segments must match the last N segments of the path (suffix match).
        if typed_segments.len() > path_segments.len() {
            continue;
        }
        let path_suffix = &path_segments[path_segments.len() - typed_segments.len()..];
        let matches = path_suffix
            .iter()
            .zip(typed_segments.iter())
            .all(|(ps, ts)| *ps == ts.to_string(db).as_str());
        if !matches {
            continue;
        }

        let crate_id = trait_id.parent_module(db).owning_crate(db);
        result.extend(trait_items_prefix_completions(
            db,
            ctx,
            trait_id,
            true,
            current_crate,
            crate_id,
        ));
    }
}
