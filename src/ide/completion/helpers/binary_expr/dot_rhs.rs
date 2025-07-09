use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{BinaryOperator, ExprBinary},
};

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
