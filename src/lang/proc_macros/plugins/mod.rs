use std::sync::Arc;

use cairo_lang_defs::plugin::{InlineMacroExprPlugin, MacroPlugin};
use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use downcast::unsafe_downcast_ref;
use scarb::inline::inline_macro_generate_code;
use scarb::regular::macro_generate_code;
use scarb_proc_macro_server_types::methods::defined_macros::DefinedMacrosResponse;
use smol_str::SmolStr;

mod downcast;
// TODO(#6666) Evict this module when this is possible.
mod scarb;

/// Creates a mapping between Scarb package names and [`PluginSuite`]s of macros
/// supported by the proc-macro-server, used by those packages.
pub fn proc_macro_plugin_suite(
    defined_macros: DefinedMacrosResponse,
) -> OrderedHashMap<SmolStr, PluginSuite> {
    defined_macros
        .crate_macro_info
        .into_iter()
        .map(|(package_name, package_macros)| {
            let mut plugin_suite = PluginSuite::default();

            plugin_suite.add_plugin_ex(Arc::new(ProcMacroPlugin {
                defined_attributes: package_macros.attributes,
                defined_derives: package_macros.derives,
                defined_executable_attributes: package_macros.executables,
            }));

            let inline_plugin = Arc::new(InlineProcMacroPlugin);

            for inline_macro in package_macros.inline_macros {
                plugin_suite.add_inline_macro_plugin_ex(&inline_macro, inline_plugin.clone());
            }

            (package_name, PluginSuite::from(plugin_suite))
        })
        .collect()
}

/// Macro plugin that searches for proc macros and forwards their resolution to the
/// proc-macro-server.
#[derive(Debug)]
struct ProcMacroPlugin {
    defined_attributes: Vec<String>,
    defined_derives: Vec<String>,
    defined_executable_attributes: Vec<String>,
}

impl MacroPlugin for ProcMacroPlugin {
    fn generate_code(
        &self,
        db: &dyn cairo_lang_syntax::node::db::SyntaxGroup,
        item_ast: cairo_lang_syntax::node::ast::ModuleItem,
        _metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
    ) -> cairo_lang_defs::plugin::PluginResult {
        // Safety: We use this plugin only in AnalysisDatabase.
        let analysis_db = unsafe { unsafe_downcast_ref(db) };

        macro_generate_code(analysis_db, item_ast, &self.defined_attributes, &self.defined_derives)
    }

    fn declared_attributes(&self) -> Vec<String> {
        [&self.defined_attributes[..], &self.defined_executable_attributes[..]].concat()
    }

    fn declared_derives(&self) -> Vec<String> {
        self.defined_derives.clone()
    }
}

/// Inline macro plugin that forwards resolution to the proc-macro-server.
#[derive(Debug)]
struct InlineProcMacroPlugin;

impl InlineMacroExprPlugin for InlineProcMacroPlugin {
    fn generate_code(
        &self,
        db: &dyn cairo_lang_syntax::node::db::SyntaxGroup,
        item_ast: &cairo_lang_syntax::node::ast::ExprInlineMacro,
        _metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
    ) -> cairo_lang_defs::plugin::InlinePluginResult {
        // Safety: We use this plugin only in AnalysisDatabase.
        let analysis_db = unsafe { unsafe_downcast_ref(db) };

        inline_macro_generate_code(analysis_db, item_ast)
    }
}
