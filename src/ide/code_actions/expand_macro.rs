use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::kind::SyntaxKind;
use lsp_types::{CodeAction, Command};

use crate::lang::db::AnalysisDatabase;

/// Code actions for macro expansion.
pub fn expand_macro(db: &AnalysisDatabase, node: SyntaxNode) -> Vec<CodeAction> {
    let mut result = vec![];
    let command = "cairo.expandMacro".to_owned();

    if node.ancestor_of_kind(db, SyntaxKind::ExprInlineMacro).is_some() {
        let title = "Expand macro recursively at caret".to_owned();

        result.push(CodeAction {
            title: title.clone(),
            command: Some(Command { title, command, ..Default::default() }),
            ..Default::default()
        });
    } else if node.ancestor_of_kind(db, SyntaxKind::Attribute).is_some() {
        let title = "Recursively expand macros for item at caret".to_owned();

        result.push(CodeAction {
            title: title.clone(),
            command: Some(Command { title, command, ..Default::default() }),
            ..Default::default()
        });
    }

    result
}
