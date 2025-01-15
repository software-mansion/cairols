use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn inline_macro() {
    test_transform!(goto_definition, r#"
    fn main() {
        prin<caret>t!("Hello, world!");
    }
    "#, @"none response")
}
