use cairo_lang_macro::{Diagnostic, ProcMacroResult, TokenStream, TokenTree, derive_macro, quote};

#[derive_macro]
pub fn simple_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        trait MyTrait<T> {
            fn foo(t: T);
        }

        impl MyTraitImpl of MyTrait<felt252> {
            fn foo(t: felt252) {}
        }
    };
    ProcMacroResult::new(ts)
}

#[derive_macro]
pub fn complex_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        #[simple_attribute_macro_v2]
        fn another_generated_function_v2() {}

        trait MyTraitV2<T> {
            fn bar(t: T);
        }

        impl MyTraitImpl of MyTraitV2<felt252> {
            fn bar(t: felt252) {}
        }
    };
    ProcMacroResult::new(ts)
}

#[derive_macro]
pub fn improper_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        fn generated_function_v2() {
            some <*> haskell <$> syntax
        }
    };
    ProcMacroResult::new(ts)
}

#[derive_macro]
pub fn error_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}

#[derive_macro]
pub fn error_derive_with_location_macro_v2(item: TokenStream) -> ProcMacroResult {
    let first_token_span = match &item.tokens[0] {
        TokenTree::Ident(t) => t.span.clone(),
    };
    ProcMacroResult::new(TokenStream::empty()).with_diagnostics(
        Diagnostic::span_error(first_token_span, "Error from procedural macro").into(),
    )
}

#[derive_macro]
pub fn mod_derive_macro_v2(_item: TokenStream) -> ProcMacroResult {
    let ts = quote! {
        mod modzik {
            let x = ;
        }
    };
    ProcMacroResult::new(ts)
}
