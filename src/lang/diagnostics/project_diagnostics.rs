use crate::lang::diagnostics::file_diagnostics::LSPDiagnostic;
use crate::toolchain::scarb::ScarbToolchain;
use itertools::{Either, Itertools};
use lsp_types::{Diagnostic, DiagnosticSeverity, Url};
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
    file_diagnostics: Arc<RwLock<HashMap<Url, Vec<Diagnostic>>>>,
}

impl ProjectDiagnostics {
    /// Creates new project diagnostics instance.
    pub fn new() -> Self {
        Self { file_diagnostics: Default::default() }
    }

    /// Inserts new diagnostics for a file if they update the existing diagnostics.
    ///
    /// Returns `true` if stored diagnostics were updated; otherwise, returns `false`.
    pub fn insert(&self, file_url: Url, diags: Vec<Diagnostic>) -> bool {
        if let Some(old_diags) = self
            .file_diagnostics
            .read()
            .expect("file diagnostics are poisoned, bailing out")
            .get(&file_url)
        {
            if old_diags == &diags {
                return false;
            }
        };

        self.file_diagnostics
            .write()
            .expect("file diagnostics are poisoned, bailing out")
            .insert(file_url.clone(), diags);
        true
    }

    pub fn apply_updates(
        &self,
        updates: HashMap<Url, Vec<LSPDiagnostic>>,
        scarb_toolchain: &ScarbToolchain,
    ) {
        for (file, lsp_diagnostics) in updates {
            // There are basically ONLY 2 cases:
            // (if there's something else here something went seriously wrong)
            // 1. Remapped entry - those we want to append to existing diagnostics
            // 2. Non-remapped ones - we want to substitute the original ones with them
            let mut new_diagnostics: Vec<Diagnostic> = if lsp_diagnostics
                .iter()
                .all(|diagnostic| matches!(diagnostic, LSPDiagnostic::Remapped(_)))
            {
                let mut new_diags: Vec<Diagnostic> =
                    lsp_diagnostics.into_iter().map(|diag| diag.unpack()).collect();

                if let Some(diags) = self
                    .file_diagnostics
                    .read()
                    .expect("file diagnostics are poisoned, bailing out")
                    .get(&file)
                {
                    diags.clone_into(&mut new_diags)
                }

                new_diags
            } else if lsp_diagnostics
                .iter()
                .all(|diagnostic| matches!(diagnostic, LSPDiagnostic::Standard(_)))
            {
                lsp_diagnostics.into_iter().map(|diag| diag.unpack()).collect()
            } else {
                panic!(
                    "Some of the diagnostics updates are of mixed types - cannot proceed with such update"
                )
            };

            // Filtering phase
            // We want to ensure better UX by avoiding showing anything but errors from code that is not
            // controlled by a user (dependencies from git/package register).
            // Therefore, we filter non-error diagnostics for files residing in Scarb cache.
            let is_dependency = scarb_toolchain.cache_path().is_some_and(|cache_path| {
                file.to_file_path().is_ok_and(|p| p.starts_with(cache_path))
            });
            if is_dependency {
                new_diagnostics.retain(|diag| diag.severity == Some(DiagnosticSeverity::ERROR));
            }

            self.insert(file, new_diagnostics);
        }
    }

    pub fn get_diagnostics_for(&self, file_url: &Url) -> Option<Vec<Diagnostic>> {
        let read_guard =
            self.file_diagnostics.read().expect("file diagnostics are poisoned, bailing out");

        let diags = read_guard.get(file_url);

        diags.cloned()
    }

    /// Removes diagnostics for files not present in the given set and returns a list of actually
    /// removed entries.
    pub fn clear_old(&self, files_to_retain: &HashSet<Url>) -> Vec<(Url, Vec<Diagnostic>)> {
        let mut file_diagnostics =
            self.file_diagnostics.write().expect("file diagnostics are poisoned, bailing out");

        let (clean, removed) =
            mem::take(&mut *file_diagnostics).into_iter().partition_map(|(file_url, diags)| {
                if files_to_retain.contains(&file_url) {
                    Either::Left((file_url, diags))
                } else {
                    Either::Right((file_url, diags))
                }
            });

        *file_diagnostics = clean;
        removed
    }
}
