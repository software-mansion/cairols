use std::collections::HashSet;

use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::FileId;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{PublishDiagnosticsParams, Url};

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
    // before `insert`. It is to make sure that if `insert` succeeds, `notify` executes as well.
    let result_diags = new_file_diagnostics.to_lsp(db, file, trace_macro_diagnostics);

    // The files diagnostics we actually need to push, are in the keys right now
    let updated_files: Vec<Url> = result_diags.keys().cloned().collect();

    project_diagnostics.apply_updates(result_diags, scarb_toolchain);
    for file in updated_files {
        // Unwrap is safe here because we know this file was updated
        let diagnostics = project_diagnostics.get_diagnostics_for(&file).unwrap();
        notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
            uri: file.clone(),
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
    for (url, _) in removed {
        let params = PublishDiagnosticsParams { uri: url, diagnostics: vec![], version: None };
        notifier.notify::<PublishDiagnostics>(params);
    }
}

fn tracing_file_url(db: &AnalysisDatabase, file: FileId) -> String {
    db.url_for_file(file).map(|u| u.to_string()).unwrap_or_default()
}
