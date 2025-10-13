use std::collections::HashMap;

use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_macro::{Diagnostic, TextSpan, TokenStream, TokenTree};
use cairo_lang_syntax::node::SyntaxNode;
use salsa::{Database, Setter};
use scarb_proc_macro_server_types::conversions::token_stream_v2_to_v1;
use scarb_proc_macro_server_types::methods::{
    CodeOrigin, ProcMacroResult,
    expand::{ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams},
};

use super::client::{RequestParams, ServerStatus};
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::proc_macros::client::plain_request_response::{
    PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
};

/// A set of queries that enable access to proc macro client from compiler plugins
/// `.generate_code()` methods.
pub trait ProcMacroGroup: Database {
    /// Returns the expansion of attribute macro.
    fn get_stored_attribute_expansion(
        &self,
        params: PlainExpandAttributeParams,
    ) -> Option<ProcMacroResult> {
        get_stored_attribute_expansion(self.as_dyn_database(), params)
    }
    /// Returns the expansion of derive macros.
    fn get_stored_derive_expansion(
        &self,
        params: PlainExpandDeriveParams,
    ) -> Option<ProcMacroResult> {
        get_stored_derive_expansion(self.as_dyn_database(), params)
    }
    /// Returns the expansion of inline macro.
    fn get_stored_inline_macros_expansion(
        &self,
        params: PlainExpandInlineParams,
    ) -> Option<ProcMacroResult> {
        get_stored_inline_macros_expansion(self.as_dyn_database(), params)
    }
    fn proc_macro_input(&self) -> &ProcMacroInput {
        proc_macro_input(self.as_dyn_database())
    }

    fn reset_proc_macro_resolutions(&mut self) {
        proc_macro_input(self.as_dyn_database())
            .set_attribute_macro_resolution(self)
            .to(Default::default());
        proc_macro_input(self.as_dyn_database())
            .set_derive_macro_resolution(self)
            .to(Default::default());
        proc_macro_input(self.as_dyn_database())
            .set_inline_macro_resolution(self)
            .to(Default::default());
    }
}

impl<T: Database + ?Sized> ProcMacroGroup for T {}

#[salsa::input]
pub struct ProcMacroInput {
    #[returns(ref)]
    pub attribute_macro_resolution: HashMap<PlainExpandAttributeParams, ProcMacroResult>,
    #[returns(ref)]
    pub derive_macro_resolution: HashMap<PlainExpandDeriveParams, ProcMacroResult>,
    #[returns(ref)]
    pub inline_macro_resolution: HashMap<PlainExpandInlineParams, ProcMacroResult>,

    pub proc_macro_server_status: ServerStatus,
}

#[salsa::tracked(returns(ref))]
fn proc_macro_input(db: &dyn Database) -> ProcMacroInput {
    ProcMacroInput::new(
        db,
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
}

fn get_stored_attribute_expansion(
    db: &dyn Database,
    params: PlainExpandAttributeParams,
) -> Option<ProcMacroResult> {
    db.proc_macro_input().attribute_macro_resolution(db).get(&params).cloned()
}

fn get_stored_derive_expansion(
    db: &dyn Database,
    params: PlainExpandDeriveParams,
) -> Option<ProcMacroResult> {
    db.proc_macro_input().derive_macro_resolution(db).get(&params).cloned()
}

fn get_stored_inline_macros_expansion(
    db: &dyn Database,
    params: PlainExpandInlineParams,
) -> Option<ProcMacroResult> {
    db.proc_macro_input().inline_macro_resolution(db).get(&params).cloned()
}

pub fn get_attribute_expansion(
    db: &dyn Database,
    mut params: ExpandAttributeParams,
) -> ProcMacroResult {
    let stabilizer = SpansStabilizer::new(&mut params.adapted_call_site, &mut params.item);

    let result = db.get_stored_attribute_expansion(params.clone().into()).unwrap_or_else(|| {
        let token_stream = params.item.clone();

        if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
            && !client.was_requested(RequestParams::ExpandAttribute(params.clone().into()))
        {
            client.request_attribute(params);
        }

        ProcMacroResult {
            token_stream: token_stream_v2_to_v1(&token_stream),
            diagnostics: Default::default(),
            code_mappings: None,
        }
    });

    stabilizer.apply_original_offsets_to_result(result)
}

pub fn get_derive_expansion(db: &dyn Database, mut params: ExpandDeriveParams) -> ProcMacroResult {
    let stabilizer = SpansStabilizer::new(&mut params.call_site, &mut params.item);

    let result = db.get_stored_derive_expansion(params.clone().into()).unwrap_or_else(|| {
        if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
            && !client.was_requested(RequestParams::ExpandDerive(params.clone().into()))
        {
            client.request_derives(params);
        }

        ProcMacroResult {
            // We don't remove the original item for derive macros, so return nothing.
            token_stream: Default::default(),
            diagnostics: Default::default(),
            code_mappings: None,
        }
    });

    stabilizer.apply_original_offsets_to_result(result)
}

pub fn get_inline_macros_expansion(
    db: &dyn Database,
    mut params: ExpandInlineMacroParams,
) -> ProcMacroResult {
    let stabilizer = SpansStabilizer::new(&mut params.call_site, &mut params.args);

    let result =
        db.get_stored_inline_macros_expansion(params.clone().into()).unwrap_or_else(|| {
            // We can't return the original node because it will make us fall into infinite recursion.
            let unit = "()".to_string();

            if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
                && !client.was_requested(RequestParams::ExpandInline(params.clone().into()))
            {
                client.request_inline_macros(params);
            }

            ProcMacroResult {
                token_stream: cairo_lang_macro_v1::TokenStream::new(unit),
                diagnostics: Default::default(),
                code_mappings: None,
            }
        });

    stabilizer.apply_original_offsets_to_result(result)
}

/// When storing a procedural macro result, parameters are used as the cache key.
/// However, this approach is insufficient because the macro result's token stream may include spans from the input token stream.
/// Input spans can change if, for example, a user makes edits earlier in the file than where the item is defined.
/// This might result in situations where the input token stream remains identical, but its spans have shifted, triggering unnecessary macro recalculations.
/// Such recalculations can lead to failures in expanding macros due to a new 'analysis in progress' status.
/// To prevent this, we adjust the input parameters (input token stream, call site) by setting their offsets to stable values (0 for the token stream and [`Self::STABLE_CALL_SITE_START`] for the call site).
/// We then submit the request using these adjusted parameters as usual for caching.
/// Upon receiving a response, we modify the result (both token stream and call site) to replace spans using the original offsets and call site, as handled by [`Self::apply_original_offset_to_span`].
struct SpansStabilizer {
    original_call_site: TextSpan,
    original_item_offset: u32,
}

impl SpansStabilizer {
    /// Arbitrary number that must be bigger than anyting macro should produce.
    ///
    /// We use trick here to set call site for const value, then all mappings and diagnostics that points to this offest will be remaped to call site, instead of being increased by item offset.
    /// This is high enough to make sure there should be no collision with item mappings and diagnostics.
    const STABLE_CALL_SITE_START: u32 = 3000000000;

    pub fn new(call_site: &mut TextSpan, token_stream: &mut TokenStream) -> Self {
        let stable_call_site = TextSpan {
            // Hack: Use arbitrary high number for call site, this way there should be no collision with item.
            start: Self::STABLE_CALL_SITE_START,
            end: call_site.end - call_site.start,
        };
        let original_call_site = std::mem::replace(call_site, stable_call_site);

        // First token start is offset of whole item.
        let original_item_offset = match &token_stream.tokens[0] {
            TokenTree::Ident(token) => token.span.start,
        };

        // Reduce all tokens spans by item offset.
        for token in &mut token_stream.tokens {
            match token {
                TokenTree::Ident(token) => {
                    token.span.start -= original_item_offset;
                    token.span.end -= original_item_offset;
                }
            }
        }

        Self { original_call_site, original_item_offset }
    }

    pub fn apply_original_offsets_to_result(self, mut result: ProcMacroResult) -> ProcMacroResult {
        if let Some(code_mappings) = &mut result.code_mappings {
            for mapping in code_mappings.iter_mut() {
                match mapping.origin {
                    CodeOrigin::Start(_) => {
                        // Should be unreachable
                    }
                    CodeOrigin::Span(ref mut span) | CodeOrigin::CallSite(ref mut span) => {
                        self.apply_original_offset_to_span(span);
                    }
                };
            }
        }

        for diagnostic in &mut result.diagnostics {
            if let Some(mut span) = diagnostic.span() {
                self.apply_original_offset_to_span(&mut span);
                *diagnostic =
                    Diagnostic::spanned(span, diagnostic.severity(), diagnostic.message());
            }
        }

        result
    }

    fn apply_original_offset_to_span(&self, span: &mut TextSpan) {
        if span.start == Self::STABLE_CALL_SITE_START {
            *span = self.original_call_site.clone();
        } else {
            *span = TextSpan {
                start: span.start + self.original_item_offset,
                end: span.end + self.original_item_offset,
            };
        }
    }
}

/// Retrieves the widest matching original node in user code, which corresponds to passed node.
pub fn get_og_node<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> Option<SyntaxNode<'db>> {
    let (og_file, og_span) =
        get_originating_location(db, node.stable_ptr(db).file_id(db), node.span(db), None);

    db.widest_node_within_span(og_file, og_span)
}
