use std::collections::{HashMap, HashSet};

use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::ExprPath;
use cairo_lang_syntax::node::helpers::GetIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_language_common::CommonGroup;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, Url, WorkspaceEdit};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::ToLsp;
use crate::lang::text_matching::text_matches;

pub fn suggest_similar_identifier<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    uri: &Url,
) -> Option<Vec<CodeAction>> {
    let typed_path_generic = ctx.node.ancestor_of_type::<ExprPath>(db)?;
    let typed_path_segments: Vec<_> = typed_path_generic
        .segments(db)
        .elements(db)
        .map(|e| e.identifier(db).to_string(db))
        .collect();

    db.get_node_resultants(typed_path_generic.as_syntax_node())?.iter().find_map(|resultant_node| {
        let resultant_expression_path = match resultant_node.kind(db) {
            SyntaxKind::ExprPath => ExprPath::from_syntax_node(db, *resultant_node),
            SyntaxKind::ExprPathInner => ExprPath::from_syntax_node(db, resultant_node.parent(db)?),
            _ => return None,
        };
        let expression_path_segments: Vec<_> = resultant_expression_path
            .segments(db)
            .as_syntax_node()
            .get_children(db)
            .iter()
            .filter(|node| node.kind(db) == SyntaxKind::PathSegmentSimple)
            .map(|segment| segment.get_text_without_trivia(db).to_string(db))
            .collect();

        if typed_path_segments != expression_path_segments {
            return None;
        }

        let expression_span =
            ctx.node.span(db).position_in_file(db, ctx.node.stable_ptr(db).file_id(db))?.to_lsp();

        let module_id = db.find_module_containing_node(*resultant_node)?;
        let items = db.visible_importables_from_module(module_id)?;

        let mut seen = HashSet::new();
        let suggestions: Vec<(String, TextEdit)> = items
            .iter()
            .filter_map(|(_item, proposed_path)| {
                segment_suggestion(proposed_path, &expression_path_segments).map(|suggestion| {
                    let edit =
                        TextEdit { range: expression_span, new_text: suggestion.to_string() };
                    (suggestion.to_string(), edit)
                })
            })
            .filter(|(suggestion, _)| seen.insert(suggestion.to_owned()))
            .collect();

        let code_actions = suggestions
            .into_iter()
            .map(|(identifier, edit)| CodeAction {
                title: format!("Did you mean `{identifier}`?"),
                kind: Some(CodeActionKind::QUICKFIX),
                edit: Some(WorkspaceEdit {
                    changes: Some(HashMap::from([(uri.clone(), vec![edit])])),
                    document_changes: None,
                    change_annotations: None,
                }),
                ..Default::default()
            })
            .collect();

        Some(code_actions)
    })
}

fn segment_suggestion<'a>(proposed_path: &'a str, typed_segments: &[String]) -> Option<&'a str> {
    let proposed_segments: Vec<_> = proposed_path.split("::").collect();

    if proposed_segments.len() != typed_segments.len() {
        return None;
    }

    proposed_segments
        .iter()
        .zip(typed_segments)
        .find(|(p, t)| *p != *t)
        .and_then(|(p, t)| text_matches(p, t).then_some(*p))
}
