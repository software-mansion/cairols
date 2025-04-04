use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{BinaryOperator, ExprBinary},
};
use if_chain::if_chain;

pub fn dot_expr_rhs(db: &AnalysisDatabase, node: &SyntaxNode) -> Option<ExprBinary> {
    if_chain!(
        if let Some(binary_expression) = node.ancestor_of_type::<ExprBinary>(db);
        if let BinaryOperator::Dot(_) = binary_expression.op(db);
        if node.is_descendant(db, &binary_expression.rhs(db).as_syntax_node());

        then {
            Some(binary_expression)
        } else {
            None
        }
    )
}
