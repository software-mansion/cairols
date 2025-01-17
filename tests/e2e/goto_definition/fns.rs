use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn fn_call() {
    test_transform!(goto_definition, r"
    fn main() { fo<caret>o(); }
    fn foo() {} // good
    mod bar {
        fn foo() {} // bad
    }
    ", @r"
    fn main() { foo(); }
    <sel>fn foo() {}</sel> // good
    mod bar {
        fn foo() {} // bad
    }
    ")
}
