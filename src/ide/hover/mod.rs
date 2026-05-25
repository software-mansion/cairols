use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::ast::{TerminalIdentifier, TerminalUnderscore};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_set::OrderedHashSet;
use cairo_language_common::CommonGroup;
use lsp_types::{Hover, HoverContents, HoverParams, MarkupContent, MarkupKind};

use crate::ide::markdown::RULE;
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
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

    // Try standard renderers first (identifiers, literals, keywords, underscores).
    let content = resultants
        .iter()
        .filter_map(|node| render_hover(db, *node))
        .collect::<OrderedHashSet<_>>() // Deduplicate so we don't display doubled hover if we point to same item from few resultants.
        .into_iter()
        .reduce(|value1, value2| format!("{value1}\n{RULE}{value2}"));

    // Fallback: show the type of the expression at the cursor position, highlighting the
    // full expression rather than just the hovered token.
    let (content, node) = match (content, node) {
        (Some(content), node) => (content, node),
        _ => resultants.iter().find_map(|node| render::type_info(db, *node))?,
    };

    // Map expanded nodes back to their originating source span so hover highlights work in user
    // code even when type info comes from a virtual file.
    let SpanInFile { file_id: range_file_id, span } = get_originating_location(
        db,
        SpanInFile { file_id: node.stable_ptr(db).file_id(db), span: node.span_without_trivia(db) },
        None,
    );
    let range = (range_file_id == file_id)
        .then(|| span.position_in_file(db, file_id).map(|p| p.to_lsp()))
        .flatten();

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range,
    })
}

fn render_hover<'db>(db: &'db AnalysisDatabase, node: SyntaxNode<'db>) -> Option<String> {
    let file_id = node.stable_ptr(db).file_id(db);

    let importables = db.visible_importables_from_module(db.find_module_containing_node(node)?)?;

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

    if node.kind(db).is_keyword_token() {
        return render::keyword(db, node.kind(db));
    }

    None

    // TODO(mkaput): If client only supports plaintext, strip markdown formatting here like RA.
}
