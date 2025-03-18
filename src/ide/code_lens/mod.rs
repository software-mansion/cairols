use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::server::client::{Notifier, Requester};
use crate::server::schedule::Task;
use crate::state::State;
use lsp_types::request::CodeLensRefresh;
use lsp_types::{CodeLens, Url};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Arc, RwLock};
use tests::TestCodeLensProvider;

mod tests;

#[derive(Default)]
pub struct CodeLensControllerState {
    lens: HashMap<Url, Vec<(CodeLens, CodeLensKind)>>,
}

#[derive(Clone, Default)]
pub struct CodeLensController {
    state: Arc<RwLock<CodeLensControllerState>>,
}

impl CodeLensController {
    pub fn on_did_change(
        &self,
        requester: &mut Requester<'_>,
        db: &AnalysisDatabase,
        config: &Config,
        files: impl Iterator<Item = FileChange>,
    ) {
        let lens_guard = self.state.read().unwrap();

        // If it was not requested before, there is nothing to invalidate.
        let files: Vec<_> =
            files.filter(|file_change| lens_guard.lens.contains_key(&file_change.url)).collect();

        // Release so any panickable action is performed while not keeping state lock.
        drop(lens_guard);

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
            let _ = requester.request::<CodeLensRefresh>((), |_| Task::nothing());
        }
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
}

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
