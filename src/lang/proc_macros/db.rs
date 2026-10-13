use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::SpanInFile;
use cairo_lang_macro::{Diagnostic, TextSpan};
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
    let stabilizer = CallSiteStabilizer::new(vec![&mut params.adapted_call_site]);

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

    stabilizer.restore(0, result)
}

/// Returns the expansion of every derive applied to an item, in the order they were given.
///
/// `fingerprints` holds the fingerprint of the macro providing each derive.
pub fn get_derive_expansions(
    db: &dyn Database,
    mut params: ExpandDeriveParams,
    fingerprints: &[u64],
) -> Vec<ProcMacroResult> {
    assert_eq!(
        params.derives.len(),
        fingerprints.len(),
        "every derive needs the fingerprint of the macro providing it"
    );

    let stabilizer = CallSiteStabilizer::new(
        params.derives.iter_mut().map(|derive| &mut derive.call_site).collect(),
    );

    let stored: Vec<_> = PlainExpandDeriveParams::of_request(&params)
        .into_iter()
        .zip(fingerprints)
        .map(|(key, &fingerprint)| db.get_stored_derive_expansion(key, fingerprint))
        .collect();

    request_missing_derives(db, &params, &stored);

    // We don't remove the original item for derive macros, so a missing result returns nothing.
    stored
        .into_iter()
        .enumerate()
        .map(|(index, result)| stabilizer.restore(index, result.unwrap_or_default()))
        .collect()
}

/// Requests the derives that are not cached yet, all of them in a single request.
fn request_missing_derives(
    db: &dyn Database,
    params: &ExpandDeriveParams,
    stored: &[Option<ProcMacroResult>],
) {
    let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected() else {
        return;
    };

    let missing = ExpandDeriveParams {
        context: params.context.clone(),
        derives: params
            .derives
            .iter()
            .zip(stored)
            .filter(|(_, result)| result.is_none())
            .map(|(derive, _)| derive.clone())
            .collect(),
        item: params.item.clone(),
    };

    if missing.derives.is_empty()
        || client.was_requested(RequestParams::ExpandDerive(PlainExpandDeriveParams::of_request(
            &missing,
        )))
    {
        return;
    }

    client.request_derives(missing);
}

pub fn get_inline_macros_expansion(
    db: &dyn Database,
    mut params: ExpandInlineMacroParams,
    fingerprint: u64,
) -> ProcMacroResult {
    let stabilizer = CallSiteStabilizer::new(vec![&mut params.call_site]);

    let result = db
        .get_stored_inline_macros_expansion(params.clone().into(), fingerprint)
        .unwrap_or_else(|| {
            let call_site = params.call_site.clone();

            if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
                && !client.was_requested(RequestParams::ExpandInline(params.clone().into()))
            {
                client.request_inline_macros(params);
            }

            // We can't return the original node because it will make us fall into infinite recursion.
            ProcMacroResult {
                token_stream: SpannedTokenStream::unspanned("()", call_site),
                ..Default::default()
            }
        });

    stabilizer.restore(0, result)
}

/// Swaps call sites for a fixed span while a request is keyed, cached and sent.
///
/// Generated code and diagnostics are attributed to the call site, so without this an expansion
/// would be cached and requested once per call site. [`Self::restore`] puts the real one back.
struct CallSiteStabilizer {
    originals: Vec<TextSpan>,
}

impl CallSiteStabilizer {
    /// Larger than any offset in a file, so it cannot collide with a span of the input.
    const STABLE_CALL_SITE_START: u32 = 3_000_000_000;

    /// Results are restored one by one, so every call site gets the same stable span.
    fn new(call_sites: Vec<&mut TextSpan>) -> Self {
        let originals = call_sites
            .into_iter()
            .map(|call_site| {
                let start = Self::STABLE_CALL_SITE_START;
                let stable = TextSpan { start, end: start + call_site.end - call_site.start };
                std::mem::replace(call_site, stable)
            })
            .collect();

        Self { originals }
    }

    fn restore(&self, index: usize, mut result: ProcMacroResult) -> ProcMacroResult {
        let original = &self.originals[index];
        let is_stable = |span: &TextSpan| span.start == Self::STABLE_CALL_SITE_START;

        for token in &mut result.token_stream.0 {
            if is_stable(&token.span) {
                token.span = original.clone();
            }
        }

        result.diagnostics = result
            .diagnostics
            .into_iter()
            .map(|diagnostic| match diagnostic.span() {
                Some(span) if is_stable(&span) => Diagnostic::spanned(
                    original.clone(),
                    diagnostic.severity(),
                    diagnostic.message(),
                ),
                _ => diagnostic,
            })
            .collect();

        result
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
