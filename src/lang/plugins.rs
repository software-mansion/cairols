use cairo_lang_semantic::plugin::AnalyzerPlugin;

pub trait AnalyzerPluginType: AnalyzerPlugin {
    fn is_cairo_lint_plugin(&self) -> bool {
        format!("{self:?}").starts_with("CairoLint ")
    }
}

impl<T: AnalyzerPlugin + ?Sized> AnalyzerPluginType for T {}
