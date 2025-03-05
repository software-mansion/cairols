use cairo_lang_macro::{Diagnostic, ProcMacroResult, TokenStream, inline_macro};

#[inline_macro]
pub fn simple_inline_macro(item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::new(item.to_string().len().to_string()))
}

#[inline_macro]
pub fn complex_inline_macro(item: TokenStream) -> ProcMacroResult {
    let result = format!("simple_inline_macro!({0}) + simple_inline_macro!({0})", item.to_string());
    ProcMacroResult::new(TokenStream::new(result))
}

#[inline_macro]
pub fn improper_inline_macro(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::new(String::from("unbound_identifier")))
}

#[inline_macro]
pub fn error_inline_macro(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}

#[inline_macro]
pub fn which_macro_package(_item: TokenStream) -> ProcMacroResult {
    let result = String::from("'cairols_test_macros'");
    ProcMacroResult::new(TokenStream::new(result))
}
