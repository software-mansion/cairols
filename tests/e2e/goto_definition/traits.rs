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
        fn foo(self: T);
    }
    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
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
        fn foo(self: T);
    }
    #[derive(Copy, Drop)]
    pub struct Bar {}
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
    }
    fn main() {
        let bar = Bar {};
        Foo::foo(bar);
    }
    ")
}

#[test]
fn self_referred_method_in_default_method_impl() {
    test_transform!(goto_definition, r"
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Self::inne<caret>r(x, x)
        }
    }
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }
    }
    ", @r"
    trait MyTrait<T> {
        fn <sel>inner</sel>(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Self::inner(x, x)
        }
    }
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }
    }
    ")
}

#[test]
fn self_reference_in_default_method_impl() {
    test_transform!(goto_definition, r"
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Se<caret>lf::inner(x, x)
        }
    }
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }
    }
    ", @r"
    trait <sel>MyTrait</sel><T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Self::inner(x, x)
        }
    }
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }
    }
    ")
}

#[test]
fn self_reference_in_method_impl() {
    test_transform!(goto_definition, r"
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T;
    }
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }

        fn foo<+Copy<T>>(x: T) -> T {
            Se<caret>lf::inner(x, x)
        }
    }
    ", @r"
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T;
    }
    impl <sel>MyImplU32</sel> of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }

        fn foo<+Copy<T>>(x: T) -> T {
            Self::inner(x, x)
        }
    }
    ")
}

#[test]
fn self_as_outside_impl() {
    test_transform!(goto_definition, r"
    fn bar<+MyTrait>() {}
    trait MyTrait {
        fn default_impl() {
            bar::<Se<caret>lf>();
        }
    }
    ", @r"
    fn bar<+MyTrait>() {}
    trait <sel>MyTrait</sel> {
        fn default_impl() {
            bar::<Self>();
        }
    }
    ")
}

#[test]
fn self_in_associated_impl_bounds() {
    test_transform!(goto_definition, r"
    trait Foorator<T> {}
    trait IntoFoorator<T> {
        type IntoFoor;
        impl Foorator: Foorator<Se<caret>lf::IntoFoor>;
    }
    ", @r"
    trait Foorator<T> {}
    trait <sel>IntoFoorator</sel><T> {
        type IntoFoor;
        impl Foorator: Foorator<Self::IntoFoor>;
    }
    ")
}

#[test]
fn self_in_return_type_position_in_trait_def() {
    test_transform!(goto_definition, r"
    pub trait NegateHelper<T> {
        type Result;
        fn negate(self: T) -> Se<caret>lf::Result;
    }
    ", @r"
    pub trait <sel>NegateHelper</sel><T> {
        type Result;
        fn negate(self: T) -> Self::Result;
    }
    ")
}

#[test]
fn self_referred_associated_type_in_method_return_type() {
    test_transform!(goto_definition, r"
    pub trait NegateHelper<T> {
        type Result;
        fn negate(self: T) -> Self::Resu<caret>lt;
    }
    ", @r"
    pub trait NegateHelper<T> {
        type <sel>Result</sel>;
        fn negate(self: T) -> Self::Result;
    }
    ")
}

#[test]
fn self_referred_associated_type_in_method_param_type() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        type Item;
        fn frobnicate(self: T, item: Self::Ite<caret>m);
    }
    ", @r"
    pub trait Foo<T> {
        type <sel>Item</sel>;
        fn frobnicate(self: T, item: Self::Item);
    }
    ")
}

// FIXME: https://github.com/software-mansion/cairols/issues/51
#[test]
fn self_in_method_bounds() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        type Item;
        fn last<+Destruct<T>, +Destruct<Se<caret>lf::Item>>(self: T);
    }
    ", @"none response")
}

// FIXME: https://github.com/software-mansion/cairols/issues/51
#[test]
fn self_referred_associated_type_in_method_bounds() {
    test_transform!(goto_definition, r"
    pub trait Foo<T> {
        type Item;
        fn last<+Destruct<T>, +Destruct<Self::Ite<caret>m>>(self: T);
    }
    ", @"none response")
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
