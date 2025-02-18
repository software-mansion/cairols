use cairo_lang_utils::Upcast;
use itertools::Itertools;
use lsp_types::{Location, ReferenceParams};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::SymbolDef;
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};

pub fn references(params: ReferenceParams, db: &AnalysisDatabase) -> Option<Vec<Location>> {
    let include_declaration = params.context.include_declaration;

    let file = db.file_for_url(&params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position.to_cairo();

    let identifier = db.find_identifier_at_position(file, position)?;

    let symbol = SymbolDef::find(db, &identifier)?;

    let locations = symbol
        .usages(db)
        .include_declaration(include_declaration)
        .locations()
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
