use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use cairo_lang_semantic::plugin::PluginSuite;
use crossbeam::channel::{Receiver, Sender};
use governor::clock::QuantaClock;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use scarb_proc_macro_server_types::jsonrpc::RpcResponse;
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use tracing::error;

use super::client::connection::ProcMacroServerConnection;
use super::client::status::ClientStatus;
use super::client::{ProcMacroClient, RequestParams};
use crate::config::Config;
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::lang::proc_macros::plugins::proc_macro_plugin_suite;
use crate::lsp::ext::{
    ProcMacroServerInitializationFailed, ProcMacroServerInitializationFailedParams,
};
use crate::server::client::Notifier;
use crate::toolchain::scarb::ScarbToolchain;

const RATE_LIMITER_PERIOD_SEC: u64 = 180;
const RATE_LIMITER_RETRIES: u32 = 5;

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
    plugin_suite: Option<PluginSuite>,
    initialization_retries: RateLimiter<NotKeyed, InMemoryState, QuantaClock>,
    channels: ProcMacroChannels,
}

impl ProcMacroClientController {
    pub fn channels(&mut self) -> ProcMacroChannels {
        self.channels.clone()
    }

    pub fn new(scarb: ScarbToolchain, notifier: Notifier) -> Self {
        Self {
            scarb,
            notifier,
            plugin_suite: Default::default(),
            initialization_retries: RateLimiter::direct(
                Quota::with_period(Duration::from_secs(
                    RATE_LIMITER_PERIOD_SEC / RATE_LIMITER_RETRIES as u64,
                ))
                .unwrap()
                .allow_burst(
                    // All retries can be used as fast as possible.
                    NonZeroU32::new(RATE_LIMITER_RETRIES).unwrap(),
                ),
            ),
            channels: ProcMacroChannels::new(),
        }
    }

    /// Start proc-macro-server after config reload.
    /// Note that this will only try to go from `ClientStatus::Pending` to
    /// `ClientStatus::Starting` if config allows this.
    pub fn on_config_change(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        if db.proc_macro_client_status().is_pending() {
            self.try_initialize(db, config);
        }
    }

    /// Forcibly restarts the proc-macro-server, shutting down any currently running instances.
    ///
    /// A new server instance is only started if there are available restart attempts left.
    /// This ensures that a fresh proc-macro-server is used.
    pub fn force_restart(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        // We have to make sure that snapshots will not report errors from previous client after we
        // create new one.
        db.cancel_all();

        // Otherwise we can get messages from old client after initialization of new one.
        self.channels.clear_all();

        if let Some(plugin_suite) = self.plugin_suite.take() {
            db.remove_plugin_suite(plugin_suite);
        }

        self.try_initialize(db, config);
    }

    /// Check if an error was reported. If so, try to restart.
    pub fn handle_error(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        if !self.try_initialize(db, config) {
            self.fatal_failed(db, ProcMacroServerInitializationFailedParams::NoMoreRetries {
                retries: RATE_LIMITER_RETRIES,
                in_minutes: RATE_LIMITER_PERIOD_SEC / 60,
            });
        }
    }

    /// If the client is ready, apply all available responses.
    pub fn on_response(&mut self, db: &mut AnalysisDatabase, config: &Config) {
        match db.proc_macro_client_status() {
            ClientStatus::Starting(client) => {
                let Ok(defined_macros) = client.finish_initialize() else {
                    self.handle_error(db, config);

                    return;
                };

                let new_plugin_suite = proc_macro_plugin_suite(defined_macros);

                // Store current plugins for identity comparison, so we can remove them if we
                // restart proc-macro-server.
                let previous_plugin_suite = self.plugin_suite.replace(new_plugin_suite.clone());

                if let Some(previous_plugin_suite) = previous_plugin_suite {
                    db.replace_plugin_suite(previous_plugin_suite, new_plugin_suite);
                } else {
                    db.add_plugin_suite(new_plugin_suite);
                }

                db.set_proc_macro_client_status(ClientStatus::Ready(client));
            }
            ClientStatus::Ready(client) => {
                self.apply_responses(db, config, &client);
            }
            _ => {}
        }
    }

    /// Tries starting proc-macro-server initialization process, if allowed by config.
    ///
    /// Returns value indicating if initialization was attempted.
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
    fn spawn_server(&mut self, db: &mut AnalysisDatabase) {
        match self.scarb.proc_macro_server() {
            Ok(proc_macro_server) => {
                let client = ProcMacroClient::new(
                    ProcMacroServerConnection::stdio(
                        proc_macro_server,
                        self.channels.response_sender.clone(),
                    ),
                    self.channels.error_sender.clone(),
                );

                client.start_initialize();

                db.set_proc_macro_client_status(ClientStatus::Starting(Arc::new(client)));
            }
            Err(err) => {
                error!("spawning proc-macro-server failed: {err:?}");

                self.fatal_failed(db, ProcMacroServerInitializationFailedParams::SpawnFail);
            }
        }
    }

    fn fatal_failed(
        &self,
        db: &mut AnalysisDatabase,
        params: ProcMacroServerInitializationFailedParams,
    ) {
        db.set_proc_macro_client_status(ClientStatus::Crashed);

        self.notifier.notify::<ProcMacroServerInitializationFailed>(params);
    }

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
    response_sender: Sender<()>,

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
