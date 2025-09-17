use salsa::Update;
use scarb_proc_macro_server_types::methods::expand::{
    ExpandAttributeParams, ExpandDeriveParams, ExpandInlineMacroParams,
};
use scarb_proc_macro_server_types::scope::ProcMacroScope;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Update)]
pub struct PlainExpandAttributeParams {
    pub context: ProcMacroScope,
    pub attr: String,
    pub args: String,
    pub item: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Update)]
pub struct PlainExpandDeriveParams {
    pub context: ProcMacroScope,
    pub derives: Vec<String>,
    pub item: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Update)]
pub struct PlainExpandInlineParams {
    pub context: ProcMacroScope,
    pub name: String,
    pub args: String,
}

impl From<ExpandAttributeParams> for PlainExpandAttributeParams {
    fn from(value: ExpandAttributeParams) -> Self {
        Self {
            context: value.context,
            attr: value.attr,
            args: value.args.to_string(),
            item: value.item.to_string(),
        }
    }
}
impl From<ExpandDeriveParams> for PlainExpandDeriveParams {
    fn from(value: ExpandDeriveParams) -> Self {
        Self { context: value.context, derives: value.derives, item: value.item.to_string() }
    }
}
impl From<ExpandInlineMacroParams> for PlainExpandInlineParams {
    fn from(value: ExpandInlineMacroParams) -> Self {
        Self { context: value.context, name: value.name, args: value.args.to_string() }
    }
}
