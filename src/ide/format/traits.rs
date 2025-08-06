use std::borrow::Cow;

use cairo_lang_defs::ids::{ImportableId, NamedLanguageElementId, TraitId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

use super::types::format_generic_args;
use crate::lang::db::AnalysisDatabase;

#[allow(dead_code)] // Used later in the stack
/// Returns a textual representation of a trait with the given [`TraitId`],
/// consisting of name, path and generic parameters.
/// Precedes the name with a shortest path allowed by `importables`.
pub fn format_trait_path<'db>(
    db: &'db AnalysisDatabase,
    trait_id: TraitId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> String {
    let importable = ImportableId::Trait(trait_id);

    let path: Cow<str> = importables
        .get(&importable)
        .map(Into::into)
        .unwrap_or_else(|| Cow::Owned(trait_id.name(db).to_string()));

    let generics = db
        .trait_generic_params(trait_id)
        .into_iter()
        .flatten()
        .map(|param| param.as_arg(db))
        .collect::<Vec<_>>();

    let formatted_generics_list = format_generic_args(db, &generics, importables);

    format!("{path}{formatted_generics_list}")
}
