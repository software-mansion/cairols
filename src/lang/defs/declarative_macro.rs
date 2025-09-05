use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeMacroDef<'db> {
    name_stable_ptr: SyntaxStablePtrId<'db>,
    name: &'db str,
}

impl<'db> DeclarativeMacroDef<'db> {
    pub fn new(name_stable_ptr: SyntaxStablePtrId<'db>, name: &'db str) -> Self {
        Self { name_stable_ptr, name }
    }

    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId<'db> {
        self.name_stable_ptr
    }

    pub fn name(&self, _db: &'db AnalysisDatabase) -> &'db str {
        self.name
    }
}
