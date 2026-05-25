use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::{Arc, RwLock};

use itertools::{Either, Itertools};
use lsp_types::{Diagnostic, Url};

/// Global storage of diagnostics for the entire analysed codebase(s).
///
/// This object can be shared between threads and accessed concurrently.
///
/// ## Identifying files
///
/// Globally, we have to always identify files by [`Url`] instead of [`FileId`],
/// as the diagnostics state is independent of analysis database swaps that invalidate interned IDs.
///
/// [`FileId`]: cairo_lang_filesystem::ids::FileId
#[derive(Clone)]
pub struct ProjectDiagnostics {
    /// A map from an [`Url`] of an on disk file to diagnostics of the file and virtual files
    /// that are descendants of the file.
    ///
    /// ## Invariants
    /// 1. Any [`Url`] key in the *outer* mapping **MUST** correspond to an on disk file.
    /// 2. Any [`Url`] key in an *inner* mapping **MUST** be either:
    ///    - equal to the [`Url`] key from the outer mapping
    ///    - corresponding to a virtual file originating from the file with the [`Url`] key from the
    ///      outer mapping
    /// 3. Any [`Url`] key from any *inner* mapping **MUST** be unique amongst all the keys from all
    ///    inner mappings. This invariant always holds if the previous one does.
    file_diagnostics: Arc<RwLock<HashMap<Url, SelfAndOriginatingFilesDiagnostics>>>,
}

/// Diagnostics for a processed on disk file and virtual files originating from the processed file.
/// Check [`crate::lang::diagnostics::file_diagnostics`] for more info.
type SelfAndOriginatingFilesDiagnostics = HashMap<Url, Vec<Diagnostic>>;

impl ProjectDiagnostics {
    /// Creates new project diagnostics instance.
    pub fn new() -> Self {
        Self { file_diagnostics: Default::default() }
    }

    /// Updates diagnostics, unless the diagnostics generation became stale.
    #[tracing::instrument(skip_all)]
    pub fn update_if_current(
        &self,
        root_on_disk_file_url: Url,
        new_diags: SelfAndOriginatingFilesDiagnostics,
        is_current: impl Fn() -> bool,
    ) -> Option<HashMap<Url, Vec<Diagnostic>>> {
        if !is_current() {
            return None;
        }

        let mut file_diagnostics =
            self.file_diagnostics.write().expect("file diagnostics are poisoned, bailing out");

        if !is_current() {
            return None;
        }

        let old_diags = file_diagnostics.get(&root_on_disk_file_url).cloned().unwrap_or_default();

        if new_diags == old_diags {
            return Some(HashMap::new());
        }

        file_diagnostics.insert(root_on_disk_file_url.clone(), new_diags.clone());

        let mut diags_to_send = HashMap::new();

        for location_file_url in old_diags.keys() {
            // If there are no diagnostics for a file that used to have diagnostics,
            // we have to send empty diagnostics to the client.
            if !new_diags.contains_key(location_file_url) {
                diags_to_send.insert(location_file_url.clone(), Vec::new());
            }
        }

        for (location_file_url, new_diags_for_url) in new_diags {
            // If diagnostics have changed for a given file, we have to send an update.
            if old_diags.get(&location_file_url) != Some(&new_diags_for_url) {
                diags_to_send.insert(location_file_url, new_diags_for_url);
            }
        }

        Some(diags_to_send)
    }

    /// Clears old diagnostics, unless the diagnostics generation became stale.
    pub fn clear_old_if_current(
        &self,
        processed_files_to_retain: &HashSet<Url>,
        is_current: impl Fn() -> bool,
    ) -> Option<Vec<Url>> {
        if !is_current() {
            return None;
        }

        let mut file_diagnostics =
            self.file_diagnostics.write().expect("file diagnostics are poisoned, bailing out");

        if !is_current() {
            return None;
        }

        let (clean, removed) = mem::take(&mut *file_diagnostics).into_iter().partition_map(
            |(processed_file_url, diags)| {
                if processed_files_to_retain.contains(&processed_file_url) {
                    Either::Left((processed_file_url, diags))
                } else {
                    Either::Right(processed_file_url)
                }
            },
        );

        *file_diagnostics = clean;
        Some(removed)
    }
}
