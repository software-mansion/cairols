use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn infered_generic() {
    test_transform!(test_hover,"
    enum Foo<
        T,
        +Copy<T>,
        impl One: core::num::traits::One<T>
    > {
        Bar: T
    }

    fn main() {
        let _ = Foo::B<caret>ar(1234);
    }
    ",@r#"
    source_context = """
        let _ = Foo::B<caret>ar(1234);
    """
    highlight = """
        let _ = Foo::<sel>Bar</sel>(1234);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    enum Foo {
        Bar: T,
    }

    T = felt252
    +Copy<T> = core::felt252Copy
    impl One: core::num::traits::One<T> = core::felt_252::Felt252One


    ```
    """
    "#)
}
