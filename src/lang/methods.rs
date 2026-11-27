use cairo_lang_defs::ids::{LanguageElementId, ModuleId, NamedLanguageElementId, TraitFunctionId};
use cairo_lang_filesystem::db::{CORELIB_CRATE_NAME, FilesGroup};
use cairo_lang_filesystem::ids::{CrateId, CrateLongId, SmolStrId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::expr::inference::infers::InferenceEmbeddings;
use cairo_lang_semantic::expr::inference::solver::SolutionSet;
use cairo_lang_semantic::expr::inference::{ImplVarTraitItemMappings, InferenceId};
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::lsp_helpers::{LspHelpers, TypeFilter};
use cairo_lang_semantic::resolve::Resolver;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::Intern;
use itertools::{Itertools, chain};
use tracing::debug;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Describes where a method comes from, relative to the module it is looked up from.
///
/// Variants are ordered from the most to the least relevant one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MethodOrigin {
    /// The trait of the method is in scope of the current module.
    CurrentScope,
    /// The trait of the method is defined in the current crate, but it is not in scope.
    CurrentCrate,
    /// The trait of the method is defined in a dependency other than the corelib.
    Dependency,
    /// The trait of the method is defined in the corelib.
    Corelib,
}

/// Returns the [`MethodOrigin`] of `trait_function` when looked up from `module_id`.
pub fn method_origin<'db>(
    db: &'db AnalysisDatabase,
    module_id: ModuleId<'db>,
    trait_function: TraitFunctionId<'db>,
) -> MethodOrigin {
    let trait_id = trait_function.trait_id(db);

    // A trait is in scope if it is visible from the module without a path qualifier.
    let is_in_scope = db
        .visible_traits_from_module(module_id)
        .and_then(|traits| traits.get(&trait_id).map(|path| !path.contains("::")))
        .unwrap_or_default();

    let trait_crate = trait_id.parent_module(db).owning_crate(db);

    if is_in_scope {
        MethodOrigin::CurrentScope
    } else if trait_crate == module_id.owning_crate(db) {
        MethodOrigin::CurrentCrate
    } else if trait_crate == CrateId::core(db) {
        MethodOrigin::Corelib
    } else {
        MethodOrigin::Dependency
    }
}

/// Finds all methods that can be called on a type.
/// The methods are ordered by their [`MethodOrigin`]:
/// 1. Methods in the current scope.
/// 2. Methods in the current crate.
/// 3. Methods in dependencies other than the corelib.
/// 4. Methods in the corelib.
pub fn find_methods_for_type<'db>(
    db: &'db AnalysisDatabase,
    resolver: &mut Resolver<'db>,
    ty: cairo_lang_semantic::TypeId<'db>,
    stable_ptr: cairo_lang_syntax::node::ids::SyntaxStablePtrId<'db>,
) -> Vec<TraitFunctionId<'db>> {
    let type_filter = match ty.head(db) {
        Some(head) => TypeFilter::TypeHead(head),
        None => TypeFilter::NoFilter,
    };

    let mut relevant_methods = Vec::new();
    // Find methods on type.
    let crate_config = db.crate_config(resolver.owning_crate_id);
    let default = &Default::default();
    let dependencies = crate_config.map(|config| &config.settings.dependencies).unwrap_or(default);

    for crate_id in chain!(
        [resolver.owning_crate_id],
        (!dependencies.contains_key(CORELIB_CRATE_NAME)).then(|| CrateId::core(db)),
        dependencies.iter().map(|(name, setting)| {
            CrateLongId::Real {
                name: SmolStrId::from(db, name.as_str()),
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
                lookup_context,
                Some(stable_ptr),
                &mut vec![],
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
    let module_id = resolver.data.module_id;

    relevant_methods
        .into_iter()
        .unique()
        // The sort is stable, so methods with the same origin keep the order they were found in.
        .sorted_by_cached_key(|method| method_origin(db, module_id, *method))
        .collect()
}

/// Finds all available traits with method.
/// Returns list of full qualified paths as strings.
pub fn available_traits_for_method<'db>(
    db: &'db AnalysisDatabase,
    node: &SyntaxNode<'db>,
    resolver: &mut Resolver<'db>,
) -> Option<Vec<String>> {
    let stable_ptr = node.ancestor_of_type::<ast::ExprBinary>(db)?.lhs(db).stable_ptr(db);

    // Get its semantic model.
    let function_with_body = db.find_lookup_item(*node)?.function_with_body()?;
    let expr_id = db.lookup_expr_by_ptr(function_with_body, stable_ptr).ok()?;

    let semantic_db: &dyn SemanticGroup = db;
    let ty = semantic_db.expr_semantic(function_with_body, expr_id).ty();

    if ty.is_missing(db) {
        return None;
    }

    let module = db.find_module_containing_node(*node)?;
    let module_visible_traits = db.visible_traits_from_module(module)?;
    let unknown_method_name = node.get_text(db);

    Some(
        find_methods_for_type(db, resolver, ty, stable_ptr.untyped())
            .into_iter()
            .filter(|method| method.name(db).to_string(db).as_str() == unknown_method_name)
            .filter_map(|method| module_visible_traits.get(&method.trait_id(db)).cloned())
            .collect(),
    )
}
