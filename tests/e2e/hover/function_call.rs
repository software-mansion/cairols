use lsp_types::Hover;

use crate::support::insta::test_transform_plain;

#[test]
fn generic_function_call() {
    test_transform_plain!(Hover, r#"
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
    fn a<T, +Copy<T>, impl One: One<T>>(b: @T) -> T

    T = felt252
    +Copy<T> = core::felt252Copy
    impl One: core::num::traits::One<T> = core::felt_252::Felt252One


    ```
    """
    "#);
}

#[test]
fn generic_function_call_macro() {
    test_transform_plain!(Hover, r#"
    #[complex_attribute_macro_v2]
    fn a<
        T,
        +Copy<T>,
        impl One: core::num::traits::One<T>
    >(
        b: @T,
    ) -> T {
        *b
    }

    #[complex_attribute_macro_v2]
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
    fn a<T, +Copy<T>, impl One: One<T>>(b: @T) -> T

    T = felt252
    +Copy<T> = core::felt252Copy
    impl One: core::num::traits::One<T> = core::felt_252::Felt252One


    ```
    """
    "#);
}

#[test]
fn generic_function_call_placeholder() {
    test_transform_plain!(Hover, r#"
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
        let _: felt252 = a::<<caret>_>(@123);
    }
    "#, @r#"
    source_context = """
        let _: felt252 = a::<<caret>_>(@123);
    """
    highlight = """
        let _: felt252 = a::<<sel>_</sel>>(@123);
    """
    popover = """
    ```cairo
    felt252
    ```
    """
    "#);
}

#[test]
fn generic_function_call_wrong() {
    test_transform_plain!(Hover, r#"
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
        let _ = a<caret>();
    }
    "#, @r#"
    source_context = """
        let _ = a<caret>();
    """
    highlight = """
        let _ = <sel>a</sel>();
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn a<T, +Copy<T>, impl One: One<T>>(b: @T) -> T
    ```
    """
    "#);
}

#[test]
fn generic_function_call_wrong_placeholder() {
    test_transform_plain!(Hover, r#"
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
        let _ = a::<<caret>_>();
    }
    "#, @r#"
    source_context = """
        let _ = a::<<caret>_>();
    """
    highlight = """
        let _ = a::<<sel>_</sel>>();
    """
    popover = """
    ```cairo
    ?
    ```
    """
    "#);
}

#[test]
fn generic_trait_function_call_placeholder() {
    test_transform_plain!(Hover, r#"
    trait Foo<T> {
        fn foo(a: @T) { }
    }

    impl FooFelt of Foo<felt252> {
        fn foo(a: @felt252) { }
    }

    fn main() {
        let _ = Foo::<<caret>_>::foo(@12);
    }
    "#, @r#"
    source_context = """
        let _ = Foo::<<caret>_>::foo(@12);
    """
    highlight = """
        let _ = Foo::<<sel>_</sel>>::foo(@12);
    """
    popover = """
    ```cairo
    felt252
    ```
    """
    "#);
}
