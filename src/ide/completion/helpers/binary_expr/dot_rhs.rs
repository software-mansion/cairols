use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{BinaryOperator, ExprBinary},
};

use crate::lang::db::AnalysisDatabase;

pub fn dot_expr_rhs(db: &AnalysisDatabase, node: &SyntaxNode) -> Option<ExprBinary> {
    if let Some(binary_expression) = node.ancestor_of_type::<ExprBinary>(db)
        && let BinaryOperator::Dot(_) = binary_expression.op(db)
        && node.is_descendant(db, &binary_expression.rhs(db).as_syntax_node())
    {
        Some(binary_expression)
    } else {
        None
    }
}
