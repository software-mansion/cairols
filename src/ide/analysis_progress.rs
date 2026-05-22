use std::cmp::PartialEq;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crossbeam::channel::{Receiver, Sender};

use crate::config::Config;
use crate::lsp::ext::{ServerStatus, ServerStatusEvent, ServerStatusParams};
use crate::server::client::Notifier;
use crate::server::schedule::thread::{self, JoinHandle, ThreadPriority};

#[derive(Copy, Clone, PartialEq)]
pub enum ProcMacroServerStatus {
    Pending,
    Connected,
    Crashed,
}

/// A struct that allows to track procmacro requests.
#[derive(Clone)]
pub struct ProcMacroServerTracker {
    procmacro_request_counter: Arc<AtomicU64>,
    events_sender: Sender<AnalysisEvent>,
}

impl ProcMacroServerTracker {
    fn new(events_sender: Sender<AnalysisEvent>) -> Self {
        Self { procmacro_request_counter: Arc::new(AtomicU64::new(0)), events_sender }
    }

    /// Signals that a request to proc macro server was made.
    pub fn register_procmacro_request(&self) {
        self.procmacro_request_counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn set_server_status(&self, status: ProcMacroServerStatus) {
        let _ = self.events_sender.send(AnalysisEvent::PMSStatusChange(status));
    }

    pub fn mark_requests_as_handled(&self, response_count: u64) {
        let _ = self.events_sender.send(AnalysisEvent::ApplyResponses { response_count });
    }

    pub fn register_defined_macros_request(&self) {
        let _ = self.events_sender.send(AnalysisEvent::DefinedMacrosRequested);
    }

    /// Atomically signals that the server transitioned to Connected and that `count`
    /// DefinedMacros requests are already in flight. Using a single event prevents
    /// a `DiagnosticsTickEnd` from slipping between the status-change and the
    /// individual `DefinedMacrosRequested` events, which would cause a spurious
    /// `AnalysisFinished` while macros are still loading.
    pub fn set_server_connected_with_pending_defined_macros(&self, count: usize) {
        let _ =
            self.events_sender.send(AnalysisEvent::PMSConnectedWithDefinedMacros { count });
    }

    pub fn register_proc_macros_request_handled(&self) {
        let _ = self.events_sender.send(AnalysisEvent::DefinedMacrosResponseReceived);
    }

    pub fn reset_requests_counter(&self) {
        self.procmacro_request_counter.store(0, Ordering::SeqCst)
    }
}

#[derive(Clone)]
pub struct AnalysisProgressController {
    server_tracker: ProcMacroServerTracker,
    status_receiver: Receiver<AnalysisStatus>,
    // Keep it last for drop.
    _status_thread: Arc<JoinHandle<()>>,
}

impl AnalysisProgressController {
    pub fn new(notifier: Notifier) -> Self {
        let (status_sender, status_receiver) = crossbeam::channel::unbounded();
        let (events_sender, events_receiver) = crossbeam::channel::unbounded();
        let server_tracker = ProcMacroServerTracker::new(events_sender);
        let status_thread = AnalysisProgressThread::spawn(events_receiver, status_sender, notifier);

        Self { server_tracker, status_receiver, _status_thread: Arc::new(status_thread) }
    }

    pub fn get_status_receiver(&self) -> Receiver<AnalysisStatus> {
        self.status_receiver.clone()
    }

    pub fn on_config_change(&self, config: &Config) {
        self.send(AnalysisEvent::ConfigLoad { enable_proc_macros: config.enable_proc_macros });
    }

    pub fn diagnostic_start(&self) {
        self.send(AnalysisEvent::DiagnosticsTickStart);
    }

    pub fn diagnostic_end(&self, was_cancelled: bool) {
        self.send(AnalysisEvent::DiagnosticsTickEnd {
            was_cancelled,
            all_request_count: self.server_tracker.procmacro_request_counter.load(Ordering::SeqCst),
        });
    }

    pub fn project_model_loaded(&self) {
        self.send(AnalysisEvent::ProjectLoaded);
    }

    pub fn server_tracker(&self) -> ProcMacroServerTracker {
        self.server_tracker.clone()
    }

    fn send(&self, event: AnalysisEvent) {
        let _ = self.server_tracker.events_sender.send(event);
    }
}

#[derive(PartialEq)]
pub enum AnalysisStatus {
    Started,
    Finished,
}

pub enum AnalysisEvent {
    ConfigLoad {
        /// Loaded asynchronously from config
        enable_proc_macros: bool,
    },
    ApplyResponses {
        response_count: u64,
    },
    DiagnosticsTickStart,
    DiagnosticsTickEnd {
        was_cancelled: bool,
        /// Number of all requests sent to this point from the moment PMS was started. It is NOT only from this tick.
        all_request_count: u64,
    },
    PMSStatusChange(ProcMacroServerStatus),
    /// Combines a Pending→Connected status transition with N pre-counted DefinedMacros requests,
    /// preventing a DiagnosticsTickEnd from racing between the two events.
    PMSConnectedWithDefinedMacros { count: usize },
    ProjectLoaded,
    DefinedMacrosRequested,
    DefinedMacrosResponseReceived,
}

struct AnalysisProgressThread {
    events_receiver: Receiver<AnalysisEvent>,
    status_sender: Sender<AnalysisStatus>,
    notifier: Notifier,
}

impl AnalysisProgressThread {
    pub fn spawn(
        events_receiver: Receiver<AnalysisEvent>,
        status_sender: Sender<AnalysisStatus>,
        notifier: Notifier,
    ) -> JoinHandle<()> {
        let this = Self { events_receiver, status_sender, notifier };

        thread::Builder::new(ThreadPriority::Worker)
            .name("cairo-ls:analysis-progress".into())
            .spawn(move || this.event_loop())
            .expect("failed to spawn analysis progress thread")
    }

    fn event_loop(self) {
        let mut analysis_in_progress = false;

        let mut project_loaded = false;
        let mut enable_proc_macros = Config::ENABLE_PROC_MACROS_DEFAULT;
        // To prevent underflow on u64 substraction (in case where [`AnalysisEvent::ApplyResponses`] comes before [`AnalysisEvent::DiagnosticsTickStart`]) use i128.
        let mut all_prev_requests_count = 0_i128;
        let mut received_responses = 0_i128;
        let mut pending_requests = 0_i128;
        // Tracks in-flight DefinedMacros requests so we don't fire AnalysisFinished while
        // the proc-macro server is still loading macros and hasn't responded yet.
        let mut defined_macros_pending = 0_i128;
        // Set when macros finish loading; cleared by the next DiagnosticsTickStart.
        // Ensures AnalysisFinished is not fired until a fresh tick that sees the loaded macros
        // has completed, so diagnostics returned to the client are never stale.
        let mut requires_post_macro_tick = false;
        let mut pms_status = ProcMacroServerStatus::default();

        let finish_analysis_if_ready =
            |analysis_in_progress: &mut bool,
             was_cancelled: bool,
             request_count: i128,
             received_responses: i128,
             pending_requests: i128,
             defined_macros_pending: i128,
             requires_post_macro_tick: bool,
             enable_proc_macros: bool,
             pms_status: ProcMacroServerStatus| {
                if *analysis_in_progress
                    && (!enable_proc_macros
                        || (pms_status == ProcMacroServerStatus::Connected
                            && pending_requests == 0
                            && defined_macros_pending == 0
                            && !requires_post_macro_tick))
                    && (!was_cancelled && request_count == received_responses)
                {
                    self.notifier.notify::<ServerStatus>(ServerStatusParams {
                        event: ServerStatusEvent::AnalysisFinished,
                    });
                    let _ = self.status_sender.send(AnalysisStatus::Finished);

                    *analysis_in_progress = false;
                }
            };

        while let Ok(event) = self.events_receiver.recv() {
            match event {
                AnalysisEvent::ConfigLoad { enable_proc_macros: new_enable_proc_macros } => {
                    enable_proc_macros = new_enable_proc_macros;

                    // Mutation event will happen after this, so no need to restart analysis here.
                }
                AnalysisEvent::ApplyResponses { response_count } => {
                    // Response count is delta, add it.
                    received_responses += response_count as i128;
                    finish_analysis_if_ready(
                        &mut analysis_in_progress,
                        false,
                        all_prev_requests_count,
                        received_responses,
                        pending_requests,
                        defined_macros_pending,
                        requires_post_macro_tick,
                        enable_proc_macros,
                        pms_status,
                    );
                }
                AnalysisEvent::DiagnosticsTickStart => {
                    pending_requests = all_prev_requests_count - received_responses;
                    // A new tick is starting; it will see up-to-date macros, so clear the gate.
                    requires_post_macro_tick = false;
                    if project_loaded {
                        self.notifier.notify::<ServerStatus>(ServerStatusParams {
                            event: ServerStatusEvent::AnalysisStarted,
                        });
                        let _ = self.status_sender.send(AnalysisStatus::Started);
                        analysis_in_progress = true;
                    }
                }
                AnalysisEvent::DiagnosticsTickEnd { was_cancelled, all_request_count } => {
                    let request_count = all_request_count.into();

                    all_prev_requests_count = request_count;
                    finish_analysis_if_ready(
                        &mut analysis_in_progress,
                        was_cancelled,
                        all_prev_requests_count,
                        received_responses,
                        pending_requests,
                        defined_macros_pending,
                        requires_post_macro_tick,
                        enable_proc_macros,
                        pms_status,
                    );
                }
                AnalysisEvent::PMSStatusChange(new_pms_status) => {
                    match (pms_status, new_pms_status) {
                        // Transition from `Pending` to `Connected` is a natural flow.
                        (ProcMacroServerStatus::Pending, ProcMacroServerStatus::Connected)
                        // If the state remains unchanged, ignore this event.
                        | (ProcMacroServerStatus::Pending, ProcMacroServerStatus::Pending)
                        | (ProcMacroServerStatus::Connected, ProcMacroServerStatus::Connected)
                        | (ProcMacroServerStatus::Crashed, ProcMacroServerStatus::Crashed) => {}
                        // Every other case means that PMS either crashed or was restarted so reset PMS related data.
                        _ => {
                            all_prev_requests_count = 0;
                            received_responses = 0;
                            defined_macros_pending = 0;
                            requires_post_macro_tick = false;

                            // Mutation event will happen after this, so no need to restart analysis here.
                        }
                    }

                    pms_status = new_pms_status;
                }
                AnalysisEvent::PMSConnectedWithDefinedMacros { count } => {
                    pms_status = ProcMacroServerStatus::Connected;
                    defined_macros_pending += count as i128;
                    for _ in 0..count {
                        self.notifier.notify::<ServerStatus>(ServerStatusParams {
                            event: ServerStatusEvent::MacrosBuildingStarted,
                        });
                    }
                }
                AnalysisEvent::ProjectLoaded => {
                    project_loaded = true;
                }
                AnalysisEvent::DefinedMacrosRequested => {
                    defined_macros_pending += 1;
                    self.notifier.notify::<ServerStatus>(ServerStatusParams {
                        event: ServerStatusEvent::MacrosBuildingStarted,
                    });
                }
                AnalysisEvent::DefinedMacrosResponseReceived => {
                    defined_macros_pending -= 1;
                    if defined_macros_pending == 0 {
                        // All macros are now loaded. The current tick's diagnostics were computed
                        // with stale macro state, so gate AnalysisFinished until a fresh tick runs.
                        requires_post_macro_tick = true;
                    }
                    self.notifier.notify::<ServerStatus>(ServerStatusParams {
                        event: ServerStatusEvent::MacrosBuildingFinished,
                    });
                }
            }
        }
    }
}
