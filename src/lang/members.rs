use crate::lang::db::AnalysisDatabase;
use cairo_lang_defs::ids::{FunctionWithBodyId, MemberId};
use cairo_lang_filesystem::ids::StrRef;
use cairo_lang_semantic::db::SemanticGroup;

/// Finds and returns the members associated with a given type in the context of the specified function.
pub fn find_members_for_type<'db>(
    db: &'db AnalysisDatabase,
    function_with_body: FunctionWithBodyId<'db>,
    ty: cairo_lang_semantic::TypeId<'db>,
) -> Option<Vec<StrRef<'db>>> {
    let resolver_data = match function_with_body {
        FunctionWithBodyId::Free(id) => db.free_function_body_resolver_data(id),
        FunctionWithBodyId::Impl(id) => db.impl_function_body_resolver_data(id),
        FunctionWithBodyId::Trait(id) => db.trait_function_resolver_data(id),
    }
    .ok()?;

    let type_enriched_members = resolver_data
        .type_enriched_members
        .get(&(ty, true)) // We don't care here about mutability
        .or_else(|| resolver_data.type_enriched_members.get(&(ty, false)))?;

    Some(type_enriched_members.members.keys().copied().collect())
}
