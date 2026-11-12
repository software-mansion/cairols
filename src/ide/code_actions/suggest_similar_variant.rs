use std::collections::HashMap;

use cairo_lang_defs::ids::{EnumId, ModuleId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::diagnostic::SemanticDiagnosticKind;
use cairo_lang_semantic::items::enm::EnumSemantic;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_language_common::CommonGroup;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, Url, WorkspaceEdit};

use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::ToLsp;
use crate::lang::text_matching::text_matches;

pub fn suggest_similar_variant<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    uri: Url,
) -> Option<Vec<CodeAction>> {
    db.get_node_resultants(ctx.node)?.iter().find_map(|resultant_node| {
        let module_id = db.find_module_containing_node(*resultant_node)?;
        let enum_id = enum_without_variant(db, module_id, resultant_node)?;

        let typed_variant_name = ctx.node.get_text_without_trivia(db).to_string(db);

        let suggestions: Vec<String> = db
            .enum_variants(enum_id)
            .ok()?
            .keys()
            .map(|name| name.to_string(db))
            .filter(|name| text_matches(name, &typed_variant_name))
            .collect();

        if suggestions.is_empty() {
            return None;
        }

        let bad_variant_span = ctx
            .node
            .span_without_trivia(db)
            .position_in_file(db, ctx.node.stable_ptr(db).file_id(db))?;

        let code_actions = suggestions
            .into_iter()
            .map(|variant_name| {
                let edits = vec![TextEdit {
                    range: bad_variant_span.to_lsp(),
                    new_text: variant_name.clone(),
                }];

                CodeAction {
                    title: format!("Did you mean `{variant_name}`?"),
                    kind: Some(CodeActionKind::QUICKFIX),
                    edit: Some(WorkspaceEdit {
                        changes: Some(HashMap::from([(uri.clone(), edits)])),
                        document_changes: None,
                        change_annotations: None,
                    }),
                    ..Default::default()
                }
            })
            .collect();

        Some(code_actions)
    })
}

/// Returns the enum from the "no such variant" diagnostic reported at `node`.
fn enum_without_variant<'db>(
    db: &'db AnalysisDatabase,
    module_id: ModuleId<'db>,
    node: &SyntaxNode<'db>,
) -> Option<EnumId<'db>> {
    let diagnostics = db.module_semantic_diagnostics(module_id).ok()?;

    diagnostics.get_all().into_iter().find_map(|diagnostic| {
        let SemanticDiagnosticKind::NoSuchVariant { enum_id, .. } = diagnostic.kind else {
            return None;
        };

        let diagnostic_node = diagnostic.stable_location.syntax_node(db);
        diagnostic_node.is_ancestor_or_self(db, node).then_some(enum_id)
    })
}
