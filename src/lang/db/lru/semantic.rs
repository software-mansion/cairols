use super::{NORMAL_CAPACITY, set};
use crate::lang::db::AnalysisDatabase;

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    let capacity = 2048;
    set!(
        in db: capacity for cairo_lang_semantic::db {
            PrivTypeIsVarFreeQuery, PrivImplIsVarFreeQuery, ConcreteFunctionSignatureQuery,
            ImplConcreteTraitQuery, GetClosureParamsQuery, ConcreteTraitFunctionGenericParamsQuery,
            TypeInfoQuery, CanonicTraitSolutionsQuery,
        }
    );

    let medium_capacity = 1024;
    set!(
        in db: medium_capacity for cairo_lang_semantic::db {
            FunctionDeclarationInlineConfigQuery, FunctionWithBodyGenericParamsQuery,
            FunctionWithBodySignatureQuery, ImplFunctionGenericParamsQuery, PrivTypeIsFullyConcreteQuery,
            ModuleItemInfoByNameQuery, ModuleItemByNameQuery,
        }
    );

    let reduced_capacity = 512;
    set!(
        in db: reduced_capacity for cairo_lang_semantic::db {
            TypeSizeInfoQuery,
        }
    );
}
