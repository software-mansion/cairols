use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use cairo_lang_filesystem::db::{FilesGroup, ext_as_virtual};
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use lsp_types::notification::PublishDiagnostics;
use lsp_types::{DiagnosticSeverity, PublishDiagnosticsParams, Url};

use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::lang::diagnostics::file_diagnostics::FilesDiagnostics;
use crate::lang::diagnostics::project_diagnostics::WindowDiagnostics;
use crate::lang::lsp::LsProtoGroup;
use crate::project::ConfigsRegistry;
use crate::server::client::Notifier;
use crate::toolchain::scarb::ScarbToolchain;

#[tracing::instrument(skip_all)]
pub fn refresh_plugin_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    config: &Config,
    config_registry: &ConfigsRegistry,
    batch: Vec<FileId<'db>>,
    window_diagnostics: WindowDiagnostics,
    notifier: Notifier,
    scarb_toolchain: ScarbToolchain,
) {
    for file in batch {
        refresh_file_plugin_diagnostics(
            db,
            config,
            config_registry,
            file,
            &window_diagnostics,
            &notifier,
            &scarb_toolchain,
        );
    }
}

/// IMPORTANT: keep updating diagnostics state between server and client ATOMIC!
/// I.e, if diagnostics are updated on the server side they MUST be sent successfully to the
/// client (and vice-versa).
#[tracing::instrument(skip_all, fields(url = tracing_file_url(db, root_on_disk_file)))]
fn refresh_file_plugin_diagnostics<'db>(
    db: &'db AnalysisDatabase,
    config: &Config,
    config_registry: &ConfigsRegistry,
    root_on_disk_file: FileId<'db>,
    window_diagnostics: &WindowDiagnostics,
    notifier: &Notifier,
    scarb_toolchain: &ScarbToolchain,
) {
    let Some(new_files_diagnostics) =
        FilesDiagnostics::collect(db, config, config_registry, scarb_toolchain, root_on_disk_file)
    else {
        return;
    };

    // IMPORTANT: DO NOT change the order of operations here. `to_lsp` may panic, so it has to come
    // before `update`. It is to make sure that if `update` succeeds, `notify` executes as well.
    let (root_on_disk_file_url, new_diags) =
        new_files_diagnostics.to_lsp(db, config.trace_macro_diagnostics);

    let mut filtered_diags = new_diags
        .into_iter()
        .filter_map(|((url, file_id), mut diagnostics)| {
            // Scarb check provides syntax/semantic/lowering diagnostics. Keep everything else here
            // so editor features depending on native diagnostics, like code actions, still work.
            diagnostics.retain(|diag| diag.source.as_deref() != Some("scarb"));

            let is_dependency = originating_file_path(db, file_id)
                .is_some_and(|p| scarb_toolchain.is_from_scarb_cache(&p));
            if is_dependency {
                diagnostics.retain(|diag| diag.severity == Some(DiagnosticSeverity::ERROR));
            }

            ((!diagnostics.is_empty()) || url == root_on_disk_file_url)
                .then_some((url, diagnostics))
        })
        .collect::<HashMap<_, _>>();

    filtered_diags.entry(root_on_disk_file_url.clone()).or_default();

    let diags_to_send = window_diagnostics.update_file(root_on_disk_file_url, filtered_diags);
    for (url, diagnostics) in diags_to_send {
        notifier.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
            uri: url,
            diagnostics,
            version: None,
        });
    }
}

/// IMPORTANT: keep updating diagnostics state between server and client ATOMIC!
/// I.e, if diagnostics are updated on the server side they MUST be sent successfully to the
/// client (and vice-versa).
#[tracing::instrument(skip_all)]
pub fn clear_old_plugin_diagnostics(
    files_to_preserve: HashSet<Url>,
    window_diagnostics: WindowDiagnostics,
    notifier: Notifier,
) {
    let removed = window_diagnostics.clear_old_files(&files_to_preserve);
    for (url, diagnostics) in removed {
        let params = PublishDiagnosticsParams { uri: url, diagnostics, version: None };
        notifier.notify::<PublishDiagnostics>(params);
    }
}

fn originating_file_path<'db>(db: &'db dyn FilesGroup, file_id: FileId<'db>) -> Option<PathBuf> {
    match file_id.long(db) {
        FileLongId::OnDisk(path) => Some(path.clone()),
        FileLongId::Virtual(vf) => originating_file_path(db, vf.parent?.file_id),
        FileLongId::External(id) => {
            originating_file_path(db, ext_as_virtual(db, *id).parent?.file_id)
        }
    }
}

fn tracing_file_url<'db>(db: &'db AnalysisDatabase, file: FileId<'db>) -> String {
    db.url_for_file(file).map(|u| u.to_string()).unwrap_or_default()
}
