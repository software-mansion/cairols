use super::{DEFAULT_CAPACITY, set};
use crate::lang::db::AnalysisDatabase;

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: 8192 for cairo_lang_semantic::db {
            PrivTypeIsVarFreeQuery,
            PrivImplIsVarFreeQuery
        }
    );
    set!(
        in db: 4096 for cairo_lang_semantic::db {
            ConcreteFunctionSignatureQuery,
            ImplConcreteTraitQuery,
            GetClosureParamsQuery,
        }
    );
    set!(
        in db: 2048 for cairo_lang_semantic::db {
            ConcreteTraitFunctionGenericParamsQuery,
            TypeInfoQuery,
        }
    );
    set!(
        in db: DEFAULT_CAPACITY for cairo_lang_semantic::db {
            CanonicTraitSolutionsQuery,
            FunctionDeclarationInlineConfigQuery,
            FunctionWithBodyGenericParamsQuery,
            FunctionWithBodySignatureQuery,
            ImplFunctionGenericParamsQuery,
            PrivTypeIsFullyConcreteQuery,
            ModuleItemInfoByNameQuery,
            ModuleItemByNameQuery,
        }
    );
}
