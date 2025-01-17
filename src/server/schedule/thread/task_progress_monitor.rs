use crate::server::trigger;

pub struct TaskHandle(trigger::Receiver<()>);

pub struct TaskTracker(trigger::Sender<()>);

impl TaskTracker {
    /// Signals that a task finished executing.
    pub fn signal_finish(&self) {
        self.0.activate(());
    }
}

impl TaskHandle {
    /// Waits until tasks finishes executing.
    pub fn join(&self) {
        self.0.wait();
    }
}

/// Creates single message channel for making it possible to wait for finishing tasks execution.
pub fn task_progress_monitor() -> (TaskTracker, TaskHandle) {
    let (sender, receiver) = trigger::trigger::<()>();
    (TaskTracker(sender), TaskHandle(receiver))
}
