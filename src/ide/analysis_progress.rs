use std::cmp::PartialEq;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::lsp::ext::{ServerStatus, ServerStatusEvent, ServerStatusParams};
use crate::server::client::Notifier;
use crossbeam::channel::{Receiver, Sender};

#[derive(Clone, PartialEq)]
pub enum ProcMacroServerStatus {
    Pending,
    Starting,
    Ready,
    Crashed,
}

/// A struct that allows to track procmacro requests.
#[derive(Clone)]
pub struct ProcMacroServerTracker {
    procmacro_request_submitted: Arc<AtomicBool>,
    // A field indicating if any proc macro request were sent during the previous diagnostics round.
    procmacro_request_submitted_previously: Arc<AtomicBool>,
    procmacro_request_counter: Arc<AtomicU64>,
    procmacro_server_status: Arc<Mutex<ProcMacroServerStatus>>,
}

impl ProcMacroServerTracker {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            procmacro_request_submitted: Arc::new(AtomicBool::new(false)),
            procmacro_request_submitted_previously: Arc::new(AtomicBool::new(false)),
            procmacro_request_counter: Arc::new(AtomicU64::new(0)),
            procmacro_server_status: Arc::new(Mutex::new(ProcMacroServerStatus::Pending)),
        }
    }

    /// Signals that a request to proc macro server was made during the current generation of
    /// diagnostics.
    pub fn register_procmacro_request(&self) {
        self.procmacro_request_submitted.store(true, Ordering::SeqCst);
        self.procmacro_request_counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn set_server_status(&self, status: ProcMacroServerStatus) {
        let mut guard = self.procmacro_server_status.lock().unwrap();
        *guard = status;
    }

    pub fn get_server_status(&self) -> ProcMacroServerStatus {
        (*(self.procmacro_server_status.lock().unwrap())).clone()
    }

    pub fn mark_requests_as_handled(&self, requests_count: u64) {
        self.procmacro_request_counter.fetch_sub(requests_count, Ordering::SeqCst);
    }

    pub fn reset_request_tracker(&self) {
        self.procmacro_request_submitted.store(false, Ordering::SeqCst);
    }

    pub fn reset_previous_request_tracker(&self) {
        self.procmacro_request_submitted_previously.store(false, Ordering::SeqCst);
    }

    pub fn store_previous_request_tracker(&self) {
        let previous_value = self.procmacro_request_submitted.load(Ordering::SeqCst);

        self.procmacro_request_submitted_previously.store(previous_value, Ordering::SeqCst);
    }

    pub fn get_did_submit_procmacro_request_previously(&self) -> bool {
        self.procmacro_request_submitted_previously.load(Ordering::SeqCst)
    }

    pub fn get_did_submit_procmacro_request(&self) -> bool {
        self.procmacro_request_submitted.load(Ordering::SeqCst)
            && self.procmacro_request_counter.load(Ordering::SeqCst) != 0
    }
}

#[derive(Clone)]
pub struct AnalysisProgressController {
    state: Arc<Mutex<AnalysisProgressControllerState>>,
    server_tracker: ProcMacroServerTracker,
}

impl AnalysisProgressController {
    pub fn get_update_receiver(&self) -> Receiver<AnalysisProgressStatus> {
        self.state.lock().unwrap().get_update_receiver()
    }

    pub fn on_config_change(&self, config: &Config) {
        self.state.lock().unwrap().on_config_change(config)
    }

    pub fn try_start_analysis(&self) {
        self.server_tracker.reset_request_tracker();
        self.state.lock().unwrap().try_start_analysis()
    }

    pub fn try_stop_analysis(&self) {
        let stopped = self.state.lock().unwrap().try_stop_analysis(
            self.server_tracker.get_did_submit_procmacro_request(),
            self.server_tracker.get_did_submit_procmacro_request_previously(),
            self.server_tracker.get_server_status(),
        );
        if stopped {
            self.server_tracker.reset_previous_request_tracker();
        } else {
            self.server_tracker.store_previous_request_tracker();
        }
    }
}

impl AnalysisProgressController {
    pub fn new(notifier: Notifier, server_tracker: ProcMacroServerTracker) -> Self {
        Self {
            server_tracker,
            state: Arc::new(Mutex::new(AnalysisProgressControllerState::new(notifier))),
        }
    }
}

// We don't need to track starts for now
#[derive(PartialEq)]
pub enum AnalysisProgressStatus {
    ResolvedAllProcMacros,
}

#[derive(Clone)]
struct AnalysisProgressUpdateChannels {
    sender: Sender<AnalysisProgressStatus>,
    receiver: Receiver<AnalysisProgressStatus>,
}

impl AnalysisProgressUpdateChannels {
    fn new() -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();
        Self { sender, receiver }
    }
}

/// Controller used to send notifications to the client about analysis progress.
/// Uses information provided from other controllers (diagnostics controller, procmacro controller)
/// to assess if diagnostics are in fact calculated.
#[derive(Clone)]
struct AnalysisProgressControllerState {
    notifier: Notifier,
    update_channels: AnalysisProgressUpdateChannels,
    /// Indicates that a notification was sent and analysis (i.e. macro expansion) is taking place.
    analysis_in_progress: bool,
    /// Loaded asynchronously from config
    procmacros_enabled: Option<bool>,
}

impl AnalysisProgressControllerState {
    fn new(notifier: Notifier) -> Self {
        let update_channels = AnalysisProgressUpdateChannels::new();
        Self { notifier, update_channels, analysis_in_progress: false, procmacros_enabled: None }
    }

    pub fn on_config_change(&mut self, config: &Config) {
        self.procmacros_enabled = Some(config.enable_proc_macros);
    }

    fn try_start_analysis(&mut self) {
        if !self.analysis_in_progress {
            self.analysis_in_progress = true;
            self.notifier.notify::<ServerStatus>(ServerStatusParams {
                event: ServerStatusEvent::AnalysisStarted,
                idle: false,
            });
        }
    }

    /// Returns `true` if the analysis was stopped and notification was sent.
    fn try_stop_analysis(
        &mut self,
        did_submit_procmacro_request: bool,
        did_submit_procmacro_request_previously: bool,
        proc_macro_server_status: ProcMacroServerStatus,
    ) -> bool {
        if !did_submit_procmacro_request && did_submit_procmacro_request_previously {
            self.update_channels.sender.send(AnalysisProgressStatus::ResolvedAllProcMacros).unwrap()
        }

        if !self.has_analysis_finished(did_submit_procmacro_request, proc_macro_server_status) {
            return false;
        }

        self.analysis_in_progress = false;
        self.notifier.notify::<ServerStatus>(ServerStatusParams {
            event: ServerStatusEvent::AnalysisFinished,
            idle: true,
        });
        true
    }

    // WARNING: This should not be polled from outside the diagnostics main thread, it may yield inaccurate results
    fn has_analysis_finished(
        &self,
        did_submit_procmacro_request: bool,
        proc_macro_server_status: ProcMacroServerStatus,
    ) -> bool {
        let config_not_loaded = self.procmacros_enabled.is_none();
        let is_ready = proc_macro_server_status == ProcMacroServerStatus::Ready;
        ((!did_submit_procmacro_request && is_ready)
            || config_not_loaded
            || (self.procmacros_enabled == Some(false)))
            && self.analysis_in_progress
    }

    fn get_update_receiver(&self) -> Receiver<AnalysisProgressStatus> {
        self.update_channels.receiver.clone()
    }
}
