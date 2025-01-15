use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn var_in_expr() {
    test_transform!(goto_definition, r"
    fn main() {
        let abc: felt252 = 0; // good
        let _ = ab<caret>c * 2;
    }
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
    ")
}

#[test]
fn fn_param_in_expr() {
    test_transform!(goto_definition, r"
    fn main(abc: felt252, def: felt252) { // good
        let _ = ab<caret>c * 2;
    }
    fn foo(abc: felt252) {} // bad
    ", @r"
    fn main(<sel>abc: felt252</sel>, def: felt252) { // good
        let _ = abc * 2;
    }
    fn foo(abc: felt252) {} // bad
    ")
}

// FIXME(#120): This used to work before goto definition refactoring.
#[test]
fn closure_param_in_expr() {
    test_transform!(goto_definition, r"
    fn foo(a: felt252) -> felt252 {
        let abc: felt252 = 0; // bad
        let c = |abc| { // good
            ab<caret>c + 3
        };
    }
    
    fn foo(abc: felt252) {} // bad
    ", @"none response")
}
