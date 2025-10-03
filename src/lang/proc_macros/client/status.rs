use std::sync::Arc;

use super::ProcMacroClient;

#[derive(Debug, Default, Clone)]
pub enum ServerStatus {
    #[default]
    Pending,
    Connected(Arc<ProcMacroClient>),
    /// Failed to start multiple times.
    /// No more actions will be taken.
    Crashed,
}

impl ServerStatus {
    pub fn connected(&self) -> Option<&ProcMacroClient> {
        if let Self::Connected(client) = self { Some(client) } else { None }
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }
}
