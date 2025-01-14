use cairo_lang_utils::Upcast;
use lsp_types::{GotoDefinitionParams, GotoDefinitionResponse, Location};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::SymbolDef;
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};

/// Get the definition location of a symbol at a given text document position.
pub fn goto_definition(
    params: GotoDefinitionParams,
    db: &AnalysisDatabase,
) -> Option<GotoDefinitionResponse> {
    let file = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();

    let identifier = db.find_identifier_at_position(file, position)?;
    let symbol = SymbolDef::find(db, &identifier)?;
    let (found_file, span) = symbol.definition_location(db)?;

    let found_uri = db.url_for_file(found_file)?;
    let range = span.position_in_file(db.upcast(), found_file)?.to_lsp();
    let location = Location { uri: found_uri, range };

    Some(GotoDefinitionResponse::Scalar(location))
}
