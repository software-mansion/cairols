use std::collections::HashMap;

use cairo_lang_syntax::node::ast::PatternIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Token, TypedSyntaxNode};
use lsp_types::{CodeAction, CodeActionKind, Diagnostic, Range, TextEdit, Url, WorkspaceEdit};

use crate::lang::db::AnalysisDatabase;

/// Create a code action that prefixes an unused variable with an `_`.
pub fn rename_unused_variable(
    db: &AnalysisDatabase,
    node: &SyntaxNode,
    diagnostic: Diagnostic,
    uri: Url,
) -> Option<CodeAction> {
    let var_name = if let Some(node) = node.ancestor_of_kind(db, SyntaxKind::PathSegmentSimple)
        && let Some(path) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(_) = path.parent_of_kind(db, SyntaxKind::StatementLet)
    {
        path.get_text(db)
    } else if let Some(ident) = node.ancestor_of_type::<PatternIdentifier>(db)
        && let Some(_) = ident.as_syntax_node().parent_of_kind(db, SyntaxKind::StatementLet)
    {
        ident.name(db).token(db).text(db).to_string()
    } else {
        return None;
    };

    let var_name = var_name.trim();

    Some(CodeAction {
        title: format!("Rename to `_{var_name}`"),
        kind: Some(CodeActionKind::QUICKFIX),
        is_preferred: Some(true),
        edit: Some(WorkspaceEdit {
            changes: Some(HashMap::from_iter([(
                uri,
                vec![TextEdit {
                    range: Range { start: diagnostic.range.start, end: diagnostic.range.start },
                    new_text: "_".to_owned(),
                }],
            )])),
            document_changes: None,
            change_annotations: None,
        }),
        diagnostics: Some(vec![diagnostic]),
        ..Default::default()
    })
}
