use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_and_macros;

#[test]
fn fn_call() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn main() { fo<caret>o(); }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {} // good

    mod bar {
        fn foo() {} // bad
    }
    ", @r"
    fn main() { foo(); }

    fn <sel>foo</sel>() {} // good

    mod bar {
        fn foo() {} // bad
    }

    ==============================

    #[complex_attribute_macro_v2]
    fn main() { foo(); }

    #[complex_attribute_macro_v2]
    fn <sel>foo</sel>() {} // good

    mod bar {
        fn foo() {} // bad
    }
    ")
}
