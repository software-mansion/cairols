use cairo_lang_semantic::db::SemanticGroup;
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};

use crate::lang::{
    analysis_context::AnalysisContext,
    db::AnalysisDatabase,
    lsp::{LsProtoGroup, ToLsp},
};

pub fn cairo_lint(db: &AnalysisDatabase, ctx: &AnalysisContext<'_>) -> Option<Vec<CodeAction>> {
    let diags = db.module_semantic_diagnostics(ctx.module_file_id.0).ok()?;

    let node_span = ctx.node.span(db);

    let diagnostics = diags
        .get_diagnostics_without_duplicates(db)
        .into_iter()
        .filter(|diagnostic| {
            diagnostic.stable_location.syntax_node(db).span(db).contains(node_span)
        })
        .collect();

    let fixes = cairo_lint::get_separated_fixes(db, diagnostics);

    let result = fixes
        .into_iter()
        .filter_map(|(file, fixes)| db.url_for_file(file).map(|url| (file, url, fixes)))
        .flat_map(|(file, file_url, fixes)| {
            fixes.into_iter().map(move |fix| CodeAction {
                title: fix.description.clone(),
                kind: Some(CodeActionKind::QUICKFIX),
                edit: Some(WorkspaceEdit {
                    changes: Some(
                        [(
                            file_url.clone(),
                            fix.suggestions
                                .into_iter()
                                .filter_map(|suggestion| {
                                    Some(TextEdit {
                                        range: suggestion.span.position_in_file(db, file)?.to_lsp(),
                                        new_text: suggestion.code,
                                    })
                                })
                                .collect(),
                        )]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            })
        })
        .collect();

    Some(result)
}
