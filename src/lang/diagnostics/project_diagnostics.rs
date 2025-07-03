use itertools::{Either, Itertools};
use lsp_types::{Diagnostic, Url};
use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::{Arc, RwLock};

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

    /// Update existing diagnostics based on new diagnostics obtained by processing an on disk file
    /// identified by `root_on_disk_file_url` and virtual files originating from it.
    ///
    /// Returns mapping from a file to its diagnostics for files which diagnostics changed
    /// as a result of the update.
    #[tracing::instrument(skip_all)]
    pub fn update(
        &self,
        root_on_disk_file_url: Url,
        new_diags: SelfAndOriginatingFilesDiagnostics,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        let old_diags = self
            .file_diagnostics
            .read()
            .expect("file diagnostics are poisoned, bailing out")
            .get(&root_on_disk_file_url)
            .cloned()
            .unwrap_or_default();

        if new_diags == old_diags {
            return HashMap::new();
        }

        self.file_diagnostics
            .write()
            .expect("file diagnostics are poisoned, bailing out")
            .insert(root_on_disk_file_url.clone(), new_diags.clone());

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

        diags_to_send
    }

    /// Removes diagnostics for files not present in the given set and returns a list of files for
    /// which diagnostics were actually cleared.
    pub fn clear_old(&self, processed_files_to_retain: &HashSet<Url>) -> Vec<Url> {
        let mut file_diagnostics =
            self.file_diagnostics.write().expect("file diagnostics are poisoned, bailing out");

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
        removed
    }
}
