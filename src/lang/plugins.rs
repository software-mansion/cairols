use std::any::TypeId;

use cairo_lang_semantic::plugin::AnalyzerPlugin;
use cairo_lint::plugin::CairoLint;

pub trait AnalyzerPluginType: AnalyzerPlugin {
    fn is_cairo_lint_plugin(&self) -> bool {
        self.plugin_type_id() == TypeId::of::<CairoLint>()
    }
}

impl<T: AnalyzerPlugin + ?Sized> AnalyzerPluginType for T {}

/// A helper trait to be used to downcast arbitrary trait objects.
pub trait DowncastRefUnchecked<'t> {
    type From;
    /// Converts the value of type [`Self::From`] to a reference to [`Self`]
    /// without verifying if the downcast is possible.
    ///
    /// # Safety
    /// The caller must ensure that the value is indeed an instance of [`Self`] and can be downcasted.
    unsafe fn downcast_ref_unchecked(value: Self::From) -> &'t Self;
}

impl<'t> DowncastRefUnchecked<'t> for CairoLint {
    type From = &'t dyn AnalyzerPlugin;

    unsafe fn downcast_ref_unchecked(value: Self::From) -> &'t Self {
        unsafe { &*(value as *const dyn AnalyzerPlugin as *const CairoLint) }
    }
}
