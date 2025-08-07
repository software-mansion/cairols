//! # CairoLS
//!
//! Implements the LSP protocol over stdin/out.
//!
//! ## Running vanilla
//!
//! This is basically the source code of the `cairo-language-server` and
//! `scarb cairo-language-server` binaries.
//!
//! ```no_run
//! # #![allow(clippy::needless_doctest_main)]
//! fn main() {
//!     cairo_language_server::start();
//! }
//! ```

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::SystemTime;
use std::{io, panic};

use anyhow::Result;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::FileLongId;
use crossbeam::channel::{Receiver, select_biased};
use lsp_server::Message;
use lsp_types::RegistrationParams;
use lsp_types::request::SemanticTokensRefresh;
use tracing::{debug, error, info};

use crate::ide::analysis_progress::AnalysisStatus;
use crate::ide::code_lens::CodeLensController;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::proc_macros;
use crate::lang::proc_macros::client::ServerStatus;
use crate::lang::proc_macros::controller::ProcMacroChannels;
use crate::lang::proc_macros::db::ProcMacroGroup;
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::lsp::capabilities::server::{
    collect_dynamic_registrations, collect_server_capabilities,
};
use crate::lsp::result::LSPResult;
use crate::project::{ProjectController, ProjectUpdate};
use crate::server::client::{Notifier, Requester, Responder};
use crate::server::connection::{Connection, ConnectionInitializer};
use crate::server::panic::is_cancelled;
use crate::server::schedule::thread::JoinHandle;
use crate::server::schedule::{Scheduler, Task, event_loop_thread};
use crate::state::{MetaState, State};

mod config;
mod env_config;
mod ide;
mod lang;
pub mod lsp;
mod project;
mod server;
mod state;
#[cfg(feature = "testing")]
pub mod testing;
mod toolchain;

/// Starts the language server.
///
/// See [the top-level documentation][lib] documentation for usage examples.
///
/// [lib]: crate#running-vanilla
pub fn start() -> ExitCode {
    let _log_guard = init_logging();
    set_panic_hook();

    info!("language server starting");
    env_config::report_to_logs();

    let exit_code = match Backend::new() {
        Ok(backend) => {
            if let Err(err) = backend.run().map(|handle| handle.join()) {
                error!("language server encountered an unrecoverable error: {err}");
                ExitCode::from(1)
            } else {
                ExitCode::from(0)
            }
        }
        Err(err) => {
            error!("language server failed during initialization: {err}");
            ExitCode::from(1)
        }
    };

    info!("language server stopped");
    exit_code
}

/// Initialize logging infrastructure for the language server.
///
/// Returns a guard that should be dropped when the LS ends, to flush log files.
fn init_logging() -> Option<impl Drop> {
    use std::fs;
    use std::io::IsTerminal;

    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::filter::{EnvFilter, LevelFilter, Targets};
    use tracing_subscriber::fmt::Layer;
    use tracing_subscriber::fmt::time::Uptime;
    use tracing_subscriber::prelude::*;

    let mut guard = None;

    let fmt_layer = Layer::new()
        .with_writer(io::stderr)
        .with_timer(Uptime::default())
        .with_ansi(io::stderr().is_terminal())
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .with_env_var(env_config::CAIRO_LS_LOG)
                .from_env_lossy(),
        );

    let profile_layer = if env_config::tracing_profile() {
        let mut path = PathBuf::from(format!(
            "./cairols-profile-{}.json",
            SystemTime::UNIX_EPOCH.elapsed().unwrap().as_micros()
        ));

        // Create the file now, so that we early panic, and `fs::canonicalize` will work.
        let profile_file = fs::File::create(&path).expect("Failed to create profile file.");

        // Try to canonicalize the path, so that it's easier to find the file from logs.
        if let Ok(canonical) = fs::canonicalize(&path) {
            path = canonical;
        }

        eprintln!("this LS run will output tracing profile to: {}", path.display());
        eprintln!(
            "open that file with https://ui.perfetto.dev (or chrome://tracing) to analyze it"
        );

        let (profile_layer, profile_layer_guard) =
            ChromeLayerBuilder::new().writer(profile_file).include_args(true).build();

        // Filter out less important Salsa logs because they are too verbose,
        // and with them the profile file quickly grows to several GBs of data.
        let profile_layer = profile_layer.with_filter(
            Targets::new().with_default(LevelFilter::TRACE).with_target("salsa", LevelFilter::WARN),
        );

        guard = Some(profile_layer_guard);
        Some(profile_layer)
    } else {
        None
    };

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry().with(fmt_layer).with(profile_layer),
    )
    .expect("Could not set up global logger.");

    guard
}

/// Sets a special panic hook that skips execution for Salsa cancellation panics.
fn set_panic_hook() {
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        if !is_cancelled(info.payload()) {
            previous_hook(info);
        }
    }))
}

struct Backend {
    connection: Connection,
    state: State,
}

impl Backend {
    fn new() -> Result<Self> {
        let connection_initializer = ConnectionInitializer::stdio();

        Self::initialize(connection_initializer, std::env::current_dir()?)
    }

    /// Initializes the connection and crate a ready to run [`Backend`] instance.
    ///
    /// As part of the initialization flow, this function exchanges client and server capabilities.
    fn initialize(connection_initializer: ConnectionInitializer, cwd: PathBuf) -> Result<Self> {
        let (id, init_params) = connection_initializer.initialize_start()?;

        let client_capabilities = init_params.capabilities;
        let server_capabilities = collect_server_capabilities(&client_capabilities);

        let connection = connection_initializer.initialize_finish(id, server_capabilities)?;
        let state = State::new(connection.make_sender(), client_capabilities, cwd);

        Ok(Self { connection, state })
    }

    /// Runs the main event loop thread and wait for its completion.
    fn run(self) -> Result<JoinHandle<Result<()>>> {
        event_loop_thread(move || {
            let Self { mut state, connection } = self;
            let proc_macro_channels = state.proc_macro_controller.channels();
            let project_updates_receiver = state.project_controller.response_receiver();
            let analysis_progress_receiver =
                state.analysis_progress_controller.get_status_receiver();
            let code_lens_request_refresh_receiver =
                state.code_lens_controller.request_refresh_receiver();

            let mut scheduler = Scheduler::new(&mut state, connection.make_sender());

            Self::dispatch_setup_tasks(&mut scheduler);

            // Notify the swapper about state mutation.
            scheduler.on_sync_mut_task(Self::register_mutation_in_swapper);

            // Attempt to swap the database to reduce memory use.
            // Because diagnostics are always refreshed afterwards, the fresh database state will
            // be quickly repopulated.
            scheduler.on_sync_mut_task(Self::maybe_swap_database);

            // Refresh diagnostics each time state changes.
            // Although it is possible to mutate state without affecting the analysis database,
            // we basically never hit such a case in CairoLS in happy paths.
            scheduler.on_sync_mut_task(Self::refresh_diagnostics);

            // Keep it last, marks that db mutation might happened.
            scheduler.on_sync_mut_task(|state, _, _| {
                state.analysis_progress_controller.mutation();
            });

            let result = Self::event_loop(
                &connection,
                proc_macro_channels,
                project_updates_receiver,
                analysis_progress_receiver,
                code_lens_request_refresh_receiver,
                scheduler,
            );

            state.db.cancel_all();

            if let Err(err) = connection.close() {
                error!("failed to close connection to the language server: {err:?}");
            }

            result
        })
    }

    /// Runs various setup tasks before entering the main event loop.
    fn dispatch_setup_tasks(scheduler: &mut Scheduler<'_>) {
        scheduler.local_mut(Self::register_dynamic_capabilities);

        scheduler.local_mut(|state, _notifier, requester, _responder| {
            let _ = state.config.reload_on_start(
                requester,
                &mut state.db,
                &mut state.proc_macro_controller,
                &mut state.analysis_progress_controller,
                &state.client_capabilities,
            );
        });
    }

    fn register_dynamic_capabilities(
        state: &mut State,
        _notifier: Notifier,
        requester: &mut Requester<'_>,
        _responder: Responder,
    ) {
        let registrations = collect_dynamic_registrations(&state.client_capabilities);

        let _ = requester
            .request::<lsp_types::request::RegisterCapability>(
                RegistrationParams { registrations },
                |()| {
                    debug!("capabilities successfully registered dynamically");
                    Task::nothing()
                },
            )
            .inspect_err(|e| {
                error!(
                    "failed to register dynamic capabilities, some features may not work \
                     properly: {e:?}"
                )
            });
    }

    // +--------------------------------------------------+
    // | Function code adopted from:                      |
    // | Repository: https://github.com/astral-sh/ruff    |
    // | File: `crates/ruff_server/src/server.rs`         |
    // | Commit: 46a457318d8d259376a2b458b3f814b9b795fe69 |
    // +--------------------------------------------------+
    fn event_loop(
        connection: &Connection,
        proc_macro_channels: ProcMacroChannels,
        project_updates_receiver: Receiver<ProjectUpdate>,
        analysis_progress_status_receiver: Receiver<AnalysisStatus>,
        code_lens_request_refresh_receiver: Receiver<()>,
        mut scheduler: Scheduler<'_>,
    ) -> Result<()> {
        let incoming = connection.incoming();

        loop {
            select_biased! {
                // Project updates may significantly change the state, therefore
                // they should be handled first in case of multiple operations being ready at once.
                // To ensure it, keep project updates channel in the first arm of `select_biased!`.
                recv(project_updates_receiver) -> project_update => {
                    let Ok(project_update) = project_update else { break };

                    scheduler.local_mut(move |state, notifier, _, _| ProjectController::handle_update(state, notifier, project_update));
                }
                recv(incoming) -> msg => {
                    let Ok(msg) = msg else { break };

                    if connection.handle_shutdown(&msg)? {
                        break;
                    }
                    let task = match msg {
                        Message::Request(req) => server::request(req),
                        Message::Notification(notification) => server::notification(notification),
                        Message::Response(response) => scheduler.response(response),
                    };
                    scheduler.dispatch(task);
                }
                recv(proc_macro_channels.poll_responses_receiver) -> response => {
                    let Ok(()) = response else { break };

                    scheduler.local_with_precondition(Self::proc_macro_response_check, Self::on_proc_macro_response);
                }
                recv(proc_macro_channels.error_receiver) -> error => {
                    let Ok(()) = error else { break };

                    scheduler.local_mut(Self::on_proc_macro_error);
                }
                recv(analysis_progress_status_receiver) -> analysis_progress_status => {
                    let Ok(analysis_status) = analysis_progress_status else { break };

                    let mut meta_state = scheduler.meta_state
                                .lock()
                                .expect(META_STATE_NOT_ACQUIRED_MSG);
                    meta_state.analysis_status = Some(analysis_status);

                    match analysis_status {
                        AnalysisStatus::Started => {
                            meta_state
                                .db_swapper
                                .start_stopwatch();

                        }
                        AnalysisStatus::Finished => {
                            meta_state.db_swapper
                                .stop_stopwatch();

                            drop(meta_state);

                            scheduler.local(|state, _, _notifier, requester, _responder|
                                Self::on_stopped_analysis(state, requester)
                            );
                        }
                    };
                }
                recv(code_lens_request_refresh_receiver) -> error => {
                    let Ok(()) = error else { break };

                    scheduler.local(|_, _, _, requester, _| {
                        CodeLensController::handle_refresh(requester);
                    });
                }
            }
        }

        Ok(())
    }

    /// Calls [`lang::proc_macros::controller::ProcMacroClientController::handle_error`] to do its
    /// work.
    fn on_proc_macro_error(state: &mut State, _: Notifier, _: &mut Requester<'_>, _: Responder) {
        state.proc_macro_controller.handle_error(&mut state.db, &state.config);
    }

    fn proc_macro_response_check(state: &State) -> bool {
        if let ServerStatus::Starting(client) | ServerStatus::Ready(client) =
            state.db.proc_macro_server_status()
        {
            client.available_responses().len() != 0
        } else {
            false
        }
    }

    /// Calls [`lang::proc_macros::controller::ProcMacroClientController::on_response`] to do its
    /// work.
    fn on_proc_macro_response(state: &mut State, _: Notifier, _: &mut Requester<'_>, _: Responder) {
        state.proc_macro_controller.on_response(&mut state.db, &state.config);
    }

    fn on_stopped_analysis(state: &State, requester: &mut Requester<'_>) {
        proc_macros::cache::save_proc_macro_cache(&state.db, &state.config);
        state
            .code_lens_controller
            .schedule_refreshing_all_lenses(state.db.clone(), state.config.clone());

        if state.client_capabilities.workspace_semantic_tokens_refresh_support()
            && let Err(err) = requester.request::<SemanticTokensRefresh>((), |_| Task::nothing())
        {
            error!("semantic tokens refresh failed: {err:#?}");
        }
    }

    fn register_mutation_in_swapper(
        _state: &mut State,
        meta_state: MetaState,
        _notifier: Notifier,
    ) {
        meta_state.lock().expect(META_STATE_NOT_ACQUIRED_MSG).db_swapper.register_mutation();
    }

    /// Calls [`lang::db::AnalysisDatabaseSwapper::maybe_swap`] to do its work.
    fn maybe_swap_database(state: &mut State, meta_state: MetaState, _notifier: Notifier) {
        meta_state.lock().expect(META_STATE_NOT_ACQUIRED_MSG).db_swapper.maybe_swap(
            &mut state.db,
            &state.open_files,
            &mut state.project_controller,
            &state.proc_macro_controller,
        );
    }

    /// Calls [`lang::diagnostics::DiagnosticsController::refresh`] to do its work.
    fn refresh_diagnostics(state: &mut State, _meta_state: MetaState, _notifier: Notifier) {
        state.diagnostics_controller.refresh(state);
    }

    /// Reload config and update project model for all open files.
    fn reload(state: &mut State, requester: &mut Requester<'_>) -> LSPResult<()> {
        state.project_controller.clear_loaded_workspaces();
        state.config.reload(requester, &state.client_capabilities)?;

        for uri in state.open_files.iter() {
            let Some(file_id) = state.db.file_for_url(uri) else { continue };
            if let FileLongId::OnDisk(file_path) = state.db.lookup_intern_file(file_id) {
                state.project_controller.request_updating_project_for_file(file_path);
            }
        }

        Ok(())
    }
}
const META_STATE_NOT_ACQUIRED_MSG: &str = "should be able to acquire the MetaState";
