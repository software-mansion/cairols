pub mod binary_expr;
pub mod completion_kind;
pub mod formatting;
pub mod item;
pub mod snippets;
pub(crate) mod span;

use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::ast::Statement;
use cairo_lang_syntax::node::kind::SyntaxKind;

use crate::lang::db::AnalysisDatabase;

/// Returns `true` when `node` sits directly in a statement/block context with no expression typed
/// yet — i.e. the node itself is an [`ExprBlock`] or a [`Statement`] variant, or its parent is.
pub fn is_empty_body_context<'db>(db: &'db AnalysisDatabase, node: &SyntaxNode<'db>) -> bool {
    node.kind(db) == SyntaxKind::ExprBlock
        || Statement::is_variant(node.kind(db))
        || node.parent(db).is_some_and(|p| p.kind(db) == SyntaxKind::ExprBlock)
        || node.parent(db).is_some_and(|p| Statement::is_variant(p.kind(db)))
}
