use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
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
        .filter_map(|node| highlights(db, &node, file))
        .flatten()
        .unique()
        .map(|range| DocumentHighlight { range, kind: None })
        .collect();

    Some(highlights)
}

fn highlights(
    db: &AnalysisDatabase,
    syntax_node: &SyntaxNode,
    file: FileId,
) -> Option<Vec<lsp_types::Range>> {
    let identifier =
        syntax_node.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
    let symbol_search = SymbolSearch::find_definition(db, &identifier)?;

    let highlights = symbol_search
        .usages(db)
        .include_declaration(true)
        .in_scope(SearchScope::file_with_subfiles(db, file))
        .originating_locations(db)
        .filter(|(found_file, _)| *found_file == file)
        .filter_map(|(file, text_span)| {
            text_span.position_in_file(db, file).as_ref().map(ToLsp::to_lsp)
        })
        .collect();

    Some(highlights)
}
