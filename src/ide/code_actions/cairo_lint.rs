use cairo_lang_defs::diagnostic_utils::StableLocation;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_semantic::{
    SemanticDiagnostic, db::SemanticGroup, diagnostic::SemanticDiagnosticKind,
};
use cairo_lint::{CairoLintToolMetadata, LinterDiagnosticParams, LinterGroup};
use lsp_types::{CodeAction, CodeActionKind, TextEdit, WorkspaceEdit};

use crate::{
    lang::{
        analysis_context::AnalysisContext,
        db::AnalysisDatabase,
        lsp::{LsProtoGroup, ToLsp},
    },
    project::ConfigsRegistry,
};

pub fn cairo_lint<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    config_registry: &ConfigsRegistry,
) -> Option<Vec<CodeAction>> {
    let linter_params = LinterDiagnosticParams {
        only_generated_files: false,
        tool_metadata: get_linter_tool_metadata(db, ctx, config_registry),
    };

    let module_id = ctx.module_id;

    // We collect the semantic diagnostics, as the unused imports diagnostics (which come from the semantic diags),
    // can be fixed with the linter.
    let semantic_diags = db.module_semantic_diagnostics(module_id).ok()?;
    let linter_diags = db.linter_diagnostics(linter_params, module_id).iter().map(|diag| {
        SemanticDiagnostic::new(
            StableLocation::new(diag.stable_ptr),
            SemanticDiagnosticKind::PluginDiagnostic(diag.clone()),
            module_id,
        )
    });

    let node_span = ctx.node.span(db);

    let diagnostics = semantic_diags
        .get_diagnostics_without_duplicates(db)
        .into_iter()
        .chain(linter_diags)
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

fn get_linter_tool_metadata<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    config_registry: &ConfigsRegistry,
) -> CairoLintToolMetadata {
    if let FileLongId::OnDisk(file_id) = ctx.node.stable_ptr(db).file_id(db).long(db)
        && let Some(file_config) = config_registry.config_for_file(file_id)
    {
        file_config.lint.clone()
    } else {
        Default::default()
    }
}
