use std::collections::HashMap;

use cairo_lang_syntax::node::{
    SyntaxNode, TypedSyntaxNode,
    ast::{self, TerminalIdentifier},
};
use cairo_language_common::CommonGroup;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, Url, WorkspaceEdit};

use crate::lang::{db::AnalysisDatabase, defs::SymbolSearch, lsp::ToLsp};

pub fn make_variable_mutable<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    uri: Url,
) -> Option<CodeAction> {
    db.get_node_resultants(node)?.iter().find_map(|resultant_node| {
        let expr_binary = resultant_node.ancestor_of_type::<ast::ExprBinary>(db)?;
        let lhs_syntax_node = expr_binary.lhs(db).as_syntax_node();
        let token = lhs_syntax_node.tokens(db).next()?;

        let identifier =
            token.ancestors_with_self(db).find_map(|node| TerminalIdentifier::cast(db, node))?;
        get_mutable_variable_action(db, identifier, uri.clone())
    })
}

pub fn make_ref_variable_mutable<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    uri: Url,
) -> Option<CodeAction> {
    db.get_node_resultants(node)?.iter().find_map(|resultant_node| {
        let identifier = resultant_node
            .ancestors_with_self(db)
            .find_map(|node| TerminalIdentifier::cast(db, node))?;
        get_mutable_variable_action(db, identifier, uri.clone())
    })
}

fn get_mutable_variable_action<'db>(
    db: &'db AnalysisDatabase,
    identifier: TerminalIdentifier<'db>,
    uri: Url,
) -> Option<CodeAction> {
    let definition_location =
        SymbolSearch::find_definition(db, &identifier)?.def.definition_originating_location(db)?;
    let definition =
        db.find_syntax_node_at_offset(definition_location.file_id, definition_location.span.start)?;

    let span_to_edit =
        definition.span_without_trivia(db).position_in_file(db, definition.file_id(db))?;

    let edits = vec![TextEdit {
        range: span_to_edit.to_lsp(),
        new_text: format!("mut {}", definition.get_text_without_trivia(db).to_string(db)),
    }];

    Some(CodeAction {
        title: format!(
            "Change \"{}\" to be mutable",
            definition.get_text_without_trivia(db).to_string(db)
        ),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            changes: Some(HashMap::from([(uri.clone(), edits)])),
            document_changes: None,
            change_annotations: None,
        }),
        ..Default::default()
    })
}
