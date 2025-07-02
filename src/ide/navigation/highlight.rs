use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use itertools::Itertools;
use lsp_types::{DocumentHighlight, DocumentHighlightParams};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::defs::SymbolSearch;
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};
use crate::lang::usages::search_scope::SearchScope;

pub fn highlight(
    params: DocumentHighlightParams,
    db: &AnalysisDatabase,
) -> Option<Vec<DocumentHighlight>> {
    let file = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();

    let identifier = db.find_identifier_at_position(file, position)?.as_syntax_node();

    let highlights = db
        .get_node_resultants(identifier)
        .unwrap_or_else(|| vec![identifier])
        .into_iter()
        .filter_map(|node| {
            let id = node.cast::<TerminalIdentifier>(db)?;
            identifier_highlights(db, &id, file)
        })
        .flatten()
        .collect();

    Some(highlights)
}

/// Finds positions in the file to highlight as references to `identifier`.
fn identifier_highlights(
    db: &AnalysisDatabase,
    identifier: &TerminalIdentifier,
    file: FileId,
) -> Option<Vec<DocumentHighlight>> {
    let symbol_search = SymbolSearch::find_definition(db, identifier)?;

    let highlights = symbol_search
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
