use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::sync::{MutexGuard, RwLock, RwLockWriteGuard};

use anyhow::{Context, Result, anyhow, ensure};
use connection::ProcMacroServerConnection;
use crossbeam::channel::Sender;
use scarb_proc_macro_server_types::jsonrpc::{RequestId, RpcRequest, RpcResponse};
use scarb_proc_macro_server_types::methods::Method;
use scarb_proc_macro_server_types::methods::defined_macros::{
    DefinedMacros, DefinedMacrosParams, DefinedMacrosResponse,
};
use scarb_proc_macro_server_types::methods::expand::{
    ExpandAttribute, ExpandAttributeParams, ExpandDerive, ExpandDeriveParams, ExpandInline,
    ExpandInlineMacroParams,
};
pub use status::ServerStatus;
use tracing::error;

use crate::ide::analysis_progress::ProcMacroServerTracker;
use crate::lang::proc_macros::client::plain_request_response::{
    PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
};

pub mod connection;
mod id_generator;
pub mod plain_request_response;
pub mod status;

#[derive(Debug, PartialEq, Eq)]
pub enum RequestParams {
    Attribute(PlainExpandAttributeParams),
    Derive(PlainExpandDeriveParams),
    Inline(PlainExpandInlineParams),
}

pub struct ProcMacroClient {
    connection: ProcMacroServerConnection,
    id_generator: id_generator::IdGenerator,
    requests_params: RwLock<HashMap<RequestId, RequestParams>>,
    error_channel: Sender<()>,
    proc_macro_server_tracker: ProcMacroServerTracker,
}

impl ProcMacroClient {
    pub fn new(
        connection: ProcMacroServerConnection,
        error_channel: Sender<()>,
        proc_macro_server_tracker: ProcMacroServerTracker,
    ) -> Self {
        Self {
            connection,
            id_generator: Default::default(),
            requests_params: Default::default(),
            error_channel,
            proc_macro_server_tracker,
        }
    }

    pub fn was_requested(&self, request_params: RequestParams) -> bool {
        let requests = self.requests_params.read().unwrap();

        requests.values().any(|params| params == &request_params)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn request_attribute(&self, params: ExpandAttributeParams) {
        self.send_request::<ExpandAttribute>(params, |params| {
            RequestParams::Attribute(params.into())
        })
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn request_derives(&self, params: ExpandDeriveParams) {
        self.send_request::<ExpandDerive>(params, |params| RequestParams::Derive(params.into()))
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn request_inline_macros(&self, params: ExpandInlineMacroParams) {
        self.send_request::<ExpandInline>(params, |params| RequestParams::Inline(params.into()))
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn start_initialize(&self) {
        if let Err(err) = self.request_defined_macros() {
            error!("failed to request defined macros: {err:?}");

            self.failed();
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn finish_initialize(&self) -> Result<DefinedMacrosResponse> {
        self.handle_defined_macros()
            .inspect_err(|err| error!("failed to fetch defined macros: {err:?}"))
    }

    /// Returns an iterator over all available responses. This iterator does not wait for new
    /// responses. As long as this iterator is not dropped, any attempt to send requests will be
    /// blocked.
    pub fn available_responses(&self) -> Responses<'_> {
        let responses = self.connection.responses.lock().unwrap();
        let requests = self.requests_params.write().unwrap();

        Responses { responses, requests }
    }

    /// Waits for the proc macro server to be killed.
    pub(super) fn kill_proc_macro_server(self) {
        // Dropping this causes the thread responsible for writing requests to PMS to finish.
        // Consequently, the handler to PMS' stdin will be dropped.
        // Due to the way PMS is implemented, this will result in its death.
        drop(self.connection.requester);

        if self.connection.server_killed_receiver.wait().is_none() {
            error!("failed to receive information that proc macro server was killed");
        }
    }

    fn request_defined_macros(&self) -> Result<()> {
        let id = self.id_generator.unique_id();

        self.send_request_untracked::<DefinedMacros>(id, &DefinedMacrosParams {})?;

        ensure!(
            id == 0,
            "fetching defined macros should be the first sent request, expected id=0 instead it \
             is: {id}"
        );

        Ok(())
    }

    fn handle_defined_macros(&self) -> Result<DefinedMacrosResponse> {
        let response = self
            .connection
            .responses
            .lock()
            .expect("responses lock should not be poisoned")
            .pop_front()
            .expect("responses should not be empty after receiving response");

        ensure!(
            response.id == 0,
            "fetching defined macros should be done before any other request is sent, received \
             response id: {}, expected 0",
            response.id
        );

        let success = response
            .into_result()
            .map_err(|error| anyhow!("proc-macro-server responded with error: {error:?}"))?;

        serde_json::from_value(success)
            .context("failed to deserialize response for defined macros request")
    }

    fn send_request_untracked<M: Method>(&self, id: RequestId, params: &M::Params) -> Result<()> {
        self.connection
            .requester
            .send(RpcRequest {
                id,
                method: M::METHOD.to_string(),
                value: serde_json::to_value(params).unwrap(),
            })
            .with_context(|| anyhow!("sending request {id} failed"))
    }

    fn send_request<M: Method>(
        &self,
        params: M::Params,
        map: impl FnOnce(M::Params) -> RequestParams,
    ) {
        let id = self.id_generator.unique_id();
        // This must be locked before sending request so sending request and tracking is atomic
        // operation.
        let mut requests_params = self.requests_params.write().unwrap();

        match self.send_request_untracked::<M>(id, &params) {
            Ok(()) => {
                self.proc_macro_server_tracker.register_procmacro_request();
                requests_params.insert(id, map(params));
            }
            Err(err) => {
                error!("Sending request to proc-macro-server failed: {err:?}");

                self.failed();
            }
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    fn failed(&self) {
        let _ = self.error_channel.try_send(());
    }
}

impl Debug for ProcMacroClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcMacroClient")
            .field("connection", &self.connection)
            .field("id_generator", &self.id_generator)
            .field("requests_params", &self.requests_params)
            .field("error_channel", &self.error_channel)
            .finish_non_exhaustive()
    }
}

pub struct Responses<'a> {
    responses: MutexGuard<'a, VecDeque<RpcResponse>>,
    requests: RwLockWriteGuard<'a, HashMap<RequestId, RequestParams>>,
}

impl Iterator for Responses<'_> {
    type Item = (RequestParams, RpcResponse);

    fn next(&mut self) -> Option<Self::Item> {
        let response = self.responses.pop_front()?;
        let params = self.requests.remove(&response.id).unwrap();

        Some((params, response))
    }
}

impl Responses<'_> {
    pub fn len(&self) -> usize {
        self.responses.len()
    }
}
