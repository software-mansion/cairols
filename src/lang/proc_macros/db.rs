use std::collections::HashMap;
use std::sync::Arc;

use scarb_proc_macro_server_types::conversions::token_stream_v2_to_v1;
use scarb_proc_macro_server_types::methods::expand::{
    ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams,
};
use scarb_proc_macro_server_types::methods::{CodeOrigin, ProcMacroResult};

use super::client::ServerStatus;
use crate::lang::proc_macros::client::plain_request_response::{
    PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
};
use cairo_lang_macro::{TextSpan, TokenStream, TokenTree};

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
    mut params: ExpandAttributeParams,
) -> ProcMacroResult {
    let original_call_site = replace_call_site(&mut params.call_site);
    let item_offset = replace_token_stream_offset(&mut params.item);

    let mut result =
        db.get_stored_attribute_expansion(params.clone().into()).unwrap_or_else(|| {
            let token_stream = params.item.clone();

            if let Some(client) = db.proc_macro_server_status().ready() {
                client.request_attribute(params);
            }

            ProcMacroResult {
                token_stream: token_stream_v2_to_v1(&token_stream),
                diagnostics: Default::default(),
                code_mappings: None,
            }
        });

    correct_spans(&mut result, original_call_site, item_offset);

    result
}

pub fn get_derive_expansion(
    db: &dyn ProcMacroGroup,
    mut params: ExpandDeriveParams,
) -> ProcMacroResult {
    let original_call_site = replace_call_site(&mut params.call_site);
    let item_offset = replace_token_stream_offset(&mut params.item);

    let mut result = db.get_stored_derive_expansion(params.clone().into()).unwrap_or_else(|| {
        if let Some(client) = db.proc_macro_server_status().ready() {
            client.request_derives(params);
        }

        ProcMacroResult {
            // We don't remove the original item for derive macros, so return nothing.
            token_stream: Default::default(),
            diagnostics: Default::default(),
            code_mappings: None,
        }
    });

    correct_spans(&mut result, original_call_site, item_offset);

    result
}

pub fn get_inline_macros_expansion(
    db: &dyn ProcMacroGroup,
    mut params: ExpandInlineMacroParams,
) -> ProcMacroResult {
    let original_call_site = replace_call_site(&mut params.call_site);
    let item_offset = replace_token_stream_offset(&mut params.args);

    let mut result =
        db.get_stored_inline_macros_expansion(params.clone().into()).unwrap_or_else(|| {
            // We can't return the original node because it will make us fall into infinite recursion.
            let unit = "()".to_string();

            if let Some(client) = db.proc_macro_server_status().ready() {
                client.request_inline_macros(params);
            }

            ProcMacroResult {
                token_stream: cairo_lang_macro_v1::TokenStream::new(unit),
                diagnostics: Default::default(),
                code_mappings: None,
            }
        });

    correct_spans(&mut result, original_call_site, item_offset);

    result
}

fn replace_token_stream_offset(token_stream: &mut TokenStream) -> u32 {
    let first_span_start = match &token_stream.tokens[0] {
        TokenTree::Ident(token) => token.span.start,
    };

    for token in &mut token_stream.tokens {
        match token {
            TokenTree::Ident(token) => {
                token.span.start -= first_span_start;
                token.span.end -= first_span_start;
            }
        }
    }

    first_span_start
}

/// Arbitrary number that must be bigger than anyting macro should produce.
const STABLE_CALL_SITE_START: u32 = 3000000000;

fn replace_call_site(call_site: &mut TextSpan) -> TextSpan {
    let stable_call_site = TextSpan {
        // Hack: Use arbitrary high number for call site, this way there should be no collision with item.
        start: STABLE_CALL_SITE_START,
        end: call_site.end - call_site.start,
    };

    std::mem::replace(call_site, stable_call_site)
}

fn correct_spans(result: &mut ProcMacroResult, original_call_site: TextSpan, item_offset: u32) {
    fn correct_span(span: &mut TextSpan, original_call_site: TextSpan, item_offset: u32) {
        if span.start == STABLE_CALL_SITE_START {
            *span = TextSpan { start: original_call_site.start, end: original_call_site.end }
        } else {
            *span = TextSpan { start: span.start + item_offset, end: span.end + item_offset };
        }
    }

    if let Some(code_mappings) = &mut result.code_mappings {
        for mapping in code_mappings.iter_mut() {
            match mapping.origin {
                CodeOrigin::Start(_) => {
                    // Should be unreachable
                }
                CodeOrigin::Span(ref mut span) | CodeOrigin::CallSite(ref mut span) => {
                    correct_span(span, original_call_site.clone(), item_offset);
                }
            };
        }
    }

    for diagnostic in &mut result.diagnostics {
        if let Some(span) = &mut diagnostic.span {
            correct_span(span, original_call_site.clone(), item_offset);
        }
    }
}
