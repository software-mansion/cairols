use crate::code_actions::quick_fix;
use crate::support::insta::test_transform;

const TRAIT_CODE: &str = r#"
    pub struct SomeStructWithConstParameter<const C: u8> {}
    pub trait MyTrait<T, U, const C: u8> {
        const CONCRETE_CONST: u32;
        const GENERIC_CONST: T;
    
        type Type;
    
        fn foo(t: T, v: U) -> T;
        fn bar(t: T) -> U;
        fn baz(s: SomeStructWithConstParameter<C>);
    
        fn generic<const V: u32, W, +Into<T, W>>(w: W);
    
        fn with_concrete_impl<W, impl SomeImpl: Into<T, W>>(w: W) -> W;
        }
"#;

fn test_fill_trait(cairo_code: &str) -> String {
    quick_fix(format!("{}{}", TRAIT_CODE, cairo_code).as_str())
}

fn test_fill_trait_nested(cairo_code: &str) -> String {
    let nested_trait = format!(
        r#"
            mod trait_module {{
                {}
            }}
            mod other_module {{
                {}
            }}
    "#,
        TRAIT_CODE, cairo_code
    );
    quick_fix(nested_trait.as_str())
}

#[test]
fn fill_empty_impl() {
    test_transform!(test_fill_trait,
        r#"
            impl EmptyImpl<caret> of MyTrait<u32, felt252, 1> {

            }
        "#,
        @r#"
    Title: Implement missing members
    Add new text: "type Type = ();

    const CONCRETE_CONST: core::integer::u32 = ();

    const GENERIC_CONST: core::integer::u32 = ();

    fn foo(t: core::integer::u32, v: core::felt252) -> core::integer::u32 {}

    fn bar(t: core::integer::u32) -> core::felt252 {}

    fn baz(s: hello::SomeStructWithConstParameter::<1>) {}

    fn generic<const V: core::integer::u32, W, +Into<core::integer::u32, W>>(w: W) {}

    fn with_concrete_impl<W, impl SomeImpl: Into<core::integer::u32, W>>(w: W) -> W {}"
    At: Range { start: Position { line: 16, character: 0 }, end: Position { line: 16, character: 0 } }
    "#
    )
}

#[test]
fn fill_impl_with_const() {
    test_transform!(test_fill_trait,
        r#"
            impl ImplWithConst<caret> of MyTrait<u32, felt252, 10> {
                const CONCRETE_CONST: u32 = 0;
            }
        "#,
        @r#"
    Title: Implement missing members
    Add new text: "type Type = ();

    const GENERIC_CONST: core::integer::u32 = ();

    fn foo(t: core::integer::u32, v: core::felt252) -> core::integer::u32 {}

    fn bar(t: core::integer::u32) -> core::felt252 {}

    fn baz(s: hello::SomeStructWithConstParameter::<10>) {}

    fn generic<const V: core::integer::u32, W, +Into<core::integer::u32, W>>(w: W) {}

    fn with_concrete_impl<W, impl SomeImpl: Into<core::integer::u32, W>>(w: W) -> W {}"
    At: Range { start: Position { line: 16, character: 34 }, end: Position { line: 16, character: 34 } }
    "#
    )
}

#[test]
fn fill_impl_with_function() {
    test_transform!(test_fill_trait,
        r#"
            impl ImplWithFoo<caret> of MyTrait<u32, felt252, 0> {
                fn foo(t: u32, v: felt252) -> u32 { 0 }
            }
        "#,
        @r#"
    Title: Implement missing members
    Add new text: "type Type = ();

    const CONCRETE_CONST: core::integer::u32 = ();

    const GENERIC_CONST: core::integer::u32 = ();

    fn bar(t: core::integer::u32) -> core::felt252 {}

    fn baz(s: hello::SomeStructWithConstParameter::<0>) {}

    fn generic<const V: core::integer::u32, W, +Into<core::integer::u32, W>>(w: W) {}

    fn with_concrete_impl<W, impl SomeImpl: Into<core::integer::u32, W>>(w: W) -> W {}"
    At: Range { start: Position { line: 16, character: 43 }, end: Position { line: 16, character: 43 } }
    "#
    )
}

#[test]
fn no_fill_impl_with_all_filled() {
    test_transform!(test_fill_trait,
        r#"
        impl ImplWithEverything<caret> of MyTrait<u32, felt252, 12> {
            const CONCRETE_CONST: u32 = 0;
            const GENERIC_CONST: u32 = 0;

            type Type = u256;

            fn foo(t: u32, v: felt252) -> u32 { 0 }
            fn bar(t: u32) -> felt252 { 0 }
            fn baz(s: SomeStructWithConstParameter::<12>) {}

            fn generic<const V: u32, W, +Into<u32, W>>(w: W) {}

            fn with_concrete_impl<W, impl SomeImpl: Into<T, W>>(w: W) -> W {}
        }
        "#,
        @"No code actions."
    )
}

#[test]
fn no_fill_non_existent_trait() {
    test_transform!(test_fill_trait,
        r#"
        impl WrongImpl<caret> NonExistentTrait<u8> {}
        "#,
        @"No code actions."
    )
}

#[test]
fn fill_trait_no_generic_args() {
    test_transform!(test_fill_trait,
        r#"
        impl ImplWithoutGenericArgs<caret> of MyTrait {}
        "#,
        @r#"
    Title: Implement missing members
    Add new text: "type Type = ();

    const CONCRETE_CONST: core::integer::u32 = ();

    const GENERIC_CONST: ?2 = ();

    fn foo(t: ?2, v: ?3) -> ?2 {}

    fn bar(t: ?2) -> ?3 {}

    fn baz(s: hello::SomeStructWithConstParameter::<?1>) {}

    fn generic<const V: core::integer::u32, W, +Into<?2, W>>(w: W) {}

    fn with_concrete_impl<W, impl SomeImpl: Into<?2, W>>(w: W) -> W {}"
    At: Range { start: Position { line: 15, character: 40 }, end: Position { line: 15, character: 40 } }
    "#
    )
}

#[test]
fn fill_trait_no_braces() {
    test_transform!(test_fill_trait,
        r#"
        impl NoBraces<caret> of MyTrait<u32, u8>;
        "#,
        @"No code actions."
    )
}

#[test]
fn fill_imported_trait() {
    test_transform!(test_fill_trait_nested,
        r#"
            use super::trait_module::MyTrait;
            
            impl EmptyImpl<caret> of MyTrait<u32, felt252, 1> {

            }
        "#,
        @r#"
    Title: Implement missing members
    Add new text: "type Type = ();

    const CONCRETE_CONST: core::integer::u32 = ();

    const GENERIC_CONST: core::integer::u32 = ();

    fn foo(t: core::integer::u32, v: core::felt252) -> core::integer::u32 {}

    fn bar(t: core::integer::u32) -> core::felt252 {}

    fn baz(s: hello::trait_module::SomeStructWithConstParameter::<1>) {}

    fn generic<const V: core::integer::u32, W, +Into<core::integer::u32, W>>(w: W) {}

    fn with_concrete_impl<W, impl SomeImpl: Into<core::integer::u32, W>>(w: W) -> W {}"
    At: Range { start: Position { line: 23, character: 0 }, end: Position { line: 23, character: 0 } }
    "#
    )
}
