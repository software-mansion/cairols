use crate::inlay_hints::inlay_hint;
use crate::support::insta::test_transform;

#[test]
fn pattern_with_generics() {
    test_transform!(inlay_hint, r#"
    struct Foo<T> {
        bar: felt252,
        baz: T,
    }

    fn main<C, +Drop<T>>(foo: C) {
        <sel>let Foo { bar, baz: _a } = Foo { bar: 12, baz: foo };</sel>
    }
    "#, @r#"
    struct Foo<T> {
        bar: felt252,
        baz: T,
    }

    fn main<C, +Drop<T>>(foo: C) {
        let Foo { bar<hint>: </hint><hint tooltip="```cairo\ncore::felt252\n```\n">core::felt252</hint>, baz: _a<hint>: </hint><hint tooltip="```cairo\nC\n```\n">C</hint> } = Foo { bar: 12, baz: foo };
    }
    "#)
}

#[test]
fn from_inline_macro() {
    test_transform!(inlay_hint, r#"
    fn main() {
        <sel>let _a = array![1234];</sel>
    }
    "#, @r#"
    fn main() {
        let _a<hint>: </hint><hint tooltip="```cairo\ncore::array::Array::<core::felt252>\n```\n">core::array::Array::<core::felt252></hint> = array![1234];
    }
    "#)
}

#[test]
fn simple() {
    test_transform!(inlay_hint, r#"
    fn main() {
        <sel>let _a = 1234;</sel>
    }
    "#, @r#"
    fn main() {
        let _a<hint>: </hint><hint tooltip="```cairo\ncore::felt252\n```\n">core::felt252</hint> = 1234;
    }
    "#)
}
