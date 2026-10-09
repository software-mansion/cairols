use cairo_lang_macro::TextSpan;
use salsa::SalsaValue;
use scarb_proc_macro_server_types::methods::expand::{
    ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams,
};
use scarb_proc_macro_server_types::scope::ProcMacroScope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, SalsaValue)]
pub struct PlainExpandAttributeParams {
    pub context: ProcMacroScope,
    pub attr: String,
    pub args: String,
    pub item: String,
    pub call_site: TextSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, SalsaValue)]
pub struct PlainExpandDeriveParams {
    pub context: ProcMacroScope,
    pub derive: String,
    pub item: String,
    pub call_site: TextSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, SalsaValue)]
pub struct PlainExpandInlineParams {
    pub context: ProcMacroScope,
    pub name: String,
    pub args: String,
    pub call_site: TextSpan,
}

impl From<ExpandAttributeParams> for PlainExpandAttributeParams {
    fn from(value: ExpandAttributeParams) -> Self {
        Self {
            context: value.context,
            attr: value.attr,
            args: value.args.to_string(),
            item: value.item.to_string(),
            call_site: value.adapted_call_site,
        }
    }
}
impl PlainExpandDeriveParams {
    /// One key per derive of the request, in the same order.
    ///
    /// Derives of a single item are requested together, but cached one by one, so that rebuilding
    /// one macro does not invalidate the derives provided by the others.
    pub fn of_request(params: &ExpandDeriveParams) -> Vec<Self> {
        let item = params.item.to_string();

        params
            .derives
            .iter()
            .map(|derive| Self {
                context: params.context.clone(),
                derive: derive.name.clone(),
                item: item.clone(),
                call_site: derive.call_site.clone(),
            })
            .collect()
    }
}
impl From<ExpandInlineMacroParams> for PlainExpandInlineParams {
    fn from(value: ExpandInlineMacroParams) -> Self {
        Self {
            context: value.context,
            name: value.name,
            args: value.args.to_string(),
            call_site: value.call_site,
        }
    }
}
