use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn inline_macro() {
    test_transform_plain!(GotoDefinition, r#"
    fn main() {
        prin<caret>t!("Hello, world!");
    }
    "#, @"none response")
}

#[test]
fn inline_macro_with_macros() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        prin<caret>t!("Hello, world!");
    }
    "#, @"none response")
}
