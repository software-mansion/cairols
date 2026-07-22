use lsp_types::SemanticTokens;

use crate::support::insta::test_transform_with_macros;

#[test]
fn inline_proc_macro_content_variable() {
    test_transform_with_macros!(SemanticTokens, r#"
    fn main() {
        let x = 5;
        simple_inline_macro_v2!(x);
    }
    "#, @r"
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>simple_inline_macro_v2</token><token=macro>!</token>(<token=variable>x</token>);
    }
    ")
}

#[test]
fn inline_proc_macro_content_expressions() {
    test_transform_with_macros!(SemanticTokens, r#"
    enum Foo {
        Bar,
    }

    fn make() -> felt252 {
        0
    }

    fn main() {
        let x = 5;
        simple_inline_macro_v2!(x + make());
    }
    "#, @r"
    <token=keyword>enum</token> <token=enum>Foo</token> {
        <token=enumMember>Bar</token>,
    }

    <token=keyword>fn</token> <token=function>make</token>() -> <token=type>felt252</token> {
        <token=number>0</token>
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
        <token=macro>simple_inline_macro_v2</token><token=macro>!</token>(<token=variable>x</token> <token=operator>+</token> <token=function>make</token>());
    }
    ")
}

#[test]
fn attribute_proc_macro() {
    test_transform_with_macros!(SemanticTokens, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        let x = 5;
    }
    "#, @r"
    #[<token=decorator>complex_attribute_macro_v2</token>]
    <token=keyword>fn</token> <token=function>main</token>() {
        <token=keyword>let</token> <token=variable>x</token> = <token=number>5</token>;
    }
    ")
}

#[test]
fn derive_proc_macro() {
    test_transform_with_macros!(SemanticTokens, r#"
    #[derive(ComplexDeriveMacroV2)]
    struct S {
        x: felt252,
    }
    "#, @r"
    #[<token=decorator>derive</token>(<token=decorator>ComplexDeriveMacroV2</token>)]
    <token=keyword>struct</token> <token=struct>S</token> {
        <token=variable>x</token>: <token=type>felt252</token>,
    }
    ")
}

#[test]
fn module_level_inline_proc_macro() {
    test_transform_with_macros!(SemanticTokens, r#"
    mod fu {
        simple_module_level_inline_macro_v2!();
    }

    fn main() {
        fu::foo();
    }
    "#, @r"
    <token=keyword>mod</token> <token=namespace>fu</token> {
        <token=macro>simple_module_level_inline_macro_v2</token><token=macro>!</token>();
    }

    <token=keyword>fn</token> <token=function>main</token>() {
        <token=namespace>fu</token>::<token=function>foo</token>();
    }
    ")
}
