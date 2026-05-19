use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

use lsp_types::{Diagnostic, Url};

/// Global storage of diagnostics currently published in this LS window.
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
pub struct WindowDiagnostics {
    file_diagnostics: Arc<RwLock<HashMap<Url, FileDiagnostics>>>,
    /// A map from a workspace manifest [`Url`] to diagnostics published for that workspace.
    ///
    /// ## Invariants
    /// 1. Any [`Url`] key in the *outer* mapping **MUST** correspond to the owner of one
    ///    workspace diagnostics set.
    /// 2. Any [`Url`] key in an *inner* mapping **MUST** be either:
    ///    - equal to the [`Url`] key from the outer mapping
    ///    - one of the files covered by the workspace identified by the [`Url`] key from the
    ///      outer mapping
    /// 3. Any [`Url`] key from any *inner* mapping **MUST** be unique amongst all the keys from all
    ///    inner mappings. This invariant always holds if the previous one does.
    workspace_diagnostics: Arc<RwLock<HashMap<Url, WorkspaceDiagnostics>>>,
}

/// Diagnostics published by a single root on-disk file and the virtual files owned by it.
type FileDiagnostics = HashMap<Url, Vec<Diagnostic>>;
/// Diagnostics published by a single workspace.
type WorkspaceDiagnostics = HashMap<Url, Vec<Diagnostic>>;

impl WindowDiagnostics {
    /// Creates new window diagnostics instance.
    pub fn new() -> Self {
        Self { file_diagnostics: Default::default(), workspace_diagnostics: Default::default() }
    }

    #[tracing::instrument(skip_all)]
    pub fn update_file(
        &self,
        root_on_disk_file_url: Url,
        new_diags: FileDiagnostics,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        Self::update_inner(&self.file_diagnostics, root_on_disk_file_url, new_diags, "file")
    }

    #[tracing::instrument(skip_all)]
    pub fn clear_old_files(&self, processed_files_to_retain: &HashSet<Url>) -> Vec<Url> {
        let mut file_diagnostics =
            self.file_diagnostics.write().expect("file diagnostics are poisoned, bailing out");

        let removed = file_diagnostics
            .keys()
            .filter(|url| !processed_files_to_retain.contains(*url))
            .cloned()
            .collect::<Vec<_>>();

        for url in &removed {
            file_diagnostics.remove(url);
        }

        removed
    }

    /// Update existing diagnostics based on new diagnostics produced by the given workspace.
    ///
    /// Returns mapping from a file to its diagnostics for files which diagnostics changed
    /// as a result of the update.
    #[tracing::instrument(skip_all)]
    pub fn update_workspace(
        &self,
        workspace_manifest_url: Url,
        new_diags: WorkspaceDiagnostics,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        Self::update_inner(
            &self.workspace_diagnostics,
            workspace_manifest_url,
            new_diags,
            "workspace",
        )
    }

    fn update_inner(
        store: &Arc<RwLock<HashMap<Url, FileDiagnostics>>>,
        owner_url: Url,
        new_diags: FileDiagnostics,
        poisoned_message_prefix: &str,
    ) -> FileDiagnostics {
        let old_diags = store
            .read()
            .unwrap_or_else(|_| {
                panic!("{poisoned_message_prefix} diagnostics are poisoned, bailing out")
            })
            .get(&owner_url)
            .cloned()
            .unwrap_or_default();

        if new_diags == old_diags {
            return HashMap::new();
        }

        store
            .write()
            .unwrap_or_else(|_| {
                panic!("{poisoned_message_prefix} diagnostics are poisoned, bailing out")
            })
            .insert(owner_url, new_diags.clone());

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
}
