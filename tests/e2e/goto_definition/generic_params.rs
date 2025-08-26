use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn type_parameter_in_type_alias() {
    test_transform_plain!(GotoDefinition, r#"
    type Type<T<caret>> = Result<T, felt252>;
    "#, @"type Type<<sel>T</sel>> = Result<T, felt252>;")
}

#[test]
fn type_parameter_in_type_alias_type_expression() {
    test_transform_plain!(GotoDefinition, r#"
    type Type<T> = Result<T<caret>, felt252>;
    "#, @"type Type<<sel>T</sel>> = Result<T, felt252>;")
}

#[test]
fn type_parameter_in_impl_alias() {
    test_transform_plain!(GotoDefinition, r#"
    impl MyDropImpl<T> of Drop<T>;
    impl Impl<T<caret>> = MyDropImpl<T>;
    "#, @r"
    impl MyDropImpl<T> of Drop<T>;
    impl Impl<<sel>T</sel>> = MyDropImpl<T>;
    ")
}

#[test]
fn type_parameter_in_impl_alias_type_expression() {
    test_transform_plain!(GotoDefinition, r#"
    impl MyDropImpl<T> of Drop<T>;
    impl Impl<T> = MyDropImpl<T<caret>>;
    "#, @r"
    impl MyDropImpl<T> of Drop<T>;
    impl Impl<<sel>T</sel>> = MyDropImpl<T>;
    ")
}

#[test]
fn type_parameter_in_function() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<T<caret>>() {}
    "#, @r"
    fn foo<<sel>T</sel>>() {}
    ")
}

#[test]
fn type_parameter_in_function_parameter_type() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    fn foo<T>(x: T<caret>) -> T {
        x
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    fn foo<<sel>T</sel>>(x: T) -> T {
        x
    }
    ")
}

#[test]
fn type_parameter_in_function_return_type() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<T>(x: T) -> T<caret> {
        x
    }
    "#, @r"
    fn foo<<sel>T</sel>>(x: T) -> T {
        x
    }
    ")
}

#[test]
fn type_parameter_in_turbofish_in_function_body() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<T, +Clone<T>, +Drop<T>>(x: T) -> T {
        Clone::<T<caret>>::clone(x)
    }
    "#, @r"
    fn foo<<sel>T</sel>, +Clone<T>, +Drop<T>>(x: T) -> T {
        Clone::<T>::clone(x)
    }
    ")
}

#[test]
fn type_parameter_in_function_type_bound() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<T, +Drop<T<caret>>>(x: T) -> T {
        x
    }
    "#, @r"
    fn foo<<sel>T</sel>, +Drop<T>>(x: T) -> T {
        x
    }
    ")
}

#[test]
fn type_parameter_in_function_impl_parameter_trait_path() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<T, impl Impl: Drop<T<caret>>>() {}
    "#, @"fn foo<<sel>T</sel>, impl Impl: Drop<T>>() {}")
}

#[test]
fn type_parameter_in_function_impl_type_constraint() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<U, V, impl Impl: IntoIterator<U>[IntoIter: V<caret>]>() {}
    "#, @"fn foo<U, <sel>V</sel>, impl Impl: IntoIterator<U>[IntoIter: V]>() {}")
}

#[test]
fn type_parameter_in_struct() {
    test_transform_plain!(GotoDefinition, r#"
    struct Struct<T<caret>> {}
    "#, @r"
    struct Struct<<sel>T</sel>> {}
    ")
}

#[test]
fn type_parameter_in_struct_member_type_simple() {
    test_transform_plain!(GotoDefinition, r#"
    struct Struct<T> {
        x: T<caret>,
    }
    "#, @r"
    struct Struct<<sel>T</sel>> {
        x: T,
    }
    ")
}

#[test]
fn type_parameter_in_member_type_in_type_expression() {
    test_transform_plain!(GotoDefinition, r#"
    struct Struct<T> {
        x: Option<T<caret>>,
    }
    "#, @r"
    struct Struct<<sel>T</sel>> {
        x: Option<T>,
    }
    ")
}

#[test]
fn type_parameter_in_struct_type_bound() {
    test_transform_plain!(GotoDefinition, r#"
    struct Struct<T, +Drop<T<caret>>> {}
    "#, @r"
    struct Struct<<sel>T</sel>, +Drop<T>> {}
    ")
}

#[test]
fn type_parameter_in_enum() {
    test_transform_plain!(GotoDefinition, r#"
    enum Enum<T<caret>> {}
    "#, @r"
    enum Enum<<sel>T</sel>> {}
    ")
}

#[test]
fn type_parameter_in_enum_variant_type_simple() {
    test_transform_plain!(GotoDefinition, r#"
    enum Enum<T> {
        First: T<caret>,
    }
    "#, @r"
    enum Enum<<sel>T</sel>> {
        First: T,
    }
    ")
}

#[test]
fn type_parameter_in_enum_variant_complex() {
    test_transform_plain!(GotoDefinition, r#"
    enum Enum<T> {
        First: T,
        Second: (T, Option<T<caret>>),
    }
    "#, @r"
    enum Enum<<sel>T</sel>> {
        First: T,
        Second: (T, Option<T>),
    }
    ")
}

#[test]
fn type_parameter_in_enum_type_bound() {
    test_transform_plain!(GotoDefinition, r#"
    enum Enum<T, +Drop<T<caret>>> {}
    "#, @r"
    enum Enum<<sel>T</sel>, +Drop<T>> {}
    ")
}

#[test]
fn type_parameter_in_trait() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T<caret>> {}
    "#, @"trait Trait<<sel>T</sel>> {}")
}

#[test]
fn type_parameter_in_trait_function() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        fn foo() -> T<caret>;
    }
    "#, @r"
    trait Trait<<sel>T</sel>> {
        fn foo() -> T;
    }
    ")
}

#[test]
fn type_parameter_standalone_in_trait_function() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        fn foo<U>() -> (T, U<caret>);
    }
    "#, @r"
    trait Trait<T> {
        fn foo<<sel>U</sel>>() -> (T, U);
    }
    ")
}

#[test]
fn type_parameter_in_trait_associated_impl() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        impl Impl: Drop<T<caret>>;
    }
    "#, @r"
    trait Trait<<sel>T</sel>> {
        impl Impl: Drop<T>;
    }
    ")
}

#[test]
fn type_parameter_in_trait_associated_const() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        const CONST: T<caret>;
    }
    "#, @r"
    trait Trait<<sel>T</sel>> {
        const CONST: T;
    }
    ")
}

#[test]
fn type_parameter_in_trait_bound() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T, +Drop<T<caret>>> {}
    "#, @"trait Trait<<sel>T</sel>, +Drop<T>> {}")
}

#[test]
fn type_parameter_in_impl() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {}
    impl Impl<T<caret>> of Trait<T> {}
    "#, @r"
    trait Trait<T> {}
    impl Impl<<sel>T</sel>> of Trait<T> {}
    ")
}

#[test]
fn type_parameter_in_impl_trait_path() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {}
    impl Impl<T> of Trait<T<caret>> {}
    "#, @r"
    trait Trait<T> {}
    impl Impl<<sel>T</sel>> of Trait<T> {}
    ")
}

#[test]
fn type_parameter_in_impl_function() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        fn foo(x: T);
    }

    impl Impl<T> of Trait<T> {
        fn foo(x: T<caret>) {}
    }
    "#, @r"
    trait Trait<T> {
        fn foo(x: T);
    }

    impl Impl<<sel>T</sel>> of Trait<T> {
        fn foo(x: T) {}
    }
    ")
}

#[test]
fn type_parameter_standalone_in_impl_function() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        fn foo(x: T);
    }

    impl Impl<T> of Trait<T> {
        fn foo<U>(x: T, y: U<caret>) {}
    }
    "#, @r"
    trait Trait<T> {
        fn foo(x: T);
    }

    impl Impl<T> of Trait<T> {
        fn foo<<sel>U</sel>>(x: T, y: U) {}
    }
    ")
}

#[test]
fn type_parameter_in_impl_associated_impl() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        impl ImplImpl: Drop<T>;
    }

    impl MyDropImpl<T> of Drop<T>;

    impl Impl<T> of Trait<T> {
        impl ImplImpl = MyDropImpl<<caret>T>;
    }
    "#, @r"
    trait Trait<T> {
        impl ImplImpl: Drop<T>;
    }

    impl MyDropImpl<T> of Drop<T>;

    impl Impl<<sel>T</sel>> of Trait<T> {
        impl ImplImpl = MyDropImpl<T>;
    }
    ")
}

#[test]
fn type_parameter_in_impl_associated_const() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T> {
        const CONST: T;
    }

    impl Impl<T> of Trait<T> {
        const CONST: T<caret> = 0x0;
    }
    "#, @r"
    trait Trait<T> {
        const CONST: T;
    }

    impl Impl<<sel>T</sel>> of Trait<T> {
        const CONST: T = 0x0;
    }
    ")
}

#[test]
fn type_parameter_in_impl_bound() {
    test_transform_plain!(GotoDefinition, r#"
    trait Trait<T, +Drop<T>> {}
    impl Impl<T, +Drop<T<caret>>> of Trait<T> {}
    "#, @r"
    trait Trait<T, +Drop<T>> {}
    impl Impl<<sel>T</sel>, +Drop<T>> of Trait<T> {}
    ")
}

#[test]
fn const_parameter_in_function() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<const C<caret>>: felt252>() {}
    "#, @"fn foo<<sel>const C</sel>>: felt252>() {}")
}

#[test]
fn const_parameter_in_function_body() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<const C>: felt252>() {
        let x = C<caret>;
    }
    "#, @r"
    fn foo<<sel>const C</sel>>: felt252>() {
        let x = C;
    }
    ")
}

#[test]
fn impl_parameter_in_function_body() {
    test_transform_plain!(GotoDefinition, r#"
    fn foo<T, impl Impl: Drop<T>>(x: T) {
        Impl<caret>::drop(x);
    }
    "#, @r"
    fn foo<T, <sel>impl Impl: Drop<T></sel>>(x: T) {
        Impl::drop(x);
    }
    ")
}
