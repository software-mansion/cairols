// +--------------------------------------------------------+
// | Code adopted from:                                     |
// | Repository: https://github.com/astral-sh/ruff          |
// | File: `crates/ruff_server/src/server/schedule/task.rs` |
// | Commit: 46a457318d8d259376a2b458b3f814b9b795fe69       |
// +--------------------------------------------------------+

use lsp_server::RequestId;
use serde::Serialize;
use tracing::error;

use crate::lsp::result::LSPError;
use crate::server::client::{Notifier, Requester, Responder};
use crate::state::State;

type LocalMutFn<'s> = Box<dyn FnOnce(&mut State, Notifier, &mut Requester<'_>, Responder) + 's>;
type LocalFn<'s> = Box<dyn FnOnce(&State, Notifier, &mut Requester<'_>, Responder) + 's>;
type LocalConditionFn<'s> = Box<dyn FnOnce(&State) -> bool + 's>;

type BackgroundFn = Box<dyn FnOnce(Notifier, Responder) + Send + 'static>;

pub type BackgroundFnBuilder<'s> = Box<dyn FnOnce(&State) -> BackgroundFn + 's>;

/// Describes how the task should be run.
#[derive(Clone, Copy, Debug, Default)]
pub enum BackgroundSchedule {
    /// The task should be run on the general high-priority background
    /// thread.
    LatencySensitive,
    /// The task should be run on a regular-priority background thread.
    #[default]
    Worker,
}

/// A [`Task`] is a future that has not yet started, and it is the job of
/// the [`super::Scheduler`] to make that happen, via [`super::Scheduler::dispatch`].
/// A task can either run on the main thread (in other words, the same thread as the
/// scheduler) or it can run in a background thread. The main difference between
/// the two is that background threads only have a read-only snapshot of the session,
/// while local tasks have exclusive access and can modify it as they please. Keep in mind that
/// local tasks will **block** the main event loop, so only use local tasks if you **need**
/// mutable state access, or you need the absolute lowest latency possible.
pub enum Task<'s> {
    Fmt(BackgroundFnBuilder<'s>),
    Background(BackgroundTaskBuilder<'s>),
    Sync(SyncTask<'s>),
    SyncMut(SyncMutTask<'s>),
    SyncConditional(SyncConditionTask<'s>),
}

// The reason why this isn't just a 'static background closure
// is because we need to take a snapshot of the state before sending
// this task to the background. The inner closure can't take the state
// as an immutable reference since it's used mutably elsewhere. So instead,
// a background task is built using an outer closure that borrows the state to take a snapshot
// that the inner closure can capture. This builder closure has a lifetime linked to the scheduler.
// When the task is dispatched, the scheduler runs the synchronous builder, which takes the state
// as a reference, to create the inner 'static closure. That closure is then moved to a background
// task pool.
pub struct BackgroundTaskBuilder<'s> {
    pub schedule: BackgroundSchedule,
    pub builder: BackgroundFnBuilder<'s>,
}

pub struct SyncMutTask<'s> {
    pub func: LocalMutFn<'s>,
}

pub struct SyncTask<'s> {
    pub func: LocalFn<'s>,
}
pub struct SyncConditionTask<'s> {
    pub precondition_func: LocalConditionFn<'s>,
    pub mut_func: LocalMutFn<'s>,
}

impl<'s> Task<'s> {
    /// Creates a new fmt task.
    pub fn fmt(
        func: impl FnOnce(&State) -> Box<dyn FnOnce(Notifier, Responder) + Send + 'static> + 's,
    ) -> Self {
        Self::Fmt(Box::new(func))
    }

    /// Creates a new background task.
    pub fn background(
        schedule: BackgroundSchedule,
        func: impl FnOnce(&State) -> Box<dyn FnOnce(Notifier, Responder) + Send + 'static> + 's,
    ) -> Self {
        Self::Background(BackgroundTaskBuilder { schedule, builder: Box::new(func) })
    }

    /// Creates a new local task.
    pub fn local_mut(
        func: impl FnOnce(&mut State, Notifier, &mut Requester<'_>, Responder) + 's,
    ) -> Self {
        Self::SyncMut(SyncMutTask { func: Box::new(func) })
    }

    /// Creates a new local task without access to the mutable state.
    pub fn local(func: impl FnOnce(&State, Notifier, &mut Requester<'_>, Responder) + 's) -> Self {
        Self::Sync(SyncTask { func: Box::new(func) })
    }

    /// Creates a new local task with access to the mutable state that runs only if a condition is satisfied.
    ///
    /// This pattern is useful when checking the precondition to running the task requires access to [`State`].
    pub fn local_with_precondition(
        precondition_func: impl FnOnce(&State) -> bool + 's,
        mut_func: impl FnOnce(&mut State, Notifier, &mut Requester<'_>, Responder) + 's,
    ) -> Self {
        Self::SyncConditional(SyncConditionTask {
            precondition_func: Box::new(precondition_func),
            mut_func: Box::new(mut_func),
        })
    }

    /// Creates a local task that immediately responds with the provided `request`.
    pub fn immediate<R>(id: RequestId, result: Result<R, LSPError>) -> Self
    where
        R: Serialize + Send + 'static,
    {
        Self::local(move |_, _, _, responder| {
            if let Err(err) = responder.respond(id, result) {
                error!("unable to send immediate response: {err:?}");
            }
        })
    }

    /// Creates a local task that does nothing.
    pub fn nothing() -> Self {
        Self::local(move |_, _, _, _| {})
    }
}
