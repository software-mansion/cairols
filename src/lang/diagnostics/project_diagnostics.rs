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
    #[expect(clippy::type_complexity)]
    file_diagnostics: Arc<RwLock<HashMap<Url, HashMap<Url, Vec<Diagnostic>>>>>,
}

impl ProjectDiagnostics {
    /// Creates new project diagnostics instance.
    pub fn new() -> Self {
        Self { file_diagnostics: Default::default() }
    }

    /// Update existing diagnostics based on new diagnostics obtained by processing a file.
    ///
    /// Returns mapping from file to its diagnostics for files which diagnostics changed
    /// as a result of the update.
    pub fn update(
        &self,
        processed_file_url: Url,
        new_diags: HashMap<Url, Vec<Diagnostic>>,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        let old_diags = self
            .file_diagnostics
            .read()
            .expect("file diagnostics are poisoned, bailing out")
            .get(&processed_file_url)
            .cloned()
            .unwrap_or_default();

        if new_diags == old_diags {
            return HashMap::new();
        }

        self.file_diagnostics
            .write()
            .expect("file diagnostics are poisoned, bailing out")
            .insert(processed_file_url.clone(), new_diags.clone());

        let mut diags_to_send = HashMap::new();

        for location_file_url in old_diags.keys() {
            // If there are no diagnostics for a file that used to have diagnostics,
            // we have to send empty diagnostics to the client.
            if !new_diags.contains_key(location_file_url) {
                diags_to_send.insert(location_file_url.clone(), Vec::new());
            }
        }

        for (location_file_url, new_diags_for_url) in new_diags {
            // If diagnostics have changed for a given file, we have to send the update.
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
