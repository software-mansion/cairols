// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff              |
// | File: `crates/ruff_server/src/server/api.rs`               |
// | Commit: 46a457318d8d259376a2b458b3f814b9b795fe69           |
// +------------------------------------------------------------+

use std::panic::{AssertUnwindSafe, catch_unwind};

use anyhow::anyhow;
use crossbeam::channel::Sender;
pub use handlers::is_cairo_file_path;
use lsp_server::{ErrorCode, ExtractError, Notification, Request, RequestId};
use lsp_types::notification::{
    Cancel, DidChangeConfiguration, DidChangeTextDocument, DidChangeWatchedFiles,
    DidCloseTextDocument, DidOpenTextDocument, DidSaveTextDocument,
    Notification as NotificationTrait, SetTrace,
};
use lsp_types::request::{
    CodeActionRequest, CodeLensRequest, Completion, DocumentHighlightRequest, ExecuteCommand,
    Formatting, GotoDefinition, HoverRequest, InlayHintRequest, References, Rename,
    Request as RequestTrait, SemanticTokensFullRequest, WillRenameFiles,
};
use tracing::{error, trace, warn};

use super::client::{Notifier, Responder};
use crate::lsp::ext::{
    ExpandMacro, ProvideVirtualFile, ShowMemoryUsage, ToolchainInfo, ViewAnalyzedCrates,
    ViewSyntaxTree,
};
use crate::lsp::result::{LSPError, LSPResult, LSPResultEx};
use crate::server::panic::cancelled_anyhow;
use crate::server::schedule::{BackgroundSchedule, Handler, RetryTaskInfo, Task};
use crate::state::{MetaState, State};

mod handlers;

pub fn request<'a>(
    request: Request,
    retry_sender: Sender<(RetryTaskInfo, Box<dyn Handler>)>,
) -> Task<'a> {
    let id = request.id.clone();

    match request.method.as_str() {
        CodeActionRequest::METHOD => background_request_task::<CodeActionRequest>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        Completion::METHOD => background_request_task::<Completion>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        ExecuteCommand::METHOD => local_request_task::<ExecuteCommand>(request),
        ExpandMacro::METHOD => background_request_task::<ExpandMacro>(
            request,
            BackgroundSchedule::Worker,
            retry_sender,
        ),
        Formatting::METHOD => background_fmt_task::<Formatting>(request, retry_sender),
        GotoDefinition::METHOD => background_request_task::<GotoDefinition>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        HoverRequest::METHOD => background_request_task::<HoverRequest>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        ProvideVirtualFile::METHOD => background_request_task::<ProvideVirtualFile>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        References::METHOD => background_request_task::<References>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        DocumentHighlightRequest::METHOD => background_request_task::<DocumentHighlightRequest>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        CodeLensRequest::METHOD => background_request_task::<CodeLensRequest>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        Rename::METHOD => background_request_task::<Rename>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        SemanticTokensFullRequest::METHOD => background_request_task::<SemanticTokensFullRequest>(
            request,
            BackgroundSchedule::Worker,
            retry_sender,
        ),
        ViewSyntaxTree::METHOD => background_request_task::<ViewSyntaxTree>(
            request,
            BackgroundSchedule::Worker,
            retry_sender,
        ),
        ToolchainInfo::METHOD => background_request_task::<ToolchainInfo>(
            request,
            BackgroundSchedule::Worker,
            retry_sender,
        ),
        ViewAnalyzedCrates::METHOD => background_request_task::<ViewAnalyzedCrates>(
            request,
            BackgroundSchedule::Worker,
            retry_sender,
        ),
        ShowMemoryUsage::METHOD => background_request_task::<ShowMemoryUsage>(
            request,
            BackgroundSchedule::Worker,
            retry_sender,
        ),
        WillRenameFiles::METHOD => background_request_task::<WillRenameFiles>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),
        InlayHintRequest::METHOD => background_request_task::<InlayHintRequest>(
            request,
            BackgroundSchedule::LatencySensitive,
            retry_sender,
        ),

        method => {
            warn!("received request {method} which does not have a handler");
            return Task::nothing();
        }
    }
    .unwrap_or_else(|error| {
        error!("encountered error when routing request with ID {id}: {error:?}");
        let result: Result<(), LSPError> = Err(error);
        Task::immediate(id, result)
    })
}

pub fn notification<'a>(notification: Notification) -> Task<'a> {
    match notification.method.as_str() {
        DidChangeTextDocument::METHOD => {
            local_notification_task::<DidChangeTextDocument>(notification)
        }
        DidChangeConfiguration::METHOD => {
            local_notification_task::<DidChangeConfiguration>(notification)
        }
        DidChangeWatchedFiles::METHOD => {
            local_notification_task::<DidChangeWatchedFiles>(notification)
        }
        DidCloseTextDocument::METHOD => {
            local_notification_task::<DidCloseTextDocument>(notification)
        }
        DidOpenTextDocument::METHOD => local_notification_task::<DidOpenTextDocument>(notification),
        DidSaveTextDocument::METHOD => local_notification_task::<DidSaveTextDocument>(notification),

        // Ignoring $/cancelRequest because CairoLS does cancellation inside-out when the state is
        // mutated, and we allow ourselves to ignore the corner case of user hitting ESC manually.
        Cancel::METHOD => Ok(Task::nothing()),

        // Ignoring $/setTrace because CairoLS never emits $/logTrace notifications anyway.
        SetTrace::METHOD => Ok(Task::nothing()),

        method => {
            warn!("received notification {method} which does not have a handler");

            return Task::nothing();
        }
    }
    .unwrap_or_else(|error| {
        error!("encountered error when routing notification: {error}");

        Task::nothing()
    })
}

fn local_request_task<'a, R: handlers::SyncRequestHandler>(
    request: Request,
) -> Result<Task<'a>, LSPError> {
    let (id, params) = cast_request::<R>(request)?;
    Ok(Task::local_mut(move |state, notifier, requester, responder| {
        let result = R::run(state, notifier, requester, params);
        respond::<R>(id, result, &responder);
    }))
}

fn background_request_task<'a, R: handlers::BackgroundDocumentRequestHandler + 'a>(
    request: Request,
    schedule: BackgroundSchedule,
    retry_sender: Sender<(RetryTaskInfo, Box<dyn Handler>)>,
) -> Result<Task<'a>, LSPError> {
    let (id, params) = cast_request::<R>(request)?;
    Ok(Task::background(
        schedule,
        create_background_fn_builder::<R>(
            id,
            params,
            RetryTaskInfo::Background(schedule),
            retry_sender,
        ),
    ))
}

fn background_fmt_task<'a, R: handlers::BackgroundDocumentRequestHandler + 'a>(
    request: Request,
    retry_sender: Sender<(RetryTaskInfo, Box<dyn Handler>)>,
) -> Result<Task<'a>, LSPError> {
    let (id, params) = cast_request::<R>(request)?;
    Ok(Task::fmt(create_background_fn_builder::<R>(id, params, RetryTaskInfo::Fmt, retry_sender)))
}

fn create_background_fn_handler_raw<R: handlers::BackgroundDocumentRequestHandler>(
    id: RequestId,
    params: serde_json::Value,
    retry_info: RetryTaskInfo,
    retry_sender: Sender<(RetryTaskInfo, Box<dyn Handler>)>,
) -> impl Handler {
    move |state_snapshot, meta_state, notifier, responder| match catch_unwind(AssertUnwindSafe(
        || {
            R::run_with_snapshot(
                state_snapshot,
                meta_state,
                notifier,
                serde_json::from_value(params.clone()).unwrap(),
            )
        },
    )) {
        Ok(result) => respond::<R>(id, result, &responder),
        Err(err) => {
            if let Ok(err) = cancelled_anyhow(err, "LSP worker thread was cancelled") {
                if R::RETRY {
                    let handler = create_background_fn_handler_raw::<R>(
                        id,
                        params,
                        retry_info,
                        retry_sender.clone(),
                    );

                    let _ = retry_sender.send((retry_info, Box::new(handler)));
                } else {
                    let err = LSPError::new(err, ErrorCode::ServerCancelled);
                    respond::<R>(id, Err(err), &responder)
                }
            } else {
                let err = LSPError::new(
                    anyhow!("caught panic in LSP worker thread"),
                    ErrorCode::InternalError,
                );
                respond::<R>(id, Err(err), &responder)
            }
        }
    }
}

fn create_background_fn_builder<R: handlers::BackgroundDocumentRequestHandler>(
    id: RequestId,
    params: <R as RequestTrait>::Params,
    retry_info: RetryTaskInfo,
    retry_sender: Sender<(RetryTaskInfo, Box<dyn Handler>)>,
) -> impl FnOnce(&State, MetaState) -> Box<dyn FnOnce(Notifier, Responder) + Send + 'static> {
    // Clone version of params.
    let params_json = serde_json::to_value(&params).unwrap();

    let handler = create_background_fn_handler_raw::<R>(id, params_json, retry_info, retry_sender);

    move |state: &State, meta_state: MetaState| {
        let state_snapshot = state.snapshot();
        Box::new(move |notifier, responder| {
            handler(state_snapshot, meta_state, notifier, responder);
        })
    }
}

fn local_notification_task<'a, N: handlers::SyncNotificationHandler>(
    notification: Notification,
) -> Result<Task<'a>, LSPError> {
    let (id, params) = cast_notification::<N>(notification)?;
    Ok(Task::local_mut(move |session, notifier, requester, _| {
        if let Err(err) = N::run(session, notifier, requester, params) {
            error!("an error occurred while running {id}: {err}");
        }
    }))
}

/// Tries to cast a serialized request from the server into
/// a parameter type for a specific request handler.
/// It is *highly* recommended to not override this function in your
/// implementation.
fn cast_request<R: RequestTrait>(request: Request) -> Result<(RequestId, R::Params), LSPError> {
    request
        .extract(R::METHOD)
        .map_err(|error| match error {
            json_error @ ExtractError::JsonError { .. } => {
                anyhow::anyhow!("JSON parsing failure:\n{json_error}")
            }
            ExtractError::MethodMismatch(_) => {
                unreachable!(
                    "a method mismatch should not be possible here unless you've used a different \
                     handler (`R`) than the one whose method name was matched against earlier"
                )
            }
        })
        .with_failure_code(ErrorCode::InternalError)
}

/// Sends back a response to the lsp_server using a [`Responder`].
fn respond<R: RequestTrait>(id: RequestId, result: LSPResult<R::Result>, responder: &Responder) {
    if let Err(err) = &result {
        match err.code {
            ErrorCode::ServerCancelled => trace!("request {id} was cancelled: {err:?}"),
            _ => error!("request {id} errored: {err:?}"),
        }
    }

    if let Err(err) = responder.respond(id.clone(), result) {
        error!("failed to respond to request {id}: {err}");
    }
}

/// Tries to cast a serialized request from the lsp_server into
/// a parameter type for a specific request handler.
fn cast_notification<N: NotificationTrait>(
    notification: Notification,
) -> Result<(&'static str, N::Params), LSPError> {
    Ok((
        N::METHOD,
        notification
            .extract(N::METHOD)
            .map_err(|error| match error {
                json_error @ ExtractError::JsonError { .. } => {
                    anyhow::anyhow!("JSON parsing failure:\n{json_error}")
                }
                ExtractError::MethodMismatch(_) => {
                    unreachable!(
                        "a method mismatch should not be possible here unless you've used a \
                         different handler (`N`) than the one whose method name was matched \
                         against earlier"
                    )
                }
            })
            .with_failure_code(ErrorCode::InternalError)?,
    ))
}
