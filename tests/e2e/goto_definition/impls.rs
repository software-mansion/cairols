use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn trait_method() {
    test_transform!(goto_definition, r"
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
        fn <sel>foo</sel>(self: T);
    }

    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    ")
}

#[test]
fn trait_const() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        const VERY_IMPORTANT_CONST: T;
    }

    impl FooBar of Foo<felt252> {
        const VERY_IMP<caret>ORTANT_CONST: felt252 = 213;
    }
    ", @r"
    pub trait Foo<T> {
        const <sel>VERY_IMPORTANT_CONST</sel>: T;
    }

    impl FooBar of Foo<felt252> {
        const VERY_IMPORTANT_CONST: felt252 = 213;
    }
    ")
}

#[test]
fn trait_types() {
    test_transform!(goto_definition, r"
    pub trait Foo {
        type VERY_IMPORTANT_TYPE;
    }

    impl FooBar of Foo {
        type VERY_IMPORT<caret>ANT_TYPE = felt252;
    }
    ", @r"
    pub trait Foo {
        type <sel>VERY_IMPORTANT_TYPE</sel>;
    }

    impl FooBar of Foo {
        type VERY_IMPORTANT_TYPE = felt252;
    }
    ")
}

#[test]
fn trait_impls() {
    test_transform!(goto_definition, r"
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
        impl <sel>VeryImportantImpl</sel>: ConstCarryingTrait;
    }

    impl ConstCarryingTraitOneTwoThree of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl FooImpl of FooTrait {
        impl VeryImportantImpl = ConstCarryingTraitOneTwoThree;
    }
    ")
}
