use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use lsp_types::notification::Notification;

use crate::id_generator::IdGenerator;
use crate::server::client::Notifier;
use crate::state::Beacon;

/// Controller used to send notifications to the client about analysis progress.
/// Uses information provided from other controllers (diagnostics controller, procmacro controller)
/// to assess if diagnostics are in fact calculated.
#[derive(Debug, Clone)]
pub struct AnalysisProgressController {
    notifier: Notifier,
    /// ID of the diagnostics "generation" - the scheduled diagnostics jobs set.
    /// Used to filter out stale threads finishing when new ones (from newer "generation")
    /// are already in progress and being tracked by the controller.
    generation_id: Arc<Mutex<u64>>,
    /// Sequential IDs of state snapshots from the current generation, used to track their status
    /// (present meaning it's still being used)
    active_snapshots: Arc<Mutex<HashSet<usize>>>,
    id_generator: Arc<IdGenerator>,
    /// If `true` - a request to procmacro server was submitted, meaning that analysis will extend
    /// beyond the current generation of diagnostics.
    did_submit_procmacro_request: Arc<Mutex<bool>>,
    /// Indicates that a notification was sent and analysis (i.e. macro expansion) is taking place.
    analysis_in_progress: Arc<Mutex<bool>>,
    /// Loaded asynchronously from config - unset if config was not loaded yet.
    /// Has to be set in order for analysis to finish.
    procmacros_enabled: Arc<Mutex<Option<bool>>>,
}

impl AnalysisProgressController {
    pub fn new(notifier: Notifier) -> Self {
        let id_generator = Arc::new(IdGenerator::default());
        Self {
            notifier,
            id_generator: id_generator.clone(),
            active_snapshots: Arc::new(Mutex::new(HashSet::default())),
            did_submit_procmacro_request: Arc::new(Mutex::new(true)),
            analysis_in_progress: Arc::new(Mutex::new(false)),
            procmacros_enabled: Arc::new(Mutex::new(None)),
            generation_id: Arc::new(Mutex::new(id_generator.unique_id())),
        }
    }

    /// Signals that a request to proc macro server was made during the current generation of
    /// diagnostics.
    pub fn register_procmacro_request(&self) {
        let mut write_guard = self.did_submit_procmacro_request.lock().unwrap();
        *write_guard = true;
    }

    /// Allows to set the procmacro configuration to whatever is in the config, upon loading it.
    pub fn set_procmacros_enabled(&self, value: bool) {
        let mut guard = self.procmacros_enabled.lock().unwrap();
        *guard = Some(value);
    }

    /// Sets handlers for tracking beacons sent to threads.
    /// The beacons are wrapping snapshots, which are signalling when diagnostics finished
    /// calculating for a given snapshot (used for calculating files diagnostics or removing
    /// stale ones)
    pub fn track_analysis<T: Send>(&self, beacons: &mut [Beacon<T>]) {
        let gen_id = self.next_generation_id();

        self.clear_active_snapshots();
        beacons.iter_mut().enumerate().for_each(|(i, beacon)| {
            self.insert_active_snapshot(i);

            let self_ref: AnalysisProgressController = self.clone();
            beacon.on_signal(move || {
                let current_gen = self_ref.get_generation_id();
                if current_gen == gen_id {
                    self_ref.remove_active_snapshot(i);
                    self_ref.try_stop_analysis();
                }
            });
        });

        self.start_analysis();
    }

    fn insert_active_snapshot(&self, snapshot_id: usize) {
        let mut active_snapshots = self.active_snapshots.lock().unwrap();
        active_snapshots.insert(snapshot_id);
    }

    fn next_generation_id(&self) -> u64 {
        let mut generation_id_guard = self.generation_id.lock().unwrap();
        *generation_id_guard = self.id_generator.unique_id();
        *generation_id_guard
    }

    fn get_generation_id(&self) -> u64 {
        *self.generation_id.lock().unwrap()
    }

    fn remove_active_snapshot(&self, snapshot_id: usize) {
        let mut active_snapshots = self.active_snapshots.lock().unwrap();
        active_snapshots.remove(&snapshot_id);
    }

    fn clear_active_snapshots(&self) {
        let active_snapshots_ref = self.active_snapshots.clone();
        active_snapshots_ref.lock().unwrap().clear();
    }

    /// Starts a next generation of diagnostics, sends a notification
    fn start_analysis(&self) {
        let mut analysis_in_progress = self.analysis_in_progress.lock().unwrap();
        if !(*analysis_in_progress) {
            *analysis_in_progress = true;
            self.notifier.notify::<DiagnosticsCalculationStart>(());
        }
    }

    /// Checks a bunch of conditions and if they are fulfilled, sends stop notification
    /// and resets the state back to start of generation defaults.
    fn try_stop_analysis(&self) {
        let mut did_submit_procmacro_request = self.did_submit_procmacro_request.lock().unwrap();
        let snapshots_empty = self.active_snapshots.lock().unwrap().is_empty();
        let mut analysis_in_progress = self.analysis_in_progress.lock().unwrap();
        let procmacros_enabled = *self.procmacros_enabled.lock().unwrap();

        if snapshots_empty
            && (!*did_submit_procmacro_request || (procmacros_enabled == Some(false)))
            && *analysis_in_progress
        {
            *analysis_in_progress = false;
            *did_submit_procmacro_request = false;
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
