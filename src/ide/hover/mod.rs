use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::lsp::{LsProtoGroup, ToCairo};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_syntax::node::ast::TerminalUnderscore;

mod render;

/// Get hover information at a given text document position.
pub fn hover(params: HoverParams, db: &AnalysisDatabase) -> Option<Hover> {
    let file_id = db.file_for_url(&params.text_document_position_params.text_document.uri)?;
    let position = params.text_document_position_params.position.to_cairo();
    let node = db.find_syntax_node_at_position(file_id, position)?;

    let importables =
        db.visible_importables_from_module(db.find_module_file_containing_node(&node)?)?;

    if let Some(hover) = render::literal(db, &node, file_id, &importables) {
        return Some(hover);
    }

    if let Some(hover) = node
        .ancestor_of_type::<TerminalUnderscore>(db)
        .and_then(|underscore| render::ty(db, underscore, file_id, &importables))
    {
        return Some(hover);
    }

    if let Some(hover) = db
        .find_identifier_at_position(file_id, position)
        .and_then(|ref id| render::definition(db, id, file_id, &importables))
    {
        return Some(hover);
    }

    None

    // TODO(mkaput): If client only supports plaintext, strip markdown formatting here like RA.
}

/// Convenience shortcut for building hover contents from markdown block.
pub fn markdown_contents(md: String) -> HoverContents {
    HoverContents::Markup(MarkupContent { kind: MarkupKind::Markdown, value: md })
}
