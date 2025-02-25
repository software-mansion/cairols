use crate::{
    macros::{MacroTestFixture, fixtures::SimpleProject},
    support::insta::test_transform,
};

#[test]
fn inline_simple() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        fn foo() {
            let _x = simple<caret>_inline_macro!(10 +<caret> 10);
        }
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
            let _x = simple<caret>_inline_macro!(10 + 10);
            let _x = simple_inline_macro!(10 +<caret> 10);
        """
        generated_code = """
        // lib.cairo
        // ---------

        9
        """
        "#
    )
}

#[test]
fn inline_complex() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        fn foo() {
            let _x = complex<caret>_inline_macro!(10 +<caret> 10);
        }
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
            let _x = complex<caret>_inline_macro!(10 + 10);
            let _x = complex_inline_macro!(10 +<caret> 10);
        """
        generated_code = """
        // lib.cairo
        // ---------

        simple_inline_macro!((10 + 10)) + simple_inline_macro!((10 + 10))
        """
        "#
    )
}

#[test]
fn inline_improper() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        fn foo() {
            let _x = improper<caret>_inline_macro!(10 +<caret> 10);
        }
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
            let _x = improper<caret>_inline_macro!(10 + 10);
            let _x = improper_inline_macro!(10 +<caret> 10);
        """
        generated_code = """
        // lib.cairo
        // ---------

        unbound_identifier
        """

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "1:13-1:44"
        message = "Identifier not found."
        severity = "Error"
        "#
    )
}

#[test]
fn inline_with_error() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        fn foo() {
            error<caret>_inline_macro!(10 +<caret> 10);
        }
        "##,

        @r#"
    [[expansions]]
    analyzed_lines = """
        error<caret>_inline_macro!(10 + 10);
        error_inline_macro!(10 +<caret> 10);
    """
    generated_code = """
    No expansion information.

    """

    [[mapped_diagnostics]]
    url = "[ROOT_URL]test_package/src/lib.cairo"
    range = "1:4-1:32"
    message = "Plugin diagnostic: Error from procedural macro"
    severity = "Error"
    "#
    )
}

#[test]
fn attribute_simple() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[simple_attribute_macro]<caret>
        fn <caret>foo() {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[simple_attribute_macro]<caret>
        fn <caret>foo() {}
        fn foo() {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[simple_attribute_macro]
        fn foo() {}

        // proc_macro_simple_attribute_macro
        // ---------------------------------

        fn generated_function() {}
        """
        "#
    )
}

#[test]
fn attribute_complex() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[complex_attribute_macro]<caret>
        fn <caret>foo() {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[complex_attribute_macro]<caret>
        fn <caret>foo() {}
        fn foo() {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[complex_attribute_macro]
        fn foo() {}

        // proc_macro_complex_attribute_macro
        // ----------------------------------

        #[simple_attribute_macro]
        fn generated_function_with_other_attribute() {}

        // proc_macro_simple_attribute_macro
        // ---------------------------------

        fn generated_function() {}
        """
        "#
    )
}

#[test]
fn attribute_improper() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[improper_attribute_macro]<caret>
        fn f<caret>oo() {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[improper_attribute_macro]<caret>
        fn f<caret>oo() {}
        fn foo() {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[improper_attribute_macro]
        fn foo() {}

        // proc_macro_improper_attribute_macro
        // -----------------------------------

        fn foo() {}
        fn added_fun() {
            a = b;
        }
        """

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:11"
        message = "Identifier not found."
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:11"
        message = "Identifier not found."
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:11"
        message = "Invalid left-hand side of assignment."
        severity = "Error"
        "#
    )
}

#[test]
fn attribute_with_error() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[error_attribute_macro]<caret>
        fn f<caret>oo() {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[error_attribute_macro]<caret>
        fn f<caret>oo() {}
        fn foo() {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[error_attribute_macro]
        fn foo() {}
        """

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-0:24"
        message = "Plugin diagnostic: Error from procedural macro"
        severity = "Error"
        "#
    )
}

#[test]
fn derive_simple() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[derive<caret>(Simple<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[derive<caret>(SimpleDeriveMacro)]
        #[derive(Simple<caret>DeriveMacro)]
        struct Empty<caret>Struct {}
        struct EmptyStruct {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[derive(SimpleDeriveMacro)]
        struct EmptyStruct {}

        // proc_macro_derive
        // -----------------

        trait MyTrait<T> {
            fn foo(t: T);
        }

        impl MyTraitImpl of MyTrait<felt252> {
            fn foo(t: felt252) {}
        }
        """
        "#
    )
}

#[test]
fn derive_complex() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[derive<caret>(Complex<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[derive<caret>(ComplexDeriveMacro)]
        #[derive(Complex<caret>DeriveMacro)]
        struct Empty<caret>Struct {}
        struct EmptyStruct {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[derive(ComplexDeriveMacro)]
        struct EmptyStruct {}

        // proc_macro_derive
        // -----------------

        #[simple_attribute_macro]
        fn generated_function() {}

        trait MyTrait<T> {
            fn foo(t: T);
        }

        impl MyTraitImpl of MyTrait<felt252> {
            fn foo(t: felt252) {}
        }

        // proc_macro_simple_attribute_macro
        // ---------------------------------

        fn generated_function() {}
        """
        "#
    )
}

#[test]
fn derive_improper() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[derive<caret>(Improper<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[derive<caret>(ImproperDeriveMacro)]
        #[derive(Improper<caret>DeriveMacro)]
        struct Empty<caret>Struct {}
        struct EmptyStruct {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[derive(ImproperDeriveMacro)]
        struct EmptyStruct {}

        // proc_macro_derive
        // -----------------

        fn generated_function() {
            some < *> haskell <
            $ > syntax
        }
        """

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Identifier not found."
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Are you missing a `::`?."
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Identifier not found."
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Are you missing a `::`?."
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Missing semicolon"
        severity = "Error"

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Identifier not found."
        severity = "Error"
        "#
    )
}

#[test]
fn derive_with_error() {
    test_transform!(
        SimpleProject::test_macro_expansion_and_diagnostics,

        r##"
        #[derive<caret>(Error<caret>DeriveMacro)]
        struct Empty<caret>Struct {<caret>}
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
        #[derive<caret>(ErrorDeriveMacro)]
        #[derive(Error<caret>DeriveMacro)]
        struct Empty<caret>Struct {}
        struct EmptyStruct {<caret>}
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[derive(ErrorDeriveMacro)]
        struct EmptyStruct {}
        """

        [[mapped_diagnostics]]
        url = "[ROOT_URL]test_package/src/lib.cairo"
        range = "0:0-1:21"
        message = "Plugin diagnostic: Error from procedural macro"
        severity = "Error"
        "#
    )
}
