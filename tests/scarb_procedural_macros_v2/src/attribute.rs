use cairo_lang_macro::{Diagnostic, ProcMacroResult, TokenStream, attribute_macro, quote};

#[attribute_macro]
pub fn simple_attribute_macro_v2(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        #item

        fn generated_function_v2() {}
    };
    ProcMacroResult::new(ts)
}

#[attribute_macro]
pub fn complex_attribute_macro_v2(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        #item

        #[simple_attribute_macro_v2]
        fn generated_function_with_other_attribute_v2() {}
    };
    ProcMacroResult::new(ts)
}

#[attribute_macro]
pub fn improper_attribute_macro_v2(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        #item

        fn added_fun_v2() {{
            a = b;
        }}
    };
    ProcMacroResult::new(ts)
}

#[attribute_macro]
pub fn error_attribute_macro_v2(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}
