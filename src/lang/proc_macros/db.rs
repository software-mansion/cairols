use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_macro::{Diagnostic, TextSpan, TokenStream, TokenTree};
use cairo_lang_proc_macros::HeapSize;
use cairo_lang_syntax::node::SyntaxNode;
use salsa::{Database, Setter};
use scarb_proc_macro_server_types::methods::{
    ProcMacroResult, SpannedTokenStream,
    expand::{ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams},
};

use super::client::{RequestParams, ServerStatus};
use crate::lang::db::{AnalysisDatabase, LsSyntaxGroup};
use crate::lang::proc_macros::client::plain_request_response::{
    PlainExpandAttributeParams, PlainExpandDeriveParams, PlainExpandInlineParams,
};
use crate::proc_macros::cache::ProcMacroCache;

/// A set of queries that enable access to proc macro client from compiler plugins
/// `.generate_code()` methods.
pub trait ProcMacroGroup: Database {
    /// Returns the expansion of attribute macro.
    fn get_stored_attribute_expansion(
        &self,
        params: PlainExpandAttributeParams,
        fingerprint: u64,
    ) -> Option<ProcMacroResult> {
        get_stored_attribute_expansion(self.as_dyn_database(), params, fingerprint)
    }
    /// Returns the expansion of derive macros.
    fn get_stored_derive_expansion(
        &self,
        params: PlainExpandDeriveParams,
        fingerprint: u64,
    ) -> Option<ProcMacroResult> {
        get_stored_derive_expansion(self.as_dyn_database(), params, fingerprint)
    }
    /// Returns the expansion of inline macro.
    fn get_stored_inline_macros_expansion(
        &self,
        params: PlainExpandInlineParams,
        fingerprint: u64,
    ) -> Option<ProcMacroResult> {
        get_stored_inline_macros_expansion(self.as_dyn_database(), params, fingerprint)
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
#[derive(HeapSize)]
pub struct ProcMacroInput {
    #[returns(ref)]
    pub attribute_macro_resolution:
        ProcMacroCache<(PlainExpandAttributeParams, u64), ProcMacroResult>,
    #[returns(ref)]
    pub derive_macro_resolution: ProcMacroCache<(PlainExpandDeriveParams, u64), ProcMacroResult>,
    #[returns(ref)]
    pub inline_macro_resolution: ProcMacroCache<(PlainExpandInlineParams, u64), ProcMacroResult>,

    pub proc_macro_server_status: ServerStatus,
}

#[cairo_lang_proc_macros::tracked(returns(ref))]
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
    fingerprint: u64,
) -> Option<ProcMacroResult> {
    db.proc_macro_input().attribute_macro_resolution(db).get(&(params, fingerprint)).cloned()
}

fn get_stored_derive_expansion(
    db: &dyn Database,
    params: PlainExpandDeriveParams,
    fingerprint: u64,
) -> Option<ProcMacroResult> {
    db.proc_macro_input().derive_macro_resolution(db).get(&(params, fingerprint)).cloned()
}

fn get_stored_inline_macros_expansion(
    db: &dyn Database,
    params: PlainExpandInlineParams,
    fingerprint: u64,
) -> Option<ProcMacroResult> {
    db.proc_macro_input().inline_macro_resolution(db).get(&(params, fingerprint)).cloned()
}

pub fn get_attribute_expansion(
    db: &dyn Database,
    mut params: ExpandAttributeParams,
    fingerprint: u64,
) -> ProcMacroResult {
    let stabilizer = SpansStabilizer::new(&mut params.adapted_call_site, &mut params.item);

    let result = db
        .get_stored_attribute_expansion(params.clone().into(), fingerprint)
        .unwrap_or_else(|| {
            let token_stream = params.item.clone();

            if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
                && !client.was_requested(RequestParams::ExpandAttribute(params.clone().into()))
            {
                client.request_attribute(params);
            }

            ProcMacroResult {
                token_stream: SpannedTokenStream::from_token_stream(&token_stream),
                ..Default::default()
            }
        });

    stabilizer.apply_original_offsets_to_result(result)
}

pub fn get_derive_expansion(
    db: &dyn Database,
    mut params: ExpandDeriveParams,
    fingerprint: u64,
) -> ProcMacroResult {
    let stabilizer = SpansStabilizer::new(&mut params.call_site, &mut params.item);

    let result =
        db.get_stored_derive_expansion(params.clone().into(), fingerprint).unwrap_or_else(|| {
            if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
                && !client.was_requested(RequestParams::ExpandDerive(params.clone().into()))
            {
                client.request_derives(params);
            }

            // We don't remove the original item for derive macros, so return nothing.
            Default::default()
        });

    stabilizer.apply_original_offsets_to_result(result)
}

pub fn get_inline_macros_expansion(
    db: &dyn Database,
    mut params: ExpandInlineMacroParams,
    fingerprint: u64,
) -> ProcMacroResult {
    let stabilizer = SpansStabilizer::new(&mut params.call_site, &mut params.args);

    let result = db
        .get_stored_inline_macros_expansion(params.clone().into(), fingerprint)
        .unwrap_or_else(|| {
            if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
                && !client.was_requested(RequestParams::ExpandInline(params.clone().into()))
            {
                client.request_inline_macros(params);
            }

            // We can't return the original node because it will make us fall into infinite
            // recursion. The placeholder is attributed to the stable call site, so that once the
            // offsets are restored below it points at the macro call in the original file.
            const UNIT: &str = "()";
            ProcMacroResult {
                token_stream: SpannedTokenStream::unspanned(
                    UNIT,
                    TextSpan::new(
                        SpansStabilizer::STABLE_CALL_SITE_START,
                        SpansStabilizer::STABLE_CALL_SITE_START + UNIT.len() as u32,
                    ),
                ),
                ..Default::default()
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
/// Upon receiving a response, we move the spans of its tokens and diagnostics back onto the original file, as handled by [`Self::apply_original_offset_to_span`].
struct SpansStabilizer {
    original_call_site: TextSpan,
    original_item_offset: u32,
}

impl SpansStabilizer {
    /// Arbitrary number that must be bigger than anyting macro should produce.
    ///
    /// We use a trick here and set the call site to a constant value, so that every token and diagnostic pointing at this offset is remapped to the call site, instead of being shifted by the item offset.
    /// This is high enough to make sure there should be no collision with item mappings and diagnostics.
    pub(crate) const STABLE_CALL_SITE_START: u32 = 3000000000;

    pub fn new(call_site: &mut TextSpan, token_stream: &mut TokenStream) -> Self {
        let stable_call_site = TextSpan {
            // Hack: Use arbitrary high number for call site, this way there should be no collision with item.
            start: Self::STABLE_CALL_SITE_START,
            end: call_site.end - call_site.start,
        };
        let original_call_site = std::mem::replace(call_site, stable_call_site);

        // First token start is offset of whole item.
        let original_item_offset = match token_stream.tokens.first() {
            Some(TokenTree::Ident(token)) => token.span.start,
            None => 0,
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
        for token in &mut result.token_stream.0 {
            self.apply_original_offset_to_span(&mut token.span);
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
    let SpanInFile { file_id, span } = get_originating_location(
        db,
        SpanInFile { file_id: node.stable_ptr(db).file_id(db), span: node.span(db) },
        None,
    );

    db.widest_node_within_span(file_id, span)
}

#[cfg(test)]
mod tests {
    use cairo_lang_macro::{AllocationContext, Severity, Token};
    use scarb_proc_macro_server_types::methods::SpannedToken;

    use super::*;

    fn token_stream(tokens: &[(&str, u32, u32)]) -> TokenStream {
        let ctx = AllocationContext::default();
        TokenStream::new(
            tokens
                .iter()
                .map(|(content, start, end)| {
                    TokenTree::Ident(Token::new_in(content, TextSpan::new(*start, *end), &ctx))
                })
                .collect(),
        )
    }

    fn spans_of(token_stream: &TokenStream) -> Vec<(u32, u32)> {
        token_stream
            .tokens
            .iter()
            .map(|token| {
                let TokenTree::Ident(token) = token;
                (token.span.start, token.span.end)
            })
            .collect()
    }

    #[test]
    fn input_spans_are_rebased_onto_the_item() {
        // An item further down the file still hashes to the same cache key as the same item at the
        // top of the file, because the stabilizer moves its first token to offset zero.
        let mut item = token_stream(&[("fn ", 100, 103), ("foo", 103, 106), ("() {}", 106, 111)]);
        let mut call_site = TextSpan::new(100, 111);

        SpansStabilizer::new(&mut call_site, &mut item);

        assert_eq!(spans_of(&item), vec![(0, 3), (3, 6), (6, 11)]);
        assert_eq!(call_site.start, SpansStabilizer::STABLE_CALL_SITE_START);
        // The call site keeps its width, so a macro can still tell how long it is.
        assert_eq!(call_site.end, 11);
    }

    #[test]
    fn result_spans_are_moved_back_onto_the_original_file() {
        let mut item = token_stream(&[("fn ", 100, 103), ("foo", 103, 106)]);
        let mut call_site = TextSpan::new(100, 106);
        let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

        let result = ProcMacroResult {
            token_stream: SpannedTokenStream(vec![SpannedToken {
                content: "fn bar".to_string(),
                span: TextSpan::new(0, 6),
            }]),
            ..Default::default()
        };

        let result = stabilizer.apply_original_offsets_to_result(result);
        assert_eq!(result.token_stream.0[0].span, TextSpan::new(100, 106));
    }

    #[test]
    fn spans_pointing_at_the_call_site_are_restored_verbatim() {
        // Macros report code they invented, rather than copied, against the call site. That marker
        // must come back as the original call site, not as an offset into the item.
        let mut item = token_stream(&[("struct S {}", 40, 51)]);
        let mut call_site = TextSpan::new(20, 33);
        let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

        let result = ProcMacroResult {
            token_stream: SpannedTokenStream(vec![SpannedToken {
                content: "impl S {}".to_string(),
                span: call_site.clone(),
            }]),
            ..Default::default()
        };

        let result = stabilizer.apply_original_offsets_to_result(result);
        assert_eq!(result.token_stream.0[0].span, TextSpan::new(20, 33));
    }

    #[test]
    fn diagnostic_spans_are_moved_back_too() {
        let mut item = token_stream(&[("fn foo() {}", 200, 211)]);
        let mut call_site = TextSpan::new(200, 211);
        let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

        let result = ProcMacroResult {
            diagnostics: vec![Diagnostic::spanned(
                TextSpan::new(3, 6),
                Severity::Error,
                "bad name".to_string(),
            )],
            ..Default::default()
        };

        let result = stabilizer.apply_original_offsets_to_result(result);
        assert_eq!(result.diagnostics[0].span(), Some(TextSpan::new(203, 206)));
    }

    #[test]
    fn an_empty_input_stream_is_handled() {
        // Nothing to rebase against, so offsets pass through unchanged rather than panicking.
        let mut item = TokenStream::empty();
        let mut call_site = TextSpan::new(0, 0);
        let stabilizer = SpansStabilizer::new(&mut call_site, &mut item);

        let result = ProcMacroResult {
            token_stream: SpannedTokenStream::unspanned("x", TextSpan::new(1, 2)),
            ..Default::default()
        };

        let result = stabilizer.apply_original_offsets_to_result(result);
        assert_eq!(result.token_stream.0[0].span, TextSpan::new(1, 2));
    }
}
