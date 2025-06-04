use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_syntax::node::ast::{Attribute, TerminalIdentifier};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use lsp_types::{Location, ReferenceParams};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::defs::SymbolSearch;
use crate::lang::lsp::{LsProtoGroup, ToCairo};

pub fn references(params: ReferenceParams, db: &AnalysisDatabase) -> Option<Vec<Location>> {
    let include_declaration = params.context.include_declaration;

    let file = db.file_for_url(&params.text_document_position.text_document.uri)?;
    let position = params.text_document_position.position.to_cairo();

    // Try to apply identifier correction before resultants.
    let node = db.find_identifier_at_position(file, position)?;

    let resultants = db.get_node_resultants(node.as_syntax_node())?;
    let locations: OrderedHashSet<_> = resultants
        .into_iter()
        .filter_map(|node| find_references(db, node, include_declaration))
        .flatten()
        .collect();

    Some(locations.into_iter().collect())
}

fn find_references(
    db: &AnalysisDatabase,
    syntax_node: SyntaxNode,
    include_declaration: bool,
) -> Option<Vec<Location>> {
    let identifier =
        syntax_node.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
    let symbol = SymbolSearch::find_definition(db, &identifier)?.def;

    Some(
        symbol
            .usages(db)
            .include_declaration(include_declaration)
            .originating_locations(db)
            .filter(|loc| {
                !is_in_derive_attribute(db, loc)
                    // We want to show definition location (if requested),
                    // even if it comes from a derive macro.
                    // Common case - impl declared in the derive macro.
                    || (include_declaration && Some(loc) == symbol.definition_location(db).as_ref())
            })
            .filter_map(|loc| db.lsp_location(loc))
            .collect(),
    )
}

/// Used to filter out references mapped to derives.
/// Such references aren't really useful since derives (when implemented properly)
/// always contain usages of an item they’re on.
fn is_in_derive_attribute(db: &AnalysisDatabase, (file, span): &(FileId, TextSpan)) -> bool {
    let Some(token) = db
        .find_syntax_node_at_offset(*file, span.start)
        // Sanity check: `span` is a span of a terminal identifier without trivia.
        // It should be the same as the span of a token at offset `span.start`.
        .filter(|node| node.span(db) == *span)
    else {
        return false;
    };

    let maybe_attribute_name = token
        .ancestor_of_type::<Attribute>(db)
        .map(|attr| attr.attr(db).as_syntax_node().get_text(db));

    maybe_attribute_name == Some("derive".to_string())
}
