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
use super::db::init_proc_macro_group;
use crate::config::Config;
use crate::ide::analysis_progress::{ProcMacroServerStatus, ProcMacroServerTracker};
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::cache::load_proc_macro_cache;
use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::lang::proc_macros::plugins::proc_macro_plugin_suites;
use crate::lang::proc_macros::response_poll::ResponsePollThread;
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::server::client::{Notifier, Requester};
use crate::server::schedule::Task;
use crate::server::schedule::thread::JoinHandle;
use crate::toolchain::scarb::ScarbToolchain;
use std::path::PathBuf;

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
    cwd: PathBuf,
    _response_poll_thread: JoinHandle<()>,
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

impl Default for ProcMacroServerStatus {
    fn default() -> Self {
        (&ServerStatus::default()).into()
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
        cwd: PathBuf,
        generate_code_complete_receiver: Receiver<()>,
    ) -> Self {
        let (poll_response_sender, poll_responses_receiver) = crossbeam::channel::bounded(1);

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
            channels: ProcMacroChannels::new(poll_responses_receiver),
            cwd,
            _response_poll_thread: ResponsePollThread::spawn(
                generate_code_complete_receiver,
                poll_response_sender,
            ),
        }
    }

    /// Start proc-macro-server after config reload.
    /// Note that this will only try to go from `ClientStatus::Pending` to
    /// `ClientStatus::Starting` if config allows this.
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn on_config_change(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        if !config.enable_proc_macros {
            // Clear resolved macros if proc macro server should be disabled.
            self.remove_current_plugins_from_db(db);
            self.crate_plugin_suites.clear();

            self.clean_up_previous_proc_macro_server(db);

            init_proc_macro_group(db);
        }

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
                    drop(client);
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
                    db,
                    client_capabilities,
                    config,
                    requester,
                );
            }
            ServerStatus::Ready(client) => {
                self.apply_responses(db, config, client);
            }
            _ => {}
        }
    }

    pub fn proc_macro_plugin_suite_for_crate(&self, id: &CrateLongId) -> Option<&PluginSuite> {
        self.crate_plugin_suites.get(id)
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
        db: &mut AnalysisDatabase,
        client_capabilities: &ClientCapabilities,
        config: &Config,
        requester: &mut Requester,
    ) {
        if client_capabilities.workspace_semantic_tokens_refresh_support() {
            if let Err(err) = requester.request::<SemanticTokensRefresh>((), |_| Task::nothing()) {
                error!("semantic tokens refresh failed: {err:#?}");
            }
        }

        // Try loading proc macro cache if availale.
        load_proc_macro_cache(db, config);
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
        self.clean_up_previous_proc_macro_server(db);

        match self.scarb.proc_macro_server(&self.cwd) {
            Ok(proc_macro_server) => {
                let client = ProcMacroClient::new(
                    ProcMacroServerConnection::stdio(proc_macro_server),
                    self.channels.error_sender.clone(),
                    self.proc_macro_server_tracker.clone(),
                );

                client.start_initialize();

                self.set_proc_macro_server_status(db, ServerStatus::Starting(Arc::new(client)));
            }
            Err(err) => {
                error!("spawning proc-macro-server failed: {err:?}");

                self.fatal_failed(db, InitializationFailedInfo::SpawnFail);
            }
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    /// NOTE: while this function is being called, there **MUST NOT** exist
    /// any [`Arc`]s with [`ProcMacroClient`] anywhere in this thread except in the `db`.
    fn clean_up_previous_proc_macro_server(&mut self, db: &mut AnalysisDatabase) {
        // We have to make sure that snapshots will not report errors from the previous client after
        // we create a new one.
        db.cancel_all();

        // At this point we are the only thread with access to the db and therefore
        // to the proc macro client.
        if let ServerStatus::Starting(client) | ServerStatus::Ready(client) =
            db.proc_macro_server_status()
        {
            // Make the db drop the strong reference to the proc macro client.
            self.set_proc_macro_server_status(db, ServerStatus::Pending);

            let client = Arc::try_unwrap(client)
                .expect("only one strong reference to client is expected at this point");

            // This has to be done *before* clearing channels, so we don't receive a response signal
            // from the old proc macro server when we come back to the main event loop.
            client.kill_proc_macro_server();
        }

        // Otherwise we can get messages from the old client after an initialization of the new one.
        self.channels.clear_all();

        // Otherwise we can be leaved with counter that can never go to 0.
        self.proc_macro_server_tracker.reset_requests_counter();
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
        client: Arc<ProcMacroClient>,
    ) {
        let mut attribute_resolutions = Arc::unwrap_or_clone(db.attribute_macro_resolution());
        let mut attribute_resolutions_changed = false;

        let mut derive_resolutions = Arc::unwrap_or_clone(db.derive_macro_resolution());
        let mut derive_resolutions_changed = false;

        let mut inline_macro_resolutions = Arc::unwrap_or_clone(db.inline_macro_resolution());
        let mut inline_macro_resolutions_changed = false;

        let mut error_occurred = false;

        let available_responses = client.available_responses();

        let request_count = available_responses.len() as u64;

        for (params, response) in available_responses {
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
            drop(client);
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

        self.proc_macro_server_tracker.mark_requests_as_handled(request_count)
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

    // A single element queue is used to notify when client occurred an error.
    pub error_receiver: Receiver<()>,

    // A single element queue is used to notify when responses should be applied.
    pub poll_responses_receiver: Receiver<()>,
}

impl ProcMacroChannels {
    fn new(poll_responses_receiver: Receiver<()>) -> Self {
        let (error_sender, error_receiver) = crossbeam::channel::bounded(1);

        Self { error_sender, error_receiver, poll_responses_receiver }
    }

    /// Make all channels empty in a non-blocking manner.
    fn clear_all(&self) {
        self.error_receiver.try_iter().for_each(|_| {});
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
