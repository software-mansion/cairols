use cairo_lang_macro::ExpansionKind as ExpansionKindV2;
use cairo_lang_utils::smol_str::SmolStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExpansionKind {
    Attr,
    Derive,
    Inline,
    // Executable,
}

impl From<ExpansionKindV2> for ExpansionKind {
    fn from(kind: ExpansionKindV2) -> Self {
        match kind {
            ExpansionKindV2::Attr => Self::Attr,
            ExpansionKindV2::Derive => Self::Derive,
            ExpansionKindV2::Inline => Self::Inline,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Expansion {
    pub name: SmolStr,
    pub kind: ExpansionKind,
}
