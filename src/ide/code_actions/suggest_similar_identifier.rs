use std::collections::{HashMap, HashSet};

use cairo_lang_defs::ids::{ImportableId, ModuleId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::diagnostic::{NotFoundItemType, SemanticDiagnosticKind};
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
        let expected_item_type = expected_item_type(db, module_id, &resultant_expression_path);

        let mut seen = HashSet::new();
        let suggestions: Vec<(String, TextEdit)> = items
            .iter()
            .filter(|(item, _)| {
                expected_item_type.is_none_or(|item_type| matches_item_type(item, item_type))
            })
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

/// Returns the kind of item the compiler expected at `path`, taken from its "path not found"
/// diagnostic.
fn expected_item_type<'db>(
    db: &'db AnalysisDatabase,
    module_id: ModuleId<'db>,
    path: &ExprPath<'db>,
) -> Option<NotFoundItemType> {
    let path_node = path.as_syntax_node();
    let diagnostics = db.module_semantic_diagnostics(module_id).ok()?;

    diagnostics.get_all().into_iter().find_map(|diagnostic| {
        let SemanticDiagnosticKind::PathNotFound(item_type) = diagnostic.kind else {
            return None;
        };

        let diagnostic_node = diagnostic.stable_location.syntax_node(db);
        diagnostic_node.is_descendant_or_self(db, &path_node).then_some(item_type)
    })
}

fn matches_item_type(item: &ImportableId, item_type: NotFoundItemType) -> bool {
    match item_type {
        NotFoundItemType::Function => matches!(
            item,
            ImportableId::FreeFunction(_)
                | ImportableId::ExternFunction(_)
                | ImportableId::Variant(_)
        ),
        NotFoundItemType::Type => matches!(
            item,
            ImportableId::Struct(_)
                | ImportableId::Enum(_)
                | ImportableId::ExternType(_)
                | ImportableId::TypeAlias(_)
        ),
        NotFoundItemType::Trait => matches!(item, ImportableId::Trait(_)),
        NotFoundItemType::Impl => {
            matches!(item, ImportableId::Impl(_) | ImportableId::ImplAlias(_))
        }
        NotFoundItemType::Macro => matches!(item, ImportableId::MacroDeclaration(_)),
        NotFoundItemType::Identifier => true,
    }
}
