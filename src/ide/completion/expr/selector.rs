use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{self, Expr, ExprPath},
};

pub fn expr_selector(db: &AnalysisDatabase, node: &SyntaxNode) -> Option<ExprPath> {
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
