use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::SpanInFile;
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
    params: ExpandAttributeParams,
    fingerprint: u64,
) -> ProcMacroResult {
    db.get_stored_attribute_expansion(params.clone().into(), fingerprint).unwrap_or_else(|| {
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
    })
}

pub fn get_derive_expansion(
    db: &dyn Database,
    params: ExpandDeriveParams,
    fingerprint: u64,
) -> ProcMacroResult {
    db.get_stored_derive_expansion(params.clone().into(), fingerprint).unwrap_or_else(|| {
        if let Some(client) = db.proc_macro_input().proc_macro_server_status(db).connected()
            && !client.was_requested(RequestParams::ExpandDerive(params.clone().into()))
        {
            client.request_derives(params);
        }

        // We don't remove the original item for derive macros, so return nothing.
        Default::default()
    })
}

pub fn get_inline_macros_expansion(
    db: &dyn Database,
    params: ExpandInlineMacroParams,
    fingerprint: u64,
) -> ProcMacroResult {
    db.get_stored_inline_macros_expansion(params.clone().into(), fingerprint).unwrap_or_else(|| {
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
    })
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
