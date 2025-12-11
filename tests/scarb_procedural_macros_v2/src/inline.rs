use cairo_lang_macro::{Diagnostic, ProcMacroResult, TokenStream, TokenTree, inline_macro, quote};

#[inline_macro]
pub fn simple_inline_macro_v2(item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        #item
    };
    ProcMacroResult::new(ts)
}

#[inline_macro]
pub fn complex_inline_macro_v2(item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        simple_inline_macro_v2!(#item) + simple_inline_macro_v2!(#item)
    };
    ProcMacroResult::new(ts)
}

#[inline_macro]
pub fn improper_inline_macro_v2(item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        {
            #item;
            unbound_identifier_v2
        }
    };
    ProcMacroResult::new(ts)
}

#[inline_macro]
pub fn error_inline_with_location_macro_v2(item: TokenStream) -> ProcMacroResult {
    let first_token_span = match &item.tokens[0] {
        TokenTree::Ident(t) => t.span.clone(),
    };
    ProcMacroResult::new(TokenStream::empty()).with_diagnostics(
        Diagnostic::span_error(first_token_span, "Error from procedural macro").into(),
    )
}

#[inline_macro]
pub fn error_inline_macro_v2(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}

#[inline_macro]
pub fn simple_module_level_inline_macro_v2(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(quote! {
        pub fn foo() -> felt252 {
            123
        }
    })
}
