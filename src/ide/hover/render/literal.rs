use cairo_lang_defs::ids::{ConstantLongId, FunctionWithBodyId, ImportableId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::ast::{
    Expr, ItemConstant, TerminalLiteralNumber, TerminalShortString, TerminalString,
};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::Intern;
use indoc::formatdoc;

use crate::ide::ty::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

/// Narrows down [`SyntaxNode`] to [`TerminalLiteralNumber`], [`TerminalString`] or
/// [`TerminalShortString`] if it represents some literal
/// and renders a hover containing its value and type, returns None otherwise.
#[tracing::instrument(level = "trace", skip_all)]
pub fn literal(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    node.ancestors_with_self(db)
        .filter_map(|node| TerminalLiteralNumber::cast(db, node))
        .filter_map(|literal| {
            let ty = find_type(db, literal.as_syntax_node(), importables)?;

            number_hover(db, &literal, &ty)
        })
        .next()
        .or_else(|| {
            node.ancestors_with_self(db)
                .filter_map(|node| TerminalString::cast(db, node))
                .filter_map(|literal| {
                    let ty = find_type(db, literal.as_syntax_node(), importables)?;

                    string_hover(&ty)
                })
                .next()
        })
        .or_else(|| {
            node.ancestors_with_self(db)
                .filter_map(|node| TerminalShortString::cast(db, node))
                .filter_map(|literal| {
                    let ty = find_type(db, literal.as_syntax_node(), importables)?;

                    short_string_hover(db, &literal, &ty)
                })
                .next()
        })
}

/// Gets the type of an expression associated with [`SyntaxNode`].
fn find_type(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    if let Some(function_id) = db.find_lookup_item(&node)?.function_with_body() {
        find_type_in_function_context(db, node, function_id, importables)
    } else {
        find_type_in_const_declaration(db, node, importables)
    }
}

/// Gets the type of an expression associated with [`SyntaxNode`] assuming it's defined in the
/// context of function.
fn find_type_in_function_context(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    function_id: FunctionWithBodyId,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let expr = Expr::from_syntax_node(db, node);
    let expr_id = db.lookup_expr_by_ptr(function_id, expr.stable_ptr(db)).ok()?;
    Some(format_type(db, db.expr_semantic(function_id, expr_id).ty(), importables))
}

/// Gets the type of an expression associated with [`SyntaxNode`] assuming it's a const item.
fn find_type_in_const_declaration(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<String> {
    let module_file_id = db.find_module_file_containing_node(&node)?;

    let const_item = node.ancestor_of_type::<ItemConstant>(db)?;
    let const_item_id = ConstantLongId(module_file_id, const_item.stable_ptr(db)).intern(db);

    Some(format_type(db, db.constant_const_type(const_item_id).ok()?, importables))
}

/// Formats the number literal writing its decimal, hexadecimal and binary value and type.
fn number_hover(
    db: &AnalysisDatabase,
    literal: &TerminalLiteralNumber,
    ty: &str,
) -> Option<String> {
    let value = literal.numeric_value(db)?;

    let representation = formatdoc!(
        "
        ```cairo
        {ty}
        ```
        ---
        value of literal: `{value} ({value:#x} | {value:#b})`
        "
    );

    Some(representation)
}

/// Formats the number literal writing it along with the `core::byte_array::ByteArray` type.
fn string_hover(ty: &str) -> Option<String> {
    let representation = formatdoc!(
        r#"
        ```cairo
        {ty}
        ```
        "#
    );

    Some(representation)
}

/// Formats the short string literal writing its textual and numeric value along with the
/// `core::felt252` type.
fn short_string_hover(
    db: &AnalysisDatabase,
    literal: &TerminalShortString,
    ty: &str,
) -> Option<String> {
    let representation = match (literal.numeric_value(db), literal.string_value(db)) {
        (None, _) => None,
        (Some(numeric), None) => Some(formatdoc!(
            "
            ```cairo
            {ty}
            ```
            ---
            value of literal: `{numeric:#x}`
            "
        )),
        (Some(numeric), Some(string)) => Some(formatdoc!(
            "
            ```cairo
            {ty}
            ```
            ---
            value of literal: `'{string}' ({numeric:#x})`
            "
        )),
    }?;

    Some(representation)
}
