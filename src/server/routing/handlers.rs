// +-----------------------------------------------------+
// | Traits adopted from:                                |
// | Repository: https://github.com/astral-sh/ruff       |
// | File: `crates/ruff_server/src/server/api/traits.rs` |
// | Commit: 46a457318d8d259376a2b458b3f814b9b795fe69    |
// +-----------------------------------------------------+

use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

use cairo_lang_filesystem::db::{FilesGroup, FilesGroupEx};
use cairo_lang_filesystem::override_file_content;
use lsp_types::notification::{
    DidChangeConfiguration, DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument,
    DidOpenTextDocument, DidSaveTextDocument, Notification,
};
use lsp_types::request::{
    CodeActionRequest, CodeLensRequest, Completion, DocumentHighlightRequest, ExecuteCommand,
    Formatting, GotoDefinition, HoverRequest, InlayHintRequest, References, Rename, Request,
    SemanticTokensFullRequest, WillRenameFiles,
};
use lsp_types::{
    CodeActionParams, CodeActionResponse, CodeLens, CodeLensParams, CompletionParams,
    CompletionResponse, DidChangeConfigurationParams, DidChangeTextDocumentParams,
    DidChangeWatchedFilesParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, DocumentFormattingParams, DocumentHighlight,
    DocumentHighlightParams, ExecuteCommandParams, FileChangeType, GotoDefinitionParams,
    GotoDefinitionResponse, Hover, HoverParams, InlayHint, InlayHintParams, ReferenceParams,
    RenameFilesParams, RenameParams, SemanticTokensParams, SemanticTokensResult,
    TextDocumentContentChangeEvent, TextDocumentPositionParams, TextEdit, Url, WorkspaceEdit,
};
use serde_json::Value;
use tracing::error;

use crate::ide::code_lens::{CodeLensController, FileChange};
use crate::lang::lsp::LsProtoGroup;
use crate::lsp::ext::{
    ExpandMacro, ProvideVirtualFile, ProvideVirtualFileRequest, ProvideVirtualFileResponse,
    ToolchainInfo, ToolchainInfoResponse, ViewAnalyzedCrates, ViewSyntaxTree,
};
use crate::lsp::result::{LSPError, LSPResult};
use crate::server::client::{Notifier, Requester};
use crate::server::commands::ServerCommands;
use crate::server::panic::is_cancelled;
use crate::state::{MetaState, State, StateSnapshot};
use crate::toolchain::info::toolchain_info;
use crate::{Backend, ide, lang};

/// A request handler that needs mutable access to the session.
/// This will block the main message receiver loop, meaning that no
/// incoming requests or notifications will be handled while `run` is
/// executing. Try to avoid doing any I/O or long-running computations.
pub trait SyncRequestHandler: Request {
    fn run(
        state: &mut State,
        notifier: Notifier,
        requester: &mut Requester<'_>,
        params: <Self as Request>::Params,
    ) -> LSPResult<<Self as Request>::Result>;
}

/// A request handler that can be run on a background thread.
pub trait BackgroundDocumentRequestHandler: Request {
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        notifier: Notifier,
        params: <Self as Request>::Params,
    ) -> LSPResult<<Self as Request>::Result>;
}

/// A notification handler that needs mutable access to the session.
/// This will block the main message receiver loop, meaning that no
/// incoming requests or notifications will be handled while `run` is
/// executing. Try to avoid doing any I/O or long-running computations.
pub trait SyncNotificationHandler: Notification {
    fn run(
        state: &mut State,
        notifier: Notifier,
        requester: &mut Requester<'_>,
        params: <Self as Notification>::Params,
    ) -> LSPResult<()>;
}

impl BackgroundDocumentRequestHandler for CodeActionRequest {
    #[tracing::instrument(name = "textDocument/codeAction", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>, LSPError> {
        Ok(catch_unwind(AssertUnwindSafe(|| {
            ide::code_actions::code_actions(params, &snapshot.configs_registry, &snapshot.db)
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("CodeActionRequest handler panicked");
            None
        }))
    }
}

impl SyncRequestHandler for ExecuteCommand {
    #[tracing::instrument(
        name = "workspace/executeCommand",
        skip_all,
        fields(command = params.command)
    )]
    fn run(
        state: &mut State,
        notifier: Notifier,
        requester: &mut Requester<'_>,
        params: ExecuteCommandParams,
    ) -> LSPResult<Option<Value>> {
        let command = ServerCommands::try_from(params.command);

        if let Ok(cmd) = command {
            match cmd {
                ServerCommands::Reload => {
                    Backend::reload(state, requester)?;
                }
                ServerCommands::ExecuteCodeLens => {
                    CodeLensController::execute_code_lens(state, notifier, &params.arguments);
                }
            }
        }

        Ok(None)
    }
}

impl BackgroundDocumentRequestHandler for HoverRequest {
    #[tracing::instrument(name = "textDocument/hover", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: HoverParams,
    ) -> LSPResult<Option<Hover>> {
        Ok(catch_unwind(AssertUnwindSafe(|| ide::hover::hover(params, &snapshot.db)))
            .unwrap_or_else(|err| {
                if is_cancelled(err.as_ref()) {
                    resume_unwind(err);
                }
                error!("HoverRequest handler panicked");
                None
            }))
    }
}

impl BackgroundDocumentRequestHandler for Formatting {
    #[tracing::instrument(name = "textDocument/formatting", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: DocumentFormattingParams,
    ) -> LSPResult<Option<Vec<TextEdit>>> {
        Ok(ide::format::documents::format_document(params, snapshot))
    }
}

impl SyncNotificationHandler for DidChangeTextDocument {
    #[tracing::instrument(
        name = "textDocument/didChange",
        skip_all,
        fields(uri = %params.text_document.uri)
    )]
    fn run(
        state: &mut State,
        _notifier: Notifier,
        _requester: &mut Requester<'_>,
        params: DidChangeTextDocumentParams,
    ) -> LSPResult<()> {
        let text = if let Ok([TextDocumentContentChangeEvent { text, .. }]) =
            TryInto::<[_; 1]>::try_into(params.content_changes)
        {
            text
        } else {
            error!("unexpected format of document change");
            return Ok(());
        };

        if let Some(file) = state.db.file_for_url(&params.text_document.uri) {
            override_file_content!(state.db, file, Some(text.into()));
        };

        state.code_lens_controller.on_did_change(
            state.db.clone(),
            state.config.clone(),
            is_cairo_file_path(&params.text_document.uri)
                .then(|| FileChange { url: params.text_document.uri.clone(), was_deleted: false })
                .into_iter(),
        );

        Ok(())
    }
}

impl SyncNotificationHandler for DidChangeConfiguration {
    #[tracing::instrument(name = "workspace/didChangeConfiguration", skip_all)]
    fn run(
        state: &mut State,
        _notifier: Notifier,
        requester: &mut Requester<'_>,
        _params: DidChangeConfigurationParams,
    ) -> LSPResult<()> {
        state.config.reload(requester, &state.client_capabilities)
    }
}

impl SyncNotificationHandler for DidChangeWatchedFiles {
    #[tracing::instrument(name = "workspace/didChangeWatchedFiles", skip_all)]
    fn run(
        state: &mut State,
        _notifier: Notifier,
        requester: &mut Requester<'_>,
        params: DidChangeWatchedFilesParams,
    ) -> LSPResult<()> {
        // Invalidate changed cairo files.
        for change in &params.changes {
            if is_cairo_file_path(&change.uri) {
                let Some(_file) = state.db.file_for_url(&change.uri) else { continue };
                // In perfect scenario we would do this only for `file` but there is no way to make it more granulary.
                state.db.cancel_all();
            }
        }

        // Reload workspace if a config file has changed.
        for change in &params.changes {
            let changed_file_path = change.uri.to_file_path().unwrap_or_default();
            let changed_file_name = changed_file_path.file_name().unwrap_or_default();
            // TODO(pmagiera): react to Scarb.lock. Keep in mind Scarb does save Scarb.lock on each
            //  metadata call, so it is easy to fall in a loop here.
            if ["Scarb.toml", "cairo_project.toml"].map(Some).contains(&changed_file_name.to_str())
            {
                Backend::reload(state, requester)?;

                state.proc_macro_controller.force_restart(&mut state.db, &state.config);
            }
        }

        state.code_lens_controller.on_did_change(
            state.db.clone(),
            state.config.clone(),
            params.changes.iter().filter(|event| is_cairo_file_path(&event.uri)).map(|event| {
                FileChange {
                    url: event.uri.clone(),
                    was_deleted: event.typ == FileChangeType::DELETED,
                }
            }),
        );

        Ok(())
    }
}

impl SyncNotificationHandler for DidCloseTextDocument {
    #[tracing::instrument(
        name = "textDocument/didClose",
        skip_all,
        fields(uri = %params.text_document.uri)
    )]
    fn run(
        state: &mut State,
        _notifier: Notifier,
        _requester: &mut Requester<'_>,
        params: DidCloseTextDocumentParams,
    ) -> LSPResult<()> {
        state.open_files.remove(&params.text_document.uri);
        if let Some(file) = state.db.file_for_url(&params.text_document.uri)
            && state.db.file_overrides().contains_key(&file)
        {
            override_file_content!(state.db, file, None);
        }

        Ok(())
    }
}

impl SyncNotificationHandler for DidOpenTextDocument {
    #[tracing::instrument(name = "textDocument/didOpen",
        skip_all,
        fields(uri = %params.text_document.uri)
    )]
    fn run(
        state: &mut State,
        _notifier: Notifier,
        _requester: &mut Requester<'_>,
        params: DidOpenTextDocumentParams,
    ) -> LSPResult<()> {
        let uri = params.text_document.uri;

        // Try to detect the crate for physical files.
        // The crate for virtual files is already known.
        if uri.scheme() == "file" {
            let Ok(path) = uri.to_file_path() else { return Ok(()) };

            state.project_controller.request_updating_project_for_file(path);
        }

        if let Some(file_id) = state.db.file_for_url(&uri) {
            state.open_files.insert(uri.clone());
            if let Some(content) = state.db.file_content(file_id)
                && content.long(&state.db).as_ref() != params.text_document.text.as_str()
            {
                override_file_content!(state.db, file_id, Some(params.text_document.text.into()));
            }

            state.code_lens_controller.on_did_change(
                state.db.clone(),
                state.config.clone(),
                is_cairo_file_path(&uri)
                    .then_some(FileChange { url: uri, was_deleted: false })
                    .into_iter(),
            );
        }

        Ok(())
    }
}

impl SyncNotificationHandler for DidSaveTextDocument {
    #[tracing::instrument(
        name = "textDocument/didSave",
        skip_all,
        fields(uri = %params.text_document.uri)
    )]
    fn run(
        state: &mut State,
        _notifier: Notifier,
        _requester: &mut Requester<'_>,
        params: DidSaveTextDocumentParams,
    ) -> LSPResult<()> {
        if let Some(file) = state.db.file_for_url(&params.text_document.uri) {
            override_file_content!(state.db, file, None);

            // In perfect scenario we would do this only for `file` but there is no way to make it more granulary.
            state.db.cancel_all();
        }

        Ok(())
    }
}

impl BackgroundDocumentRequestHandler for GotoDefinition {
    #[tracing::instrument(name = "textDocument/definition", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: GotoDefinitionParams,
    ) -> LSPResult<Option<GotoDefinitionResponse>> {
        Ok(catch_unwind(AssertUnwindSafe(|| {
            ide::navigation::goto_definition::goto_definition(params, &snapshot.db)
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("GotoDefinition handler panicked");
            None
        }))
    }
}

impl BackgroundDocumentRequestHandler for Completion {
    #[tracing::instrument(name = "textDocument/completion", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: CompletionParams,
    ) -> LSPResult<Option<CompletionResponse>> {
        Ok(catch_unwind(AssertUnwindSafe(|| ide::completion::complete(params, &snapshot.db)))
            .unwrap_or_else(|err| {
                if is_cancelled(err.as_ref()) {
                    resume_unwind(err);
                }
                error!("Completion handler panicked");
                None
            }))
    }
}

impl BackgroundDocumentRequestHandler for SemanticTokensFullRequest {
    #[tracing::instrument(name = "textDocument/semanticTokens/full", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        meta_state: MetaState,
        _notifier: Notifier,
        params: SemanticTokensParams,
    ) -> LSPResult<Option<SemanticTokensResult>> {
        Ok(catch_unwind(AssertUnwindSafe(|| {
            ide::semantic_highlighting::semantic_highlight_full(params, &snapshot.db, meta_state)
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("SemanticTokensFullRequest handler panicked");
            None
        }))
    }
}

impl BackgroundDocumentRequestHandler for ProvideVirtualFile {
    #[tracing::instrument(name = "vfs/provide", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: ProvideVirtualFileRequest,
    ) -> LSPResult<ProvideVirtualFileResponse> {
        let content = snapshot
            .db
            .file_for_url(&params.uri)
            .and_then(|file_id| snapshot.db.file_content(file_id))
            .map(|content| content.long(&snapshot.db).to_string());

        Ok(ProvideVirtualFileResponse { content })
    }
}

impl BackgroundDocumentRequestHandler for ViewAnalyzedCrates {
    #[tracing::instrument(name = "cairo/viewAnalyzedCrates", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        _params: (),
    ) -> LSPResult<String> {
        Ok(ide::introspection::crates::inspect_analyzed_crates(
            &snapshot.db,
            &snapshot.config,
            &snapshot.configs_registry,
            &snapshot.scarb_toolchain,
        ))
    }
}

impl BackgroundDocumentRequestHandler for ExpandMacro {
    #[tracing::instrument(name = "cairo/expandMacro", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: TextDocumentPositionParams,
    ) -> LSPResult<Option<String>> {
        Ok(ide::macros::expand::expand_macro(&snapshot.db, &params))
    }
}

impl BackgroundDocumentRequestHandler for ToolchainInfo {
    #[tracing::instrument(name = "cairo/toolchainInfo", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        _params: (),
    ) -> LSPResult<ToolchainInfoResponse> {
        toolchain_info(snapshot)
    }
}

impl BackgroundDocumentRequestHandler for References {
    #[tracing::instrument(name = "textDocument/references", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: ReferenceParams,
    ) -> LSPResult<Option<Vec<lsp_types::Location>>> {
        Ok(catch_unwind(AssertUnwindSafe(|| {
            ide::navigation::references::references(params, &snapshot.db)
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("References handler panicked");
            None
        }))
    }
}

impl BackgroundDocumentRequestHandler for DocumentHighlightRequest {
    #[tracing::instrument(name = "textDocument/documentHighlight", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: DocumentHighlightParams,
    ) -> LSPResult<Option<Vec<DocumentHighlight>>> {
        Ok(catch_unwind(AssertUnwindSafe(|| {
            ide::navigation::highlight::highlight(params, &snapshot.db)
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("DocumentHighlightRequest handler panicked");
            None
        }))
    }
}

impl BackgroundDocumentRequestHandler for Rename {
    #[tracing::instrument(name = "textDocument/rename", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: RenameParams,
    ) -> LSPResult<Option<WorkspaceEdit>> {
        catch_unwind(AssertUnwindSafe(|| {
            ide::navigation::rename::rename(params, &snapshot.db, &snapshot.client_capabilities)
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("Rename handler panicked");
            Ok(None)
        })
    }
}

impl BackgroundDocumentRequestHandler for ViewSyntaxTree {
    #[tracing::instrument(name = "cairo/viewSyntaxTree", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: TextDocumentPositionParams,
    ) -> LSPResult<Option<String>> {
        Ok(is_cairo_file_path(&params.text_document.uri)
            .then(|| {
                ide::introspection::syntax_tree::get_syntax_tree_for_file(&snapshot.db, params)
            })
            .flatten())
    }
}

impl BackgroundDocumentRequestHandler for CodeLensRequest {
    #[tracing::instrument(name = "textDocument/codeLens", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: CodeLensParams,
    ) -> LSPResult<Option<Vec<CodeLens>>> {
        Ok(catch_unwind(AssertUnwindSafe(|| {
            snapshot.code_lens_controller.code_lens(
                params.text_document.uri,
                &snapshot.db,
                &snapshot.config,
            )
        }))
        .unwrap_or_else(|err| {
            if is_cancelled(err.as_ref()) {
                resume_unwind(err);
            }
            error!("CodeLensRequest handler panicked");
            None
        }))
    }
}

impl BackgroundDocumentRequestHandler for WillRenameFiles {
    #[tracing::instrument(name = "workspace/willRenameFiles", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: RenameFilesParams,
    ) -> LSPResult<Option<WorkspaceEdit>> {
        Ok(lang::rename_file::rename_files(&snapshot.db, params))
    }
}

impl BackgroundDocumentRequestHandler for InlayHintRequest {
    #[tracing::instrument(name = "textDocument/inlayHint", skip_all)]
    fn run_with_snapshot(
        snapshot: StateSnapshot,
        _meta_state: MetaState,
        _notifier: Notifier,
        params: InlayHintParams,
    ) -> LSPResult<Option<Vec<InlayHint>>> {
        Ok(catch_unwind(AssertUnwindSafe(|| ide::inlay_hints::inlay_hints(&snapshot.db, params)))
            .unwrap_or_else(|err| {
                if is_cancelled(err.as_ref()) {
                    resume_unwind(err);
                }
                error!("InlayHintRequest handler panicked");
                None
            }))
    }
}

pub fn is_cairo_file_path(file_path: &Url) -> bool {
    file_path.path().ends_with(".cairo")
}
