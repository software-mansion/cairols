use lsp_types::{ClientCapabilities, ResourceOperationKind};

macro_rules! try_or_default {
    ($expr:expr) => {
        || -> Option<_> { Some($expr) }().unwrap_or_default()
    };
}

/// Extension methods for the [`ClientCapabilities`] struct.
pub trait ClientCapabilitiesExt {
    /// The client supports dynamic registration for the `workspace/didChangeWatchedFiles`
    /// notification.
    fn did_change_watched_files_dynamic_registration(&self) -> bool;

    /// The client supports `workspace/configuration` requests.
    fn workspace_configuration_support(&self) -> bool;

    /// The client supports `workspace/semanticTokens/refresh` requests.
    fn workspace_semantic_tokens_refresh_support(&self) -> bool;

    /// The client supports renaming files and directories as a part of `WorkspaceEdit` requests.
    fn workspace_edit_rename_resource_support(&self) -> bool;

    /// The client supports dynamic registration for text document synchronization capabilities.
    fn text_document_synchronization_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for completion capabilities.
    fn completion_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for execute command capabilities.
    fn execute_command_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for semantic tokens capabilities.
    fn semantic_tokens_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for formatting capabilities.
    fn formatting_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for hover capabilities.
    fn hover_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for definition capabilities.
    fn definition_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for code action capabilities.
    fn code_action_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for references provider capabilities.
    fn references_provider_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for rename provider capabilities.
    fn rename_provider_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for document highlight provider capabilities.
    fn document_highlight_provider_dynamic_registration(&self) -> bool;

    /// The client supports dynamic registration for code lens provider capabilities.
    fn code_lens_provider_dynamic_registration(&self) -> bool;

    /// The client supports sending `workspace/willRenameFiles` requests.
    fn workspace_will_rename_files_support(&self) -> bool;
}

impl ClientCapabilitiesExt for ClientCapabilities {
    fn did_change_watched_files_dynamic_registration(&self) -> bool {
        try_or_default!(
            self.workspace.as_ref()?.did_change_watched_files.as_ref()?.dynamic_registration?
        )
    }

    fn workspace_configuration_support(&self) -> bool {
        try_or_default! {
            self.workspace.as_ref()?.configuration?
        }
    }

    fn workspace_semantic_tokens_refresh_support(&self) -> bool {
        try_or_default!(self.workspace.as_ref()?.semantic_tokens.as_ref()?.refresh_support?)
    }

    fn workspace_edit_rename_resource_support(&self) -> bool {
        try_or_default! {
            self.workspace.as_ref()?.workspace_edit.as_ref()?
            .resource_operations.as_ref()?.contains(&ResourceOperationKind::Rename)
        }
    }

    fn text_document_synchronization_dynamic_registration(&self) -> bool {
        try_or_default!(
            self.text_document.as_ref()?.synchronization.as_ref()?.dynamic_registration?
        )
    }

    fn completion_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.completion.as_ref()?.dynamic_registration?)
    }

    fn execute_command_dynamic_registration(&self) -> bool {
        try_or_default!(self.workspace.as_ref()?.execute_command.as_ref()?.dynamic_registration?)
    }

    fn semantic_tokens_dynamic_registration(&self) -> bool {
        try_or_default!(
            self.text_document.as_ref()?.semantic_tokens.as_ref()?.dynamic_registration?
        )
    }

    fn formatting_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.formatting.as_ref()?.dynamic_registration?)
    }

    fn hover_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.hover.as_ref()?.dynamic_registration?)
    }

    fn definition_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.definition.as_ref()?.dynamic_registration?)
    }

    fn code_action_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.code_action.as_ref()?.dynamic_registration?)
    }

    fn references_provider_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.references.as_ref()?.dynamic_registration?)
    }

    fn rename_provider_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.rename.as_ref()?.dynamic_registration?)
    }

    fn document_highlight_provider_dynamic_registration(&self) -> bool {
        try_or_default!(
            self.text_document.as_ref()?.document_highlight.as_ref()?.dynamic_registration?
        )
    }

    fn code_lens_provider_dynamic_registration(&self) -> bool {
        try_or_default!(self.text_document.as_ref()?.code_lens.as_ref()?.dynamic_registration?)
    }

    fn workspace_will_rename_files_support(&self) -> bool {
        try_or_default!(self.workspace.as_ref()?.file_operations.as_ref()?.will_rename?)
    }
}
