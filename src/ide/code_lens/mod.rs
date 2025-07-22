use crate::config::Config;
use crate::ide::code_lens::executables::{ExecutableCodeLens, ExecutableCodeLensProvider};
use crate::ide::code_lens::tests::TestCodeLens;
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::server::client::{Notifier, Requester};
use crate::server::schedule::thread::{JoinHandle, ThreadPriority};
use crate::server::schedule::{Task, thread};
use crate::state::State;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{ModuleId, ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_executable::plugin::EXECUTABLE_ATTR;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::ast::ModuleItem;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use crossbeam::channel::{self, Receiver, Sender};
use itertools::Itertools;
use lsp_types::request::CodeLensRefresh;
use lsp_types::{CodeLens, Url};
use scarb_metadata::CompilationUnitMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, RwLock};
use tests::TestCodeLensProvider;

mod executables;
mod tests;

// Different kinds of supported code lens
#[derive(PartialEq)]
pub enum LSCodeLens {
    Test(TestCodeLens),
    Executable(ExecutableCodeLens),
}

impl LSCodeLens {
    fn get_lens(&self) -> CodeLens {
        match self {
            LSCodeLens::Test(test_code_lens) => test_code_lens.get_lens(),
            LSCodeLens::Executable(executable_code_lens) => executable_code_lens.get_lens(),
        }
    }
}

pub type FileCodeLens = HashMap<String, LSCodeLens>;

#[derive(Default)]
pub struct CodeLensControllerState {
    lens: HashMap<Url, FileCodeLens>,
}

#[derive(Clone)]
pub struct CodeLensController {
    state: Arc<RwLock<CodeLensControllerState>>,
    refresh_sender: Sender<RefreshCodeLensRequest>,
    request_refresh_receiver: Receiver<()>,
    // Keep it last so we can drop channels.
    // Otherwise, the refresh thread will never stop, and the
    // JoinHandle drop will cause deadlock by waiting for the thread to join.
    _refresh_thread: Arc<JoinHandle<()>>,
}

impl CodeLensController {
    pub fn new() -> Self {
        let (refresh_sender, refresh_receiver) = channel::unbounded();
        // If there would be more than single element in queue we should ignore it and send request to client only once.
        // Dedup it on queue level for simplicity.
        let (request_refresh_sender, request_refresh_receiver) = channel::bounded(1);

        let state = Default::default();

        Self {
            state: Arc::clone(&state),
            refresh_sender,
            request_refresh_receiver,
            _refresh_thread: CodeLensRefreshThread::spawn(
                state,
                request_refresh_sender,
                refresh_receiver,
            )
            .into(),
        }
    }

    pub fn request_refresh_receiver(&self) -> Receiver<()> {
        self.request_refresh_receiver.clone()
    }

    pub fn handle_refresh(requester: &mut Requester<'_>) {
        let _ = requester.request::<CodeLensRefresh>((), |_| Task::nothing());
    }

    #[tracing::instrument(skip_all)]
    pub fn schedule_refreshing_all_lenses(
        &self,
        db: salsa::Snapshot<AnalysisDatabase>,
        config: Config,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) {
        let lens_guard = self.state.read().unwrap();

        // Invalidate all the files in the state
        let files: Vec<_> = lens_guard
            .lens
            .keys()
            .map(|url| FileChange { url: url.clone(), was_deleted: false })
            .collect();

        // Release so any panickable action is performed while not keeping state lock.
        drop(lens_guard);

        self.schedule_refresh(db, config, files, compilation_units);
    }

    #[tracing::instrument(name = "CodeLensController::on_did_change", skip_all)]
    pub fn on_did_change(
        &self,
        db: salsa::Snapshot<AnalysisDatabase>,
        config: Config,
        files: impl Iterator<Item = FileChange>,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) {
        let lens_guard = self.state.read().unwrap();

        // If it was not requested before, there is nothing to invalidate.
        let files: Vec<_> =
            files.filter(|file_change| lens_guard.lens.contains_key(&file_change.url)).collect();

        // Release so any panickable action is performed while not keeping state lock.
        drop(lens_guard);

        self.schedule_refresh(db, config, files, compilation_units);
    }

    pub fn code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) -> Option<Vec<CodeLens>> {
        let state = self.state.read().unwrap();

        let code_lens = if let Some(code_lens) = state.lens.get(&url) {
            code_lens.values().map(|ls_lens| ls_lens.get_lens()).collect()
        } else {
            drop(state);

            let result = calculate_code_lens(url.clone(), db, config, compilation_units)?;

            // Lock state only if calculating did *not* panic, so the lock will not be poisoned.
            let mut state = self.state.write().unwrap();
            let entry = state.lens.entry(url);

            entry.insert_entry(result).get().values().map(|ls_lens| ls_lens.get_lens()).collect()
        };

        Some(code_lens)
    }

    pub fn execute_code_lens(state: &State, notifier: Notifier, args: &[Value]) -> Option<()> {
        let lens_args = parse_args(args)?;

        let (function_id, file_url) = lens_args;

        // Drop state guard before doing any panickable actions.
        let code_lens_state = state.code_lens_controller.state.read().ok()?;
        let ls_code_lens = code_lens_state.lens.get(&file_url)?.get(&function_id)?;

        match ls_code_lens {
            LSCodeLens::Test(test_code_lens) => {
                TestCodeLensProvider.execute_code_lens(state, notifier, file_url, test_code_lens)
            }
            LSCodeLens::Executable(executable_code_lens) => ExecutableCodeLensProvider
                .execute_code_lens(state, notifier, file_url, executable_code_lens),
        }
    }

    #[tracing::instrument(skip_all)]
    fn schedule_refresh(
        &self,
        db: salsa::Snapshot<AnalysisDatabase>,
        config: Config,
        files: Vec<FileChange>,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) {
        let _ = self.refresh_sender.send(RefreshCodeLensRequest {
            db,
            config,
            files,
            compilation_units,
        });
    }
}

struct RefreshCodeLensRequest {
    db: salsa::Snapshot<AnalysisDatabase>,
    config: Config,
    files: Vec<FileChange>,
    compilation_units: Vec<CompilationUnitMetadata>,
}

struct CodeLensRefreshThread {
    state: Arc<RwLock<CodeLensControllerState>>,
    request_refresh_sender: Sender<()>,
    refresh_receiver: Receiver<RefreshCodeLensRequest>,
}

impl CodeLensRefreshThread {
    fn spawn(
        state: Arc<RwLock<CodeLensControllerState>>,
        request_refresh_sender: Sender<()>,
        refresh_receiver: Receiver<RefreshCodeLensRequest>,
    ) -> JoinHandle<()> {
        let this = Self { state, request_refresh_sender, refresh_receiver };

        thread::Builder::new(ThreadPriority::Worker)
            .name("cairo-ls:code-lens-refresher".into())
            .spawn(move || this.event_loop())
            .expect("failed to spawn code lens refresher thread")
    }

    fn event_loop(self) {
        while let Ok(message) = self.refresh_receiver.recv() {
            let message = self.refresh_receiver.try_iter().fold(message, |mut acc, next_messge| {
                acc.db = next_messge.db; // Leave only single snapshot, drop others.
                acc.config = next_messge.config; // Use last sent config.
                acc.compilation_units = next_messge.compilation_units; // Use last sent cus.

                acc.files.extend(next_messge.files);

                acc
            });

            let _ = catch_unwind(AssertUnwindSafe(|| {
                self.refresh_lenses_for(
                    &message.db,
                    &message.config,
                    message.files.into_iter().unique(),
                    message.compilation_units,
                );
            }));
        }
    }

    #[tracing::instrument(skip_all)]
    fn refresh_lenses_for(
        &self,
        db: &AnalysisDatabase,
        config: &Config,
        files: impl IntoIterator<Item = FileChange>,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) {
        // Collect so any panickable action is performed while not keeping state lock.
        let entries: Vec<_> = files
            .into_iter()
            .filter_map(|file_change| {
                calculate_code_lens(file_change.url.clone(), db, config, compilation_units.clone())
                    .map(|code_lenses| (file_change, code_lenses))
            })
            .collect();

        let mut lens_guard = self.state.write().unwrap();

        let mut should_refresh = false;

        for (file_change, code_lenses) in entries {
            let entry = lens_guard.lens.entry(file_change.url.clone());

            should_refresh = should_refresh
                || match &entry {
                    Entry::Occupied(occupied) if file_change.was_deleted => {
                        !occupied.get().is_empty()
                    }
                    Entry::Occupied(occupied) => occupied.get() != &code_lenses,
                    Entry::Vacant(_) => !code_lenses.is_empty(),
                };

            if file_change.was_deleted {
                lens_guard.lens.remove(&file_change.url);
            } else {
                entry.insert_entry(code_lenses);
            }
        }

        if should_refresh {
            let _ = self.request_refresh_sender.try_send(());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileChange {
    pub url: Url,
    pub was_deleted: bool,
}

pub trait LensOwner {
    fn get_lens(&self) -> CodeLens;
}

trait CodeLensProvider {
    type LensOwner: LensOwner;
    fn calculate_code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
        compilation_units: Vec<CompilationUnitMetadata>,
    ) -> Option<FileCodeLens>;

    fn execute_code_lens(
        &self,
        state: &State,
        notifier: Notifier,
        url: Url,
        lens_owner: &Self::LensOwner,
    ) -> Option<()>;
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CodeLensKind {
    Test,
    Executable,
}

const TEST_EXECUTABLES: [&str; 2] = ["test", "snforge_internal_test_executable"];
const PLAIN_EXECUTABLES: [&str; 1] = [EXECUTABLE_ATTR];

fn calculate_code_lens(
    url: Url,
    db: &AnalysisDatabase,
    config: &Config,
    compilation_units: Vec<CompilationUnitMetadata>,
) -> Option<FileCodeLens> {
    let mut result = HashMap::new();

    result.extend(TestCodeLensProvider.calculate_code_lens(
        url.clone(),
        db,
        config,
        compilation_units.clone(),
    )?);
    result.extend(ExecutableCodeLensProvider.calculate_code_lens(
        url,
        db,
        config,
        compilation_units,
    )?);

    Some(result)
}

fn parse_args(args: &[Value]) -> Option<(String, Url)> {
    let [Value::String(function_full_path), Value::String(url)] = args else {
        return None;
    };
    let url: Url = url.parse().ok()?;

    Some((function_full_path.clone(), url))
}

pub struct AnnotatedNode {
    pub full_path: String,
    pub attribute_ptr: SyntaxStablePtrId,
}
/// Collects functions with given attributes on them
/// Returns tuples of (full path, pointer to found attribute)
pub fn collect_functions_with_attrs(
    db: &AnalysisDatabase,
    module: ModuleId,
    attributes: Vec<&str>,
) -> Vec<AnnotatedNode> {
    let mut result = vec![];

    if let Ok(functions) = db.module_free_functions(module) {
        for (free_function_id, function) in functions.iter() {
            let function_full_path = ModuleItemId::FreeFunction(*free_function_id).full_path(db);
            result.extend(
                attributes
                    .iter()
                    .filter_map(|attr_name| function.find_attr(db, attr_name))
                    .map(|attr| AnnotatedNode {
                        full_path: function_full_path.clone(),
                        attribute_ptr: attr.stable_ptr(db).untyped(),
                    })
                    // If for some weird reason we found both, push only first (i.e. prefer `#[test]`).
                    .next(),
            );
        }
    }

    result
}

pub fn get_original_node_and_file(
    db: &AnalysisDatabase,
    ptr: SyntaxStablePtrId,
) -> Option<(SyntaxNode, FileId)> {
    let (file, span) =
        get_originating_location(db, ptr.file_id(db), ptr.lookup(db).span_without_trivia(db), None);

    db.find_syntax_node_at_offset(file, span.start)?
        .ancestors_with_self(db)
        .find(|n| ModuleItem::cast(db, *n).is_some())
        .map(|syntax_node| (syntax_node, file))
}
