use crate::lang::{
    analysis_context::AnalysisContext,
    db::AnalysisDatabase,
    lsp::{LsProtoGroup, ToLsp},
};
use cairo_lang_semantic::db::SemanticGroup;
use lsp_types::{CodeAction, TextEdit, WorkspaceEdit};

pub fn cairo_lint(db: &AnalysisDatabase, ctx: &AnalysisContext<'_>) -> Option<Vec<CodeAction>> {
    let diags = db.module_semantic_diagnostics(ctx.module_id).ok()?;

    let node_span = ctx.node.span(db);

    let diagnostics = diags
        .get_diagnostics_without_duplicates(db)
        .into_iter()
        .filter(|diagnostic| {
            diagnostic.stable_location.syntax_node(db).span(db).contains(node_span)
        })
        .collect();

    let fixes = cairo_lint::get_fixes(db, diagnostics);

    let result = fixes
        .into_iter()
        .filter_map(|(file, fixes)| db.url_for_file(file).map(|url| (file, url, fixes)))
        .flat_map(|(file, file_url, fixes)| {
            fixes.into_iter().flat_map(move |fix| {
                Some(CodeAction {
                    title: "Fix lint".to_string(),
                    edit: Some(WorkspaceEdit {
                        changes: Some(
                            [(
                                file_url.clone(),
                                vec![TextEdit {
                                    range: fix.span.position_in_file(db, file)?.to_lsp(),
                                    new_text: fix.suggestion,
                                }],
                            )]
                            .into_iter()
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
            })
        })
        .collect();

    Some(result)
}
