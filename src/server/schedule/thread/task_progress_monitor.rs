use crate::server::trigger;

#[derive(Eq, PartialEq)]
pub enum TaskResult {
    Done,
    Cancelled,
}

pub struct TaskHandle(trigger::Receiver<TaskResult>);

pub struct TaskTracker(trigger::Sender<TaskResult>);

impl TaskTracker {
    /// Signals that a task finished executing.
    pub fn signal_finish(&self, task_result: TaskResult) {
        self.0.activate(task_result);
    }
}

impl TaskHandle {
    /// Waits until tasks finishes executing.
    pub fn join(&self) -> TaskResult {
        self.0.wait().unwrap_or(TaskResult::Cancelled)
    }
}

/// Creates single message channel for making it possible to wait for finishing tasks execution.
pub fn task_progress_monitor() -> (TaskTracker, TaskHandle) {
    let (sender, receiver) = trigger::trigger::<TaskResult>();
    (TaskTracker(sender), TaskHandle(receiver))
}
