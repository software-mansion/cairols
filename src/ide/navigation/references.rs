use cairo_lang_utils::Upcast;
use itertools::Itertools;
use lsp_types::{Location, ReferenceParams};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::inspect::defs::SymbolDef;
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};

pub fn references(params: ReferenceParams, db: &AnalysisDatabase) -> Option<Vec<Location>> {
    let include_declaration = params.context.include_declaration;

    let file = db.file_for_url(&params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position.to_cairo();

    let identifier = db.find_identifier_at_position(file, position)?;

    let symbol = SymbolDef::find(db, &identifier)?;

    // TODO(mkaput): Think about how to deal with `mod foo;` vs `mod foo { ... }`.
    // Location where the searched symbol is declared.
    // This can rarely be `None`, for example, for macros.
    // For all cases we cover here, definition == declaration.
    let declaration = symbol.definition_location(db);

    let locations = {
        let declaration = declaration.filter(|_| include_declaration);

        // TODO(mkaput): Implement this.
        let references = vec![];

        declaration.into_iter().chain(references)
    }
    .unique()
    .filter_map(|(file, span)| {
        let found_uri = db.url_for_file(file)?;
        let range = span.position_in_file(db.upcast(), file)?.to_lsp();
        let location = Location { uri: found_uri, range };
        Some(location)
    })
    .collect();

    Some(locations)
}
