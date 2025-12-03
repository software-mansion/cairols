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
use itertools::Itertools;
use salsa::Database;
use scarb::inline::inline_macro_generate_code;
use scarb::regular::macro_generate_code;
use scarb_proc_macro_server_types::methods::defined_macros::{
    CompilationUnitComponentMacros, DebugInfo, DefinedMacrosResponse, MacroWithHash,
};
use scarb_proc_macro_server_types::scope::{CompilationUnitComponent, ProcMacroScope, Workspace};

// TODO(#6666) Evict this module when this is possible.
mod scarb;

/// Creates a mapping between serialized Scarb PackageIds and [`PluginSuite`]s of macros
/// supported by the proc-macro-server, used by those packages.
pub fn proc_macro_plugin_suites(
    defined_macros: DefinedMacrosResponse,
    workspace: Workspace,
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

                let plugin_scope =
                    ProcMacroScope { component: component.clone(), workspace: workspace.clone() };

                plugin_suite.add_plugin_ex(Arc::new(ProcMacroPlugin {
                    scope: plugin_scope.clone(),
                    source_packages: source_packages.clone(),
                    defined_attributes: attributes,
                    defined_derives: derives,
                    defined_inlines: inline_macros.clone(),
                    defined_executable_attributes: executables,
                }));

                // Reuse same plugin if possible to reduce memory usage.
                let grouped_inline_macros_by_hash =
                    inline_macros.into_iter().into_group_map_by(|m| m.hash);

                for (fingerprint, inline_macros) in grouped_inline_macros_by_hash {
                    let inline_plugin = Arc::new(InlineProcMacroPlugin {
                        scope: plugin_scope.clone(),
                        source_packages: source_packages.clone(),
                        fingerprint,
                    });

                    for inline_macro in inline_macros {
                        plugin_suite
                            .add_inline_macro_plugin_ex(&inline_macro.name, inline_plugin.clone());
                    }
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
    defined_attributes: Vec<MacroWithHash>,
    defined_derives: Vec<MacroWithHash>,
    defined_inlines: Vec<MacroWithHash>,
    defined_executable_attributes: Vec<String>,
}

impl ProcMacroPlugin {
    pub fn source_packages(&self) -> &[String] {
        &self.source_packages
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

        macro_generate_code(
            db,
            self.scope.clone(),
            item_ast,
            &self.defined_attributes,
            &self.defined_derives,
            &self.defined_inlines,
            metadata,
        )
    }

    fn declared_attributes<'db>(&self, db: &'db dyn Database) -> Vec<SmolStrId<'db>> {
        self.defined_attributes
            .iter()
            .map(|m| &m.name)
            .chain(self.defined_executable_attributes.iter())
            .map(|s| SmolStrId::from(db, s))
            .collect()
    }

    fn declared_derives<'db>(&self, db: &'db dyn Database) -> Vec<SmolStrId<'db>> {
        self.defined_derives
            .iter()
            .map(|derive| derive.name.to_case(Case::Pascal))
            .map(|s| SmolStrId::from(db, s))
            .collect()
    }
}

/// Inline macro plugin that forwards resolution to the proc-macro-server.
#[derive(Debug)]
pub struct InlineProcMacroPlugin {
    scope: ProcMacroScope,
    source_packages: Vec<String>,
    fingerprint: u64,
}

impl InlineProcMacroPlugin {
    pub fn source_packages(&self) -> &[String] {
        &self.source_packages
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
        inline_macro_generate_code(db, self.scope.clone(), item_ast, self.fingerprint)
    }
}
