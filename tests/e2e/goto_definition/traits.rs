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
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }
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
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
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
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
    }
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

#[test]
fn starknet_interface_dispatcher() {
    test_transform!(goto_definition, r"
    mod interface {
        #[starknet::interface]
        pub trait Foo<T> { }
    }

    use interface::FooDispa<caret>tcher;
    ", @r"
    mod interface {
        <sel>#[starknet::interface]</sel>
        pub trait Foo<T> { }
    }

    use interface::FooDispatcher;
    ")
}

#[test]
fn generate_trait() {
    test_transform!(goto_definition, r"
    mod interface {
        #[generate_trait]
        pub impl FooImpl of Foo { }
    }

    use interface::Fo<caret>o;
    ", @r"
    mod interface {
        #[generate_trait]
        pub impl FooImpl of <sel>Foo</sel> { }
    }

    use interface::Foo;
    ")
}
