use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use cairo_lang_filesystem::db::{FilesGroup, ext_as_virtual};
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{DiagnosticSeverity, PublishDiagnosticsParams, Url};

use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::lang::diagnostics::file_diagnostics::FilesDiagnostics;
use crate::lang::diagnostics::project_diagnostics::ProjectDiagnostics;
use crate::lang::lsp::LsProtoGroup;
use crate::project::ConfigsRegistry;
use crate::server::client::Notifier;
use crate::toolchain::scarb::ScarbToolchain;

/// Refresh diagnostics and send diffs to the client.
#[tracing::instrument(skip_all)]
pub fn refresh_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    config: &Config,
    config_registry: &ConfigsRegistry,
    batch: Vec<FileId<'db>>,
    project_diagnostics: ProjectDiagnostics,
    notifier: Notifier,
    scarb_toolchain: ScarbToolchain,
    generation: u64,
    latest_generation: Arc<AtomicU64>,
) {
    for file in batch {
        if !is_current_generation(generation, &latest_generation) {
            return;
        }
        refresh_file_diagnostics(
            db,
            config,
            config_registry,
            file,
            &project_diagnostics,
            &notifier,
            &scarb_toolchain,
            generation,
            &latest_generation,
        );
    }
}

/// Refresh diagnostics for a single on disk file.
///
/// IMPORTANT: keep updating diagnostics state between server and client ATOMIC!
/// I.e, if diagnostics are updated on the server side they MUST be sent successfully to the
/// client (and vice-versa).
#[tracing::instrument(skip_all, fields(url = tracing_file_url(db, root_on_disk_file)))]
fn refresh_file_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    config: &Config,
    config_registry: &ConfigsRegistry,
    root_on_disk_file: FileId<'db>,
    project_diagnostics: &ProjectDiagnostics,
    notifier: &Notifier,
    scarb_toolchain: &ScarbToolchain,
    generation: u64,
    latest_generation: &AtomicU64,
) {
    if !is_current_generation(generation, latest_generation) {
        return;
    }

    let Some(new_files_diagnostics) =
        FilesDiagnostics::collect(db, config, config_registry, scarb_toolchain, root_on_disk_file)
    else {
        return;
    };

    // IMPORTANT: DO NOT change the order of operations here. `to_lsp` may panic, so it has to come
    // before `update`. It is to make sure that if `update` succeeds, `notify` executes as well.
    let (root_on_disk_file_url, new_diags) =
        new_files_diagnostics.to_lsp(db, config.trace_macro_diagnostics);

    let new_diags = new_diags
        .into_iter()
        .filter_map(|((url, file_id), mut diagnostics)| {
            // We want to ensure better UX by avoiding showing anything but errors from code that is
            // not controlled by a user (dependencies from git/package register).
            // Therefore, we filter non-error diagnostics for files residing in Scarb cache
            // and virtual files that are their descendants.
            let is_dependency = originating_file_path(db, file_id)
                .is_some_and(|p| scarb_toolchain.is_from_scarb_cache(&p));

            if is_dependency {
                diagnostics.retain(|diag| diag.severity == Some(DiagnosticSeverity::ERROR));
                // Return early here to not filter out entry with empty diagnostics for
                // the processed file if it happens to be here. It is necessary for our tests.
                if diagnostics.is_empty() {
                    return None;
                }
            }

            Some((url, diagnostics))
        })
        .collect();

    let Some(diags_to_send) =
        project_diagnostics.update_if_current(root_on_disk_file_url, new_diags, || {
            is_current_generation(generation, latest_generation)
        })
    else {
        return;
    };

    if !is_current_generation(generation, latest_generation) {
        return;
    }

    for (url, diagnostics) in diags_to_send {
        if !is_current_generation(generation, latest_generation) {
            return;
        }
        notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
            uri: url,
            diagnostics,
            version: None,
        });
    }
}

/// For an on disk file - returns a path to it.
/// For a virtual file - returns a path to its first on disk ancestor.
fn originating_file_path<'db>(db: &'db dyn FilesGroup, file_id: FileId<'db>) -> Option<PathBuf> {
    match file_id.long(db) {
        FileLongId::OnDisk(path) => Some(path.clone()),
        FileLongId::Virtual(vf) => originating_file_path(db, vf.parent?.file_id),
        FileLongId::External(id) => {
            originating_file_path(db, ext_as_virtual(db, *id).parent?.file_id)
        }
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
    generation: u64,
    latest_generation: Arc<AtomicU64>,
) {
    let Some(removed) = project_diagnostics.clear_old_if_current(&files_to_preserve, || {
        is_current_generation(generation, &latest_generation)
    }) else {
        return;
    };

    if !is_current_generation(generation, &latest_generation) {
        return;
    }

    for url in removed {
        if !is_current_generation(generation, &latest_generation) {
            return;
        }
        let params = PublishDiagnosticsParams { uri: url, diagnostics: vec![], version: None };
        notifier.notify::<PublishDiagnostics>(params);
    }
}

fn tracing_file_url<'db>(db: &'db AnalysisDatabase, file: FileId<'db>) -> String {
    db.url_for_file(file).map(|u| u.to_string()).unwrap_or_default()
}

fn is_current_generation(generation: u64, latest_generation: &AtomicU64) -> bool {
    latest_generation.load(Ordering::Acquire) == generation
}
