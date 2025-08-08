use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{BinaryOperator, ExprBinary},
};

use crate::lang::db::AnalysisDatabase;

pub fn dot_expr_rhs<'db>(
    db: &'db AnalysisDatabase,
    node: &SyntaxNode<'db>,
    has_node_switched: bool,
) -> Option<ExprBinary<'db>> {
    if let Some(binary_expression) = node.ancestor_of_type::<ExprBinary>(db)
        && let BinaryOperator::Dot(_) = binary_expression.op(db)
        && (node.is_descendant_or_self(db, &binary_expression.rhs(db).as_syntax_node())
            || node.is_descendant_or_self(db, &binary_expression.op(db).as_syntax_node())
            || (has_node_switched
                && node.is_descendant_or_self(db, &binary_expression.lhs(db).as_syntax_node())))
    {
        Some(binary_expression)
    } else {
        None
    }
}
