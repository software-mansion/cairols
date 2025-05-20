use cairo_lang_macro::{
    Diagnostic, ProcMacroResult, TokenStream, TokenTree, attribute_macro, quote,
};

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

#[attribute_macro]
pub fn error_attribute_with_location_macro_v2(
    _args: TokenStream,
    item: TokenStream,
) -> ProcMacroResult {
    let first_token_span = match &item.tokens[0] {
        TokenTree::Ident(t) => t.span.clone(),
    };
    ProcMacroResult::new(TokenStream::empty()).with_diagnostics(
        Diagnostic::span_error(first_token_span, "Error from procedural macro").into(),
    )
}

#[attribute_macro]
pub fn mod_attribute_macro_v2(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        mod modzik {
            #item
            let x = ;
        }
    };
    ProcMacroResult::new(ts)
}
