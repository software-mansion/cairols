use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextPosition, TextSpan};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, Terminal};
use cairo_lang_utils::Upcast;

// TODO(mkaput): Make this a real Salsa query group with sensible LRU.
/// LS-specific extensions to the syntax group of the Cairo compiler.
#[salsa::query_group(LsSyntaxDatabase)]
pub trait LsSyntaxGroup: ParserGroup + Upcast<dyn ParserGroup> {
    /// Finds the most specific [`SyntaxNode`] at the given [`TextOffset`] in the file.
    fn find_syntax_node_at_offset(&self, file: FileId, offset: TextOffset) -> Option<SyntaxNode>;

    /// Finds the widest [`SyntaxNode`] within the given [`TextSpan`] in the file.
    fn widest_node_within_span(&self, file: FileId, span: TextSpan) -> Option<SyntaxNode>;

    /// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
    fn find_syntax_node_at_position(
        &self,
        file: FileId,
        position: TextPosition,
    ) -> Option<SyntaxNode>;

    /// Finds a [`TerminalIdentifier`] at the given [`TextPosition`] in the file.
    ///
    /// The lookup for identifiers is slightly more sophisticated than just looking for an arbitrary
    /// syntax node because identifiers usually are what the user is interested in.
    /// In case when the user position is `ident<caret>()`, while regular syntax node lookup would
    /// return the left paren, a much better UX would be to correct the lookup to the identifier.
    /// Such corrections are always valid and deterministic, because grammar-wise it is not possible
    /// to have two identifiers/keywords being glued to each other.
    fn find_identifier_at_position(
        &self,
        file: FileId,
        position: TextPosition,
    ) -> Option<TerminalIdentifier>;
}

/// Finds the most specific [`SyntaxNode`] at the given [`TextOffset`] in the file.
fn find_syntax_node_at_offset(
    db: &dyn LsSyntaxGroup,
    file: FileId,
    offset: TextOffset,
) -> Option<SyntaxNode> {
    Some(db.file_syntax(file).to_option()?.lookup_offset(db, offset))
}

/// Finds the widest [`SyntaxNode`] within the given [`TextSpan`] in the file.
fn widest_node_within_span(
    db: &dyn LsSyntaxGroup,
    file: FileId,
    span: TextSpan,
) -> Option<SyntaxNode> {
    let precise_node = db.find_syntax_node_at_offset(file, span.start)?;

    precise_node
        .ancestors_with_self(db)
        .take_while(|new_node| span.contains(new_node.span(db)))
        .last()
}

/// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
fn find_syntax_node_at_position(
    db: &dyn LsSyntaxGroup,
    file: FileId,
    position: TextPosition,
) -> Option<SyntaxNode> {
    Some(db.file_syntax(file).to_option()?.lookup_position(db, position))
}

/// Finds a [`TerminalIdentifier`] at the given [`TextPosition`] in the file.
///
/// The lookup for identifiers is slightly more sophisticated than just looking for an arbitrary
/// syntax node because identifiers usually are what the user is interested in.
/// In case when the user position is `ident<caret>()`, while regular syntax node lookup would
/// return the left paren, a much better UX would be to correct the lookup to the identifier.
/// Such corrections are always valid and deterministic, because grammar-wise it is not possible
/// to have two identifiers/keywords being glued to each other.
fn find_identifier_at_position(
    db: &dyn LsSyntaxGroup,
    file: FileId,
    position: TextPosition,
) -> Option<TerminalIdentifier> {
    let find = |position: TextPosition| {
        let node = db.find_syntax_node_at_position(file, position)?;
        TerminalIdentifier::cast_token(db, node)
    };

    find(position).or_else(|| {
        // Try searching one character to the left.
        let col = position.col.checked_sub(1)?;
        find(TextPosition { col, ..position })
    })
}
