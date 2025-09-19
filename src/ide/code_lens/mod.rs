use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::vec;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{ModuleId, ModuleItemId, TopLevelLanguageElementId};
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::ast::ModuleItem;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use crossbeam::channel::{self, Receiver, Sender};
use itertools::Itertools;
use lsp_types::notification::ShowMessage;
use lsp_types::request::CodeLensRefresh;
use lsp_types::{CodeLens, MessageType, ShowMessageParams, Url};
use serde_json::{Number, Value};

use crate::config::Config;
use crate::ide::code_lens::executables::{
    ExecutableCodeLens, ExecutableCodeLensConstructionParams, ExecutableCodeLensProvider,
};
use crate::ide::code_lens::tests::{
    TestCodeLens, TestCodeLensConstructionParams, TestCodeLensProvider,
};
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::lsp::ext::{ExecuteInTerminal, ExecuteInTerminalParams};
use crate::server::client::{Notifier, Requester};
use crate::server::schedule::thread::{JoinHandle, ThreadPriority};
use crate::server::schedule::{Task, thread};
use crate::state::State;

mod executables;
mod tests;

trait CodeLensBuilder {
    fn build_lens(self, index: usize) -> LSCodeLens;
}

trait CodeLensProvider {
    type ConstructionParams<'a>;
    type LensBuilder: CodeLensBuilder;
    fn create_lens(params: Self::ConstructionParams<'_>) -> Vec<Self::LensBuilder>;
}

trait CodeLensInterface {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()>;
    fn lens(&self) -> CodeLens;
}

#[derive(Clone, PartialEq)]
pub enum LSCodeLens {
    Test(TestCodeLens),
    Executable(ExecutableCodeLens),
}

impl CodeLensInterface for LSCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        match self {
            LSCodeLens::Test(test_code_lens) => test_code_lens.execute(file_url, state, notifier),
            LSCodeLens::Executable(executable_code_lens) => {
                executable_code_lens.execute(file_url, state, notifier)
            }
        }
    }
    fn lens(&self) -> CodeLens {
        match self {
            LSCodeLens::Test(test) => test.lens(),
            LSCodeLens::Executable(executable) => executable.lens(),
        }
    }
}

pub type FileCodeLens = Vec<LSCodeLens>;

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
    pub fn schedule_refreshing_all_lenses(&self, db: AnalysisDatabase, config: Config) {
        let lens_guard = self.state.read().unwrap();

        // Invalidate all the files in the state
        let files: Vec<_> = lens_guard
            .lens
            .keys()
            .map(|url| FileChange { url: url.clone(), was_deleted: false })
            .collect();

        // Release so any panickable action is performed while not keeping state lock.
        drop(lens_guard);

        self.schedule_refresh(db, config, files);
    }

    #[tracing::instrument(name = "CodeLensController::on_did_change", skip_all)]
    pub fn on_did_change(
        &self,
        db: AnalysisDatabase,
        config: Config,
        files: impl Iterator<Item = FileChange>,
    ) {
        let lens_guard = self.state.read().unwrap();

        // If it was not requested before, there is nothing to invalidate.
        let files: Vec<_> =
            files.filter(|file_change| lens_guard.lens.contains_key(&file_change.url)).collect();

        // Release so any panickable action is performed while not keeping state lock.
        drop(lens_guard);

        self.schedule_refresh(db, config, files);
    }

    pub fn code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
    ) -> Option<Vec<CodeLens>> {
        let lens_state = self.state.read().unwrap();

        let file_code_lens: FileCodeLens = if let Some(code_lens) = lens_state.lens.get(&url) {
            code_lens.clone()
        } else {
            drop(lens_state);

            let result = calculate_code_lens(url.clone(), db, config)?;

            // Lock state only if calculating did *not* panic, so the lock will not be poisoned.
            let mut state = self.state.write().unwrap();
            let entry = state.lens.entry(url);

            entry.insert_entry(result.clone());
            result
        };

        let code_lens = file_code_lens
            .into_iter()
            .map(|lens| lens.lens())
            .sorted_by_key(|lens| lens.command.clone().unwrap_or_default().title)
            .collect();

        Some(code_lens)
    }

    pub fn execute_code_lens(state: &State, notifier: Notifier, args: &[Value]) -> Option<()> {
        let (file_url, index) = parse_args(args)?;

        // Drop state guard before doing any panickable actions.
        let ls_code_lens = {
            let code_lens_state = state.code_lens_controller.state.read().ok()?;
            let file_lens_state = code_lens_state.lens.get(&file_url)?;
            let item_ref = file_lens_state.get(index)?;
            item_ref.clone()
        };

        ls_code_lens.execute(file_url, state, &notifier);
        Some(())
    }

    #[tracing::instrument(skip_all)]
    fn schedule_refresh(&self, db: AnalysisDatabase, config: Config, files: Vec<FileChange>) {
        let _ = self.refresh_sender.send(RefreshCodeLensRequest { db, config, files });
    }
}

struct RefreshCodeLensRequest {
    db: AnalysisDatabase,
    config: Config,
    files: Vec<FileChange>,
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
            let message =
                self.refresh_receiver.try_iter().fold(message, |mut acc, next_message| {
                    acc.db = next_message.db; // Leave only single snapshot, drop others.
                    acc.config = next_message.config; // Use last sent config.

                    acc.files.extend(next_message.files);
                    acc
                });

            let _ = catch_unwind(AssertUnwindSafe(|| {
                self.refresh_lenses_for(
                    &message.db,
                    &message.config,
                    message.files.into_iter().unique(),
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
    ) {
        // Collect so any panickable action is performed while not keeping state lock.
        let entries: Vec<_> = files
            .into_iter()
            .filter_map(|file_change| {
                calculate_code_lens(file_change.url.clone(), db, config)
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

fn calculate_code_lens(url: Url, db: &AnalysisDatabase, config: &Config) -> Option<FileCodeLens> {
    let mut result: FileCodeLens = vec![];
    let test_lens_builders = TestCodeLensProvider::create_lens(TestCodeLensConstructionParams {
        db,
        url: url.clone(),
        config: config.clone(),
    });
    let executable_lens_builders =
        ExecutableCodeLensProvider::create_lens(ExecutableCodeLensConstructionParams { url, db });

    push_lens_builders(&mut result, test_lens_builders);
    push_lens_builders(&mut result, executable_lens_builders);

    Some(result)
}

fn push_lens_builders<T: CodeLensBuilder>(lens: &mut FileCodeLens, lens_builders: Vec<T>) {
    for lens_builder in lens_builders {
        lens.push(lens_builder.build_lens(lens.len()));
    }
}

fn make_lens_args(file_url: Url, lens_index: usize) -> Vec<Value> {
    vec![Value::String(file_url.to_string()), Value::Number(Number::from(lens_index))]
}

fn parse_args(args: &[Value]) -> Option<(Url, usize)> {
    let [Value::String(url), Value::Number(lens_index)] = args else {
        return None;
    };
    let url: Url = url.parse().ok()?;
    let lens_index = lens_index.as_u64().unwrap() as usize;

    Some((url, lens_index))
}

struct AnnotatedNode<'db> {
    pub full_path: String,
    pub attribute_ptr: SyntaxStablePtrId<'db>,
}
/// Collects functions with given attributes on them
/// Returns struct with full path and a pointer to found attribute
fn collect_functions_with_attrs<'db>(
    db: &'db AnalysisDatabase,
    module: ModuleId<'db>,
    attributes: &'db [&'db str],
) -> Vec<AnnotatedNode<'db>> {
    let mut result = vec![];

    if let Ok(functions) = db.module_free_functions_ids(module) {
        for free_function_id in functions {
            let function = free_function_id.long(db).1.lookup(db);
            let function_full_path = ModuleItemId::FreeFunction(*free_function_id).full_path(db);
            result.extend(
                attributes
                    .iter()
                    .filter_map(|attr_name| function.find_attr(db, attr_name))
                    .map(|attr| AnnotatedNode {
                        full_path: function_full_path.clone(),
                        attribute_ptr: attr.stable_ptr(db).untyped(),
                    })
                    // If for some reason we found multiple attributes relevant for the code lens kind, push only the first one.
                    .next(),
            );
        }
    }

    result
}

fn get_original_module_item_and_file<'db>(
    db: &'db AnalysisDatabase,
    ptr: SyntaxStablePtrId<'db>,
) -> Option<(ModuleItem<'db>, FileId<'db>)> {
    let (file, span) =
        get_originating_location(db, ptr.file_id(db), ptr.lookup(db).span_without_trivia(db), None);

    db.find_syntax_node_at_offset(file, span.start)?.ancestors_with_self(db).find_map(|n| {
        let module_item = ModuleItem::cast(db, n);
        module_item.map(|module_item| (module_item, file))
    })
}

fn send_execute_in_terminal(state: &State, notifier: &Notifier, command: String, cwd: PathBuf) {
    if state.client_capabilities.execute_in_terminal_support() {
        notifier.notify::<ExecuteInTerminal>(ExecuteInTerminalParams { cwd, command });
    } else {
        notifier.notify::<ShowMessage>(ShowMessageParams {
            typ: MessageType::INFO,
            message: format!(
                "To execute the code lens, run command: `{command}` in directory {}",
                cwd.display()
            ),
        });
    }
}
