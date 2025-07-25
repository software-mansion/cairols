use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, RwLock};

use crossbeam::channel::{self, Receiver, Sender};
use itertools::Itertools;
use lsp_types::request::CodeLensRefresh;
use lsp_types::{CodeLens, Url};
use serde_json::Value;
use tests::TestCodeLensProvider;

use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::server::client::{Notifier, Requester};
use crate::server::schedule::thread::{JoinHandle, ThreadPriority};
use crate::server::schedule::{Task, thread};
use crate::state::State;

mod tests;

#[derive(Default)]
pub struct CodeLensControllerState {
    lens: HashMap<Url, Vec<(CodeLens, CodeLensKind)>>,
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

        self.schedule_refresh(db, config, files);
    }

    #[tracing::instrument(name = "CodeLensController::on_did_change", skip_all)]
    pub fn on_did_change(
        &self,
        db: salsa::Snapshot<AnalysisDatabase>,
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
        let state = self.state.read().unwrap();

        let code_lens = if let Some(code_lens) = state.lens.get(&url) {
            code_lens.iter().map(|(code_lens, _kind)| code_lens).cloned().collect()
        } else {
            drop(state);

            let result = calculate_code_lens(url.clone(), db, config)?;

            // Lock state only if calculating did *not* panic, so the lock will not be poisoned.
            let mut state = self.state.write().unwrap();
            let entry = state.lens.entry(url);

            entry
                .insert_entry(result)
                .get()
                .iter()
                .map(|(code_lens, _kind)| code_lens)
                .cloned()
                .collect()
        };

        Some(code_lens)
    }

    pub fn execute_code_lens(state: &State, notifier: Notifier, args: &[Value]) -> Option<()> {
        let (index, url) = parse_args(args)?;

        // Drop state guard before doing any panickable actions.
        let (code_lens, kind) =
            state.code_lens_controller.state.read().ok()?.lens.get(&url)?.get(index)?.clone();

        match kind {
            CodeLensKind::Test => {
                TestCodeLensProvider.execute_code_lens(state, notifier, url, code_lens)
            }
        }
    }

    #[tracing::instrument(skip_all)]
    fn schedule_refresh(
        &self,
        db: salsa::Snapshot<AnalysisDatabase>,
        config: Config,
        files: Vec<FileChange>,
    ) {
        let _ = self.refresh_sender.send(RefreshCodeLensRequest { db, config, files });
    }
}

struct RefreshCodeLensRequest {
    db: salsa::Snapshot<AnalysisDatabase>,
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
            let message = self.refresh_receiver.try_iter().fold(message, |mut acc, next_messge| {
                acc.db = next_messge.db; // Leave only single snapshot, drop others.
                acc.config = next_messge.config; // Use last sent config.
                acc.files.extend(next_messge.files);

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

trait CodeLensProvider {
    fn calculate_code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
    ) -> Option<Vec<CodeLens>>;

    fn execute_code_lens(
        &self,
        state: &State,
        notifier: Notifier,
        url: Url,
        code_lens: CodeLens,
    ) -> Option<()>;
}

#[derive(Clone, Copy, PartialEq)]
enum CodeLensKind {
    Test,
}

fn calculate_code_lens(
    url: Url,
    db: &AnalysisDatabase,
    config: &Config,
) -> Option<Vec<(CodeLens, CodeLensKind)>> {
    let mut result = vec![];

    result.extend(
        TestCodeLensProvider
            .calculate_code_lens(url, db, config)?
            .into_iter()
            .map(|code_lens| (code_lens, CodeLensKind::Test)),
    );

    Some(result)
}

fn parse_args(args: &[Value]) -> Option<(usize, Url)> {
    let [Value::Number(num), Value::String(url)] = args else { return None };
    let index = num.as_u64()? as usize;
    let url: Url = url.parse().ok()?;

    Some((index, url))
}
