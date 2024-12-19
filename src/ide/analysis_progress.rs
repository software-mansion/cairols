use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use lsp_types::notification::Notification;

use crate::id_generator::IdGenerator;
use crate::server::client::Notifier;
use crate::state::Beacon;

/// A facade for `AnalysisProgressController` that allows to track progress of diagnostics
/// generation and procmacro requests.
#[derive(Clone)]
pub struct AnalysisProgressTracker {
    controller: AnalysisProgressController,
}

impl AnalysisProgressTracker {
    /// Signals that a request to proc macro server was made during the current generation of
    /// diagnostics.
    pub fn register_procmacro_request(&self) {
        self.controller.set_did_submit_procmacro_request(true);
    }

    /// Sets handlers for tracking beacons sent to threads.
    /// The beacons are wrapping snapshots, which are signalling when diagnostics finished
    /// calculating for a given snapshot (used for calculating files diagnostics or removing
    /// stale ones)
    pub fn track_analysis<'a>(&self, beacons: impl Iterator<Item = &'a mut Beacon>) {
        let gen_id = self.controller.next_generation_id();

        self.controller.clear_active_snapshots();

        beacons.enumerate().for_each(|(i, beacon)| {
            self.controller.insert_active_snapshot(i);

            let controller_ref: AnalysisProgressController = self.controller.clone();
            beacon.on_signal(move || controller_ref.on_snapshot_deactivate(gen_id, i));
        });

        self.controller.start_analysis();
    }
}

/// Controller used to send notifications to the client about analysis progress.
/// Uses information provided from other controllers (diagnostics controller, procmacro controller)
/// to assess if diagnostics are in fact calculated.
#[derive(Debug, Clone)]
pub struct AnalysisProgressController {
    notifier: Notifier,
    /// ID of the diagnostics "generation" - the scheduled diagnostics jobs set.
    /// Used to filter out stale threads finishing when new ones (from newer "generation")
    /// are already in progress and being tracked by the controller.
    generation_id: Arc<AtomicU64>,
    /// Sequential IDs of state snapshots from the current generation, used to track their status
    /// (present meaning it's still being used)
    active_snapshots: Arc<Mutex<HashSet<usize>>>,
    id_generator: Arc<IdGenerator>,
    /// If `true` - a request to procmacro server was submitted, meaning that analysis will extend
    /// beyond the current generation of diagnostics.
    did_submit_procmacro_request: Arc<AtomicBool>,
    /// Indicates that a notification was sent and analysis (i.e. macro expansion) is taking place.
    analysis_in_progress: Arc<AtomicBool>,
    /// Loaded asynchronously from config - unset if config was not loaded yet.
    /// Has to be set in order for analysis to finish.
    procmacros_enabled: Arc<Mutex<Option<bool>>>,
}

impl AnalysisProgressController {
    pub fn tracker(&self) -> AnalysisProgressTracker {
        AnalysisProgressTracker { controller: self.clone() }
    }

    pub fn new(notifier: Notifier) -> Self {
        let id_generator = Arc::new(IdGenerator::default());
        Self {
            notifier,
            id_generator: id_generator.clone(),
            active_snapshots: Arc::new(Mutex::new(HashSet::default())),
            did_submit_procmacro_request: Arc::new(AtomicBool::new(false)),
            analysis_in_progress: Arc::new(AtomicBool::new(false)),
            procmacros_enabled: Arc::new(Mutex::new(None)),
            generation_id: Arc::new(AtomicU64::new(id_generator.unique_id())),
        }
    }

    pub fn set_did_submit_procmacro_request(&self, value: bool) {
        self.did_submit_procmacro_request.store(value, Ordering::SeqCst);
    }

    /// Allows to set the procmacro configuration to whatever is in the config, upon loading it.
    pub fn set_procmacros_enabled(&self, value: bool) {
        let mut guard = self.procmacros_enabled.lock().unwrap();
        *guard = Some(value);
    }

    pub fn insert_active_snapshot(&self, snapshot_id: usize) {
        let mut active_snapshots = self.active_snapshots.lock().unwrap();
        active_snapshots.insert(snapshot_id);
    }

    pub fn on_snapshot_deactivate(&self, snapshot_gen_id: u64, snapshot_id: usize) {
        let current_gen = self.get_generation_id();
        if current_gen == snapshot_gen_id {
            self.remove_active_snapshot(snapshot_id);
            self.try_stop_analysis();
        }
    }

    pub fn next_generation_id(&self) -> u64 {
        let new_gen_id = self.id_generator.unique_id();
        self.generation_id.store(new_gen_id, Ordering::SeqCst);
        new_gen_id
    }

    pub fn get_generation_id(&self) -> u64 {
        self.generation_id.load(Ordering::SeqCst)
    }

    pub fn remove_active_snapshot(&self, snapshot_id: usize) {
        let mut active_snapshots = self.active_snapshots.lock().unwrap();
        active_snapshots.remove(&snapshot_id);
    }

    pub fn clear_active_snapshots(&self) {
        let active_snapshots_ref = self.active_snapshots.clone();
        active_snapshots_ref.lock().unwrap().clear();
    }

    /// Starts a next generation of diagnostics, sends a notification
    fn start_analysis(&self) {
        let analysis_in_progress = self.analysis_in_progress.load(Ordering::SeqCst);
        let config_loaded = self.procmacros_enabled.lock().unwrap().is_some();
        // We want to clear this flag always when starting a new generation to track the requests
        // properly
        self.did_submit_procmacro_request.store(false, Ordering::SeqCst);

        if !analysis_in_progress && config_loaded {
            self.analysis_in_progress.store(true, Ordering::SeqCst);
            self.notifier.notify::<DiagnosticsCalculationStart>(());
        }
    }

    /// Checks a bunch of conditions and if they are fulfilled, sends stop notification
    /// and resets the state back to start of generation defaults.
    fn try_stop_analysis(&self) {
        let did_submit_procmacro_request = self.did_submit_procmacro_request.load(Ordering::SeqCst);
        let snapshots_empty = self.active_snapshots.lock().unwrap().is_empty();
        let analysis_in_progress = self.analysis_in_progress.load(Ordering::SeqCst);
        let procmacros_enabled = *self.procmacros_enabled.lock().unwrap();

        if snapshots_empty
            && (!did_submit_procmacro_request || (procmacros_enabled == Some(false)))
            && analysis_in_progress
        {
            self.did_submit_procmacro_request.store(false, Ordering::SeqCst);
            self.analysis_in_progress.store(false, Ordering::SeqCst);

            self.notifier.notify::<DiagnosticsCalculationFinish>(());
        }
    }
}

/// Notifies about diagnostics generation which is beginning to calculate
#[derive(Debug)]
pub struct DiagnosticsCalculationStart;

impl Notification for DiagnosticsCalculationStart {
    type Params = ();
    const METHOD: &'static str = "cairo/diagnosticsCalculationStart";
}

/// Notifies about diagnostics generation which ended calculating
#[derive(Debug)]
pub struct DiagnosticsCalculationFinish;

impl Notification for DiagnosticsCalculationFinish {
    type Params = ();
    const METHOD: &'static str = "cairo/diagnosticsCalculationFinish";
}
