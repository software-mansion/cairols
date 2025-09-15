use cairo_lang_defs::ids::{ConstantLongId, FunctionWithBodyId, ImportableId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::constant::ConstantSemantic;
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::{TypeId, TypeLongId};
use cairo_lang_syntax::node::ast::{
    Expr, ItemConstant, TerminalLiteralNumber, TerminalShortString, TerminalString,
};
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lang_utils::{Intern, Upcast};
use indoc::formatdoc;

use crate::ide::format::types::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Narrows down [`SyntaxNode`] to [`TerminalLiteralNumber`], [`TerminalString`] or
/// [`TerminalShortString`] if it represents some literal
/// and renders a hover containing its value and type, returns None otherwise.
#[tracing::instrument(level = "trace", skip_all)]
pub fn literal<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    node.ancestors_with_self(db)
        .filter_map(|node| TerminalLiteralNumber::cast(db, node))
        .filter_map(|literal| {
            let ty = find_type(db, literal.as_syntax_node())?;

            number_hover(db, &literal, &ty, importables)
        })
        .next()
        .or_else(|| {
            node.ancestors_with_self(db)
                .filter_map(|node| TerminalString::cast(db, node))
                .filter_map(|literal| {
                    let ty = find_type(db, literal.as_syntax_node())?;

                    string_hover(db, &ty, importables)
                })
                .next()
        })
        .or_else(|| {
            node.ancestors_with_self(db)
                .filter_map(|node| TerminalShortString::cast(db, node))
                .filter_map(|literal| {
                    let ty = find_type(db, literal.as_syntax_node())?;

                    short_string_hover(db, &literal, ty, importables)
                })
                .next()
        })
}

/// Gets the type of an expression associated with [`SyntaxNode`].
fn find_type<'db>(db: &'db AnalysisDatabase, node: SyntaxNode<'db>) -> Option<TypeId<'db>> {
    if let Some(function_id) = db.find_lookup_item(node)?.function_with_body() {
        find_type_in_function_context(db, node, function_id)
    } else {
        find_type_in_const_declaration(db, node)
    }
}

/// Gets the type of an expression associated with [`SyntaxNode`] assuming it's defined in the
/// context of function.
fn find_type_in_function_context<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    function_id: FunctionWithBodyId<'db>,
) -> Option<TypeId<'db>> {
    let expr = Expr::from_syntax_node(db, node);
    let expr_id = db.lookup_expr_by_ptr(function_id, expr.stable_ptr(db)).ok()?;
    let semantic_db: &dyn SemanticGroup = db.upcast();
    let type_id = semantic_db.expr_semantic(function_id, expr_id).ty();

    Some(type_id)
}

/// Gets the type of an expression associated with [`SyntaxNode`] assuming it's a const item.
fn find_type_in_const_declaration<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> Option<TypeId<'db>> {
    let module_file_id = db.find_module_file_containing_node(node)?;
    let const_item = node.ancestor_of_type::<ItemConstant>(db)?;
    let const_item_id = ConstantLongId(module_file_id, const_item.stable_ptr(db)).intern(db);
    let type_id = db.constant_semantic_data(const_item_id).ok()?.ty();

    Some(type_id)
}

fn is_type_missing(type_id: &TypeId, db: &AnalysisDatabase) -> bool {
    matches!(type_id.long(db), TypeLongId::Missing(_))
}

/// Formats the number literal writing its decimal, hexadecimal and binary value and type.
fn number_hover<'db>(
    db: &'db AnalysisDatabase,
    literal: &TerminalLiteralNumber<'db>,
    type_id: &TypeId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    let value = literal.numeric_value(db)?;

    let mut representation = formatdoc!("value of literal: `{value} ({value:#x} | {value:#b})`");

    if !is_type_missing(type_id, db) {
        let ty = format_type(db, *type_id, importables);
        representation = formatdoc!(
            "
            ```cairo
            {ty}
            ```
            ---
            {representation}
            "
        );
    }

    Some(representation)
}

/// Formats the number literal writing it along with the `core::byte_array::ByteArray` type.
fn string_hover<'db>(
    db: &'db AnalysisDatabase,
    type_id: &TypeId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    if is_type_missing(type_id, db) {
        None
    } else {
        let ty = format_type(db, *type_id, importables);
        Some(formatdoc!(
            "
            ```cairo
            {ty}
            ```
            "
        ))
    }
}

/// Formats the short string literal writing its textual and numeric value along with the
/// `core::felt252` type.
fn short_string_hover<'db>(
    db: &'db AnalysisDatabase,
    literal: &TerminalShortString<'db>,
    type_id: TypeId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> Option<String> {
    let mut representation = match (literal.numeric_value(db), literal.string_value(db)) {
        (None, _) => None,
        (Some(numeric), None) => Some(formatdoc!("value of literal: `{numeric:#x}`")),
        (Some(numeric), Some(string)) => {
            Some(formatdoc!("value of literal: `'{string}' ({numeric:#x})`"))
        }
    }?;

    if !is_type_missing(&type_id, db) {
        let ty = format_type(db, type_id, importables);
        representation = formatdoc!(
            "
            ```cairo
            {ty}
            ```
            ---
            {representation}
            "
        );
    }

    Some(representation)
}
