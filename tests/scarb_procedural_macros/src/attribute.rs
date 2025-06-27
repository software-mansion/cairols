use cairo_lang_macro::{Diagnostic, ProcMacroResult, TokenStream, attribute_macro};
use indoc::indoc;

#[attribute_macro]
pub fn simple_attribute_macro(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    let result = String::from("fn generated_function() {}");
    ProcMacroResult::new(TokenStream::new(result))
}

#[attribute_macro]
pub fn complex_attribute_macro(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    let result = String::from(indoc!(
        r#"
        #[simple_attribute_macro]
        fn generated_function_with_other_attribute() {}
        "#
    ));

    ProcMacroResult::new(TokenStream::new(result))
}

#[attribute_macro]
pub fn improper_attribute_macro(_args: TokenStream, item: TokenStream) -> ProcMacroResult {
    let result = format!("{item} fn added_fun() {{ a = b; }}"); // Syntax error
    ProcMacroResult::new(TokenStream::new(result))
}

#[attribute_macro]
pub fn error_attribute_macro(_args: TokenStream, _item: TokenStream) -> ProcMacroResult {
    ProcMacroResult::new(TokenStream::empty())
        .with_diagnostics(Diagnostic::error("Error from procedural macro").into())
}
