use std::fmt::{Display, Formatter};
use std::mem;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::CrateLongId;
use cairo_lang_semantic::db::PluginSuiteInput;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lang_utils::smol_str::ToSmolStr;
use crossbeam::channel::{Receiver, Sender};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use lsp_types::notification::ShowMessage;
use lsp_types::request::SemanticTokensRefresh;
use lsp_types::{ClientCapabilities, MessageType, ShowMessageParams};
use scarb_proc_macro_server_types::jsonrpc::RpcResponse;
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use tracing::error;

use super::client::connection::ProcMacroServerConnection;
use super::client::status::ServerStatus;
use super::client::{ProcMacroClient, RequestParams};
use crate::config::Config;
use crate::ide::analysis_progress::{ProcMacroServerStatus, ProcMacroServerTracker};
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::lang::proc_macros::plugins::proc_macro_plugin_suites;
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::server::client::{Notifier, Requester};
use crate::server::schedule::Task;
use crate::toolchain::scarb::ScarbToolchain;

const RESTART_RATE_LIMITER_PERIOD_SEC: u64 = 180;
const RESTART_RATE_LIMITER_RETRIES: u32 = 5;

/// Manages lifecycle of proc-macro-server client.
///
/// The following diagram describes the lifecycle of proc-macro-server.
/// ```mermaid
/// flowchart TB
///     StartServer["Start Server"] --> Initialize["Initialize"]
///     Initialize --> MainLoop["LS Main Loop"]
///     MainLoop --> CheckResponse["Check for Response on_response()"]
///     CheckResponse -- "true" --> IsStarting["Are we Starting?"]
///     IsStarting -- "yes" --> FinishInitialize["Finish Initialize"]
///     FinishInitialize -- "success" --> MainLoop
///     FinishInitialize -- "on failure" --> RestartServer["Restart Server"]
///     IsStarting -- "no" --> ProcessResponses["Process All Available Responses"]
///     ProcessResponses -- "success" --> MainLoop
///     ProcessResponses -- "on failure" --> RestartServer["Restart Server"]
///     MainLoop --> CheckError["Check for Error handle_error()"]
///     CheckError -- "true" --> RestartServer["Restart Server"]
///     RestartServer --> Initialize
/// ```
pub struct ProcMacroClientController {
    scarb: ScarbToolchain,
    notifier: Notifier,
    crate_plugin_suites: OrderedHashMap<CrateLongId, PluginSuite>,
    initialization_retries: RateLimiter<NotKeyed, InMemoryState, QuantaClock>,
    channels: ProcMacroChannels,
    proc_macro_server_tracker: ProcMacroServerTracker,
}

impl From<&ServerStatus> for ProcMacroServerStatus {
    fn from(value: &ServerStatus) -> Self {
        match value {
            ServerStatus::Pending => ProcMacroServerStatus::Pending,
            ServerStatus::Starting(_) => ProcMacroServerStatus::Starting,
            ServerStatus::Ready(_) => ProcMacroServerStatus::Ready,
            ServerStatus::Crashed => ProcMacroServerStatus::Crashed,
        }
    }
}

impl ProcMacroClientController {
    pub fn channels(&mut self) -> ProcMacroChannels {
        self.channels.clone()
    }

    pub fn new(
        scarb: ScarbToolchain,
        notifier: Notifier,
        proc_macro_server_tracker: ProcMacroServerTracker,
    ) -> Self {
        Self {
            scarb,
            notifier,
            proc_macro_server_tracker,
            crate_plugin_suites: Default::default(),
            initialization_retries: RateLimiter::direct(
                Quota::with_period(Duration::from_secs(
                    RESTART_RATE_LIMITER_PERIOD_SEC / RESTART_RATE_LIMITER_RETRIES as u64,
                ))
                .unwrap()
                .allow_burst(
                    // All retries can be used as fast as possible.
                    NonZeroU32::new(RESTART_RATE_LIMITER_RETRIES).unwrap(),
                ),
            ),
            channels: ProcMacroChannels::new(),
        }
    }

    /// Start proc-macro-server after config reload.
    /// Note that this will only try to go from `ClientStatus::Pending` to
    /// `ClientStatus::Starting` if config allows this.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn on_config_change(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        if db.proc_macro_server_status().is_pending() {
            self.try_initialize(db, config);
        }
    }

    fn set_proc_macro_server_status(&self, db: &mut AnalysisDatabase, server_status: ServerStatus) {
        let tracker_server_status = ProcMacroServerStatus::from(&server_status);
        db.set_proc_macro_server_status(server_status);
        self.proc_macro_server_tracker.set_server_status(tracker_server_status);
    }

    /// Forcibly restarts the proc-macro-server, shutting down any currently running instances.
    ///
    /// A new server instance is only started if there are available restart attempts left.
    /// This ensures that a fresh proc-macro-server is used.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn force_restart(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        for (crate_id, plugins) in mem::take(&mut self.crate_plugin_suites) {
            let interned_plugins = db.intern_plugin_suite(plugins);
            db.remove_crate_plugin_suite(db.intern_crate(crate_id), &interned_plugins);
        }

        self.try_initialize(db, config);
    }

    /// Check if an error was reported. If so, try to restart.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn handle_error(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        if !self.try_initialize(db, config) {
            self.fatal_failed(db, InitializationFailedInfo::NoMoreRetries);
        }
    }

    /// If the client is ready, apply all available responses.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn on_response(
        &mut self,
        db: &mut AnalysisDatabase,
        config: &Config,
        client_capabilities: &ClientCapabilities,
        requester: &mut Requester,
    ) {
        match db.proc_macro_server_status() {
            ServerStatus::Starting(client) => {
                let Ok(defined_macros) = client.finish_initialize() else {
                    self.handle_error(db, config);

                    return;
                };

                self.remove_current_plugins_from_db(db);

                self.crate_plugin_suites = proc_macro_plugin_suites(defined_macros)
                    .into_iter()
                    .map(|(component, suite)| {
                        // Here we rely on the contract that `name` and `discriminator` of the
                        // `CompilationUnitComponent` are identical to those from `scarb-metadata`,
                        // so the `CrateLondId`s constructed here are identical to those built in
                        // `project::crate_data::Crate::apply`.
                        let crate_name = component.name.to_smolstr();
                        let crate_long_id = CrateLongId::Real {
                            name: crate_name,
                            discriminator: component.discriminator.map(Into::into),
                        };

                        (crate_long_id, suite)
                    })
                    .collect();

                for (crate_long_id, plugin_suite) in self.crate_plugin_suites.iter() {
                    let crate_id = db.intern_crate(crate_long_id.clone());
                    let interned_plugin_suite = db.intern_plugin_suite(plugin_suite.clone());
                    db.add_crate_plugin_suite(crate_id, interned_plugin_suite);
                }

                self.set_proc_macro_server_status(db, ServerStatus::Ready(client));

                ProcMacroClientController::on_supported_macros_response(
                    client_capabilities,
                    requester,
                );
            }
            ServerStatus::Ready(client) => {
                self.apply_responses(db, config, &client);
            }
            _ => {}
        }
    }

    pub fn proc_macro_plugin_suite_for_crate(&self, id: CrateLongId) -> Option<&PluginSuite> {
        self.crate_plugin_suites.get(&id)
    }

    fn remove_current_plugins_from_db(&self, db: &mut AnalysisDatabase) {
        for (crate_id, suite) in self.crate_plugin_suites.iter() {
            let interned_suite = db.intern_plugin_suite(suite.clone());
            db.remove_crate_plugin_suite(db.intern_crate(crate_id.clone()), &interned_suite);
        }
    }

    /// Sends `workspace/semanticTokens/refresh` if supported by the client to make sure macros
    /// declared by proc macros are properly colored.
    ///
    /// Usage: should be called when the set of known macros is changed and all plugins with known
    /// macros will be in the db before the mutable db reference is released.
    fn on_supported_macros_response(
        client_capabilities: &ClientCapabilities,
        requester: &mut Requester,
    ) {
        if client_capabilities.workspace_semantic_tokens_refresh_support() {
            if let Err(err) = requester.request::<SemanticTokensRefresh>((), |_| Task::nothing()) {
                error!("semantic tokens refresh failed: {err:#?}");
            }
        }
    }

    /// Tries starting proc-macro-server initialization process, if allowed by config.
    ///
    /// Returns value indicating if initialization was attempted.
    #[tracing::instrument(level = "trace", skip_all)]
    fn try_initialize(&mut self, db: &mut AnalysisDatabase, config: &Config) -> bool {
        // Keep the rate limiter check as second condition when config doesn't allow it to make
        // sure it is not impacted.
        let initialize = config.enable_proc_macros && self.initialization_retries.check().is_ok();

        if initialize {
            self.spawn_server(db);
        }

        initialize
    }

    /// Spawns proc-macro-server.
    #[tracing::instrument(level = "trace", skip_all)]
    fn spawn_server(&mut self, db: &mut AnalysisDatabase) {
        // We have to make sure that snapshots will not report errors from previous client after we
        // create new one.
        db.cancel_all();

        // Otherwise we can get messages from old client after initialization of new one.
        self.channels.clear_all();

        match self.scarb.proc_macro_server() {
            Ok(proc_macro_server) => {
                let client = ProcMacroClient::new(
                    ProcMacroServerConnection::stdio(
                        proc_macro_server,
                        self.channels.response_sender.clone(),
                    ),
                    self.channels.error_sender.clone(),
                    self.proc_macro_server_tracker.clone(),
                );

                client.start_initialize();

                db.set_proc_macro_server_status(ServerStatus::Starting(Arc::new(client)));
            }
            Err(err) => {
                error!("spawning proc-macro-server failed: {err:?}");

                self.fatal_failed(db, InitializationFailedInfo::SpawnFail);
            }
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn fatal_failed(
        &self,
        db: &mut AnalysisDatabase,
        initialization_failed_info: InitializationFailedInfo,
    ) {
        self.set_proc_macro_server_status(db, ServerStatus::Crashed);

        self.notifier.notify::<ShowMessage>(ShowMessageParams {
            typ: MessageType::ERROR,
            message: initialization_failed_info.to_string(),
        });
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn apply_responses(
        &mut self,
        db: &mut AnalysisDatabase,
        config: &Config,
        client: &ProcMacroClient,
    ) {
        let mut attribute_resolutions = Arc::unwrap_or_clone(db.attribute_macro_resolution());
        let mut attribute_resolutions_changed = false;

        let mut derive_resolutions = Arc::unwrap_or_clone(db.derive_macro_resolution());
        let mut derive_resolutions_changed = false;

        let mut inline_macro_resolutions = Arc::unwrap_or_clone(db.inline_macro_resolution());
        let mut inline_macro_resolutions_changed = false;

        let mut error_occurred = false;

        for (params, response) in client.available_responses() {
            match parse_proc_macro_response(response) {
                Ok(result) => {
                    match params {
                        RequestParams::Attribute(params) => {
                            attribute_resolutions.insert(params, result);
                            attribute_resolutions_changed = true;
                        }
                        RequestParams::Derive(params) => {
                            derive_resolutions.insert(params, result);
                            derive_resolutions_changed = true;
                        }
                        RequestParams::Inline(params) => {
                            inline_macro_resolutions.insert(params, result);
                            inline_macro_resolutions_changed = true;
                        }
                    };
                }
                Err(error) => {
                    error_occurred = true;

                    error!("{error:#?}");
                    break;
                }
            }
        }

        // This must be called AFTER `client.available_responses()` is dropped, otherwise we can
        // deadlock.
        if error_occurred {
            self.handle_error(db, config);
        }

        // Set input only if resolution changed, this way we don't recompute queries if there were
        // no updates.
        if attribute_resolutions_changed {
            db.set_attribute_macro_resolution(Arc::new(attribute_resolutions));
        }
        if derive_resolutions_changed {
            db.set_derive_macro_resolution(Arc::new(derive_resolutions));
        }
        if inline_macro_resolutions_changed {
            db.set_inline_macro_resolution(Arc::new(inline_macro_resolutions));
        }
    }
}

fn parse_proc_macro_response(response: RpcResponse) -> Result<ProcMacroResult> {
    let success = response
        .into_result()
        .map_err(|error| anyhow!("proc-macro-server responded with error: {error:?}"))?;

    serde_json::from_value(success).context("failed to deserialize response into `ProcMacroResult`")
}

#[derive(Clone)]
pub struct ProcMacroChannels {
    // A single element queue is used to notify when client occurred an error.
    error_sender: Sender<()>,

    // A single element queue is used to notify when the response queue is pushed.
    pub response_receiver: Receiver<()>,

    // A single element queue is used to notify when the response queue is pushed.
    pub response_sender: Sender<()>,

    // A single element queue is used to notify when client occurred an error.
    pub error_receiver: Receiver<()>,
}

impl ProcMacroChannels {
    fn new() -> Self {
        let (response_sender, response_receiver) = crossbeam::channel::bounded(1);
        let (error_sender, error_receiver) = crossbeam::channel::bounded(1);

        Self { response_sender, response_receiver, error_sender, error_receiver }
    }

    /// Make all channels empty in a non-blocking manner.
    fn clear_all(&self) {
        self.error_receiver.try_iter().for_each(|_| {});
        self.response_receiver.try_iter().for_each(|_| {});
    }
}

enum InitializationFailedInfo {
    NoMoreRetries,
    SpawnFail,
}

impl Display for InitializationFailedInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InitializationFailedInfo::NoMoreRetries => {
                write!(
                    f,
                    "Starting proc-macro-server failed {RESTART_RATE_LIMITER_RETRIES} times in {} \
                     minutes.",
                    RESTART_RATE_LIMITER_PERIOD_SEC / 60
                )
            }
            InitializationFailedInfo::SpawnFail => {
                write!(f, "Starting proc-macro-server failed fatally.")
            }
        }?;

        write!(
            f,
            " The proc-macro-server will not be restarted, procedural macros will not be \
             analyzed. See the output for more information"
        )
    }
}
