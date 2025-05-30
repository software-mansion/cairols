use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_plain;

#[test]
fn inline_macro() {
    test_transform_plain!(GotoDefinition, r#"
    fn main() {
        prin<caret>t!("Hello, world!");
    }
    "#, @r"
    none response
    ")
}
