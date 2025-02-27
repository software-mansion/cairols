// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff              |
// | File: `crates/ruff_server/src/server/schedule/thread.rs`   |
// | Commit: 46a457318d8d259376a2b458b3f814b9b795fe69           |
// +------------------------------------------------------------+

// ==============================================================

// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/rust-lang/rust-analyzer.git |
// | File: `crates/stdx/src/thread.rs`                          |
// | Commit: 03b3cb6be9f21c082f4206b35c7fe7f291c94eaa           |
// +------------------------------------------------------------+
//! A utility module for working with threads that automatically joins threads upon drop
//! and abstracts over operating system quality of service (QoS) APIs
//! through the concept of a “thread priority”.
//!
//! The priority of a thread is frozen at thread creation time,
//! i.e. there is no API to change the priority of a thread once it has been spawned.
//!
//! As a system, rust-analyzer should have the property that
//! old manual scheduling APIs are replaced entirely by QoS.
//! To maintain this invariant, we panic when it is clear that
//! old scheduling APIs have been used.
//!
//! Moreover, we also want to ensure that every thread has a priority set explicitly
//! to force a decision about its importance to the system.
//! Thus, [`ThreadPriority`] has no default value
//! and every entry point to creating a thread requires a [`ThreadPriority`] upfront.

// Keeps us from getting warnings about the word `QoS`
#![allow(clippy::doc_markdown)]

use std::fmt;

mod pool;
mod priority;
pub mod task_progress_monitor;

pub use self::pool::Pool;
pub use self::priority::ThreadPriority;

pub struct Builder {
    priority: ThreadPriority,
    inner: jod_thread::Builder,
}

impl Builder {
    pub fn new(priority: ThreadPriority) -> Builder {
        Builder { priority, inner: jod_thread::Builder::new() }
    }

    pub fn name(self, name: String) -> Builder {
        Builder { inner: self.inner.name(name), ..self }
    }

    pub fn stack_size(self, size: usize) -> Builder {
        Builder { inner: self.inner.stack_size(size), ..self }
    }

    pub fn spawn<F, T>(self, f: F) -> std::io::Result<JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        let inner_handle = self.inner.spawn(move || {
            self.priority.apply_to_current_thread();
            f()
        })?;

        Ok(JoinHandle { inner: Some(inner_handle) })
    }
}

pub struct JoinHandle<T = ()> {
    // `inner` is an `Option` so that we can
    // take ownership of the contained `JoinHandle`.
    inner: Option<jod_thread::JoinHandle<T>>,
}

impl<T> JoinHandle<T> {
    pub fn join(mut self) -> T {
        self.inner.take().unwrap().join()
    }
}

impl<T> fmt::Debug for JoinHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("JoinHandle { .. }")
    }
}
