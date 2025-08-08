use cairo_lang_defs::ids::{NamedLanguageElementId, TraitFunctionId};
use cairo_lang_filesystem::db::{CORELIB_CRATE_NAME, FilesGroup};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::expr::inference::infers::InferenceEmbeddings;
use cairo_lang_semantic::expr::inference::solver::SolutionSet;
use cairo_lang_semantic::expr::inference::{ImplVarTraitItemMappings, InferenceId};
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::lsp_helpers::TypeFilter;
use cairo_lang_semantic::resolve::Resolver;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::Intern;
use itertools::chain;
use tracing::debug;

use super::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;

/// Finds all methods that can be called on a type.
pub fn find_methods_for_type(
    db: &AnalysisDatabase,
    resolver: &mut Resolver<'_>,
    ty: cairo_lang_semantic::TypeId,
    stable_ptr: cairo_lang_syntax::node::ids::SyntaxStablePtrId,
) -> Vec<TraitFunctionId> {
    let type_filter = match ty.head(db) {
        Some(head) => TypeFilter::TypeHead(head),
        None => TypeFilter::NoFilter,
    };

    let mut relevant_methods = Vec::new();
    // Find methods on type.
    let dependencies = db
        .crate_config(resolver.owning_crate_id)
        .map(|config| config.settings.dependencies)
        .unwrap_or_default();

    for crate_id in chain!(
        [resolver.owning_crate_id],
        (!dependencies.contains_key(CORELIB_CRATE_NAME)).then(|| CrateId::core(db)),
        dependencies.iter().map(|(name, setting)| {
            CrateLongId::Real {
                name: name.clone().into(),
                discriminator: setting.discriminator.clone(),
            }
            .intern(db)
        })
    ) {
        let methods = db.methods_in_crate(crate_id, type_filter.clone());
        for trait_function in methods.iter().copied() {
            let clone_data =
                &mut resolver.inference().clone_with_inference_id(db, InferenceId::NoContext);
            let mut inference = clone_data.inference(db);
            let lookup_context = resolver.impl_lookup_context();
            // Check if trait function signature's first param can fit our expr type.
            let Some((concrete_trait_id, _)) = inference.infer_concrete_trait_by_self(
                trait_function,
                ty,
                &lookup_context,
                Some(stable_ptr),
                |_| {},
            ) else {
                debug!("can't fit");
                continue;
            };

            // Find impls for it.

            // ignore the result as nothing can be done with the error, if any.
            inference.solve().ok();
            if !matches!(
                inference.trait_solution_set(
                    concrete_trait_id,
                    ImplVarTraitItemMappings::default(),
                    lookup_context
                ),
                Ok(SolutionSet::Unique(_) | SolutionSet::Ambiguous(_))
            ) {
                continue;
            }
            relevant_methods.push(trait_function);
        }
    }
    relevant_methods
}

/// Finds all available traits with method.
/// Returns list of full qualified paths as strings.
pub fn available_traits_for_method(
    db: &AnalysisDatabase,
    ctx: &AnalysisContext<'_>,
) -> Option<Vec<String>> {
    let stable_ptr = ctx.node.ancestor_of_type::<ast::ExprBinary>(db)?.lhs(db).stable_ptr(db);

    // Get its semantic model.
    let function_with_body = ctx.lookup_item_id?.function_with_body()?;
    let expr_id = db.lookup_expr_by_ptr(function_with_body, stable_ptr).ok()?;

    let ty = db.expr_semantic(function_with_body, expr_id).ty();

    if ty.is_missing(db) {
        return None;
    }

    let module_visible_traits = db.visible_traits_from_module(ctx.module_file_id)?;
    let unknown_method_name = ctx.node.get_text(db);

    Some(
        find_methods_for_type(db, &mut ctx.resolver(db), ty, stable_ptr.untyped())
            .into_iter()
            .filter(|method| method.name(db) == unknown_method_name)
            .filter_map(|method| module_visible_traits.get(&method.trait_id(db)).cloned())
            .collect(),
    )
}
