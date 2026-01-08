use lsp_types::Hover;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn type_parameter_in_type_alias() {
    test_transform_plain!(Hover, r#"
    type Type<T<caret>> = Result<T, felt252>;
    "#, @r#"
    source_context = """
    type Type<T<caret>> = Result<T, felt252>;
    """
    highlight = """
    type Type<<sel>T</sel>> = Result<T, felt252>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_type_alias_type_expression() {
    test_transform_plain!(Hover, r#"
    type Type<T> = Result<T<caret>, felt252>;
    "#, @r#"
    source_context = """
    type Type<T> = Result<T<caret>, felt252>;
    """
    highlight = """
    type Type<T> = Result<<sel>T</sel>, felt252>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_alias() {
    test_transform_plain!(Hover, r#"
    impl MyDropImpl<T> of Drop<T>;
    impl Impl<T<caret>> = MyDropImpl<T>;
    "#, @r#"
    source_context = """
    impl Impl<T<caret>> = MyDropImpl<T>;
    """
    highlight = """
    impl Impl<<sel>T</sel>> = MyDropImpl<T>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_alias_type_expression() {
    test_transform_plain!(Hover, r#"
    impl MyDropImpl<T> of Drop<T>;
    impl Impl<T> = MyDropImpl<T<caret>>;
    "#, @r#"
    source_context = """
    impl Impl<T> = MyDropImpl<T<caret>>;
    """
    highlight = """
    impl Impl<T> = MyDropImpl<<sel>T</sel>>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_function() {
    test_transform_plain!(Hover, r#"
    fn foo<T<caret>>() {}
    "#, @r#"
    source_context = """
    fn foo<T<caret>>() {}
    """
    highlight = """
    fn foo<<sel>T</sel>>() {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_function_parameter_type() {
    test_transform_with_macros!(Hover, r#"
    #[complex_attribute_macro_v2]
    fn foo<T>(x: T<caret>) -> T {
        x
    }
    "#, @r#"
    source_context = """
    fn foo<T>(x: T<caret>) -> T {
    """
    highlight = """
    fn foo<T>(x: <sel>T</sel>) -> T {
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_function_return_type() {
    test_transform_plain!(Hover, r#"
    fn foo<T>(x: T) -> T<caret> {
        x
    }
    "#, @r#"
    source_context = """
    fn foo<T>(x: T) -> T<caret> {
    """
    highlight = """
    fn foo<T>(x: T) -> <sel>T</sel> {
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_turbofish_in_function_body() {
    test_transform_plain!(Hover, r#"
    fn foo<T, +Clone<T>, +Drop<T>>(x: T) -> T {
        Clone::<T<caret>>::clone(x)
    }
    "#, @r#"
    source_context = """
        Clone::<T<caret>>::clone(x)
    """
    highlight = """
        Clone::<<sel>T</sel>>::clone(x)
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_function_type_bound() {
    test_transform_plain!(Hover, r#"
    fn foo<T, +Drop<T<caret>>>(x: T) -> T {
        x
    }
    "#, @r#"
    source_context = """
    fn foo<T, +Drop<T<caret>>>(x: T) -> T {
    """
    highlight = """
    fn foo<T, +Drop<<sel>T</sel>>>(x: T) -> T {
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_function_impl_parameter_trait_path() {
    test_transform_plain!(Hover, r#"
    fn foo<T, impl Impl: Drop<T<caret>>>() {}
    "#, @r#"
    source_context = """
    fn foo<T, impl Impl: Drop<T<caret>>>() {}
    """
    highlight = """
    fn foo<T, impl Impl: Drop<<sel>T</sel>>>() {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_function_impl_type_constraint() {
    test_transform_plain!(Hover, r#"
    fn foo<U, V, impl Impl: IntoIterator<U>[IntoIter: V<caret>]>() {}
    "#, @r#"
    source_context = """
    fn foo<U, V, impl Impl: IntoIterator<U>[IntoIter: V<caret>]>() {}
    """
    highlight = """
    fn foo<U, V, impl Impl: IntoIterator<U>[IntoIter: <sel>V</sel>]>() {}
    """
    popover = """
    ```cairo
    V
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_struct() {
    test_transform_plain!(Hover, r#"
    struct Struct<T<caret>> {}
    "#, @r#"
    source_context = """
    struct Struct<T<caret>> {}
    """
    highlight = """
    struct Struct<<sel>T</sel>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_struct_member_type_simple() {
    test_transform_plain!(Hover, r#"
    struct Struct<T> {
        x: T<caret>,
    }
    "#, @r#"
    source_context = """
        x: T<caret>,
    """
    highlight = """
        x: <sel>T</sel>,
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_member_type_in_type_expression() {
    test_transform_plain!(Hover, r#"
    struct Struct<T> {
        x: Option<T<caret>>,
    }
    "#, @r#"
    source_context = """
        x: Option<T<caret>>,
    """
    highlight = """
        x: Option<<sel>T</sel>>,
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_struct_type_bound() {
    test_transform_plain!(Hover, r#"
    struct Struct<T, +Drop<T<caret>>> {}
    "#, @r#"
    source_context = """
    struct Struct<T, +Drop<T<caret>>> {}
    """
    highlight = """
    struct Struct<T, +Drop<<sel>T</sel>>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_enum() {
    test_transform_plain!(Hover, r#"
    enum Enum<T<caret>> {}
    "#, @r#"
    source_context = """
    enum Enum<T<caret>> {}
    """
    highlight = """
    enum Enum<<sel>T</sel>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_enum_variant_type_simple() {
    test_transform_plain!(Hover, r#"
    enum Enum<T> {
        First: T<caret>,
    }
    "#, @r#"
    source_context = """
        First: T<caret>,
    """
    highlight = """
        First: <sel>T</sel>,
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_enum_variant_complex() {
    test_transform_plain!(Hover, r#"
    enum Enum<T> {
        First: T,
        Second: (T, Option<T<caret>>),
    }
    "#, @r#"
    source_context = """
        Second: (T, Option<T<caret>>),
    """
    highlight = """
        Second: (T, Option<<sel>T</sel>>),
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_enum_type_bound() {
    test_transform_plain!(Hover, r#"
    enum Enum<T, +Drop<T<caret>>> {}
    "#, @r#"
    source_context = """
    enum Enum<T, +Drop<T<caret>>> {}
    """
    highlight = """
    enum Enum<T, +Drop<<sel>T</sel>>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_trait() {
    test_transform_plain!(Hover, r#"
    trait Trait<T<caret>> {}
    "#, @r#"
    source_context = """
    trait Trait<T<caret>> {}
    """
    highlight = """
    trait Trait<<sel>T</sel>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_trait_function() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        fn foo() -> T<caret>;
    }
    "#, @r#"
    source_context = """
        fn foo() -> T<caret>;
    """
    highlight = """
        fn foo() -> <sel>T</sel>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_standalone_in_trait_function() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        fn foo<U>() -> (T, U<caret>);
    }
    "#, @r#"
    source_context = """
        fn foo<U>() -> (T, U<caret>);
    """
    highlight = """
        fn foo<U>() -> (T, <sel>U</sel>);
    """
    popover = """
    ```cairo
    U
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_trait_associated_impl() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        impl Impl: Drop<T<caret>>;
    }
    "#, @r#"
    source_context = """
        impl Impl: Drop<T<caret>>;
    """
    highlight = """
        impl Impl: Drop<<sel>T</sel>>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_trait_associated_const() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        const CONST: T<caret>;
    }
    "#, @r#"
    source_context = """
        const CONST: T<caret>;
    """
    highlight = """
        const CONST: <sel>T</sel>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_trait_bound() {
    test_transform_plain!(Hover, r#"
    trait Trait<T, +Drop<T<caret>>> {}
    "#, @r#"
    source_context = """
    trait Trait<T, +Drop<T<caret>>> {}
    """
    highlight = """
    trait Trait<T, +Drop<<sel>T</sel>>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {}
    impl Impl<T<caret>> of Trait<T> {}
    "#, @r#"
    source_context = """
    impl Impl<T<caret>> of Trait<T> {}
    """
    highlight = """
    impl Impl<<sel>T</sel>> of Trait<T> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_trait_path() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {}
    impl Impl<T> of Trait<T<caret>> {}
    "#, @r#"
    source_context = """
    impl Impl<T> of Trait<T<caret>> {}
    """
    highlight = """
    impl Impl<T> of Trait<<sel>T</sel>> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_function() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        fn foo(x: T);
    }

    impl Impl<T> of Trait<T> {
        fn foo(x: T<caret>) {}
    }
    "#, @r#"
    source_context = """
        fn foo(x: T<caret>) {}
    """
    highlight = """
        fn foo(x: <sel>T</sel>) {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_standalone_in_impl_function() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        fn foo(x: T);
    }

    impl Impl<T> of Trait<T> {
        fn foo<U>(x: T, y: U<caret>) {}
    }
    "#, @r#"
    source_context = """
        fn foo<U>(x: T, y: U<caret>) {}
    """
    highlight = """
        fn foo<U>(x: T, y: <sel>U</sel>) {}
    """
    popover = """
    ```cairo
    U
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_associated_impl() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        impl ImplImpl: Drop<T>;
    }

    impl MyDropImpl<T> of Drop<T>;

    impl Impl<T> of Trait<T> {
        impl ImplImpl = MyDropImpl<<caret>T>;
    }
    "#, @r#"
    source_context = """
        impl ImplImpl = MyDropImpl<<caret>T>;
    """
    highlight = """
        impl ImplImpl = MyDropImpl<<sel>T</sel>>;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_associated_const() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {
        const CONST: T;
    }

    impl Impl<T> of Trait<T> {
        const CONST: T<caret> = 0x0;
    }
    "#, @r#"
    source_context = """
        const CONST: T<caret> = 0x0;
    """
    highlight = """
        const CONST: <sel>T</sel> = 0x0;
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn type_parameter_in_impl_bound() {
    test_transform_plain!(Hover, r#"
    trait Trait<T, +Drop<T>> {}
    impl Impl<T, +Drop<T<caret>>> of Trait<T> {}
    "#, @r#"
    source_context = """
    impl Impl<T, +Drop<T<caret>>> of Trait<T> {}
    """
    highlight = """
    impl Impl<T, +Drop<<sel>T</sel>>> of Trait<T> {}
    """
    popover = """
    ```cairo
    T
    ```
    """
    "#)
}

#[test]
fn const_parameter_in_function() {
    test_transform_plain!(Hover, r#"
    fn foo<const C<caret>>: felt252>() {}
    "#, @r#"
    source_context = """
    fn foo<const C<caret>>: felt252>() {}
    """
    highlight = """
    fn foo<const <sel>C</sel>>: felt252>() {}
    """
    popover = """
    ```cairo
    const C: ?
    ```
    """
    "#)
}

#[test]
fn const_parameter_in_function_body() {
    test_transform_plain!(Hover, r#"
    fn foo<const C>: felt252>() {
        let x = C<caret>;
    }
    "#, @r#"
    source_context = """
        let x = C<caret>;
    """
    highlight = """
        let x = <sel>C</sel>;
    """
    popover = """
    ```cairo
    const C: ?
    ```
    """
    "#)
}

#[test]
fn impl_parameter_in_function_body() {
    test_transform_plain!(Hover, r#"
    fn foo<T, impl Impl: Drop<T>>(x: T) {
        Impl<caret>::drop(x);
    }
    "#, @r#"
    source_context = """
        Impl<caret>::drop(x);
    """
    highlight = """
        <sel>Impl</sel>::drop(x);
    """
    popover = """
    ```cairo
    impl Impl: Drop<T>
    ```
    """
    "#)
}

#[test]
fn unnecessary_generic_param_enclosure() {
    test_transform_plain!(Hover, r#"
    trait TestTrait<T> {}

    trait TestTraitWithoutGenerics {}

    impl TestTraitImp<caret>l<T, impl metadata: TestTraitWithoutGenerics> of TestTrait<T> {}
    "#, @r#"
    source_context = """
    impl TestTraitImp<caret>l<T, impl metadata: TestTraitWithoutGenerics> of TestTrait<T> {}
    """
    highlight = """
    impl <sel>TestTraitImpl</sel><T, impl metadata: TestTraitWithoutGenerics> of TestTrait<T> {}
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl TestTraitImpl<T, impl metadata: TestTraitWithoutGenerics> of TestTrait<T>;
    ```
    """
    "#)
}
