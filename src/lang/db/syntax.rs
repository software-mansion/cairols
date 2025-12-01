use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::{TextPosition, TextSpan};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::ast::TerminalIdentifier;
use cairo_lang_syntax::node::{SyntaxNode, Terminal};
use cairo_language_common::CommonGroup;
use salsa::Database;

/// LS-specific extensions to the syntax group of the Cairo compiler.
pub trait LsSyntaxGroup: Database {
    /// Finds the widest [`SyntaxNode`] within the given [`TextSpan`] in the file.
    fn widest_node_within_span<'db>(
        &'db self,
        file: FileId<'db>,
        span: TextSpan,
    ) -> Option<SyntaxNode<'db>> {
        widest_node_within_span(self.as_dyn_database(), file, span)
    }

    /// Like [`LsSyntaxGroup::widest_node_within_span`] but uses [`SyntaxNode::span_without_trivia`] instead of [`SyntaxNode::span`]
    fn widest_node_within_span_without_trivia<'db>(
        &'db self,
        file: FileId<'db>,
        span: TextSpan,
    ) -> Option<SyntaxNode<'db>> {
        widest_node_within_ex(self.as_dyn_database(), file, span, |db, node| {
            node.span_without_trivia(db)
        })
    }

    /// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
    fn find_syntax_node_at_position<'db>(
        &'db self,
        file: FileId<'db>,
        position: TextPosition,
    ) -> Option<SyntaxNode<'db>> {
        find_syntax_node_at_position(self.as_dyn_database(), file, position)
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
        &'db self,
        file: FileId<'db>,
        position: TextPosition,
    ) -> Option<TerminalIdentifier<'db>> {
        find_identifier_at_position(self.as_dyn_database(), file, position)
    }
}

impl<T: Database + ?Sized> LsSyntaxGroup for T {}

/// Finds the widest [`SyntaxNode`] within the given [`TextSpan`] in the file.
#[salsa::tracked]
fn widest_node_within_span<'db>(
    db: &'db dyn Database,
    file: FileId<'db>,
    span: TextSpan,
) -> Option<SyntaxNode<'db>> {
    widest_node_within_ex(db, file, span, |db, node| node.span(db))
}

fn widest_node_within_ex<'db>(
    db: &'db dyn Database,
    file: FileId<'db>,
    span: TextSpan,
    obtain_span: fn(&'db dyn Database, &SyntaxNode<'db>) -> TextSpan,
) -> Option<SyntaxNode<'db>> {
    let precise_node = db.find_syntax_node_at_offset(file, span.start)?;

    let nodes: Vec<_> = precise_node
        .ancestors_with_self(db)
        .take_while(|new_node| span.contains(obtain_span(db, new_node)))
        .collect();

    let last_node = nodes.last().cloned()?;
    let last_node_span = obtain_span(db, &last_node);

    nodes.into_iter().rev().take_while(|node| obtain_span(db, node) == last_node_span).last()
}

/// Finds the most specific [`SyntaxNode`] at the given [`TextPosition`] in the file.
#[salsa::tracked]
fn find_syntax_node_at_position<'db>(
    db: &'db dyn Database,
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
#[salsa::tracked]
fn find_identifier_at_position<'db>(
    db: &'db dyn Database,
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
