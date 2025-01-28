use crate::goto_definition::goto_definition;
use crate::support::insta::test_transform;

#[test]
fn trait_name_in_impl() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    pub struct Bar {}
    impl FooBar of Fo<caret>o<Bar> {
        fn foo(self: Bar) {}
    }
    ", @r"
    <sel>pub trait Foo<T> {
        fn foo(self: T);
    }</sel>
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    ")
}

#[test]
fn full_path_trait_name_in_expr() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    fn main() {
        let bar = Bar {};
        Fo<caret>o::foo(bar);
    }
    ", @r"
    <sel>pub trait Foo<T> {
        fn foo(self: T);
    }</sel>
    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    fn main() {
        let bar = Bar {};
        Foo::foo(bar);
    }
    ")
}

#[test]
fn method_in_impl() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn fo<caret>o(self: Bar) {}
    }
    ", @r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    pub struct Bar {}
    <sel>impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }</sel>
    ")
}

#[test]
fn dot_method_in_expr() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    fn main() {
        let bar = Bar {};
        bar.fo<caret>o();
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
    fn main() {
        let bar = Bar {};
        bar.foo();
    }
    ")
}

#[test]
fn full_path_method_in_expr() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        fn foo(self: T);
    }
    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    fn main() {
        let bar = Bar {};
        Foo::fo<caret>o(bar);
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
    fn main() {
        let bar = Bar {};
        Foo::foo(bar);
    }
    ")
}
