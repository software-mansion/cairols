use lsp_types::{Location, ReferenceParams};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::SymbolDef;
use crate::lang::lsp::{LsProtoGroup, ToCairo};

pub fn references(params: ReferenceParams, db: &AnalysisDatabase) -> Option<Vec<Location>> {
    let include_declaration = params.context.include_declaration;

    let file = db.file_for_url(&params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position.to_cairo();

    let identifier = db.find_identifier_at_position(file, position)?;

    let symbol = SymbolDef::find(db, &identifier)?;

    Some(symbol.locations(db, include_declaration))
}
