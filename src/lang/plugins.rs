use std::any::TypeId;

use cairo_lang_semantic::plugin::AnalyzerPlugin;
use cairo_lint_core::plugin::CairoLint;

pub trait AnalyzerPluginType: AnalyzerPlugin {
    fn is_cairo_lint_plugin(&self) -> bool {
        self.plugin_type_id() == TypeId::of::<CairoLint>()
    }
}

impl<T: AnalyzerPlugin + ?Sized> AnalyzerPluginType for T {}
