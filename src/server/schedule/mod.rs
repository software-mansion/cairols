// +---------------------------------------------------+
// | Code adopted from:                                |
// | Repository: https://github.com/astral-sh/ruff     |
// | File: `crates/ruff_server/src/server/schedule.rs` |
// | Commit: 46a457318d8d259376a2b458b3f814b9b795fe69  |
// +---------------------------------------------------+

use std::sync::Arc;

use anyhow::Result;

use self::task::BackgroundTaskBuilder;
use self::thread::{JoinHandle, ThreadPriority};
use crate::server::client::{Client, Notifier, Requester, Responder};
use crate::server::connection::ClientSender;
use crate::server::schedule::task::BackgroundFnBuilder;
use crate::state::{MetaState, State};

mod task;
pub mod thread;

pub(super) use self::task::BackgroundSchedule;
pub use self::task::{SyncMutTask, Task};
use crate::server::schedule::task::{SyncConditionTask, SyncTask};

/// The event loop thread is actually a secondary thread that we spawn from the
/// _actual_ main thread. This secondary thread has a larger stack size
/// than some OS defaults (Windows, for example) and is also designated as
/// high priority.
pub fn event_loop_thread(
    func: impl FnOnce() -> Result<()> + Send + 'static,
) -> Result<JoinHandle<Result<()>>> {
    // Override OS defaults to avoid stack overflows on platforms with low stack size defaults.
    const MAIN_THREAD_STACK_SIZE: usize = 2 * 1024 * 1024;
    const MAIN_THREAD_NAME: &str = "cairols:main";
    Ok(thread::Builder::new(ThreadPriority::LatencySensitive)
        .name(MAIN_THREAD_NAME.into())
        .stack_size(MAIN_THREAD_STACK_SIZE)
        .spawn(func)?)
}

type SyncTaskHook = Box<dyn Fn(&mut State, Arc<MetaState>, Notifier)>;

pub struct Scheduler<'s> {
    state: &'s mut State,
    client: Client<'s>,
    background_pool: thread::Pool,
    /// Since the editor may wait for fmt response, we have a separate thread pool with one thread
    /// for fmt tasks.
    /// It ensures that when all other threads from the `background_pool` are busy when receiving
    /// fmt request, it will still be processed fast.
    fmt_pool: thread::Pool,
    sync_mut_task_hooks: Vec<SyncTaskHook>,
    pub meta_state: Arc<MetaState>,
}

impl<'s> Scheduler<'s> {
    pub fn new(state: &'s mut State, sender: ClientSender) -> Self {
        Self {
            state,
            client: Client::new(sender),
            background_pool: thread::Pool::new(usize::MAX, "worker"),
            fmt_pool: thread::Pool::new(1, "fmt"),
            sync_mut_task_hooks: Default::default(),
            meta_state: Default::default(),
        }
    }

    /// Creates a task to handle a response from the client.
    pub fn response(&mut self, response: lsp_server::Response) -> Task<'s> {
        self.client.requester.pop_response_task(response)
    }

    /// Dispatches a `task` by either running it as a blocking function or
    /// executing it on a background thread pool.
    pub fn dispatch(&mut self, task: Task<'s>) {
        let build_task_fn = |func: BackgroundFnBuilder| {
            let static_func = func(self.state, self.meta_state.clone());
            let notifier = self.client.notifier();
            let responder = self.client.responder();

            move || static_func(notifier, responder)
        };

        match task {
            Task::SyncMut(SyncMutTask { func }) => {
                let notifier = self.client.notifier();
                let responder = self.client.responder();
                func(self.state, notifier.clone(), &mut self.client.requester, responder);

                for hook in &self.sync_mut_task_hooks {
                    hook(self.state, self.meta_state.clone(), notifier.clone());
                }
            }
            Task::Sync(SyncTask { func }) => {
                let notifier = self.client.notifier();
                let responder = self.client.responder();
                func(
                    self.state,
                    self.meta_state.clone(),
                    notifier.clone(),
                    &mut self.client.requester,
                    responder,
                );
            }
            Task::SyncConditional(SyncConditionTask { precondition_func, mut_func }) => {
                if precondition_func(self.state) {
                    let notifier = self.client.notifier();
                    let responder = self.client.responder();
                    mut_func(self.state, notifier.clone(), &mut self.client.requester, responder);

                    for hook in &self.sync_mut_task_hooks {
                        hook(self.state, self.meta_state.clone(), notifier.clone());
                    }
                };
            }
            Task::Background(BackgroundTaskBuilder { schedule, builder: func }) => {
                let task = build_task_fn(func);
                match schedule {
                    BackgroundSchedule::Worker => {
                        self.background_pool.spawn(ThreadPriority::Worker, task);
                    }
                    BackgroundSchedule::LatencySensitive => {
                        self.background_pool.spawn(ThreadPriority::LatencySensitive, task);
                    }
                }
            }
            Task::Fmt(func) => {
                let task = build_task_fn(func);

                self.fmt_pool.spawn(ThreadPriority::LatencySensitive, task);
            }
        }
    }

    /// Dispatches a local task with access to the mutable state.
    ///
    /// This is a shortcut for `dispatch(Task::local_mut(func))`.
    pub fn local_mut(
        &mut self,
        func: impl FnOnce(&mut State, Notifier, &mut Requester<'_>, Responder) + 's,
    ) {
        self.dispatch(Task::local_mut(func));
    }

    /// Dispatches a local `task`.
    ///
    /// This is a shortcut for `dispatch(Task::local(func))`.
    pub fn local(
        &mut self,
        func: impl FnOnce(&State, Arc<MetaState>, Notifier, &mut Requester<'_>, Responder) + 's,
    ) {
        self.dispatch(Task::local(func));
    }

    /// Dispatches a local conditional `task`.
    ///
    /// This is a shortcut for `dispatch(Task::local_with_precondition(precondition_func, mut_func))`.
    pub fn local_with_precondition(
        &mut self,
        precondition_func: impl FnOnce(&State) -> bool + 's,
        mut_func: impl FnOnce(&mut State, Notifier, &mut Requester<'_>, Responder) + 's,
    ) {
        self.dispatch(Task::local_with_precondition(precondition_func, mut_func));
    }

    /// Registers a hook to be called each time a synchronous task with access to mutable state is executed.
    ///
    /// All hooks are called right after task execution, in the same thread and with the same
    /// context.
    /// This mechanism is useful for doing various bookkeeping in reaction to user interaction
    /// such as scheduling diagnostics computation, starting manual GC, etc.
    /// This includes reacting to state changes, though note that this hook will be called even
    /// after tasks that might, but did not mutate the state.
    pub fn on_sync_mut_task(
        &mut self,
        hook: impl Fn(&mut State, Arc<MetaState>, Notifier) + 'static,
    ) {
        self.sync_mut_task_hooks.push(Box::new(hook));
    }
}
