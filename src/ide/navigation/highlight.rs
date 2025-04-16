use itertools::Itertools;
use lsp_types::{DocumentHighlight, DocumentHighlightParams};

use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::defs::SymbolSearch;
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};
use crate::lang::usages::search_scope::SearchScope;

pub fn highlight(
    params: DocumentHighlightParams,
    db: &AnalysisDatabase,
) -> Option<Vec<DocumentHighlight>> {
    let file = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();

    let identifier = db.find_identifier_at_position(file, position)?;

    let symbol_search = SymbolSearch::find_definition(db, &identifier)?;

    let highlights = symbol_search
        .def
        .usages(db)
        .include_declaration(true)
        .in_scope(SearchScope::file(file))
        .locations()
        .unique()
        .filter(|(found_file, _)| *found_file == file)
        .filter_map(|(file, text_span)| {
            text_span.position_in_file(db, file).as_ref().map(ToLsp::to_lsp)
        })
        .map(|range| DocumentHighlight { range, kind: None })
        .collect();

    Some(highlights)
}
