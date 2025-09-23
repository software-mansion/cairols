use cairo_lang_defs::ids::FunctionWithBodyId;
use cairo_lang_filesystem::ids::SmolStrId;
use cairo_lang_semantic::items::free_function::FreeFunctionSemantic;
use cairo_lang_semantic::items::imp::ImplSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;

use crate::lang::db::AnalysisDatabase;

/// Finds and returns the members associated with a given type in the context of the specified function.
pub fn find_members_for_type<'db>(
    db: &'db AnalysisDatabase,
    function_with_body: FunctionWithBodyId<'db>,
    ty: cairo_lang_semantic::TypeId<'db>,
) -> Option<Vec<SmolStrId<'db>>> {
    let resolver_data = match function_with_body {
        FunctionWithBodyId::Free(id) => db.free_function_body_resolver_data(id).ok(),
        FunctionWithBodyId::Impl(id) => db.impl_function_body_resolver_data(id).ok(),
        FunctionWithBodyId::Trait(id) => db.trait_function_body_resolver_data(id).ok()?,
    }?;

    let type_enriched_members = resolver_data
        .type_enriched_members
        .get(&(ty, true)) // We don't care here about mutability
        .or_else(|| resolver_data.type_enriched_members.get(&(ty, false)))?;

    Some(type_enriched_members.members.keys().copied().collect())
}
