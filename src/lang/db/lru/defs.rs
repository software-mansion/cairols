use crate::lang::db::AnalysisDatabase;

use super::{INCREASED_CAPACITY, set};

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    let reduced_capacity = 512;

    set!(
        in db: reduced_capacity for cairo_lang_defs::db {
            PrivModuleSubFilesQuery
        }
    );

    // set!(
    //     in db: INCREASED_CAPACITY for cairo_lang_defs::db {
    //         AllowedAttributesQuery, AllowedStatementAttributesQuery, CrateModulesQuery,
    //         DeclaredDerivesQuery, DeclaredPhantomTypeAttributesQuery, FileModulesQuery,
    //         IsSubmoduleInlineQuery, ModuleAncestorsQuery, ModuleConstantByIdQuery, ModuleConstantsIdsQuery,
    //         ModuleConstantsQuery, ModuleDirQuery, ModuleEnumByIdQuery, ModuleEnumsIdsQuery,
    //         ModuleEnumsQuery, ModuleExternFunctionByIdQuery, ModuleExternFunctionsIdsQuery,
    //         ModuleExternFunctionsQuery, ModuleExternTypeByIdQuery, ModuleExternTypesIdsQuery,
    //         ModuleExternTypesQuery, ModuleFileQuery, ModuleFilesQuery, ModuleFreeFunctionByIdQuery,
    //         ModuleFreeFunctionsIdsQuery, ModuleFreeFunctionsQuery, ModuleGeneratedFileAuxDataQuery,
    //         ModuleGlobalUseByIdQuery, ModuleGlobalUsesQuery, ModuleImplAliasByIdQuery,
    //         ModuleImplAliasesIdsQuery, ModuleImplAliasesQuery, ModuleImplByIdQuery, ModuleImplsIdsQuery,
    //         ModuleImplsQuery, ModuleItemNameStablePtrQuery, ModuleItemsQuery, ModuleMainFileQuery,
    //         ModulePluginDiagnosticsNotesQuery, ModulePluginDiagnosticsQuery, ModuleStructByIdQuery,
    //         ModuleStructsIdsQuery, ModuleStructsQuery, ModuleSubmodulesIdsQuery, ModuleSubmodulesQuery,
    //         ModuleTraitByIdQuery, ModuleTraitsIdsQuery, ModuleTraitsQuery, ModuleTypeAliasByIdQuery,
    //         ModuleTypeAliasesIdsQuery, ModuleTypeAliasesQuery, ModuleUseByIdQuery, ModuleUsesIdsQuery,
    //         ModuleUsesQuery, PrivFileToModuleMappingQuery, PrivModuleDataQuery, PrivModuleSubFilesQuery,
    //     }
    // );
}
