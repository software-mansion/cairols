use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn generic_function_call() {
    test_transform!(test_hover, r#"
    fn a<
        T,
        +Copy<T>,
        impl One: core::num::traits::One<T>
    >(
        b: @T,
    ) -> T {
        *b
    }

    fn main() {
        let _: felt252 = a<caret>(@123);
    }
    "#, @r#"
    source_context = """
        let _: felt252 = a<caret>(@123);
    """
    highlight = """
        let _: felt252 = <sel>a</sel>(@123);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn a<T, +Copy<T>, One>(b: @T) -> T

    T = core::felt252
    +Copy<T> = core::felt252Copy
    impl One: core::num::traits::One<T> = core::felt_252::Felt252One


    ```
    """
    "#);
}
