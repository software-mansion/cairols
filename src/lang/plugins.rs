use cairo_lang_semantic::plugin::AnalyzerPlugin;

pub trait AnalyzerPluginType: AnalyzerPlugin {
    fn is_cairo_lint_plugin(&self) -> bool {
        let name = format!("{self:?}");
        // Workaround for case when this was called on [`cairo_lang_semantic::ids::AnalyzerPluginLongId`]
        let name = name.strip_prefix("AnalyzerPluginLongId(").unwrap_or(&name);

        name.starts_with("CairoLint ")
    }
}

impl<T: AnalyzerPlugin + ?Sized> AnalyzerPluginType for T {}
