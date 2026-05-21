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
        self.update_inner(
            &self.file_diagnostics,
            &self.workspace_diagnostics,
            root_on_disk_file_url,
            new_diags,
            "file",
        )
    }

    #[tracing::instrument(skip_all)]
    pub fn clear_old_files(
        &self,
        processed_files_to_retain: &HashSet<Url>,
    ) -> HashMap<Url, Vec<Diagnostic>> {
        let mut file_diagnostics =
            self.file_diagnostics.write().expect("file diagnostics are poisoned, bailing out");

        let removed = file_diagnostics
            .keys()
            .filter(|url| !processed_files_to_retain.contains(*url))
            .cloned()
            .collect::<Vec<_>>();

        let affected_urls = removed
            .iter()
            .flat_map(|url| file_diagnostics.get(url).into_iter().flat_map(HashMap::keys))
            .cloned()
            .collect::<HashSet<_>>();

        for url in &removed {
            file_diagnostics.remove(url);
        }

        drop(file_diagnostics);

        affected_urls
            .into_iter()
            .map(|url| {
                let diagnostics = self.merged_diagnostics_for_url(&url);
                (url, diagnostics)
            })
            .collect()
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
        self.update_inner(
            &self.workspace_diagnostics,
            &self.file_diagnostics,
            workspace_manifest_url,
            new_diags,
            "workspace",
        )
    }

    fn update_inner(
        &self,
        store: &Arc<RwLock<HashMap<Url, FileDiagnostics>>>,
        other_store: &Arc<RwLock<HashMap<Url, FileDiagnostics>>>,
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

        let other_diags = other_store
            .read()
            .unwrap_or_else(|_| {
                panic!("{poisoned_message_prefix} diagnostics are poisoned, bailing out")
            })
            .clone();

        let affected_urls =
            old_diags.keys().chain(new_diags.keys()).cloned().collect::<HashSet<_>>();

        let mut diags_to_send = HashMap::new();

        for location_file_url in affected_urls {
            let old_store_covers_url = old_diags.contains_key(&location_file_url);
            let new_store_covers_url = new_diags.contains_key(&location_file_url);
            let old_merged = Self::merge_diagnostics(
                old_diags.get(&location_file_url),
                Self::collect_diagnostics_for_url(&other_diags, &location_file_url).as_ref(),
            );
            let new_merged = self.merged_diagnostics_for_url(&location_file_url);

            // Even if the merged diagnostics stay empty, clients still need the publish event
            // when a source starts or stops covering a file.
            if old_merged != new_merged || old_store_covers_url != new_store_covers_url {
                diags_to_send.insert(location_file_url, new_merged);
            }
        }

        diags_to_send
    }

    fn merged_diagnostics_for_url(&self, url: &Url) -> Vec<Diagnostic> {
        let file_diags =
            self.file_diagnostics.read().expect("file diagnostics are poisoned, bailing out");
        let workspace_diags = self
            .workspace_diagnostics
            .read()
            .expect("workspace diagnostics are poisoned, bailing out");

        let file = Self::collect_diagnostics_for_url(&file_diags, url);
        let workspace = Self::collect_diagnostics_for_url(&workspace_diags, url);
        Self::merge_diagnostics(file.as_ref(), workspace.as_ref())
    }

    fn collect_diagnostics_for_url(
        store: &HashMap<Url, FileDiagnostics>,
        url: &Url,
    ) -> Option<Vec<Diagnostic>> {
        let diagnostics = store
            .values()
            .filter_map(|diagnostics| diagnostics.get(url))
            .flat_map(|diagnostics| diagnostics.iter().cloned())
            .collect::<Vec<_>>();

        (!diagnostics.is_empty()).then_some(diagnostics)
    }

    fn merge_diagnostics(
        primary: Option<&Vec<Diagnostic>>,
        secondary: Option<&Vec<Diagnostic>>,
    ) -> Vec<Diagnostic> {
        primary.into_iter().flatten().chain(secondary.into_iter().flatten()).cloned().collect()
    }
}
