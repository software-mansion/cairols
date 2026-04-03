use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use cairo_lang_filesystem::db::FilesGroup;
use crossbeam::channel::Sender;
use lsp_types::{ClientCapabilities, Position, TextDocumentContentChangeEvent, Url};
use tracing::error;

use crate::config::Config;
use crate::ide::analysis_progress::{AnalysisEvent, AnalysisProgressController};
use crate::ide::code_lens::CodeLensController;
use crate::lang::db::{AnalysisDatabase, AnalysisDatabaseSwapper};
use crate::lang::diagnostics::DiagnosticsController;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::project::{ConfigsRegistry, ProjectController};
use crate::server::client::Client;
use crate::server::connection::ClientSender;
use crate::toolchain::scarb::ScarbToolchain;

/// State of Language server.
pub struct State {
    pub db: AnalysisDatabase,
    pub open_files: Owned<HashSet<Url>>,
    pub open_file_texts: Owned<HashMap<Url, Arc<str>>>,
    pub pending_open_file_overrides: Owned<HashMap<Url, Arc<str>>>,
    pub config: Owned<Config>,
    pub client_capabilities: Owned<ClientCapabilities>,
    pub scarb_toolchain: ScarbToolchain,
    pub diagnostics_controller: DiagnosticsController,
    pub proc_macro_controller: ProcMacroClientController,
    pub project_controller: ProjectController,
    pub analysis_progress_controller: AnalysisProgressController,
    pub code_lens_controller: CodeLensController,
}

impl State {
    pub fn new(
        sender: ClientSender,
        client_capabilities: ClientCapabilities,
        cwd: PathBuf,
    ) -> Self {
        let notifier = Client::new(sender).notifier();
        let scarb_toolchain = ScarbToolchain::new(notifier.clone());

        let analysis_progress_controller = AnalysisProgressController::new(notifier.clone());

        let diagnostics_controller = DiagnosticsController::new(
            notifier.clone(),
            analysis_progress_controller.clone(),
            scarb_toolchain.clone(),
        );

        let proc_macro_controller = ProcMacroClientController::new(
            scarb_toolchain.clone(),
            notifier.clone(),
            analysis_progress_controller.server_tracker(),
            cwd,
            diagnostics_controller.generate_code_complete_receiver(),
        );

        Self {
            db: AnalysisDatabase::new(),
            open_files: Default::default(),
            open_file_texts: Default::default(),
            pending_open_file_overrides: Default::default(),
            config: Default::default(),
            client_capabilities: Owned::new(client_capabilities.into()),
            scarb_toolchain: scarb_toolchain.clone(),
            diagnostics_controller,
            analysis_progress_controller,
            proc_macro_controller,
            project_controller: ProjectController::initialize(scarb_toolchain, notifier),
            code_lens_controller: CodeLensController::new(),
        }
    }

    pub fn apply_open_file_text(&mut self, uri: Url, text: Arc<str>) {
        self.open_files.insert(uri.clone());
        self.open_file_texts.insert(uri.clone(), text.clone());

        let Some(file_input) = self.resolve_file_input(&uri) else {
            self.pending_open_file_overrides.insert(uri, text);
            return;
        };

        let current_content =
            self.db.file_for_url(&uri).and_then(|file_id| self.db.file_content(file_id));
        if current_content != Some(text.as_ref()) {
            self.db.set_editor_file_content_for_input(file_input, Some(text));
        }
        self.pending_open_file_overrides.remove(&uri);
    }

    pub fn apply_open_file_changes(
        &mut self,
        uri: Url,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) {
        let [TextDocumentContentChangeEvent { range: None, text, .. }] = content_changes.as_slice()
        else {
            let Some(mut text) = self.current_open_file_text(&uri) else {
                error!(%uri, "cannot apply incremental change: no known base text");
                return;
            };

            for change in content_changes {
                if let Some(range) = change.range {
                    let start = index_in_text(&text, range.start);
                    let end = index_in_text(&text, range.end);
                    if start > end
                        || end > text.len()
                        || !text.is_char_boundary(start)
                        || !text.is_char_boundary(end)
                    {
                        error!(%uri, start, end, text_len = text.len(), "invalid change range");
                        return;
                    }
                    text.replace_range(start..end, &change.text);
                } else {
                    text = change.text;
                }
            }

            self.apply_open_file_text(uri, text.into());
            return;
        };

        self.apply_open_file_text(uri, text.clone().into());
    }

    pub fn clear_open_file(&mut self, uri: &Url) {
        self.open_files.remove(uri);
        self.open_file_texts.remove(uri);
        self.pending_open_file_overrides.remove(uri);
        if let Some(file_input) = self.resolve_file_input(uri) {
            self.db.set_editor_file_content_for_input(file_input, None);
        }
    }

    pub fn apply_pending_open_file_overrides(&mut self) {
        let pending = self
            .pending_open_file_overrides
            .iter()
            .map(|(uri, text)| (uri.clone(), text.clone()))
            .collect::<Vec<_>>();
        for (uri, text) in pending {
            if self.resolve_file_input(&uri).is_some() {
                self.apply_open_file_text(uri, text);
            }
        }
    }

    pub fn reconcile_open_file_overrides_in_db(&mut self) {
        Self::reconcile_open_file_overrides_into_db(
            &mut self.db,
            &self.open_file_texts,
            &self.pending_open_file_overrides,
        );
    }

    fn resolve_file_input(&self, uri: &Url) -> Option<cairo_lang_filesystem::ids::FileInput> {
        self.db.file_for_url(uri).map(|file_id| self.db.file_input(file_id).clone())
    }

    fn current_open_file_text(&self, uri: &Url) -> Option<String> {
        if let Some(text) = self.open_file_texts.get(uri) {
            return Some(text.to_string());
        }
        if let Some(text) = self.pending_open_file_overrides.get(uri) {
            return Some(text.to_string());
        }

        self.db
            .file_for_url(uri)
            .and_then(|file_id| self.db.file_content(file_id).map(str::to_owned))
    }

    pub fn snapshot(&mut self) -> StateSnapshot {
        self.reconcile_open_file_overrides_in_db();

        StateSnapshot {
            db: self.db.clone(),
            scarb_toolchain: self.scarb_toolchain.clone(),
            open_files: self.open_files.snapshot(),
            open_file_texts: self.open_file_texts.snapshot(),
            config: self.config.snapshot(),
            client_capabilities: self.client_capabilities.snapshot(),
            configs_registry: self.project_controller.configs_registry(),
            code_lens_controller: self.code_lens_controller.clone(),
        }
    }

    fn reconcile_open_file_overrides_into_db(
        db: &mut AnalysisDatabase,
        open_file_texts: &HashMap<Url, Arc<str>>,
        pending_open_file_overrides: &HashMap<Url, Arc<str>>,
    ) {
        for (uri, text) in open_file_texts.iter().chain(
            pending_open_file_overrides
                .iter()
                .filter(|(uri, _)| !open_file_texts.contains_key(*uri)),
        ) {
            let Some(file_input) =
                db.file_for_url(uri).map(|file_id| db.file_input(file_id).clone())
            else {
                continue;
            };

            let current_content = db.file_for_url(uri).and_then(|file_id| db.file_content(file_id));
            if current_content != Some(text.as_ref()) {
                db.set_editor_file_content_for_input(file_input, Some(text.clone()));
            }
        }
    }
}

pub struct MetaStateInner {
    /// Database swapper.
    /// # Safety
    /// Swapper does not communicate with other critical modules and do not access the state.
    /// Using it also does not affect the analysis. Thus, it's safe to place it here and access via interior mutability.
    pub db_swapper: AnalysisDatabaseSwapper,
}

impl MetaStateInner {
    pub fn new(analysis_event_sender: Sender<AnalysisEvent>) -> Self {
        let db_swapper = AnalysisDatabaseSwapper::new(analysis_event_sender);
        Self { db_swapper }
    }
}

/// State keeps information about LS state (swapper, analysis state or other internal info)
/// Mutations of this struct are allowed in background tasks and do not trigger hooks.
pub type MetaState = Arc<Mutex<MetaStateInner>>;

/// Readonly snapshot of Language server state.
#[derive(Clone)]
pub struct StateSnapshot {
    pub db: AnalysisDatabase,
    pub scarb_toolchain: ScarbToolchain,
    pub open_files: Snapshot<HashSet<Url>>,
    pub open_file_texts: Snapshot<HashMap<Url, Arc<str>>>,
    pub config: Snapshot<Config>,
    pub client_capabilities: Snapshot<ClientCapabilities>,
    pub configs_registry: Snapshot<ConfigsRegistry>,
    pub code_lens_controller: CodeLensController,
}

impl std::panic::UnwindSafe for StateSnapshot {}

/// Represents owned value that can be mutated.
/// Allows creating snapshot from self.
#[derive(Debug, Default)]
pub struct Owned<T: ?Sized>(Arc<T>);

/// Readonly snapshot of [`Owned`] value.
#[derive(Debug, Default, Clone)]
pub struct Snapshot<T: ?Sized>(Arc<T>);

impl<T: ?Sized> Owned<T> {
    pub fn new(inner: Arc<T>) -> Self {
        Self(inner)
    }

    /// Creates a snapshot of value's current state.
    pub fn snapshot(&self) -> Snapshot<T> {
        Snapshot(self.0.clone())
    }
}

impl<T: ?Sized> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl<T: ?Sized> Deref for Snapshot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn index_in_text(text: &str, position: Position) -> usize {
    let mut offset = 0;
    let mut lines = text.lines();
    for line in lines.by_ref().take(position.line as usize) {
        offset += line.len() + "\n".len();
    }
    if let Some(line) = lines.next() {
        offset += min(position.character as usize, line.len());
    }
    offset
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;

    use cairo_lang_filesystem::db::FilesGroup;
    use lsp_types::ClientCapabilities;
    use tempfile::tempdir;

    use super::State;
    use crate::lang::lsp::LsProtoGroup;
    use crate::server::connection::ClientSender;

    #[test]
    fn apply_open_file_text_updates_host_and_snapshot_db_content() {
        let workspace = tempdir().expect("failed to create tempdir");
        let file_path = workspace.path().join("lib.cairo");
        fs::write(&file_path, "fn main() { let x = 1; }\n").expect("failed to write file");

        let mut state = State::new(
            ClientSender::black_hole(),
            ClientCapabilities::default(),
            workspace.path().to_path_buf(),
        );
        let uri = lsp_types::Url::from_file_path(&file_path).expect("failed to build file url");
        let updated: Arc<str> = "fn main() { let very_long_name = 1; }\n".into();

        state.apply_open_file_text(uri.clone(), updated.clone());

        let file = state.db.file_for_url(&uri).expect("file should resolve");
        let overrides = state
            .db
            .collect_open_file_overrides(std::iter::once(state.db.file_input(file).clone()));
        assert_eq!(
            overrides.get(state.db.file_input(file)).map(|text| text.as_ref()),
            Some(updated.as_ref())
        );
        assert_eq!(state.db.file_content(file), Some(updated.as_ref()));

        let snapshot = state.snapshot();
        let file = snapshot.db.file_for_url(&uri).expect("snapshot file should resolve");
        assert_eq!(snapshot.db.file_content(file), Some(updated.as_ref()));
    }
}
