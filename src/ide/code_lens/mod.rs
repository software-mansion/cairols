use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::server::client::Notifier;
use crate::state::State;
use lsp_types::{CodeLens, Url};
use serde_json::Value;
use std::collections::HashMap;
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
    pub fn code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
    ) -> Option<Vec<CodeLens>> {
        let result = calculate_code_lens(url.clone(), db, config)?;

        // Lock state only if calculating did *not* panicked, so the lock will not be poisoned.
        let mut state = self.state.write().unwrap();
        let entry = state.lens.entry(url);

        let code_lens = entry
            .insert_entry(result)
            .get()
            .iter()
            .map(|(code_lens, _kind)| code_lens)
            .cloned()
            .collect();

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

#[derive(Clone, Copy)]
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
