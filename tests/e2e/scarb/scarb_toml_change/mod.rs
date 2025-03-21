use lsp_types::{
    ClientCapabilities, DidChangeWatchedFilesClientCapabilities, WorkspaceClientCapabilities,
};

mod invalid;
mod removing_dependency;
mod removing_member;

fn caps(base: ClientCapabilities) -> ClientCapabilities {
    ClientCapabilities {
        text_document: base.text_document.or_else(Default::default),
        workspace: base.workspace.or_else(Default::default).map(|it| WorkspaceClientCapabilities {
            did_change_watched_files: Some(DidChangeWatchedFilesClientCapabilities {
                dynamic_registration: None,
                relative_pattern_support: None,
            }),
            ..it
        }),
        ..base
    }
}
