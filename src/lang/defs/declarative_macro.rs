use crate::lang::db::AnalysisDatabase;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclarativeMacroDef<'db> {
    definition_stable_ptr: SyntaxStablePtrId<'db>,
    name: &'db str,
}

impl<'db> DeclarativeMacroDef<'db> {
    pub fn new(definition_stable_ptr: SyntaxStablePtrId<'db>, name: &'db str) -> Self {
        Self { definition_stable_ptr, name }
    }

    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId<'db> {
        self.definition_stable_ptr
    }

    pub fn name(&self, _db: &'db AnalysisDatabase) -> &'db str {
        self.name
    }
}
