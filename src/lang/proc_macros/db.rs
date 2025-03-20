use std::collections::HashMap;
use std::sync::Arc;

use super::client::ServerStatus;
use crate::lang::proc_macros::client::plain_request_response::{
    PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
};
use crate::lang::proc_macros::conversions::token_stream_v2_to_v1;
use cairo_lang_macro::{TextSpan, Token, TokenStream, TokenTree};
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use scarb_proc_macro_server_types::methods::expand::{
    ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams,
};

/// A set of queries that enable access to proc macro client from compiler plugins
/// `.generate_code()` methods.
#[salsa::query_group(ProcMacroDatabase)]
pub trait ProcMacroGroup {
    #[salsa::input]
    fn attribute_macro_resolution(
        &self,
    ) -> Arc<HashMap<PlainExpandAttributeParams, ProcMacroResult>>;
    #[salsa::input]
    fn derive_macro_resolution(&self) -> Arc<HashMap<PlainExpandDeriveParams, ProcMacroResult>>;
    #[salsa::input]
    fn inline_macro_resolution(&self) -> Arc<HashMap<PlainExpandInlineParams, ProcMacroResult>>;

    #[salsa::input]
    fn proc_macro_server_status(&self) -> ServerStatus;

    /// Returns the expansion of attribute macro.
    fn get_stored_attribute_expansion(
        &self,
        params: PlainExpandAttributeParams,
    ) -> Option<ProcMacroResult>;
    /// Returns the expansion of derive macros.
    fn get_stored_derive_expansion(
        &self,
        params: PlainExpandDeriveParams,
    ) -> Option<ProcMacroResult>;
    /// Returns the expansion of inline macro.
    fn get_stored_inline_macros_expansion(
        &self,
        params: PlainExpandInlineParams,
    ) -> Option<ProcMacroResult>;
}

pub fn init_proc_macro_group(db: &mut dyn ProcMacroGroup) {
    db.set_attribute_macro_resolution(Default::default());
    db.set_derive_macro_resolution(Default::default());
    db.set_inline_macro_resolution(Default::default());
    db.set_proc_macro_server_status(Default::default());
}

fn get_stored_attribute_expansion(
    db: &dyn ProcMacroGroup,
    params: PlainExpandAttributeParams,
) -> Option<ProcMacroResult> {
    db.attribute_macro_resolution().get(&params).cloned()
}

fn get_stored_derive_expansion(
    db: &dyn ProcMacroGroup,
    params: PlainExpandDeriveParams,
) -> Option<ProcMacroResult> {
    db.derive_macro_resolution().get(&params).cloned()
}

fn get_stored_inline_macros_expansion(
    db: &dyn ProcMacroGroup,
    params: PlainExpandInlineParams,
) -> Option<ProcMacroResult> {
    db.inline_macro_resolution().get(&params).cloned()
}

pub fn get_attribute_expansion(
    db: &dyn ProcMacroGroup,
    params: ExpandAttributeParams,
) -> ProcMacroResult {
    get_stored_attribute_expansion(db, params.clone().into()).unwrap_or_else(|| {
        let token_stream = params.item.clone();

        if let Some(client) = db.proc_macro_server_status().ready() {
            client.request_attribute(params);
        }

        ProcMacroResult {
            token_stream: token_stream_v2_to_v1(&token_stream),
            diagnostics: Default::default(),
            code_mappings: None,
        }
    })
}

pub fn get_derive_expansion(
    db: &dyn ProcMacroGroup,
    params: ExpandDeriveParams,
) -> ProcMacroResult {
    db.get_stored_derive_expansion(params.clone().into()).unwrap_or_else(|| {
        let token_stream = params.item.clone();

        if let Some(client) = db.proc_macro_server_status().ready() {
            client.request_derives(params);
        }

        ProcMacroResult {
            token_stream: token_stream_v2_to_v1(&token_stream),
            diagnostics: Default::default(),
            code_mappings: None,
        }
    })
}

pub fn get_inline_macros_expansion(
    db: &dyn ProcMacroGroup,
    params: ExpandInlineMacroParams,
) -> ProcMacroResult {
    db.get_stored_inline_macros_expansion(params.clone().into()).unwrap_or_else(|| {
        // we can't return original node because it will make infinite recursive resolving.
        let unit = "()".to_string();
        let span = TextSpan { start: 0, end: unit.len().try_into().unwrap() };
        let token_stream = TokenStream::new(vec![TokenTree::Ident(Token::new(unit, span))]);

        if let Some(client) = db.proc_macro_server_status().ready() {
            client.request_inline_macros(params);
        }

        ProcMacroResult {
            token_stream: token_stream_v2_to_v1(&token_stream),
            diagnostics: Default::default(),
            code_mappings: None,
        }
    })
}
