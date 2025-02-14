use std::collections::HashSet;

use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::FileId;
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{DiagnosticSeverity, PublishDiagnosticsParams, Url};

use crate::lang::db::AnalysisDatabase;
use crate::lang::diagnostics::file_diagnostics::FileDiagnostics;
use crate::lang::diagnostics::project_diagnostics::ProjectDiagnostics;
use crate::lang::lsp::LsProtoGroup;
use crate::project::find_scarb_cache_path;
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
    let mut params = new_file_diagnostics.to_lsp(db, file, trace_macro_diagnostics);

    // We want to ensure better UX by avoiding showing anything but errors from code that is not
    // controlled by a user (dependencies from git/package register).
    // Therefore, we filter non-error diagnostics for files residing in Scarb cache.
    let is_dependency = find_scarb_cache_path(scarb_toolchain)
        .is_some_and(|cache_path| params.uri.to_file_path().unwrap().starts_with(cache_path));
    if is_dependency {
        params.diagnostics.retain(|diag| diag.severity == Some(DiagnosticSeverity::ERROR));
    }

    if project_diagnostics.insert(new_file_diagnostics.clone()) {
        notifier.notify::<PublishDiagnostics>(params);
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
    for mut file_diagnostics in removed {
        // It might be that we are removing a file that actually had some diagnostics.
        // For example, this might happen if a file with a syntax error is deleted.
        // We are reusing just removed `FileDiagnostics` instead of constructing a fresh one
        // to preserve any extra state it might contain.
        file_diagnostics.clear();

        let params = PublishDiagnosticsParams {
            uri: file_diagnostics.url,
            diagnostics: vec![],
            version: None,
        };
        notifier.notify::<PublishDiagnostics>(params);
    }
}

fn tracing_file_url(db: &AnalysisDatabase, file: FileId) -> String {
    db.url_for_file(file).map(|u| u.to_string()).unwrap_or_default()
}
