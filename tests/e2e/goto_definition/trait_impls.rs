use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn trait_method() {
    test_transform_plain!(GotoDefinition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn f<caret>oo(self: Bar) {}
    }
    ", @r"
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
    }
    ")
}

#[test]
fn trait_const() {
    test_transform_plain!(GotoDefinition, r"
    pub trait Foo<T> {
        const VERY_IMPORTANT_CONST: T;
    }

    impl FooBar of Foo<felt252> {
        const VERY_IMP<caret>ORTANT_CONST: felt252 = 213;
    }
    ", @r"
    pub trait Foo<T> {
        const VERY_IMPORTANT_CONST: T;
    }

    impl FooBar of Foo<felt252> {
        const <sel>VERY_IMPORTANT_CONST</sel>: felt252 = 213;
    }
    ")
}

#[test]
fn trait_types() {
    test_transform_plain!(GotoDefinition, r"
    pub trait Foo {
        type VERY_IMPORTANT_TYPE;
    }

    impl FooBar of Foo {
        type VERY_IMPORT<caret>ANT_TYPE = felt252;
    }
    ", @r"
    pub trait Foo {
        type VERY_IMPORTANT_TYPE;
    }

    impl FooBar of Foo {
        type <sel>VERY_IMPORTANT_TYPE</sel> = felt252;
    }
    ")
}

#[test]
fn trait_impls() {
    test_transform_plain!(GotoDefinition, r"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl VeryImportantImpl: ConstCarryingTrait;
    }

    impl ConstCarryingTraitOneTwoThree of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl FooImpl of FooTrait {
        impl VeryIm<caret>portantImpl = ConstCarryingTraitOneTwoThree;
    }
    ", @r"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl VeryImportantImpl: ConstCarryingTrait;
    }

    impl ConstCarryingTraitOneTwoThree of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl FooImpl of FooTrait {
        impl <sel>VeryImportantImpl</sel> = ConstCarryingTraitOneTwoThree;
    }
    ")
}

#[test]
fn trait_impls_via_usage() {
    test_transform_plain!(GotoDefinition, r"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl VeryImportantImpl: ConstCarryingTrait;
    }

    impl ConstCarryingTraitOneTwoThree of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl FooImpl of FooTrait {
        impl VeryImportantImpl = ConstCarryingTraitOneTwoThree;
    }

    fn foo() {
        let _v = FooImpl::VeryImportan<caret>tImpl::value;
    }
    ", @r"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl VeryImportantImpl: ConstCarryingTrait;
    }

    impl ConstCarryingTraitOneTwoThree of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl FooImpl of FooTrait {
        impl <sel>VeryImportantImpl</sel> = ConstCarryingTraitOneTwoThree;
    }

    fn foo() {
        let _v = FooImpl::VeryImportantImpl::value;
    }
    ")
}

#[test]
fn impl_fn_usage_via_struct() {
    test_transform_plain!(GotoDefinition, r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }

    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }

    fn main() {
        let foo = Foo {};
        let x = foo.a<caret>rea();
    }
    ", @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }

    impl FooImpl of FooTrait {
        fn <sel>area</sel>(self: @Foo) -> u64 { 0 }
    }

    fn main() {
        let foo = Foo {};
        let x = foo.area();
    }
    ")
}

#[test]
fn impl_fn_usage_via_trait() {
    test_transform_plain!(GotoDefinition, r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }

    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }

    fn main() {
        let foo = Foo {};
        let y = FooTrait::ar<caret>ea(@foo);
    }
    ", @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }

    impl FooImpl of FooTrait {
        fn <sel>area</sel>(self: @Foo) -> u64 { 0 }
    }

    fn main() {
        let foo = Foo {};
        let y = FooTrait::area(@foo);
    }
    ")
}

#[test]
fn trait_method_with_macros() {
    test_transform_with_macros!(GotoDefinition, r"
    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[derive(Copy, Drop)]
    pub struct Bar {}

    #[complex_attribute_macro_v2]
    impl FooBar of Foo<Bar> {
        fn f<caret>oo(self: Bar) {}
    }
    ", @r"
    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[derive(Copy, Drop)]
    pub struct Bar {}

    #[complex_attribute_macro_v2]
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
    }
    ")
}
