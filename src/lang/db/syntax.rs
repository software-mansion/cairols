use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextPosition};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, Terminal};
use cairo_lang_utils::Upcast;

// TODO(mkaput): Make this a real Salsa query group with sensible LRU.
/// LS-specific extensions to the syntax group of the Cairo compiler.
pub trait LsSyntaxGroup: Upcast<dyn ParserGroup> {
    /// Finds the most specific [`SyntaxNode`] at the given [`TextOffset`] in the file.
    fn find_syntax_node_at_offset(&self, file: FileId, offset: TextOffset) -> Option<SyntaxNode> {
        let db = self.upcast();
        Some(db.file_syntax(file).to_option()?.lookup_offset(db.upcast(), offset))
    }

    /// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
    fn find_syntax_node_at_position(
        &self,
        file: FileId,
        position: TextPosition,
    ) -> Option<SyntaxNode> {
        let db = self.upcast();
        Some(db.file_syntax(file).to_option()?.lookup_position(db.upcast(), position))
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
        &self,
        file: FileId,
        position: TextPosition,
    ) -> Option<TerminalIdentifier> {
        let db = self.upcast();
        let syntax_db = db.upcast();

        let find = |position: TextPosition| {
            let node = self.find_syntax_node_at_position(file, position)?;
            TerminalIdentifier::cast_token(syntax_db, node)
        };

        find(position).or_else(|| {
            // Try searching one character to the left.
            let col = position.col.checked_sub(1)?;
            find(TextPosition { col, ..position })
        })
    }
}

impl<T> LsSyntaxGroup for T where T: Upcast<dyn ParserGroup> + ?Sized {}
