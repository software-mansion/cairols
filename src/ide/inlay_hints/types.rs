use crate::{ide::ty::InferredValue, lang::db::AnalysisDatabase};
use cairo_lang_semantic::{GenericArgumentId, TypeId, TypeLongId};
use cairo_lang_syntax::node::{
    ast::{Expr, GenericArg, GenericArgValue, TerminalUnderscore, UnaryOperator},
    helpers::PathSegmentEx,
};
use cairo_lang_utils::LookupIntern;

// Find all `_` in provided type caluse.
pub fn find_underscores(
    db: &AnalysisDatabase,
    type_clause: Expr,
    ty: TypeId,
) -> Vec<(TerminalUnderscore, InferredValue)> {
    let mut result = vec![];

    find_underscores_ex(db, type_clause, ty, &mut result);

    result
}

fn find_underscores_ex(
    db: &AnalysisDatabase,
    type_clause: Expr,
    ty: TypeId,
    result: &mut Vec<(TerminalUnderscore, InferredValue)>,
) {
    match (type_clause, ty.lookup_intern(db)) {
        (Expr::Path(path), TypeLongId::Concrete(ty)) => {
            let ty_generics = ty.generic_args(db);

            for segment in path.segments(db).elements(db) {
                // Should happen only in last segment, but better safe than sorry.
                if let Some(generics) = segment.generic_args(db) {
                    for (generic, ty) in generics.into_iter().zip(ty_generics.clone()) {
                        let value = match generic {
                            GenericArg::Named(named) => named.value(db),
                            GenericArg::Unnamed(unnamed) => unnamed.value(db),
                        };

                        match value {
                            GenericArgValue::Underscore(underscore) => {
                                result.extend(
                                    InferredValue::try_from_generic_arg_id(ty)
                                        .map(|value| (underscore, value)),
                                );
                            }
                            GenericArgValue::Expr(expr) => {
                                if let GenericArgumentId::Type(ty) = ty {
                                    find_underscores_ex(db, expr.expr(db), ty, result);
                                }
                            }
                        }
                    }
                }
            }
        }
        (Expr::FixedSizeArray(fixed_size_array), TypeLongId::FixedSizeArray { type_id, .. }) => {
            for expr in fixed_size_array.exprs(db).elements(db) {
                find_underscores_ex(db, expr, type_id, result);
            }
        }
        (Expr::Unary(unary_syntax), TypeLongId::Snapshot(ty))
            if matches!(unary_syntax.op(db), UnaryOperator::At(_)) =>
        {
            find_underscores_ex(db, unary_syntax.expr(db), ty, result);
        }
        (Expr::Tuple(tuple_syntax), TypeLongId::Tuple(types)) => {
            for (expr, ty) in tuple_syntax.expressions(db).elements(db).into_iter().zip(types) {
                find_underscores_ex(db, expr, ty, result);
            }
        }
        _ => {}
    }
}
