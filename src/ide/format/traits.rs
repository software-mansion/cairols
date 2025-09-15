use cairo_lang_defs::ids::{ImportableId, NamedLanguageElementId, TraitId};
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

use super::types::format_generic_args;
use crate::lang::db::AnalysisDatabase;

/// Returns a textual representation of a trait with the given [`TraitId`],
/// consisting of name, path and generic parameters.
/// Precedes the name with a shortest path allowed by `importables`.
///
/// # Example
/// For `core::num` importable in scope, `Zero` trait is going to be formatted: `num::traits::Zero<T>`.
pub fn format_trait_path<'db>(
    db: &'db AnalysisDatabase,
    trait_id: TraitId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> String {
    let importable = ImportableId::Trait(trait_id);
    let path = importables.get(&importable).map(String::as_str).unwrap_or(trait_id.name(db));

    let generics = db
        .trait_generic_params(trait_id)
        .into_iter()
        .flatten()
        .map(|param| param.as_arg(db))
        .collect::<Vec<_>>();

    let formatted_generics_list = format_generic_args(db, &generics, importables);

    format!("{path}{formatted_generics_list}")
}
