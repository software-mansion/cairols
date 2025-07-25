use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::ast::{TerminalIdentifier, TerminalUnderscore};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

use crate::ide::markdown::RULE;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo, ToLsp};

mod render;

/// Get hover information at a given text document position.
pub fn hover(params: HoverParams, db: &AnalysisDatabase) -> Option<Hover> {
    let file_id = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();
    // Try to apply identifier correction before resultants.
    let node = db
        .find_identifier_at_position(file_id, position)
        .map(|ident| ident.as_syntax_node())
        .or_else(|| db.find_syntax_node_at_position(file_id, position))?;

    let resultants = db.get_node_resultants(node)?;

    let hover_content = resultants
        .into_iter()
        .filter_map(|node| render_hover(db, node))
        .collect::<OrderedHashSet<_>>() // Deduplicate so we want display doubled hover if we point to same item from few resultants.
        .into_iter()
        .reduce(|value1, value2| format!("{value1}\n{RULE}{value2}"))?;

    let hover = Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: hover_content,
        }),
        range: node.span_without_trivia(db).position_in_file(db, file_id).map(|p| p.to_lsp()),
    };

    Some(hover)
}

fn render_hover(db: &AnalysisDatabase, node: SyntaxNode) -> Option<String> {
    let file_id = node.stable_ptr(db).file_id(db);

    let importables =
        db.visible_importables_from_module(db.find_module_file_containing_node(node)?)?;

    if let Some(hover) = render::literal(db, node, &importables) {
        return Some(hover);
    }

    if let Some(hover) = node
        .ancestor_of_type::<TerminalUnderscore>(db)
        .and_then(|underscore| render::ty(db, underscore, &importables))
    {
        return Some(hover);
    }

    if let Some(hover) = node
        .ancestors_with_self(db)
        .find_map(|node| TerminalIdentifier::cast(db, node))
        .and_then(|ref id| render::definition(db, id, file_id, &importables))
    {
        return Some(hover);
    }

    None

    // TODO(mkaput): If client only supports plaintext, strip markdown formatting here like RA.
}
