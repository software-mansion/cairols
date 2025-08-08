use lsp_types::request::References;

use crate::support::insta::test_transform_with_macros;

#[test]
fn type_parameter_in_function() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    fn foo<T<caret>, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]>(x: T) -> T {
        let y = x.clone();
        Drop::<T>::drop(x);
        y
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    fn foo<<sel>T</sel>, V, +Drop<<sel>T</sel>>, +Clone<<sel>T</sel>>, impl Impl: IntoIterator<V>[IntoIter: <sel>T</sel>]>(x: <sel>T</sel>) -> <sel>T</sel> {
        let y = x.clone();
        Drop::<<sel>T</sel>>::drop(x);
        y
    }
    ")
}

#[test]
fn type_parameter_in_struct() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    struct Struct<T<caret>, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]> {
        x: T,
        y: Option<T>,
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    struct Struct<<sel>T</sel>, V, +Drop<<sel>T</sel>>, +Clone<<sel>T</sel>>, impl Impl: IntoIterator<V>[IntoIter: <sel>T</sel>]> {
        x: <sel>T</sel>,
        y: Option<<sel>T</sel>>,
    }
    ")
}

#[test]
fn type_parameter_in_enum() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    enum Enum<T<caret>, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]> {
        x: T,
        y: (T, Option<T>),
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    enum Enum<<sel>T</sel>, V, +Drop<<sel>T</sel>>, +Clone<<sel>T</sel>>, impl Impl: IntoIterator<V>[IntoIter: <sel>T</sel>]> {
        x: <sel>T</sel>,
        y: (<sel>T</sel>, Option<<sel>T</sel>>),
    }
    ")
}

#[test]
fn type_parameter_in_trait() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    trait Trait<T<caret>, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]> {
        impl Impl: Copy<T>;
        const CONST: T;

        fn foo(x: T) -> T {
            let y = x.clone();
            Drop::<T>::drop(x);
            y
        }
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    trait Trait<<sel>T</sel>, V, +Drop<<sel>T</sel>>, +Clone<<sel>T</sel>>, impl Impl: IntoIterator<V>[IntoIter: <sel>T</sel>]> {
        impl Impl: Copy<<sel>T</sel>>;
        const CONST: <sel>T</sel>;

        fn foo(x: <sel>T</sel>) -> <sel>T</sel> {
            let y = x.clone();
            Drop::<<sel>T</sel>>::drop(x);
            y
        }
    }
    ")
}

#[test]
fn type_parameter_in_impl() {
    test_transform_with_macros!(References, r#"
    trait Trait<T, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]> {
        type Type;
        impl Impl: Copy<T>;
        const CONST: T;
        fn foo(x: T) -> T {}
    }

    impl MyCopyImpl<T> of Copy<T>;

    #[complex_attribute_macro_v2]
    impl Impl<T<caret>, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]> of Trait<T> {
        type Type = Option<T>;
        impl Impl = MyCopyImpl<T>;
        const CONST: T = 0x0;

        fn foo(x: T) -> T {
            let y = Clone::<T>::clone(x);
            Drop::<T>::drop(x);
            y
        }
    }
    "#, @r"
    trait Trait<T, V, +Drop<T>, +Clone<T>, impl Impl: IntoIterator<V>[IntoIter: T]> {
        type Type;
        impl Impl: Copy<T>;
        const CONST: T;
        fn foo(x: T) -> T {}
    }

    impl MyCopyImpl<T> of Copy<T>;

    #[complex_attribute_macro_v2]
    impl Impl<<sel>T</sel>, V, +Drop<<sel>T</sel>>, +Clone<<sel>T</sel>>, impl Impl: IntoIterator<V>[IntoIter: <sel>T</sel>]> of Trait<<sel>T</sel>> {
        type Type = Option<<sel>T</sel>>;
        impl Impl = MyCopyImpl<<sel>T</sel>>;
        const CONST: <sel>T</sel> = 0x0;

        fn foo(x: <sel>T</sel>) -> <sel>T</sel> {
            let y = Clone::<<sel>T</sel>>::clone(x);
            Drop::<<sel>T</sel>>::drop(x);
            y
        }
    }
    ")
}

#[test]
fn const_parameter_in_function() {
    test_transform_with_macros!(References, r#"
    #[complex_attribute_macro_v2]
    fn foo<const C<caret>: u32>() -> felt252 {
        bar::<C>();
        C.into()
    }

    fn bar<const C: u32> {}
    "#, @r"
    #[complex_attribute_macro_v2]
    fn foo<<sel=declaration>const <sel>C</sel>: u32</sel>>() -> felt252 {
        bar::<<sel>C</sel>>();
        <sel>C</sel>.into()
    }

    fn bar<const C: u32> {}
    ")
}

#[test]
fn impl_parameter_in_impl() {
    test_transform_with_macros!(References, r#"
    trait Trait<T, +Copy<T>, +Drop<T>, impl Impl: Copy<T>> {
        impl ImplImpl: Copy<T>;
        fn foo(x: T) {}
    }

    #[complex_attribute_macro_v2]
    impl TraitImpl<T, +Copy<T>, +Drop<T>, impl Impl<caret>: Copy<T>> of Trait<T, Impl> {
        impl ImplImpl = Impl;

        fn foo(x: T) {
            let y = Impl::copy(x);
        }
    }
    "#, @r"
    trait Trait<T, +Copy<T>, +Drop<T>, impl Impl: Copy<T>> {
        impl ImplImpl: Copy<T>;
        fn foo(x: T) {}
    }

    #[complex_attribute_macro_v2]
    impl TraitImpl<T, +Copy<T>, +Drop<T>, <sel=declaration>impl <sel>Impl</sel>: Copy<T></sel>> of Trait<T, <sel>Impl</sel>> {
        impl ImplImpl = <sel>Impl</sel>;

        fn foo(x: T) {
            let y = <sel>Impl</sel>::copy(x);
        }
    }
    ")
}
