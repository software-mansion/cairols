use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::FileId;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{DiagnosticSeverity, PublishDiagnosticsParams, Url};
use std::collections::HashSet;

use crate::lang::db::AnalysisDatabase;
use crate::lang::diagnostics::file_diagnostics::FileDiagnostics;
use crate::lang::diagnostics::project_diagnostics::ProjectDiagnostics;
use crate::lang::lsp::LsProtoGroup;
use crate::server::client::Notifier;
use crate::toolchain::scarb::ScarbToolchain;

/// Refresh diagnostics and send diffs to the client.
#[tracing::instrument(skip_all)]
pub fn refresh_diagnostics(
    db: &AnalysisDatabase,
    batch: Vec<FileId>,
    trace_macro_diagnostics: bool,
    project_diagnostics: ProjectDiagnostics,
    notifier: Notifier,
    scarb_toolchain: ScarbToolchain,
) {
    let mut processed_modules: HashSet<ModuleId> = HashSet::default();

    for file in batch {
        refresh_file_diagnostics(
            db,
            file,
            trace_macro_diagnostics,
            &mut processed_modules,
            &project_diagnostics,
            &notifier,
            &scarb_toolchain,
        );
    }
}

/// Refresh diagnostics for a single file.
///
/// IMPORTANT: keep updating diagnostics state between server and client ATOMIC!
/// I.e, if diagnostics are updated on the server side they MUST be sent successfully to the
/// client (and vice-versa).
#[tracing::instrument(skip_all, fields(url = tracing_file_url(db, file)))]
fn refresh_file_diagnostics(
    db: &AnalysisDatabase,
    file: FileId,
    trace_macro_diagnostics: bool,
    processed_modules: &mut HashSet<ModuleId>,
    project_diagnostics: &ProjectDiagnostics,
    notifier: &Notifier,
    scarb_toolchain: &ScarbToolchain,
) {
    let Some(new_file_diagnostics) = FileDiagnostics::collect(db, file, processed_modules) else {
        return;
    };

    // IMPORTANT: DO NOT change the order of operations here. `to_lsp` may panic, so it has to come
    // before `update`. It is to make sure that if `update` succeeds, `notify` executes as well.
    let (processed_file_url, new_diags) = new_file_diagnostics.to_lsp(db, trace_macro_diagnostics);

    let new_diags = new_diags
        .into_iter()
        .filter_map(|(file, mut diagnostics)| {
            // TODO: fix for virtual files from deps - check path of parent files for virtual files
            // We want to ensure better UX by avoiding showing anything but errors from code that is
            // not controlled by a user (dependencies from git/package register).
            // Therefore, we filter non-error diagnostics for files residing in Scarb cache.
            let is_dependency = scarb_toolchain.cache_path().is_some_and(|cache_path| {
                file.to_file_path().is_ok_and(|p| p.starts_with(cache_path))
            });

            if is_dependency {
                diagnostics.retain(|diag| diag.severity == Some(DiagnosticSeverity::ERROR));
                if diagnostics.is_empty() {
                    return None;
                }
            }

            Some((file, diagnostics))
        })
        .collect();

    let diags_to_send = project_diagnostics.update(processed_file_url, new_diags);
    for (url, diagnostics) in diags_to_send {
        notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
            uri: url,
            diagnostics,
            version: None,
        });
    }
}

/// Wipes diagnostics for any files not present in the preserve set.
///
/// IMPORTANT: keep updating diagnostics state between server and client ATOMIC!
/// I.e, if diagnostics are updated on the server side they MUST be sent successfully to the
/// client (and vice-versa).
#[tracing::instrument(skip_all)]
pub fn clear_old_diagnostics(
    files_to_preserve: HashSet<Url>,
    project_diagnostics: ProjectDiagnostics,
    notifier: Notifier,
) {
    let removed = project_diagnostics.clear_old(&files_to_preserve);
    for url in removed {
        let params = PublishDiagnosticsParams { uri: url, diagnostics: vec![], version: None };
        notifier.notify::<PublishDiagnostics>(params);
    }
}

fn tracing_file_url(db: &AnalysisDatabase, file: FileId) -> String {
    db.url_for_file(file).map(|u| u.to_string()).unwrap_or_default()
}
