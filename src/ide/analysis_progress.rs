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
    Starting,
    Ready,
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

    pub fn events_sender(&self) -> Sender<AnalysisEvent> {
        self.events_sender.clone()
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

    pub fn mutation(&self) {
        self.send(AnalysisEvent::Mutation);
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
    Mutation,
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
    DatabaseSwap,
    ProjectLoaded,
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
        let mut pms_status = ProcMacroServerStatus::default();

        while let Ok(event) = self.events_receiver.recv() {
            match event {
                AnalysisEvent::ConfigLoad { enable_proc_macros: new_enable_proc_macros } => {
                    enable_proc_macros = new_enable_proc_macros;

                    // Mutation event will happen after this, so no need to restart analysis here.
                }
                AnalysisEvent::ApplyResponses { response_count } => {
                    // Response count is delta, add it.
                    received_responses += response_count as i128;
                }
                AnalysisEvent::DiagnosticsTickStart => {
                    pending_requests = all_prev_requests_count - received_responses;
                }
                AnalysisEvent::DiagnosticsTickEnd { was_cancelled, all_request_count } => {
                    let request_count = all_request_count.into();

                    if analysis_in_progress
                        && (!enable_proc_macros
                            || (pms_status == ProcMacroServerStatus::Ready
                                && pending_requests == 0))
                        && (!was_cancelled && request_count == received_responses)
                    {
                        self.notifier.notify::<ServerStatus>(ServerStatusParams {
                            event: ServerStatusEvent::AnalysisFinished,
                            idle: true,
                        });
                        let _ = self.status_sender.send(AnalysisStatus::Finished);

                        analysis_in_progress = false;
                    }

                    all_prev_requests_count = request_count;
                }
                AnalysisEvent::Mutation | AnalysisEvent::DatabaseSwap => {
                    if project_loaded && !analysis_in_progress {
                        self.notifier.notify::<ServerStatus>(ServerStatusParams {
                            event: ServerStatusEvent::AnalysisStarted,
                            idle: false,
                        });
                        let _ = self.status_sender.send(AnalysisStatus::Started);

                        analysis_in_progress = true;
                    }
                }
                AnalysisEvent::PMSStatusChange(new_pms_status) => {
                    match (pms_status, new_pms_status) {
                        // Pending -> Starting and Starting -> Ready are natural flow, ignore this case.
                        (ProcMacroServerStatus::Pending, ProcMacroServerStatus::Starting)
                        | (ProcMacroServerStatus::Starting, ProcMacroServerStatus::Ready)
                        // If state is unchanged, ignore this event.
                        | (ProcMacroServerStatus::Pending, ProcMacroServerStatus::Pending)
                        | (ProcMacroServerStatus::Starting, ProcMacroServerStatus::Starting)
                        | (ProcMacroServerStatus::Ready, ProcMacroServerStatus::Ready)
                        | (ProcMacroServerStatus::Crashed, ProcMacroServerStatus::Crashed) => { }
                        // Every other case means that PMS either crashed or was restarted so reset PMS related data.
                        _ => {
                            all_prev_requests_count = 0;
                            received_responses = 0;

                            // Mutation event will happen after this, so no need to restart analysis here.
                        }
                    }

                    pms_status = new_pms_status;
                }
                AnalysisEvent::ProjectLoaded => {
                    project_loaded = true;
                }
            }
        }
    }
}
