use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_and_macros;

#[test]
fn trait_name_in_impl() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        fn foo(self: T);
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    pub struct Bar {}

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    pub struct Bar {}

    #[complex_attribute_macro_v2]
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }
    ")
}

#[test]
fn full_path_trait_name_in_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        fn foo(self: T);
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    #[derive(Copy, Drop)]
    pub struct Bar {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    pub trait <sel>Foo</sel><T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    #[derive(Copy, Drop)]
    pub struct Bar {}

    #[complex_attribute_macro_v2]
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }

    #[complex_attribute_macro_v2]
    fn main() {
        let bar = Bar {};
        Foo::foo(bar);
    }
    ")
}

#[test]
fn dot_method_in_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        fn foo(self: T);
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    #[derive(Copy, Drop)]
    pub struct Bar {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    #[derive(Copy, Drop)]
    pub struct Bar {}

    #[complex_attribute_macro_v2]
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
    }

    #[complex_attribute_macro_v2]
    fn main() {
        let bar = Bar {};
        bar.foo();
    }
    ")
}

#[test]
fn full_path_method_in_expr() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        fn foo(self: T);
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    #[derive(Copy, Drop)]
    pub struct Bar {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl FooBar of Foo<Bar> {
        fn foo(self: Bar) {}
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        fn foo(self: T);
    }

    #[complex_attribute_macro_v2]
    #[derive(Copy, Drop)]
    pub struct Bar {}

    #[complex_attribute_macro_v2]
    impl FooBar of Foo<Bar> {
        fn <sel>foo</sel>(self: Bar) {}
    }

    #[complex_attribute_macro_v2]
    fn main() {
        let bar = Bar {};
        Foo::foo(bar);
    }
    ")
}

#[test]
fn self_referred_method_in_default_method_impl() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Self::inne<caret>r(x, x)
        }
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    trait MyTrait<T> {
        fn <sel>inner</sel>(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Self::inner(x, x)
        }
    }

    #[complex_attribute_macro_v2]
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }
    }
    ")
}

#[test]
fn self_reference_in_default_method_impl() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Se<caret>lf::inner(x, x)
        }
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    trait <sel>MyTrait</sel><T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T {
            Self::inner(x, x)
        }
    }

    #[complex_attribute_macro_v2]
    impl MyImplU32 of MyTrait<u32> {
        fn inner(x: u32, y: u32) -> u32 { x + y }
    }
    ")
}

#[test]
fn self_reference_in_method_impl() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    trait MyTrait<T> {
        fn inner(x: T, y: T) -> T;
        fn foo<+Copy<T>>(x: T) -> T;
    }

    #[complex_attribute_macro_v2]
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
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn bar<+MyTrait>() {}

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    fn bar<+MyTrait>() {}

    #[complex_attribute_macro_v2]
    trait <sel>MyTrait</sel> {
        fn default_impl() {
            bar::<Self>();
        }
    }
    ")
}

#[test]
fn self_in_associated_impl_bounds() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Foorator<T> {}

    <macro>#[complex_attribute_macro_v2]</macro>
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

    ==============================

    #[complex_attribute_macro_v2]
    trait Foorator<T> {}

    #[complex_attribute_macro_v2]
    trait <sel>IntoFoorator</sel><T> {
        type IntoFoor;
        impl Foorator: Foorator<Self::IntoFoor>;
    }
    ")
}

#[test]
fn self_in_return_type_position_in_trait_def() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait NegateHelper<T> {
        type Result;
        fn negate(self: T) -> Se<caret>lf::Result;
    }
    ", @r"
    pub trait <sel>NegateHelper</sel><T> {
        type Result;
        fn negate(self: T) -> Self::Result;
    }

    ==============================

    #[complex_attribute_macro_v2]
    pub trait <sel>NegateHelper</sel><T> {
        type Result;
        fn negate(self: T) -> Self::Result;
    }
    ")
}

#[test]
fn self_referred_associated_type_in_method_return_type() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait NegateHelper<T> {
        type Result;
        fn negate(self: T) -> Self::Resu<caret>lt;
    }
    ", @r"
    pub trait NegateHelper<T> {
        type <sel>Result</sel>;
        fn negate(self: T) -> Self::Result;
    }

    ==============================

    #[complex_attribute_macro_v2]
    pub trait NegateHelper<T> {
        type <sel>Result</sel>;
        fn negate(self: T) -> Self::Result;
    }
    ")
}

#[test]
fn self_referred_associated_type_in_method_param_type() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        type Item;
        fn frobnicate(self: T, item: Self::Ite<caret>m);
    }
    ", @r"
    pub trait Foo<T> {
        type <sel>Item</sel>;
        fn frobnicate(self: T, item: Self::Item);
    }

    ==============================

    #[complex_attribute_macro_v2]
    pub trait Foo<T> {
        type <sel>Item</sel>;
        fn frobnicate(self: T, item: Self::Item);
    }
    ")
}

// FIXME: https://github.com/software-mansion/cairols/issues/51
#[test]
fn self_in_method_bounds() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        type Item;
        fn last<+Destruct<T>, +Destruct<Se<caret>lf::Item>>(self: T);
    }
    ", @r"
    none response

    ==============================

    none response
    ")
}

// FIXME: https://github.com/software-mansion/cairols/issues/51
#[test]
fn self_referred_associated_type_in_method_bounds() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    pub trait Foo<T> {
        type Item;
        fn last<+Destruct<T>, +Destruct<Self::Ite<caret>m>>(self: T);
    }
    ", @r"
    none response

    ==============================

    none response
    ")
}

// FIXME: modzik
#[test]
fn starknet_interface_dispatcher() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    mod interface {
        #[starknet::interface]
        pub trait Foo<T> { }
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    use interface::FooDispa<caret>tcher;
    ", @r"
    mod interface {
        <sel>#[starknet::interface]</sel>
        pub trait Foo<T> { }
    }

    use interface::FooDispatcher;

    ==============================

    none response
    ")
}

#[test]
fn generate_trait() {
    test_transform_and_macros!(GotoDefinition, r"
    <macro>#[complex_attribute_macro_v2]</macro>
    mod interface {
        #[generate_trait]
        pub impl FooImpl of Foo { }
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    use interface::Fo<caret>o;
    ", @r"
    mod interface {
        #[generate_trait]
        pub impl FooImpl of <sel>Foo</sel> { }
    }

    use interface::Foo;

    ==============================

    #[complex_attribute_macro_v2]
    mod interface {
        #[generate_trait]
        pub impl FooImpl of <sel>Foo</sel> { }
    }

    #[complex_attribute_macro_v2]
    use interface::Foo;
    ")
}
