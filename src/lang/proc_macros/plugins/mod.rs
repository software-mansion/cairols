use std::collections::HashSet;
use std::sync::Arc;

use cairo_lang_defs::plugin::{InlineMacroExprPlugin, MacroPlugin};
use cairo_lang_filesystem::ids::SmolStrId;
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{MaybeImplBody, MaybeTraitBody, ModuleItem};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use convert_case::{Case, Casing};
use downcast::unsafe_downcast_ref;
use itertools::Itertools;
use salsa::Database;
use scarb::inline::inline_macro_generate_code;
use scarb::regular::macro_generate_code;
use scarb_proc_macro_server_types::methods::defined_macros::{
    CompilationUnitComponentMacros, DebugInfo, DefinedMacrosResponse,
};
use scarb_proc_macro_server_types::scope::{CompilationUnitComponent, ProcMacroScope};

use crate::lang::plugins::DowncastRefUnchecked;

mod downcast;
// TODO(#6666) Evict this module when this is possible.
mod scarb;

/// Creates a mapping between serialized Scarb PackageIds and [`PluginSuite`]s of macros
/// supported by the proc-macro-server, used by those packages.
pub fn proc_macro_plugin_suites(
    defined_macros: DefinedMacrosResponse,
) -> OrderedHashMap<CompilationUnitComponent, PluginSuite> {
    defined_macros
        .macros_for_cu_components
        .into_iter()
        .map(
            |CompilationUnitComponentMacros {
                 component,
                 attributes,
                 inline_macros,
                 derives,
                 executables,
                 debug_info: DebugInfo { source_packages },
             }| {
                let mut plugin_suite = PluginSuite::default();

                let plugin_scope = ProcMacroScope { component: component.clone() };

                plugin_suite.add_plugin_ex(Arc::new(ProcMacroPlugin {
                    scope: plugin_scope.clone(),
                    source_packages: source_packages.clone(),
                    defined_attributes: attributes,
                    defined_derives: derives,
                    defined_executable_attributes: executables,
                }));

                let inline_plugin =
                    Arc::new(InlineProcMacroPlugin { scope: plugin_scope, source_packages });

                for inline_macro in inline_macros {
                    plugin_suite.add_inline_macro_plugin_ex(&inline_macro, inline_plugin.clone());
                }

                (component, plugin_suite)
            },
        )
        .collect()
}

/// Macro plugin that searches for proc macros and forwards their resolution to the
/// proc-macro-server.
#[derive(Debug)]
pub struct ProcMacroPlugin {
    scope: ProcMacroScope,
    source_packages: Vec<String>,
    defined_attributes: Vec<String>,
    defined_derives: Vec<String>,
    defined_executable_attributes: Vec<String>,
}

impl ProcMacroPlugin {
    pub fn source_packages(&self) -> &[String] {
        &self.source_packages
    }
}

impl<'t> DowncastRefUnchecked<'t> for ProcMacroPlugin {
    type From = &'t dyn MacroPlugin;

    unsafe fn downcast_ref_unchecked(value: Self::From) -> &'t Self {
        unsafe { &*(value as *const dyn MacroPlugin as *const Self) }
    }
}

impl MacroPlugin for ProcMacroPlugin {
    #[tracing::instrument(level = "trace", skip_all)]
    fn generate_code<'db>(
        &self,
        db: &'db dyn Database,
        item_ast: cairo_lang_syntax::node::ast::ModuleItem<'db>,
        metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
    ) -> cairo_lang_defs::plugin::PluginResult<'db> {
        // Check on inner attributes too.
        let inner_attrs: HashSet<_> = match &item_ast {
            ModuleItem::Impl(imp) => {
                if let MaybeImplBody::Some(body) = imp.body(db) {
                    body.items(db)
                        .elements(db)
                        .flat_map(|item| item.attributes_elements(db).collect_vec())
                        .map(|attr| attr.attr(db).as_syntax_node().get_text_without_trivia(db))
                        .collect()
                } else {
                    Default::default()
                }
            }
            ModuleItem::Trait(trt) => {
                if let MaybeTraitBody::Some(body) = trt.body(db) {
                    body.items(db)
                        .elements(db)
                        .flat_map(|item| item.attributes_elements(db).collect_vec())
                        .map(|attr| attr.attr(db).as_syntax_node().get_text_without_trivia(db))
                        .collect()
                } else {
                    Default::default()
                }
            }
            _ => Default::default(),
        };

        if !self.declared_attributes(db).into_iter().any(|declared_attr| {
            let name = declared_attr.to_string(db);
            item_ast.has_attr(db, &name) || inner_attrs.contains(&declared_attr)
        })
            // Plugins can implement own derives.
            && !item_ast.has_attr(db, "derive")
            // Plugins does not declare module inline macros they support.
            && !matches!(item_ast, ModuleItem::InlineMacro(_))
        {
            return Default::default();
        };

        // Safety: We use this plugin only in AnalysisDatabase.
        let analysis_db = unsafe { unsafe_downcast_ref(db) };

        macro_generate_code(
            analysis_db,
            self.scope.clone(),
            item_ast,
            &self.defined_attributes,
            &self.defined_derives,
            metadata,
        )
    }

    fn declared_attributes<'db>(&self, db: &'db dyn Database) -> Vec<SmolStrId<'db>> {
        [&self.defined_attributes[..], &self.defined_executable_attributes[..]]
            .concat()
            .into_iter()
            .map(|s| SmolStrId::from(db, s))
            .collect()
    }

    fn declared_derives<'db>(&self, db: &'db dyn Database) -> Vec<SmolStrId<'db>> {
        self.defined_derives
            .iter()
            .map(|derive| derive.to_case(Case::Pascal))
            .map(|s| SmolStrId::from(db, s))
            .collect()
    }
}

/// Inline macro plugin that forwards resolution to the proc-macro-server.
#[derive(Debug)]
pub struct InlineProcMacroPlugin {
    scope: ProcMacroScope,
    source_packages: Vec<String>,
}

impl InlineProcMacroPlugin {
    pub fn source_packages(&self) -> &[String] {
        &self.source_packages
    }
}

impl<'t> DowncastRefUnchecked<'t> for InlineProcMacroPlugin {
    type From = &'t dyn InlineMacroExprPlugin;

    unsafe fn downcast_ref_unchecked(value: Self::From) -> &'t Self {
        unsafe { &*(value as *const dyn InlineMacroExprPlugin as *const Self) }
    }
}

impl InlineMacroExprPlugin for InlineProcMacroPlugin {
    #[tracing::instrument(level = "trace", skip_all)]
    fn generate_code<'db>(
        &self,
        db: &'db dyn Database,
        item_ast: &cairo_lang_syntax::node::ast::ExprInlineMacro<'db>,
        _metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
    ) -> cairo_lang_defs::plugin::InlinePluginResult<'db> {
        // Safety: We use this plugin only in AnalysisDatabase.
        let analysis_db = unsafe { unsafe_downcast_ref(db) };

        inline_macro_generate_code(analysis_db, self.scope.clone(), item_ast)
    }
}
