use cairo_lang_defs::plugin::{
    InlineMacroExprPlugin, MacroPlugin, MacroPluginMetadata, PluginResult,
};
use cairo_lang_executable::plugin::executable_plugin_suite;
use cairo_lang_semantic::plugin::{AnalyzerPlugin, PluginSuite};
use cairo_lang_starknet::starknet_plugin_suite;
use cairo_lang_syntax::node::ast::ModuleItem;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_test_plugin::{test_assert_suite, test_plugin_suite};
use scarb_metadata::{CompilationUnitCairoPluginMetadata, Metadata};

/// Representation of known built-in plugins available in the Cairo compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinPlugin {
    AssertMacros,
    Executable,
    CairoRun,
    CairoTest,
    Starknet,
}

impl BuiltinPlugin {
    /// Creates a new instance of `BuiltinPlugin` corresponding to the given
    /// [`CompilationUnitCairoPluginMetadata`].
    /// Returns `None` if `plugin_metadata` does not describe any known built-in plugin.
    pub fn from_plugin_metadata(
        metadata: &Metadata,
        plugin_metadata: &CompilationUnitCairoPluginMetadata,
    ) -> Option<Self> {
        // Discard those plugins which are not built-in
        // before checking their discriminators in the next step.
        if !metadata.is_builtin_plugin(plugin_metadata).unwrap_or_default() {
            return None;
        }

        // The package discriminator has form: "<name> <version> (<source>)".
        let package_id_repr = &plugin_metadata.package.repr;

        let package_metadata = metadata
            .packages
            .iter()
            .find(|package_metadata| &package_metadata.id.repr == package_id_repr)?;

        match package_metadata.name.as_str() {
            "assert_macros" => Some(Self::AssertMacros),
            "cairo_execute" => Some(Self::Executable),
            "cairo_run" => Some(Self::CairoRun),
            "cairo_test" => Some(Self::CairoTest),
            "starknet" => Some(Self::Starknet),
            _ => None,
        }
    }

    /// Creates a [`PluginSuite`] corresponding to the represented plugin.
    pub fn suite(&self) -> PluginSuite {
        match self {
            BuiltinPlugin::AssertMacros => test_assert_suite(),
            BuiltinPlugin::CairoTest => test_plugin_suite(),
            BuiltinPlugin::CairoRun => cairo_run_plugin_suite(),
            BuiltinPlugin::Executable => executable_plugin_suite(),
            BuiltinPlugin::Starknet => starknet_plugin_suite(),
        }
    }

    // TODO(#324): Improve those functions as soon as plugins implement `Eq`.
    pub fn try_from_compiler_macro_plugin(plugin: &dyn MacroPlugin) -> Option<Self> {
        match format!("{plugin:?}").as_str() {
            "TestPlugin" => Some(Self::CairoTest),
            "CairoRunPlugin" => Some(Self::CairoRun),
            "ExecutablePlugin" => Some(Self::Executable),
            "StarknetPlugin" | "StorageInterfacesPlugin" => Some(Self::Starknet),
            _ => None,
        }
    }

    pub fn try_from_compiler_inline_macro_plugin(
        plugin: &dyn InlineMacroExprPlugin,
    ) -> Option<Self> {
        match format!("{plugin:?}").as_str() {
            "AssertEqMacro" | "AssertNeMacro" | "AssertLtMacro" | "AssertLeMacro"
            | "AssertGtMacro" | "AssertGeMacro" => Some(Self::AssertMacros),
            "StorageInterfacesPlugin"
            | "SelectorMacro"
            | "GetDepComponentMacro"
            | "GetDepComponentMutMacro" => Some(Self::Starknet),
            _ => None,
        }
    }

    pub fn try_from_compiler_analyzer_plugin(plugin: &dyn AnalyzerPlugin) -> Option<Self> {
        match format!("{plugin:?}").as_str() {
            "RawExecutableAnalyzer" => Some(Self::Executable),
            "ABIAnalyzer" | "StorageAnalyzer" => Some(Self::Starknet),
            _ => None,
        }
    }
}

fn cairo_run_plugin_suite() -> PluginSuite {
    let mut suite = PluginSuite::default();
    suite.add_plugin::<CairoRunPlugin>();
    suite
}

const CAIRO_RUN_EXECUTABLE: &str = "main";

// The implementation is copied from Scarb to allow analyzing packages that use `cairo-run`.

/// A plugin that defines an executable attribute for cairo-run.
/// No code generation is performed.
#[derive(Debug, Default)]
struct CairoRunPlugin {}

impl MacroPlugin for CairoRunPlugin {
    fn generate_code(
        &self,
        _db: &dyn SyntaxGroup,
        _item_ast: ModuleItem,
        _metadata: &MacroPluginMetadata<'_>,
    ) -> PluginResult {
        PluginResult::default()
    }

    fn declared_attributes(&self) -> Vec<String> {
        vec![CAIRO_RUN_EXECUTABLE.to_string()]
    }

    fn executable_attributes(&self) -> Vec<String> {
        self.declared_attributes()
    }
}
