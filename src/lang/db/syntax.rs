use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextOffset, TextPosition, TextSpan};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{SyntaxNode, Terminal};
use cairo_lang_utils::Upcast;

// TODO(mkaput): Make this a real Salsa query group with sensible LRU.
/// LS-specific extensions to the syntax group of the Cairo compiler.
#[cairo_lang_proc_macros::query_group(LsSyntaxDatabase)]
pub trait LsSyntaxGroup: ParserGroup + for<'db> Upcast<'db, dyn ParserGroup> {
    /// Finds the most specific [`SyntaxNode`] at the given [`TextOffset`] in the file.
    fn find_syntax_node_at_offset<'db>(
        &'db self,
        file: FileId<'db>,
        offset: TextOffset,
    ) -> Option<SyntaxNode<'db>>;

    /// Finds the widest [`SyntaxNode`] within the given [`TextSpan`] in the file.
    fn widest_node_within_span<'db>(
        &'db self,
        file: FileId<'db>,
        span: TextSpan,
    ) -> Option<SyntaxNode<'db>>;

    /// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
    fn find_syntax_node_at_position<'db>(
        &'db self,
        file: FileId<'db>,
        position: TextPosition,
    ) -> Option<SyntaxNode<'db>>;

    /// Finds a [`TerminalIdentifier`] at the given [`TextPosition`] in the file.
    ///
    /// The lookup for identifiers is slightly more sophisticated than just looking for an arbitrary
    /// syntax node because identifiers usually are what the user is interested in.
    /// In case when the user position is `ident<caret>()`, while regular syntax node lookup would
    /// return the left paren, a much better UX would be to correct the lookup to the identifier.
    /// Such corrections are always valid and deterministic, because grammar-wise it is not possible
    /// to have two identifiers/keywords being glued to each other.
    fn find_identifier_at_position<'db>(
        &'db self,
        file: FileId<'db>,
        position: TextPosition,
    ) -> Option<TerminalIdentifier<'db>>;
}

/// Finds the most specific [`SyntaxNode`] at the given [`TextOffset`] in the file.
fn find_syntax_node_at_offset<'db>(
    db: &'db dyn LsSyntaxGroup,
    file: FileId<'db>,
    offset: TextOffset,
) -> Option<SyntaxNode<'db>> {
    Some(db.file_syntax(file).to_option()?.lookup_offset(db, offset))
}

/// Finds the widest [`SyntaxNode`] within the given [`TextSpan`] in the file.
fn widest_node_within_span<'db>(
    db: &'db dyn LsSyntaxGroup,
    file: FileId<'db>,
    span: TextSpan,
) -> Option<SyntaxNode<'db>> {
    let precise_node = db.find_syntax_node_at_offset(file, span.start)?;

    precise_node
        .ancestors_with_self(db)
        .take_while(|new_node| span.contains(new_node.span(db)))
        .last()
}

/// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
fn find_syntax_node_at_position<'db>(
    db: &'db dyn LsSyntaxGroup,
    file: FileId<'db>,
    position: TextPosition,
) -> Option<SyntaxNode<'db>> {
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
fn find_identifier_at_position<'db>(
    db: &'db dyn LsSyntaxGroup,
    file: FileId<'db>,
    position: TextPosition,
) -> Option<TerminalIdentifier<'db>> {
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

pub trait SyntaxNodeExt {
    /// Faster than [`SyntaxNode::tokens`] because we don't travel each leaf, and does not allocate.
    fn for_each_terminal<'db>(
        &self,
        db: &'db dyn SyntaxGroup,
        callback: impl FnMut(&SyntaxNode<'db>),
    ) where
        Self: 'db;
}

impl<'a> SyntaxNodeExt for SyntaxNode<'a> {
    fn for_each_terminal<'db>(
        &self,
        db: &'db dyn SyntaxGroup,
        mut callback: impl FnMut(&SyntaxNode<'db>),
    ) where
        Self: 'db,
    {
        for_each_terminals_ex(self, db, &mut callback)
    }
}

fn for_each_terminals_ex<'db>(
    node: &SyntaxNode<'db>,
    db: &'db dyn SyntaxGroup,
    callback: &mut impl FnMut(&SyntaxNode<'db>),
) {
    if node.green_node(db).kind.is_terminal() {
        callback(node);
        return;
    }

    for child in node.get_children(db) {
        for_each_terminals_ex(child, db, callback);
    }
}
