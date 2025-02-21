use std::sync::Arc;

use cairo_lang_defs::plugin::{InlineMacroExprPlugin, MacroPlugin};
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use convert_case::{Case, Casing};
use downcast::unsafe_downcast_ref;
use scarb::inline::inline_macro_generate_code;
use scarb::regular::macro_generate_code;
use scarb_proc_macro_server_types::methods::defined_macros::{
    DefinedMacrosResponse, PackageDefinedMacrosInfo,
};
use scarb_proc_macro_server_types::scope::ProcMacroScope;
use smol_str::SmolStr;

mod downcast;
// TODO(#6666) Evict this module when this is possible.
mod scarb;

/// Creates a mapping between serialized Scarb PackageIds and [`PluginSuite`]s of macros
/// supported by the proc-macro-server, used by those packages.
pub fn proc_macro_plugin_suites(
    defined_macros: DefinedMacrosResponse,
) -> OrderedHashMap<SmolStr, PluginSuite> {
    defined_macros
        .macros_by_package_id
        .into_iter()
        .map(|(package_id, component_plugins)| {
            let PackageDefinedMacrosInfo { attributes, inline_macros, derives, executables } =
                component_plugins;
            let mut plugin_suite = PluginSuite::default();

            let plugin_scope = ProcMacroScope { package_id: package_id.clone() };

            plugin_suite.add_plugin_ex(Arc::new(ProcMacroPlugin {
                scope: plugin_scope.clone(),
                defined_attributes: attributes,
                defined_derives: derives,
                defined_executable_attributes: executables,
            }));

            let inline_plugin = Arc::new(InlineProcMacroPlugin { scope: plugin_scope });

            for inline_macro in inline_macros {
                plugin_suite.add_inline_macro_plugin_ex(&inline_macro, inline_plugin.clone());
            }

            (package_id.into(), plugin_suite)
        })
        .collect()
}

/// Macro plugin that searches for proc macros and forwards their resolution to the
/// proc-macro-server.
#[derive(Debug)]
struct ProcMacroPlugin {
    scope: ProcMacroScope,
    defined_attributes: Vec<String>,
    defined_derives: Vec<String>,
    defined_executable_attributes: Vec<String>,
}

impl MacroPlugin for ProcMacroPlugin {
    #[tracing::instrument(level = "trace", skip_all)]
    fn generate_code(
        &self,
        db: &dyn cairo_lang_syntax::node::db::SyntaxGroup,
        item_ast: cairo_lang_syntax::node::ast::ModuleItem,
        _metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
    ) -> cairo_lang_defs::plugin::PluginResult {
        // Safety: We use this plugin only in AnalysisDatabase.
        let analysis_db = unsafe { unsafe_downcast_ref(db) };

        macro_generate_code(
            analysis_db,
            self.scope.clone(),
            item_ast,
            &self.defined_attributes,
            &self.defined_derives,
        )
    }

    fn declared_attributes(&self) -> Vec<String> {
        [&self.defined_attributes[..], &self.defined_executable_attributes[..]].concat()
    }

    fn declared_derives(&self) -> Vec<String> {
        self.defined_derives.iter().map(|derive| derive.to_case(Case::Pascal)).collect()
    }
}

/// Inline macro plugin that forwards resolution to the proc-macro-server.
#[derive(Debug)]
struct InlineProcMacroPlugin {
    scope: ProcMacroScope,
}

impl InlineMacroExprPlugin for InlineProcMacroPlugin {
    #[tracing::instrument(level = "trace", skip_all)]
    fn generate_code(
        &self,
        db: &dyn cairo_lang_syntax::node::db::SyntaxGroup,
        item_ast: &cairo_lang_syntax::node::ast::ExprInlineMacro,
        _metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
    ) -> cairo_lang_defs::plugin::InlinePluginResult {
        // Safety: We use this plugin only in AnalysisDatabase.
        let analysis_db = unsafe { unsafe_downcast_ref(db) };

        inline_macro_generate_code(analysis_db, self.scope.clone(), item_ast)
    }
}
