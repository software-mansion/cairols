use std::sync::Arc;

use cairo_lang_semantic::plugin::PluginSuite;
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use scarb_proc_macro_server_types::methods::defined_macros::DefinedMacrosResponse;
use scarb_proc_macro_server_types::scope::{CompilationUnitComponent, ProcMacroScope, Workspace};

mod backend;

pub use backend::PmsBackend;

/// Macro plugin that searches for proc macros and forwards their resolution to the
/// proc-macro-server.
pub type ProcMacroPlugin = scarb_proc_macro_host::ProcMacroHostPlugin<PmsBackend>;

/// Inline macro plugin that forwards resolution to the proc-macro-server.
pub type InlineProcMacroPlugin = scarb_proc_macro_host::ProcMacroInlinePlugin<PmsBackend>;

/// Creates a mapping between serialized Scarb PackageIds and [`PluginSuite`]s of macros
/// supported by the proc-macro-server, used by those packages.
pub fn proc_macro_plugin_suites(
    defined_macros: DefinedMacrosResponse,
    workspace: Workspace,
) -> OrderedHashMap<CompilationUnitComponent, PluginSuite> {
    defined_macros
        .macros_for_cu_components
        .into_iter()
        .map(|component_macros| {
            let component = component_macros.component.clone();
            let scope =
                ProcMacroScope { component: component.clone(), workspace: workspace.clone() };
            let backend = Arc::new(PmsBackend::new(scope, component_macros));
            let plugin_suite =
                ProcMacroPlugin::build_plugin_suite(Arc::new(ProcMacroPlugin::new(backend)));

            (component, plugin_suite)
        })
        .collect()
}
