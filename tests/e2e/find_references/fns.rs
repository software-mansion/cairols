use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn fn_via_definition() {
    test_transform!(find_references, r#"
    fn pow<caret>2(x: felt252) -> felt252 { x * x }
    fn main() {
        let x = pow2(2) + pow2(3);
    }
    "#, @r"
    fn <sel=declaration>pow2</sel>(x: felt252) -> felt252 { x * x }
    fn main() {
        let x = <sel>pow2</sel>(2) + <sel>pow2</sel>(3);
    }
    ")
}

#[test]
fn fn_via_call() {
    test_transform!(find_references, r#"
    fn pow2(x: felt252) -> felt252 { x * x }
    fn main() {
        let x = po<caret>w2(2) + pow2(3);
    }
    "#, @r"
    fn <sel=declaration>pow2</sel>(x: felt252) -> felt252 { x * x }
    fn main() {
        let x = <sel>pow2</sel>(2) + <sel>pow2</sel>(3);
    }
    ")
}

#[test]
fn unused_function() {
    test_transform!(find_references, r#"
    fn pow<caret>2(x: felt252) -> felt252 { x * x }
    fn main() {
        let pow2 = 2;
        let x = pow2 + pow2;
    }
    "#, @r"
    fn <sel=declaration>pow2</sel>(x: felt252) -> felt252 { x * x }
    fn main() {
        let pow2 = 2;
        let x = pow2 + pow2;
    }
    ")
}
