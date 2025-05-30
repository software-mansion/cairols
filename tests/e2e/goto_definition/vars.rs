use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_and_macros;

#[test]
fn var_via_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        let abc: felt252 = 0; // good
        let _ = ab<caret>c * 2;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let abc: felt252 = 1;  // bad
    }
    ", @r"
    fn main() {
        let <sel>abc</sel>: felt252 = 0; // good
        let _ = abc * 2;
    }

    fn foo() {
        let abc: felt252 = 1;  // bad
    }

    ==============================

    #[complex_attribute_macro_v2]
    fn main() {
        let <sel>abc</sel>: felt252 = 0; // good
        let _ = abc * 2;
    }

    #[complex_attribute_macro_v2]
    fn foo() {
        let abc: felt252 = 1;  // bad
    }
    ")
}

#[test]
fn var_via_binding() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() {
        let a<caret>bc: felt252 = 0; // good
        let _ = abc * 2;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let abc: felt252 = 1;  // bad
    }
    ", @r"
    fn main() {
        let <sel>abc</sel>: felt252 = 0; // good
        let _ = abc * 2;
    }

    fn foo() {
        let abc: felt252 = 1;  // bad
    }

    ==============================

    #[complex_attribute_macro_v2]
    fn main() {
        let <sel>abc</sel>: felt252 = 0; // good
        let _ = abc * 2;
    }

    #[complex_attribute_macro_v2]
    fn foo() {
        let abc: felt252 = 1;  // bad
    }
    ")
}

#[test]
fn fn_param_via_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main(abc: felt252, def: felt252) { // good
        let _ = ab<caret>c * 2;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(abc: felt252) {} // bad
    ", @r"
    fn main(<sel>abc</sel>: felt252, def: felt252) { // good
        let _ = abc * 2;
    }

    fn foo(abc: felt252) {} // bad

    ==============================

    #[complex_attribute_macro_v2]
    fn main(<sel>abc</sel>: felt252, def: felt252) { // good
        let _ = abc * 2;
    }

    #[complex_attribute_macro_v2]
    fn foo(abc: felt252) {} // bad
    ")
}

#[test]
fn fn_param_via_binding() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main(a<caret>bc: felt252, def: felt252) { // good
        let _ = abc * 2;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(abc: felt252) {} // bad
    ", @r"
    fn main(<sel>abc</sel>: felt252, def: felt252) { // good
        let _ = abc * 2;
    }

    fn foo(abc: felt252) {} // bad

    ==============================

    #[complex_attribute_macro_v2]
    fn main(<sel>abc</sel>: felt252, def: felt252) { // good
        let _ = abc * 2;
    }

    #[complex_attribute_macro_v2]
    fn foo(abc: felt252) {} // bad
    ")
}

#[test]
fn closure_param_via_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |abc| { // good
            ab<caret>c + 3
        };
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(abc: felt252) {} // bad
    ", @r"
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |<sel>abc</sel>| { // good
            abc + 3
        };
    }

    fn foo(abc: felt252) {} // bad

    ==============================

    #[complex_attribute_macro_v2]
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |<sel>abc</sel>| { // good
            abc + 3
        };
    }

    #[complex_attribute_macro_v2]
    fn foo(abc: felt252) {} // bad
    ")
}

#[test]
fn closure_param_via_binding() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |a<caret>bc| { // good
            abc + 3
        };
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(abc: felt252) {} // bad
    ", @r"
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |<sel>abc</sel>| { // good
            abc + 3
        };
    }

    fn foo(abc: felt252) {} // bad

    ==============================

    #[complex_attribute_macro_v2]
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |<sel>abc</sel>| { // good
            abc + 3
        };
    }

    #[complex_attribute_macro_v2]
    fn foo(abc: felt252) {} // bad
    ")
}
