use std::collections::HashMap;

use cairo_lang_syntax::node::ast::PatternIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Token, TypedSyntaxNode};
use if_chain::if_chain;
use lsp_types::{CodeAction, CodeActionKind, Diagnostic, TextEdit, Url, WorkspaceEdit};

use crate::lang::db::AnalysisDatabase;

/// Create a code action that prefixes an unused variable with an `_`.
pub fn rename_unused_variable(
    db: &AnalysisDatabase,
    node: &SyntaxNode,
    diagnostic: Diagnostic,
    uri: Url,
) -> Option<CodeAction> {
    let mut var_name = None;

    if_chain!(
        if let Some(node) = node.ancestor_of_kind(db, SyntaxKind::PathSegmentSimple);
        if let Some(path) = node.parent_of_kind(db, SyntaxKind::ExprPath);
        if let Some(_) = path.parent_of_kind(db, SyntaxKind::StatementLet);

        then {
           var_name = Some(path.get_text(db));
        }
    );

    if_chain!(
        if var_name.is_none();
        if let Some(ident) = node.ancestor_of_type::<PatternIdentifier>(db);
        if let Some(_) = ident.as_syntax_node().parent_of_kind(db, SyntaxKind::StatementLet);

        then {
            var_name = Some(ident.name(db).token(db).text(db).to_string());
        }
    );

    let var_name = var_name?;
    let var_name = var_name.trim();

    Some(CodeAction {
        title: format!("Rename to `_{var_name}`"),
        kind: Some(CodeActionKind::QUICKFIX),
        is_preferred: Some(true),
        edit: Some(WorkspaceEdit {
            changes: Some(HashMap::from_iter([(
                uri,
                // The diagnostic range is just the first char of the variable name, so we can just
                // pass an underscore as the new text it won't replace the current variable name,
                // and it will prefix it with `_`
                vec![TextEdit { range: diagnostic.range, new_text: "_".to_owned() }],
            )])),
            document_changes: None,
            change_annotations: None,
        }),
        diagnostics: Some(vec![diagnostic]),
        ..Default::default()
    })
}
