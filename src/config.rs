use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context;
use lsp_server::ErrorCode;
use lsp_types::request::WorkspaceConfiguration;
use lsp_types::{ClientCapabilities, ConfigurationItem, ConfigurationParams};
use serde_json::Value;
use tracing::{debug, error, warn};

use crate::ide::analysis_progress::AnalysisProgressController;
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::controller::ProcMacroClientController;
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::lsp::result::{LSPResult, LSPResultEx};
use crate::server::client::Requester;
use crate::server::schedule::Task;

// TODO(mkaput): Write a macro that will auto-generate this struct and the `reload` logic.
// TODO(mkaput): Write a test that checks that fields in this struct are sorted alphabetically.
// TODO(mkaput): Write a tool that syncs `configuration` in VSCode extension's `package.json`.
/// Runtime configuration for the language server.
///
/// The properties stored in this struct **may** change during LS lifetime (through the
/// [`Self::reload`] method).
/// Therefore, holding any references or copies of this struct or its values for
/// longer periods of time should be avoided, unless the copy will be reactively updated on
/// `workspace/didChangeConfiguration` requests.
#[derive(Debug, Clone)]
pub struct Config {
    /// A user-provided path to the `core` crate source code for use in projects where `core` is
    /// unmanaged by the toolchain.
    ///
    /// The path may omit the `corelib/src` or `src` suffix.
    ///
    /// The property is set by the user under the `cairo1.corelibPath` key in client configuration.
    pub unmanaged_core_path: Option<PathBuf>,
    /// Whether to include the trace of the generation location of diagnostic location mapped by
    /// macros.
    ///
    /// The property is set by the user under the `cairo1.traceMacroDiagnostics` key in client
    /// configuration.
    pub trace_macro_diagnostics: bool,
    /// Whether to resolve procedural macros or ignore them.
    ///
    /// The property is set by the user under the `cairo1.enableProcMacros` key in client
    /// configuration.
    pub enable_proc_macros: bool,

    /// Whether to include the cairo-lint in the analysis.
    ///
    /// The property is set by the user under the `cairo1.enableLinter` key in client
    /// configuration.
    pub enable_linter: bool,

    /// A user-provided command used if `test_runner` is [`TestRunner::Custom`].
    ///
    /// The property is set by the user under the `cairo1.runTestCommand` key in client
    /// configuration.
    pub run_test_command: String,

    /// Test runner that should be used in code lens.
    ///
    /// The property is set by the user under the `cairo1.testRunner` key in client
    /// configuration.
    pub test_runner: TestRunner,

    /// Whether to use experimental cache for procedural macros.
    ///
    /// This is *NOT* invalidated and can produce wrong inputs. In this case removing cache file manually should fix it.
    pub enable_experimental_proc_macro_cache: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            unmanaged_core_path: None,
            trace_macro_diagnostics: false,
            enable_proc_macros: Self::ENABLE_PROC_MACROS_DEFAULT,
            enable_linter: true,
            run_test_command: String::new(),
            test_runner: TestRunner::Auto,
            enable_experimental_proc_macro_cache: false,
        }
    }
}

impl Config {
    pub const ENABLE_PROC_MACROS_DEFAULT: bool = true;
    /// Reloads the configuration from the language client.
    ///
    /// ## Note
    /// Contrary to [`Config::reload`], it applies changes to the state appropriate for default
    /// config if the client does not support config reloading.
    /// Therefore, this function should be called only once per LS lifetime.
    pub fn reload_on_start(
        &mut self,
        requester: &mut Requester<'_>,
        db: &mut AnalysisDatabase,
        proc_macro_controller: &mut ProcMacroClientController,
        analysis_progress_controller: &mut AnalysisProgressController,
        client_capabilities: &ClientCapabilities,
    ) -> LSPResult<()> {
        if !client_capabilities.workspace_configuration_support() {
            warn!(
                "client does not support `workspace/configuration` requests, config will not be \
                 reloaded"
            );

            self.apply_changes(db, proc_macro_controller, analysis_progress_controller);

            return Ok(());
        }

        self.reload_inner(requester)
    }

    /// Reloads the configuration from the language client.
    ///
    /// ## Note
    /// Contrary to [`Config::reload_on_start`], it does _not_ apply any changes to the state
    /// if the client does not support config reloading.
    pub fn reload(
        &mut self,
        requester: &mut Requester<'_>,
        client_capabilities: &ClientCapabilities,
    ) -> LSPResult<()> {
        if !client_capabilities.workspace_configuration_support() {
            warn!(
                "client does not support `workspace/configuration` requests, config will not be \
                 reloaded"
            );

            return Ok(());
        }

        self.reload_inner(requester)
    }

    fn reload_inner(&mut self, requester: &mut Requester<'_>) -> LSPResult<()> {
        let items = vec![
            ConfigurationItem { scope_uri: None, section: Some("cairo1.corelibPath".to_owned()) },
            ConfigurationItem {
                scope_uri: None,
                section: Some("cairo1.traceMacroDiagnostics".to_owned()),
            },
            ConfigurationItem {
                scope_uri: None,
                section: Some("cairo1.enableProcMacros".to_owned()),
            },
            ConfigurationItem { scope_uri: None, section: Some("cairo1.enableLinter".to_owned()) },
            ConfigurationItem {
                scope_uri: None,
                section: Some("cairo1.runTestCommand".to_owned()),
            },
            ConfigurationItem { scope_uri: None, section: Some("cairo1.testRunner".to_owned()) },
            ConfigurationItem {
                scope_uri: None,
                section: Some("cairo1.experimental.enableProcMacroCache".to_owned()),
            },
        ];
        let expected_len = items.len();

        let handler = move |response: Vec<Value>| {
            let response_len = response.len();
            if response_len != expected_len {
                error!(
                    "server returned unexpected number of configuration items, expected: \
                     {expected_len}, got: {response_len}"
                );
                return Task::nothing();
            }

            // This conversion is O(1), and makes popping from front also O(1).
            let mut response = VecDeque::from(response);

            Task::local_mut(move |state, _, _, _| {
                *state.config = Config::default();

                state.config.unmanaged_core_path = response
                    .pop_front()
                    .as_ref()
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                    .map(Into::into);

                if let Some(value) = response.pop_front().as_ref().and_then(Value::as_bool) {
                    state.config.trace_macro_diagnostics = value;
                }

                if let Some(value) = response.pop_front().as_ref().and_then(Value::as_bool) {
                    state.config.enable_proc_macros = value;
                }

                if let Some(value) = response.pop_front().as_ref().and_then(Value::as_bool) {
                    state.config.enable_linter = value;
                }

                if let Some(value) = response.pop_front().as_ref().and_then(Value::as_str) {
                    state.config.run_test_command = value.to_string();
                }

                if let Some(value) = response.pop_front().as_ref().and_then(Value::as_str)
                    && let Ok(value) = value.parse()
                {
                    state.config.test_runner = value;
                }
                if let Some(value) = response.pop_front().as_ref().and_then(Value::as_bool) {
                    state.config.enable_experimental_proc_macro_cache = value;
                }

                debug!("reloaded configuration: {:#?}", state.config);

                state.config.apply_changes(
                    &mut state.db,
                    &mut state.proc_macro_controller,
                    &mut state.analysis_progress_controller,
                );
            })
        };

        requester
            .request::<WorkspaceConfiguration>(ConfigurationParams { items }, handler)
            .context("failed to query language client for configuration items")
            .with_failure_code(ErrorCode::RequestFailed)
            .inspect_err(|e| warn!("{e:?}"))
    }

    fn apply_changes(
        &self,
        db: &mut AnalysisDatabase,
        proc_macro_controller: &mut ProcMacroClientController,
        analysis_progress_controller: &mut AnalysisProgressController,
    ) {
        proc_macro_controller.handle_config_update(db, self);
        analysis_progress_controller.on_config_change(self);
    }
}

#[derive(Debug, Default, Clone)]
pub enum TestRunner {
    #[default]
    Auto,
    Snforge,
    CairoTest,
    Custom,
}

impl FromStr for TestRunner {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "snforge" => Ok(Self::Snforge),
            "cairo-test" => Ok(Self::CairoTest),
            "custom" => Ok(Self::Custom),
            _ => Err(()),
        }
    }
}
