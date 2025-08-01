use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{self, Expr, ExprPath},
};

use crate::lang::db::AnalysisDatabase;

pub fn expr_selector<'db>(
    db: &'db AnalysisDatabase,
    node: &SyntaxNode<'db>,
) -> Option<ExprPath<'db>> {
    for node in node.ancestors_with_self(db) {
        if ast::Statement::is_variant(node.kind(db)) {
            return None;
        }

        if let Some(expr) = Expr::cast(db, node) {
            // In other cases there is nothing we can complete.
            return if let Expr::Path(path) = expr { Some(path) } else { None };
        }
    }

    None
}
