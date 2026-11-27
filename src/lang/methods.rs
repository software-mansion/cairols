use cairo_lang_defs::ids::{
    ImportableId, LanguageElementId, ModuleId, NamedLanguageElementId, TraitFunctionId, TraitId,
};
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
use cairo_lang_utils::unordered_hash_set::UnorderedHashSet;
use itertools::{Itertools, chain};
use tracing::debug;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Describes where a method comes from, relative to the module it is looked up from.
///
/// Variants are ordered from the most to the least relevant one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MethodOrigin {
    /// The trait of the method is in scope of the current module, not only because of the prelude.
    CurrentScope,
    /// The trait of the method is defined in the current crate, but it is not in scope.
    CurrentCrate,
    /// The trait of the method is defined in a dependency other than the corelib.
    Dependency,
    /// The trait of the method is in scope of the current module only because of the prelude.
    Prelude,
    /// The trait of the method is defined in the corelib and it is not in scope.
    Corelib,
}

/// Computes [`MethodOrigin`]s of methods looked up from a single module.
struct MethodOriginResolver<'db> {
    module_crate: CrateId<'db>,
    core_crate: CrateId<'db>,
    /// Traits usable without a path qualifier in the module.
    in_scope_traits: UnorderedHashSet<TraitId<'db>>,
    /// Traits defined in or imported into the module itself.
    module_traits: UnorderedHashSet<TraitId<'db>>,
}

impl<'db> MethodOriginResolver<'db> {
    fn new(db: &'db AnalysisDatabase, module_id: ModuleId<'db>) -> Self {
        let in_scope_traits = db
            .visible_traits_from_module(module_id)
            .map(|traits| {
                traits
                    .iter()
                    .filter(|(_, path)| !path.contains("::"))
                    .map(|(trait_id, _)| *trait_id)
                    .collect()
            })
            .unwrap_or_default();

        // Traits in scope that are not defined in or imported into the module come from the
        // prelude.
        let module_traits = db
            .visible_importables_in_module(module_id, module_id, true)
            .iter()
            .filter_map(|(importable, path)| match importable {
                ImportableId::Trait(trait_id) if !path.contains("::") => Some(*trait_id),
                _ => None,
            })
            .collect();

        Self {
            module_crate: module_id.owning_crate(db),
            core_crate: CrateId::core(db),
            in_scope_traits,
            module_traits,
        }
    }

    fn origin(
        &self,
        db: &'db AnalysisDatabase,
        trait_function: TraitFunctionId<'db>,
    ) -> MethodOrigin {
        let trait_id = trait_function.trait_id(db);

        if self.in_scope_traits.contains(&trait_id) {
            return if self.module_traits.contains(&trait_id) {
                MethodOrigin::CurrentScope
            } else {
                MethodOrigin::Prelude
            };
        }

        let trait_crate = trait_id.parent_module(db).owning_crate(db);
        if trait_crate == self.module_crate {
            MethodOrigin::CurrentCrate
        } else if trait_crate == self.core_crate {
            MethodOrigin::Corelib
        } else {
            MethodOrigin::Dependency
        }
    }
}

/// Finds all methods that can be called on a type, together with their [`MethodOrigin`].
///
/// The methods are sorted by their origin, so that quick fixes can propose the most relevant
/// method first:
/// 1. Methods in the current scope.
/// 2. Methods in the current crate.
/// 3. Methods in dependencies other than the corelib.
/// 4. Methods in scope because of the prelude.
/// 5. Other methods in the corelib.
pub fn find_methods_for_type<'db>(
    db: &'db AnalysisDatabase,
    resolver: &mut Resolver<'db>,
    ty: cairo_lang_semantic::TypeId<'db>,
    stable_ptr: cairo_lang_syntax::node::ids::SyntaxStablePtrId<'db>,
) -> Vec<(TraitFunctionId<'db>, MethodOrigin)> {
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
    let origin_resolver = MethodOriginResolver::new(db, resolver.data.module_id);

    relevant_methods
        .into_iter()
        .unique()
        .map(|method| (method, origin_resolver.origin(db, method)))
        // The sort is stable, so methods with the same origin keep the order they were found in.
        .sorted_by_key(|(_, origin)| *origin)
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
            .map(|(method, _)| method)
            .filter(|method| method.name(db).to_string(db).as_str() == unknown_method_name)
            .filter_map(|method| module_visible_traits.get(&method.trait_id(db)).cloned())
            .collect(),
    )
}
