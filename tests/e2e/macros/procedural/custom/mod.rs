use crate::{macros::fixtures::ProjectWithCustomMacros, test_macro_expansion_and_diagnostics};

#[test]
fn inline_simple() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        fn foo() {
            let _x = simple<caret>_inline_macro!(10 +<caret> 10);
        }
        "#
    );
}

#[test]
fn inline_complex() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        fn foo() {
            let _x = complex<caret>_inline_macro!(10 +<caret> 10);
        }
        "#
    );
}

#[test]
fn inline_improper() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        fn foo() {
            let _x = improper<caret>_inline_macro!(10 +<caret> 10);
        }
        "#
    );
}

#[test]
fn inline_with_error() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        fn foo() {
            error<caret>_inline_macro!(10 +<caret> 10);
        }
        "#
    );
}

#[test]
fn attribute_simple() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[simple_attribute_macro]<caret>
        fn <caret>foo() {<caret>}
        "#
    );
}

#[test]
fn attribute_complex() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[complex_attribute_macro]<caret>
        fn <caret>foo() {<caret>}
        "#
    );
}

#[test]
fn attribute_improper() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[improper_attribute_macro]<caret>
        fn f<caret>oo() {<caret>}
        "#
    );
}

#[test]
fn attribute_with_error() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[error_attribute_macro]<caret>
        fn f<caret>oo() {<caret>}
        "#
    );
}

#[test]
fn derive_simple() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[derive<caret>(Simple<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "#
    );
}

#[test]
fn derive_complex() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[derive<caret>(Complex<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "#
    );
}

#[test]
fn derive_improper() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[derive<caret>(Improper<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "#
    );
}

#[test]
fn derive_with_error() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCustomMacros,
        r#"
        #[derive<caret>(Error<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "#
    );
}
