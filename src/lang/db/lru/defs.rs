use super::{DEFAULT_CAPACITY, set};
use crate::lang::db::AnalysisDatabase;

pub fn set_lru_capacity(db: &mut AnalysisDatabase) {
    set!(
        in db: DEFAULT_CAPACITY for cairo_lang_defs::db {
            AllowedAttributesQuery, AllowedStatementAttributesQuery, CrateModulesQuery,
            DeclaredDerivesQuery, DeclaredPhantomTypeAttributesQuery, FileModulesQuery,
            IsSubmoduleInlineQuery, ModuleAncestorsQuery, ModuleConstantByIdQuery, ModuleConstantsIdsQuery,
            ModuleConstantsQuery, ModuleDirQuery, ModuleEnumByIdQuery, ModuleEnumsIdsQuery,
            ModuleEnumsQuery, ModuleExternFunctionByIdQuery, ModuleExternFunctionsIdsQuery,
            ModuleExternFunctionsQuery, ModuleExternTypeByIdQuery, ModuleExternTypesIdsQuery,
            ModuleExternTypesQuery, ModuleFileQuery, ModuleFilesQuery, ModuleFreeFunctionByIdQuery,
            ModuleFreeFunctionsIdsQuery, ModuleFreeFunctionsQuery, ModuleGeneratedFileAuxDataQuery,
            ModuleGlobalUseByIdQuery, ModuleGlobalUsesQuery, ModuleImplAliasByIdQuery,
            ModuleImplAliasesIdsQuery, ModuleImplAliasesQuery, ModuleImplByIdQuery, ModuleImplsIdsQuery,
            ModuleImplsQuery, ModuleItemNameStablePtrQuery, ModuleItemsQuery, ModuleMainFileQuery,
            ModulePluginDiagnosticsNotesQuery, ModulePluginDiagnosticsQuery, ModuleStructByIdQuery,
            ModuleStructsIdsQuery, ModuleStructsQuery, ModuleSubmodulesIdsQuery, ModuleSubmodulesQuery,
            ModuleTraitByIdQuery, ModuleTraitsIdsQuery, ModuleTraitsQuery, ModuleTypeAliasByIdQuery,
            ModuleTypeAliasesIdsQuery, ModuleTypeAliasesQuery, ModuleUseByIdQuery, ModuleUsesIdsQuery,
            ModuleUsesQuery, PrivFileToModuleMappingQuery, PrivModuleDataQuery, PrivModuleSubFilesQuery,
        }
    );
}
