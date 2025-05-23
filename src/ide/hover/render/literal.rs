use cairo_lang_defs::ids::{ConstantLongId, FunctionWithBodyId, ImportableId};
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_semantic::TypeLongId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_syntax::node::ast::{
    Expr, ItemConstant, TerminalLiteralNumber, TerminalShortString, TerminalString,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::{Intern, LookupIntern};
use indoc::formatdoc;
use lsp_types::Hover;

use crate::ide::hover::markdown_contents;
use crate::ide::ty::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::lsp::ToLsp;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

/// Narrows down [`SyntaxNode`] to [`TerminalLiteralNumber`], [`TerminalString`] or
/// [`TerminalShortString`] if it represents some literal
/// and renders a hover containing its value and type, returns None otherwise.
#[tracing::instrument(level = "trace", skip_all)]
pub fn literal(
    db: &AnalysisDatabase,
    node: &SyntaxNode,
    file_id: FileId,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<Hover> {
    match node.kind(db) {
        SyntaxKind::TokenLiteralNumber => {
            let parent = node.parent(db)?;
            let literal = TerminalLiteralNumber::from_syntax_node(db, parent);
            let ty = find_type(db, parent, importables)?;
            number_hover(db, &literal, &ty, file_id)
        }
        SyntaxKind::TokenString => {
            let parent = node.parent(db)?;
            let literal = TerminalString::from_syntax_node(db, parent);
            let ty = find_type(db, parent, importables)?;
            string_hover(db, &literal, &ty, file_id)
        }
        SyntaxKind::TokenShortString => {
            let parent = node.parent(db)?;
            let literal = TerminalShortString::from_syntax_node(db, parent);
            let ty = find_type(db, parent, importables)?;
            short_string_hover(db, &literal, &ty, file_id)
        }
        _ => None,
    }
}

/// Gets the type of an expression associated with [`SyntaxNode`].
fn find_type(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<HoverType> {
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
) -> Option<HoverType> {
    let expr = Expr::from_syntax_node(db, node);
    let expr_id = db.lookup_expr_by_ptr(function_id, expr.stable_ptr(db)).ok()?;
    let type_id = db.expr_semantic(function_id, expr_id).ty();
    let formatted = format_type(db, type_id, importables);

    Some(HoverType { ty: type_id.lookup_intern(db), formatted })
}

/// Gets the type of an expression associated with [`SyntaxNode`] assuming it's a const item.
fn find_type_in_const_declaration(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    importables: &OrderedHashMap<ImportableId, String>,
) -> Option<HoverType> {
    let module_file_id = db.find_module_file_containing_node(&node)?;

    let const_item = node.ancestor_of_type::<ItemConstant>(db)?;
    let const_item_id = ConstantLongId(module_file_id, const_item.stable_ptr(db)).intern(db);

    let type_id = db.constant_const_type(const_item_id).ok()?;
    let formatted = format_type(db, type_id, importables);

    Some(HoverType { ty: type_id.lookup_intern(db), formatted })
}

struct HoverType {
    ty: TypeLongId,
    formatted: String,
}

impl HoverType {
    fn is_missing(&self) -> bool {
        matches!(&self.ty, TypeLongId::Missing(_))
    }
}

/// Formats the number literal writing its decimal, hexadecimal and binary value and type.
fn number_hover(
    db: &AnalysisDatabase,
    literal: &TerminalLiteralNumber,
    hover_type: &HoverType,
    file_id: FileId,
) -> Option<Hover> {
    let value = literal.numeric_value(db)?;

    let mut representation = formatdoc!("value of literal: `{value} ({value:#x} | {value:#b})`");

    if !hover_type.is_missing() {
        let ty = hover_type.formatted.clone();
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

    Some(Hover {
        contents: markdown_contents(representation),
        range: literal
            .as_syntax_node()
            .span_without_trivia(db)
            .position_in_file(db, file_id)
            .map(|position| position.to_lsp()),
    })
}

/// Formats the number literal writing it along with the `core::byte_array::ByteArray` type.
fn string_hover(
    db: &AnalysisDatabase,
    literal: &TerminalString,
    hover_type: &HoverType,
    file_id: FileId,
) -> Option<Hover> {
    if hover_type.is_missing() {
        None
    } else {
        let ty = hover_type.formatted.clone();
        let representation = formatdoc!(
            "
            ```cairo
            {ty}
            ```
            "
        );

        Some(Hover {
            contents: markdown_contents(representation),
            range: literal
                .as_syntax_node()
                .span_without_trivia(db)
                .position_in_file(db, file_id)
                .map(|position| position.to_lsp()),
        })
    }
}

/// Formats the short string literal writing its textual and numeric value along with the
/// `core::felt252` type.
fn short_string_hover(
    db: &AnalysisDatabase,
    literal: &TerminalShortString,
    hover_type: &HoverType,
    file_id: FileId,
) -> Option<Hover> {
    let mut representation = match (literal.numeric_value(db), literal.string_value(db)) {
        (None, _) => None,
        (Some(numeric), None) => Some(formatdoc!("value of literal: `{numeric:#x}`")),
        (Some(numeric), Some(string)) => {
            Some(formatdoc!("value of literal: `'{string}' ({numeric:#x})`"))
        }
    }?;

    if !hover_type.is_missing() {
        let ty = hover_type.formatted.clone();
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

    Some(Hover {
        contents: markdown_contents(representation),
        range: literal
            .as_syntax_node()
            .span_without_trivia(db)
            .position_in_file(db, file_id)
            .map(|position| position.to_lsp()),
    })
}
