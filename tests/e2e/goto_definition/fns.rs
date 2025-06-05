use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn fn_call() {
    test_transform_plain!(GotoDefinition, r"
    fn main() { fo<caret>o(); }
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
    ")
}

#[test]
fn fn_call_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    fn main() { fo<caret>o(); }
    #[complex_attribute_macro_v2]
    fn foo() {} // good
    mod bar {
        fn foo() {} // bad
    }
    ", @r"
    #[complex_attribute_macro_v2]
    fn main() { foo(); }
    #[complex_attribute_macro_v2]
    fn <sel>foo</sel>() {} // good
    mod bar {
        fn foo() {} // bad
    }
    ")
}
