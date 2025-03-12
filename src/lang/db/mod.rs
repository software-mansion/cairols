use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::ThreadId;

use cairo_lang_defs::db::{
    DefsDatabase, DefsGroup, DefsGroupEx, init_defs_group, try_ext_as_virtual_impl,
};
use cairo_lang_defs::ids::{InlineMacroExprPluginId, MacroPluginId};
use cairo_lang_doc::db::DocDatabase;
use cairo_lang_executable::plugin::executable_plugin_suite;
use cairo_lang_filesystem::cfg::{Cfg, CfgSet};
use cairo_lang_filesystem::db::{
    AsFilesGroupMut, ExternalFiles, FilesDatabase, FilesGroup, init_files_group,
};
use cairo_lang_filesystem::ids::{CrateId, VirtualFile};
use cairo_lang_lowering::db::{LoweringDatabase, LoweringGroup, init_lowering_group};
use cairo_lang_lowering::utils::InliningStrategy;
use cairo_lang_parser::db::{ParserDatabase, ParserGroup};
use cairo_lang_semantic::db::{
    PluginSuiteInput, SemanticDatabase, SemanticGroup, SemanticGroupEx, init_semantic_group,
};
use cairo_lang_semantic::ids::AnalyzerPluginId;
use cairo_lang_semantic::inline_macros::get_default_plugin_suite;
use cairo_lang_semantic::plugin::{InternedPluginSuite, PluginSuite};
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_syntax::node::db::{SyntaxDatabase, SyntaxGroup};
use cairo_lang_test_plugin::test_plugin_suite;
use cairo_lang_utils::Upcast;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use cairo_lint_core::plugin::cairo_lint_allow_plugin_suite;
use dashmap::DashMap;
use events::SalsaEvent;
use itertools::Itertools;
use salsa::debug::DebugQueryTable;
use salsa::{Database, Durability};
use tracing::trace;

pub use self::semantic::*;
pub use self::swapper::*;
pub use self::syntax::*;
use super::proc_macros::db::{ProcMacroDatabase, init_proc_macro_group};
use crate::TRICKS;

#[allow(dead_code, unused_imports)]
mod events;
#[allow(dead_code, unused_imports)]
mod lru;
mod semantic;
mod swapper;
mod syntax;

/// The Cairo compiler Salsa database tailored for language server usage.
#[salsa::database(
    DefsDatabase,
    FilesDatabase,
    LoweringDatabase,
    ParserDatabase,
    SemanticDatabase,
    SyntaxDatabase,
    DocDatabase,
    ProcMacroDatabase
)]
pub struct AnalysisDatabase {
    storage: salsa::Storage<Self>,
    events: DashMap<ThreadId, Vec<SalsaEvent>>,
}

impl AnalysisDatabase {
    /// Creates a new instance of the database.
    pub fn new() -> Self {
        let mut db = Self { storage: Default::default(), events: Default::default() };
        // let group_storage = salsa::plumbing::HasQueryGroup::<DefsDatabase>::group_storage(&db);

        // db.storage.query_store_mut().0.syntax_database.for_each_query(runtime, op);

        init_files_group(&mut db);
        init_defs_group(&mut db);
        init_semantic_group(&mut db);
        init_lowering_group(&mut db, InliningStrategy::Default);
        // proc-macro-server can be restarted many times but we want to keep these data across
        // multiple server starts, so init it once per database, not per server.
        init_proc_macro_group(&mut db);

        db.set_cfg_set(Self::initial_cfg_set().into());

        // lru::set_lru_capacities(&mut db);

        // Those pluins are relevant for projects with `cairo_project.toml` (e.g. our tests).
        let default_plugin_suite = Self::default_global_plugin_suite();

        let default_plugin_suite = db.intern_plugin_suite(default_plugin_suite);
        db.set_default_plugins_from_suite(default_plugin_suite);

        db
    }

    fn dump_events(&self) {
        let path = PathBuf::from("./salsa-events.csv");

        let mut writer = if std::fs::exists(&path).unwrap_or_default() {
            let file = std::fs::File::options()
                .append(true)
                .open(&path)
                .expect("Failed to create events file.");
            csv::WriterBuilder::new().has_headers(false).from_writer(file)
        } else {
            let file = std::fs::File::create(&path).expect("Failed to create file");
            csv::Writer::from_writer(file)
        };

        let mut count = 0;

        for thread_events in self.events.iter() {
            for event in thread_events.iter() {
                writer.serialize(event).unwrap();
                count += 1;
            }
        }

        trace!("Dumped {count} events");

        self.events.clear();
    }

    /// Returns the [`CfgSet`] that should be assumed in the initial database state
    /// and in [`CfgSet`] for workspace members.
    /// This enables code fragments tagged with `#[cfg(test)]` and `#[cfg(target: 'test')]` to be
    /// included in analysis by Language Server.
    pub(crate) fn initial_cfg_set() -> CfgSet {
        CfgSet::from_iter([Cfg::name("test"), Cfg::kv("target", "test")])
    }

    /// Returns the [`CfgSet`] that should be assumed for dependencies.
    /// This enables code fragments tagged with `#[cfg(target: 'test')]` to be
    /// included in analysis by Language Server.
    pub(crate) fn initial_cfg_set_for_deps() -> CfgSet {
        CfgSet::from_iter([Cfg::kv("target", "test")])
    }

    /// Trigger cancellation in any background tasks that might still be running.
    pub fn cancel_all(&mut self) {
        self.salsa_runtime_mut().synthetic_write(Durability::LOW);
    }

    /// Removes the plugins from [`InternedPluginSuite`] for a crate with [`CrateId`].
    pub fn remove_crate_plugin_suite(&mut self, crate_id: CrateId, plugins: &InternedPluginSuite) {
        self.with_plugins_mut(crate_id, |macro_plugins, analyzer_plugins, inline_macro_plugins| {
            remove_plugin_suite(plugins, macro_plugins, analyzer_plugins, inline_macro_plugins)
        })
    }

    /// Adds plugin suit to database.
    pub fn add_crate_plugin_suite(&mut self, crate_id: CrateId, plugins: InternedPluginSuite) {
        self.with_plugins_mut(
            crate_id,
            move |macro_plugins, analyzer_plugins, inline_macro_plugins| {
                add_plugin_suite(plugins, macro_plugins, analyzer_plugins, inline_macro_plugins)
            },
        )
    }

    fn with_plugins_mut(
        &mut self,
        crate_id: CrateId,
        action: impl FnOnce(
            &mut Vec<MacroPluginId>,
            &mut Vec<AnalyzerPluginId>,
            &mut OrderedHashMap<String, InlineMacroExprPluginId>,
        ),
    ) {
        let mut macro_plugins = self.crate_macro_plugins(crate_id).to_vec();
        let mut analyzer_plugins = self.crate_analyzer_plugins(crate_id).to_vec();
        let mut inline_macro_plugins =
            Arc::unwrap_or_clone(self.crate_inline_macro_plugins(crate_id));

        action(&mut macro_plugins, &mut analyzer_plugins, &mut inline_macro_plugins);

        self.set_override_crate_macro_plugins(crate_id, macro_plugins.into_iter().collect());
        self.set_override_crate_analyzer_plugins(crate_id, analyzer_plugins.into_iter().collect());
        self.set_override_crate_inline_macro_plugins(crate_id, Arc::new(inline_macro_plugins));
    }

    fn default_global_plugin_suite() -> PluginSuite {
        let tricks = TRICKS.get_or_init(Default::default);

        [
            get_default_plugin_suite(),
            starknet_plugin_suite(),
            test_plugin_suite(),
            executable_plugin_suite(),
            cairo_lint_allow_plugin_suite(),
        ]
        .into_iter()
        .chain(tricks.extra_plugin_suites.iter().flat_map(|f| f()))
        .fold(PluginSuite::default(), |mut acc, suite| {
            acc.add(suite);
            acc
        })
    }
}

fn remove_plugin_suite(
    plugins: &InternedPluginSuite,
    macro_plugins: &mut Vec<MacroPluginId>,
    analyzer_plugins: &mut Vec<AnalyzerPluginId>,
    inline_macro_plugins: &mut OrderedHashMap<String, InlineMacroExprPluginId>,
) {
    macro_plugins.retain(|plugin| !plugins.macro_plugins.contains(plugin));
    analyzer_plugins.retain(|plugin| !plugins.analyzer_plugins.contains(plugin));
    inline_macro_plugins
        .retain(|_, plugin| !plugins.inline_macro_plugins.values().contains(plugin));
}

fn add_plugin_suite(
    plugins: InternedPluginSuite,
    macro_plugins: &mut Vec<MacroPluginId>,
    analyzer_plugins: &mut Vec<AnalyzerPluginId>,
    inline_macro_plugins: &mut OrderedHashMap<String, InlineMacroExprPluginId>,
) {
    macro_plugins.extend_from_slice(&plugins.macro_plugins);
    analyzer_plugins.extend_from_slice(&plugins.analyzer_plugins);
    inline_macro_plugins.extend(Arc::unwrap_or_clone(plugins.inline_macro_plugins));
}

macro_rules! track {
    ($($location:path {$($query:ident),+ $(,)?}),* $(,)?) => {
        $(
            use $location::{$($query),*};
        )*

        fn query_table_sizes(db: &AnalysisDatabase) -> std::collections::HashMap<&'static str, usize> {
            let mut sizes = std::collections::HashMap::new();

            $($(
                sizes.insert(
                    stringify!($query),
                    std::mem::size_of_val(
                        &salsa::plumbing::get_query_table::<$query>(db).entries::<Vec<_>>()[..],
                    )
                );
            )*)*

            sizes
        }
    };
}

impl salsa::Database for AnalysisDatabase {
    fn salsa_event(&self, event: salsa::Event) {
        if matches!(event.kind, salsa::EventKind::WillCheckCancellation) {
            return;
        }

        const DEBOUNCE: usize = 1_000_000;

        thread_local! {
            static TRIGGER_COUNTER: RefCell<usize> = Default::default();
            static QUERY_TABLE_SIZES: RefCell<HashMap<&'static str, usize>> = Default::default();
        }

        let activate = TRIGGER_COUNTER.with_borrow_mut(|counter| {
            *counter += 1;
            *counter >= DEBOUNCE
        });

        if !activate {
            return;
        }

        TRIGGER_COUNTER.with_borrow_mut(|counter| *counter = 0);

        track!(
            cairo_lang_filesystem::db {
                CratesQuery,
                CrateConfigQuery,
                PrivRawFileContentQuery,
                FileContentQuery,
                FileSummaryQuery,
                BlobContentQuery,
                GetFlagQuery,
            },
            cairo_lang_parser::db {
                PrivFileSyntaxDataQuery,
                FileSyntaxQuery,
                FileModuleSyntaxQuery,
                FileExprSyntaxQuery,
                FileSyntaxDiagnosticsQuery,
            },
            cairo_lang_syntax::node::db { GetChildrenQuery },
            cairo_lang_defs::db {
                AllowedAttributesQuery,
                AllowedStatementAttributesQuery,
                CrateModulesQuery,
                DeclaredDerivesQuery,
                DeclaredPhantomTypeAttributesQuery,
                FileModulesQuery,
                IsSubmoduleInlineQuery,
                ModuleAncestorsQuery,
                ModuleConstantByIdQuery,
                ModuleConstantsIdsQuery,
                ModuleConstantsQuery,
                ModuleDirQuery,
                ModuleEnumByIdQuery,
                ModuleEnumsIdsQuery,
                ModuleEnumsQuery,
                ModuleExternFunctionByIdQuery,
                ModuleExternFunctionsIdsQuery,
                ModuleExternFunctionsQuery,
                ModuleExternTypeByIdQuery,
                ModuleExternTypesIdsQuery,
                ModuleExternTypesQuery,
                ModuleFileQuery,
                ModuleFilesQuery,
                ModuleFreeFunctionByIdQuery,
                ModuleFreeFunctionsIdsQuery,
                ModuleFreeFunctionsQuery,
                ModuleGeneratedFileAuxDataQuery,
                ModuleGlobalUseByIdQuery,
                ModuleGlobalUsesQuery,
                ModuleImplAliasByIdQuery,
                ModuleImplAliasesIdsQuery,
                ModuleImplAliasesQuery,
                ModuleImplByIdQuery,
                ModuleImplsIdsQuery,
                ModuleImplsQuery,
                ModuleItemNameStablePtrQuery,
                ModuleItemsQuery,
                ModuleMainFileQuery,
                ModulePluginDiagnosticsNotesQuery,
                ModulePluginDiagnosticsQuery,
                ModuleStructByIdQuery,
                ModuleStructsIdsQuery,
                ModuleStructsQuery,
                ModuleSubmodulesIdsQuery,
                ModuleSubmodulesQuery,
                ModuleTraitByIdQuery,
                ModuleTraitsIdsQuery,
                ModuleTraitsQuery,
                ModuleTypeAliasByIdQuery,
                ModuleTypeAliasesIdsQuery,
                ModuleTypeAliasesQuery,
                ModuleUseByIdQuery,
                ModuleUsesIdsQuery,
                ModuleUsesQuery,
                PrivFileToModuleMappingQuery,
                PrivModuleDataQuery,
                PrivModuleSubFilesQuery,
            },
            cairo_lang_semantic::db {
                PrivConstantSemanticDataQuery,
                ConstantSemanticDiagnosticsQuery,
                ConstantSemanticDataQuery,
                ConstantResolverDataQuery,
                ConstantConstValueQuery,
                ConstantConstTypeQuery,
                ConstCalcInfoQuery,
                PrivUseSemanticDataQuery,
                UseSemanticDiagnosticsQuery,
                UseResolverDataQuery,
                PrivGlobalUseSemanticDataQuery,
                PrivGlobalUseImportedModuleQuery,
                GlobalUseSemanticDiagnosticsQuery,
                PrivModuleUseStarModulesQuery,
                PrivModuleSemanticDataQuery,
                ModuleItemByNameQuery,
                ModuleItemInfoByNameQuery,
                ModuleAllUsedItemsQuery,
                ModuleAttributesQuery,
                ModuleUsableTraitIdsQuery,
                PrivStructDeclarationDataQuery,
                StructDeclarationDiagnosticsQuery,
                StructAttributesQuery,
                StructGenericParamsQuery,
                StructGenericParamsDataQuery,
                StructDeclarationResolverDataQuery,
                PrivStructDefinitionDataQuery,
                StructDefinitionDiagnosticsQuery,
                StructMembersQuery,
                StructDefinitionResolverDataQuery,
                ConcreteStructMembersQuery,
                PrivEnumDeclarationDataQuery,
                EnumDeclarationDiagnosticsQuery,
                EnumGenericParamsQuery,
                EnumGenericParamsDataQuery,
                EnumAttributesQuery,
                EnumDeclarationResolverDataQuery,
                PrivEnumDefinitionDataQuery,
                EnumDefinitionDiagnosticsQuery,
                EnumVariantsQuery,
                VariantSemanticQuery,
                EnumDefinitionResolverDataQuery,
                ModuleTypeAliasSemanticDiagnosticsQuery,
                ModuleTypeAliasResolvedTypeQuery,
                ModuleTypeAliasGenericParamsQuery,
                ModuleTypeAliasResolverDataQuery,
                PrivModuleTypeAliasGenericParamsDataQuery,
                PrivModuleTypeAliasSemanticDataQuery,
                ImplAliasImplDefQuery,
                PrivImplAliasSemanticDataQuery,
                ImplAliasSemanticDiagnosticsQuery,
                ImplAliasResolvedImplQuery,
                ImplAliasGenericParamsQuery,
                ImplAliasGenericParamsDataQuery,
                ImplAliasResolverDataQuery,
                ImplAliasAttributesQuery,
                TraitSemanticDeclarationDiagnosticsQuery,
                TraitGenericParamsQuery,
                TraitGenericParamsDataQuery,
                TraitAttributesQuery,
                TraitResolverDataQuery,
                PrivTraitDeclarationDataQuery,
                TraitSemanticDefinitionDiagnosticsQuery,
                TraitRequiredItemNamesQuery,
                TraitItemByNameQuery,
                TraitItemInfoByNameQuery,
                TraitAllUsedItemsQuery,
                TraitFunctionsQuery,
                TraitFunctionByNameQuery,
                TraitTypesQuery,
                TraitTypeByNameQuery,
                TraitConstantsQuery,
                TraitConstantByNameQuery,
                TraitImplsQuery,
                TraitImplByNameQuery,
                PrivTraitDefinitionDataQuery,
                TraitTypeDiagnosticsQuery,
                TraitTypeGenericParamsQuery,
                TraitTypeAttributesQuery,
                TraitTypeResolverDataQuery,
                PrivTraitTypeGenericParamsDataQuery,
                PrivTraitTypeDataQuery,
                TraitConstantDiagnosticsQuery,
                TraitConstantAttributesQuery,
                TraitConstantTypeQuery,
                TraitConstantResolverDataQuery,
                PrivTraitConstantDataQuery,
                ConcreteTraitConstantTypeQuery,
                TraitImplDiagnosticsQuery,
                TraitImplAttributesQuery,
                TraitImplConcreteTraitQuery,
                TraitImplResolverDataQuery,
                PrivTraitImplDataQuery,
                ConcreteTraitImplConcreteTraitQuery,
                TraitFunctionDeclarationDiagnosticsQuery,
                TraitFunctionSignatureQuery,
                TraitFunctionGenericParamsQuery,
                PrivTraitFunctionGenericParamsDataQuery,
                TraitFunctionAttributesQuery,
                TraitFunctionResolverDataQuery,
                TraitFunctionDeclarationInlineConfigQuery,
                TraitFunctionDeclarationImplicitPrecedenceQuery,
                TraitFunctionDeclarationImplicitsQuery,
                PrivTraitFunctionDeclarationDataQuery,
                TraitFunctionBodyDiagnosticsQuery,
                TraitFunctionBodyQuery,
                PrivTraitFunctionBodyDataQuery,
                ConcreteTraitFunctionGenericParamsQuery,
                ConcreteTraitFunctionSignatureQuery,
                ModuleImplIdsForTraitFilterQuery,
                ImplImplIdsForTraitFilterQuery,
                CanonicTraitSolutionsQuery,
                ImplSemanticDeclarationDiagnosticsQuery,
                ImplDefGenericParamsDataQuery,
                ImplDefGenericParamsQuery,
                ImplDefResolverDataQuery,
                ImplDefConcreteTraitQuery,
                ImplDefSubstitutionQuery,
                ImplDefAttributesQuery,
                ImplConcreteTraitQuery,
                ImplDefTraitQuery,
                PrivImplDeclarationDataQuery,
                ImplSemanticDefinitionDiagnosticsQuery,
                ImplItemByNameQuery,
                ImplItemInfoByNameQuery,
                ImplImplicitImplByNameQuery,
                ImplAllUsedItemsQuery,
                ImplTypesQuery,
                ImplTypeIdsQuery,
                ImplTypeByIdQuery,
                ImplTypeByTraitTypeQuery,
                ImplConstantsQuery,
                ImplImplsQuery,
                ImplImplIdsQuery,
                ImplImplByIdQuery,
                ImplImplByTraitImplQuery,
                IsImplicitImplImplQuery,
                ImplConstantByTraitConstantQuery,
                ImplFunctionsQuery,
                ImplFunctionByTraitFunctionQuery,
                PrivImplDefinitionDataQuery,
                PrivImplIsFullyConcreteQuery,
                PrivImplIsVarFreeQuery,
                ImplTypeDefSemanticDiagnosticsQuery,
                ImplTypeDefResolvedTypeQuery,
                ImplTypeDefGenericParamsQuery,
                ImplTypeDefAttributesQuery,
                ImplTypeDefResolverDataQuery,
                ImplTypeDefTraitTypeQuery,
                PrivImplTypeSemanticDataQuery,
                PrivImplTypeDefGenericParamsDataQuery,
                DerefChainQuery,
                ImplTypeConcreteImplizedQuery,
                ImplConstantDefSemanticDiagnosticsQuery,
                ImplConstantDefValueQuery,
                ImplConstantDefResolverDataQuery,
                ImplConstantDefTraitConstantQuery,
                PrivImplConstantSemanticDataQuery,
                ImplConstantImplizedByContextQuery,
                ImplConstantConcreteImplizedValueQuery,
                ImplConstantConcreteImplizedTypeQuery,
                ImplImplDefSemanticDiagnosticsQuery,
                ImplImplDefResolverDataQuery,
                ImplImplDefTraitImplQuery,
                ImplImplDefImplQuery,
                PrivImplImplSemanticDataQuery,
                PrivImplImplDefGenericParamsDataQuery,
                ImplicitImplImplSemanticDiagnosticsQuery,
                ImplicitImplImplImplQuery,
                PrivImplicitImplImplSemanticDataQuery,
                ImplImplImplizedByContextQuery,
                ImplImplConcreteImplizedQuery,
                ImplImplConcreteTraitQuery,
                ImplFunctionDeclarationDiagnosticsQuery,
                ImplFunctionSignatureQuery,
                ImplFunctionGenericParamsQuery,
                PrivImplFunctionGenericParamsDataQuery,
                ImplFunctionAttributesQuery,
                ImplFunctionResolverDataQuery,
                ImplFunctionDeclarationInlineConfigQuery,
                ImplFunctionDeclarationImplicitPrecedenceQuery,
                ImplFunctionDeclarationImplicitsQuery,
                ImplFunctionTraitFunctionQuery,
                PrivImplFunctionDeclarationDataQuery,
                ImplFunctionBodyDiagnosticsQuery,
                ImplFunctionBodyQuery,
                ImplFunctionBodyResolverDataQuery,
                PrivImplFunctionBodyDataQuery,
                TraitTypeImplizedByContextQuery,
                FreeFunctionDeclarationDiagnosticsQuery,
                FreeFunctionSignatureQuery,
                FreeFunctionDeclarationImplicitsQuery,
                FreeFunctionDeclarationImplicitPrecedenceQuery,
                FreeFunctionGenericParamsQuery,
                FreeFunctionGenericParamsDataQuery,
                FreeFunctionDeclarationResolverDataQuery,
                FreeFunctionDeclarationInlineConfigQuery,
                PrivFreeFunctionDeclarationDataQuery,
                FreeFunctionBodyDiagnosticsQuery,
                FreeFunctionBodyResolverDataQuery,
                PrivFreeFunctionBodyDataQuery,
                FunctionDeclarationDiagnosticsQuery,
                FunctionDeclarationInlineConfigQuery,
                FunctionDeclarationImplicitPrecedenceQuery,
                FunctionWithBodySignatureQuery,
                FunctionWithBodyGenericParamsQuery,
                FunctionWithBodyAttributesQuery,
                FunctionBodyDiagnosticsQuery,
                FunctionBodyExprQuery,
                FunctionBodyQuery,
                PrivExternFunctionDeclarationDataQuery,
                ExternFunctionDeclarationInlineConfigQuery,
                ExternFunctionDeclarationDiagnosticsQuery,
                ExternFunctionSignatureQuery,
                ExternFunctionDeclarationGenericParamsQuery,
                ExternFunctionDeclarationGenericParamsDataQuery,
                ExternFunctionDeclarationImplicitsQuery,
                ExternFunctionDeclarationRefsQuery,
                ExternFunctionDeclarationResolverDataQuery,
                PrivExternTypeDeclarationDataQuery,
                ExternTypeDeclarationDiagnosticsQuery,
                ExternTypeDeclarationGenericParamsQuery,
                ExternTypeDeclarationGenericParamsDataQuery,
                ExternTypeAttributesQuery,
                FunctionTitleSignatureQuery,
                FunctionTitleGenericParamsQuery,
                ConcreteFunctionSignatureQuery,
                ConcreteFunctionClosureParamsQuery,
                GetClosureParamsQuery,
                GenericTypeGenericParamsQuery,
                GenericParamSemanticQuery,
                GenericParamDiagnosticsQuery,
                GenericParamResolverDataQuery,
                GenericImplParamTraitQuery,
                PrivGenericParamDataQuery,
                GenericParamsTypeConstraintsQuery,
                SingleValueTypeQuery,
                TypeSizeInfoQuery,
                TypeInfoQuery,
                PrivTypeIsFullyConcreteQuery,
                PrivTypeIsVarFreeQuery,
                PrivTypeShortNameQuery,
                ExprSemanticQuery,
                PatternSemanticQuery,
                StatementSemanticQuery,
                LookupResolvedGenericItemByPtrQuery,
                LookupResolvedConcreteItemByPtrQuery,
                ModuleSemanticDiagnosticsQuery,
                FileSemanticDiagnosticsQuery,
                CoreCrateQuery,
                CoreModuleQuery,
                CoreInfoQuery,
                CrateAnalyzerPluginsQuery,
                DeclaredAllowsQuery,
                MethodsInModuleQuery,
                MethodsInCrateQuery,
                VisibleImportablesFromModuleQuery,
                VisibleImportablesInModuleQuery,
                VisibleImportablesInCrateQuery,
                VisibleTraitsFromModuleQuery
            },
            cairo_lang_doc::db {
                GetItemDocumentationQuery,
                GetItemDocumentationAsTokensQuery,
                GetItemSignatureQuery,
                GetItemSignatureWithLinksQuery
            },
            cairo_lang_lowering::db {
                PrivFunctionWithBodyMultiLoweringQuery,
                CachedMultiLoweringsQuery,
                PrivFunctionWithBodyLoweringQuery,
                FunctionWithBodyLoweringWithBorrowCheckQuery,
                FunctionWithBodyLoweringQuery,
                PrivConcreteFunctionWithBodyLoweredFlatQuery,
                ConcreteFunctionWithBodyPostpanicLoweredQuery,
                OptimizedConcreteFunctionWithBodyLoweredQuery,
                InlinedFunctionWithBodyLoweredQuery,
                FinalConcreteFunctionWithBodyLoweredQuery,
                ConcreteFunctionWithBodyDirectCalleesQuery,
                ConcreteFunctionWithBodyInlinedDirectCalleesQuery
            },
            crate::lang::proc_macros::db {
                GetAttributeExpansionQuery,
                GetDeriveExpansionQuery,
                GetInlineMacrosExpansionQuery
            }
        );

        let sizes = query_table_sizes(self);

        trace!("query_table_sizes = {sizes:#?}");
    }
}

impl salsa::ParallelDatabase for AnalysisDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(AnalysisDatabase {
            storage: self.storage.snapshot(),
            events: Default::default(),
        })
    }
}

impl ExternalFiles for AnalysisDatabase {
    fn try_ext_as_virtual(&self, external_id: salsa::InternId) -> Option<VirtualFile> {
        try_ext_as_virtual_impl(self.upcast(), external_id)
    }
}

impl AsFilesGroupMut for AnalysisDatabase {
    fn as_files_group_mut(&mut self) -> &mut (dyn FilesGroup + 'static) {
        self
    }
}

impl Upcast<dyn FilesGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn FilesGroup + 'static) {
        self
    }
}

impl Upcast<dyn SyntaxGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn SyntaxGroup + 'static) {
        self
    }
}

impl Upcast<dyn DefsGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn DefsGroup + 'static) {
        self
    }
}

impl Upcast<dyn SemanticGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn SemanticGroup + 'static) {
        self
    }
}

impl Upcast<dyn LoweringGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn LoweringGroup + 'static) {
        self
    }
}

impl Upcast<dyn ParserGroup> for AnalysisDatabase {
    fn upcast(&self) -> &(dyn ParserGroup + 'static) {
        self
    }
}
