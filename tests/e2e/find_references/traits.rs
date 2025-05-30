use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn trait_via_definition() {
    test_transform!(find_references, r#"
    pub trait ShapeGeometry<T> {
        fn area(self: T) -> u64;
    }
    mod rectangle {
        use super::ShapeGeometry;
        #[derive(Copy, Drop)]
        pub struct Rectangle {}
        impl RectangleGeometry of ShapeGeometry<Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }
    use rectangle::Rectangle;
    fn main() {
        let rect = Rectangle {};
        let area = ShapeGeo<caret>metry::area(rect);
    }
    "#, @r"
    pub trait <sel=declaration>ShapeGeometry</sel><T> {
        fn area(self: T) -> u64;
    }
    mod rectangle {
        use super::<sel>ShapeGeometry</sel>;
        #[derive(Copy, Drop)]
        pub struct Rectangle {}
        impl RectangleGeometry of <sel>ShapeGeometry</sel><Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }
    use rectangle::Rectangle;
    fn main() {
        let rect = Rectangle {};
        let area = <sel>ShapeGeometry</sel>::area(rect);
    }
    ")
}

#[test]
fn trait_method_via_definition() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn are<caret>a(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.area();
        let y = FooTrait::area(foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn <sel=declaration>area</sel>(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn <sel>area</sel>(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.<sel>area</sel>();
        let y = FooTrait::<sel>area</sel>(foo);
    }
    ")
}

#[test]
fn trait_method_via_impl_call() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn are<caret>a() -> u64;
    }
    impl FooImpl of FooTrait {
        fn area() -> u64 { 0 }
    }

    fn main() {
        let y = FooImpl::area();
        let z = FooImpl::area();
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn <sel=declaration>area</sel>() -> u64;
    }
    impl FooImpl of FooTrait {
        fn <sel>area</sel>() -> u64 { 0 }
    }

    fn main() {
        let y = FooImpl::<sel>area</sel>();
        let z = FooImpl::<sel>area</sel>();
    }
    ")
}

#[test]
fn trait_method_via_dot_call() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.are<caret>a();
        let y = FooTrait::area(foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn <sel=declaration>area</sel>(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn <sel>area</sel>(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.<sel>area</sel>();
        let y = FooTrait::<sel>area</sel>(foo);
    }
    ")
}

#[test]
fn trait_method_via_path_call() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.area();
        let y = FooTrait::are<caret>a(foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn <sel=declaration>area</sel>(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn <sel>area</sel>(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.<sel>area</sel>();
        let y = FooTrait::<sel>area</sel>(foo);
    }
    ")
}

#[test]
fn impl_method_via_definition() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn are<caret>a(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.area();
        let y = FooTrait::area(@foo);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait {
        fn area(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn <sel=declaration>area</sel>(self: @Foo) -> u64 { 0 }
    }
    #[derive(Drop)]
    struct Bar {}
    trait BarTrait {
        fn area(self: @Bar) -> u64;
    }
    impl BarImpl of BarTrait {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x = foo.<sel>area</sel>();
        let y = FooTrait::<sel>area</sel>(@foo);
    }
    ")
}

#[test]
fn dual_implementations_of_trait_via_trait_function() {
    test_transform!(find_references, r#"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait<T> {
        fn are<caret>a(self: @T) -> u64;
    }
    impl FooImpl of FooTrait<Foo> {
        fn area(self: @Foo) -> u64 { 0 }
    }

    #[derive(Drop)]
    struct Bar {}
    impl BarImpl of FooTrait<Bar> {
        fn area(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x_foo = foo.area();
        let y_foo = FooTrait::area(foo);

        let bar = Bar {};
        let x_bar = bar.area();
        let y_bar = FooTrait::area(@bar);
    }
    "#, @r"
    #[derive(Drop)]
    struct Foo {}
    trait FooTrait<T> {
        fn <sel=declaration>area</sel>(self: @T) -> u64;
    }
    impl FooImpl of FooTrait<Foo> {
        fn <sel>area</sel>(self: @Foo) -> u64 { 0 }
    }

    #[derive(Drop)]
    struct Bar {}
    impl BarImpl of FooTrait<Bar> {
        fn <sel>area</sel>(self: @Bar) -> u64 { 0 }
    }
    fn main() {
        let foo = Foo {};
        let x_foo = foo.<sel>area</sel>();
        let y_foo = FooTrait::<sel>area</sel>(foo);

        let bar = Bar {};
        let x_bar = bar.<sel>area</sel>();
        let y_bar = FooTrait::<sel>area</sel>(@bar);
    }
    ")
}

#[test]
fn dual_implementations_of_trait_via_trait_type() {
    test_transform!(find_references, r#"
    trait FooTrait {
        type Overrid<caret>eMe;
    }
    impl FooImpl of FooTrait {
        type OverrideMe = u256;
    }

    impl BarImpl of FooTrait {
        type OverrideMe = felt252;
    }
    fn main() {
        let v1: FooImpl::OverrideMe = 123;
        let v2: BarImpl::OverrideMe = 123;
    }
    "#, @r"
    trait FooTrait {
        type <sel=declaration>OverrideMe</sel>;
    }
    impl FooImpl of FooTrait {
        type <sel>OverrideMe</sel> = u256;
    }

    impl BarImpl of FooTrait {
        type <sel>OverrideMe</sel> = felt252;
    }
    fn main() {
        let v1: FooImpl::<sel>OverrideMe</sel> = 123;
        let v2: BarImpl::<sel>OverrideMe</sel> = 123;
    }
    ")
}

#[test]
fn dual_implementations_of_trait_via_trait_impl() {
    test_transform!(find_references, r#"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl ValueCa<caret>rrier: ConstCarryingTrait;
    }

    impl Carry123 of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl Carry456 of ConstCarryingTrait {
        const value: felt252 = 456;
    }

    impl Foo123 of FooTrait {
        impl ValueCarrier = Carry123;
    }

    impl Bar456 of FooTrait {
        impl ValueCarrier = Carry456;
    }
    "#, @r"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl <sel=declaration>ValueCarrier</sel>: ConstCarryingTrait;
    }

    impl Carry123 of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl Carry456 of ConstCarryingTrait {
        const value: felt252 = 456;
    }

    impl Foo123 of FooTrait {
        impl <sel>ValueCarrier</sel> = Carry123;
    }

    impl Bar456 of FooTrait {
        impl <sel>ValueCarrier</sel> = Carry456;
    }
    ")
}

#[test]
fn impl_impl_via_usage() {
    test_transform!(find_references, r#"
    trait Bush {
        fn bush() -> felt252;
    }

    trait BushTrait {
        impl BushImplementation: Bush;
    }

    impl GeorgeBush of Bush {
        fn bush() -> felt252 {
            911
        }
    }

    impl KateBush of Bush {
        fn bush() -> felt252 {
            1978
        }
    }

    impl GeorgeImpl of BushTrait {
        impl BushImplementation = GeorgeBush;
    }

    impl KateImpl of BushTrait {
        impl BushImplementation = KateBush;
    }


    fn main() {
        let v1 = GeorgeImpl::BushImplemen<caret>tation::bush();
        let v2 = GeorgeImpl::BushImplementation::bush();
        let v3 = KateImpl::BushImplementation::bush();
        let v4 = KateImpl::BushImplementation::bush();
    }
    "#, @r"
    trait Bush {
        fn bush() -> felt252;
    }

    trait BushTrait {
        impl BushImplementation: Bush;
    }

    impl GeorgeBush of Bush {
        fn bush() -> felt252 {
            911
        }
    }

    impl KateBush of Bush {
        fn bush() -> felt252 {
            1978
        }
    }

    impl GeorgeImpl of BushTrait {
        impl <sel=declaration>BushImplementation</sel> = GeorgeBush;
    }

    impl KateImpl of BushTrait {
        impl BushImplementation = KateBush;
    }


    fn main() {
        let v1 = GeorgeImpl::<sel>BushImplementation</sel>::bush();
        let v2 = GeorgeImpl::<sel>BushImplementation</sel>::bush();
        let v3 = KateImpl::BushImplementation::bush();
        let v4 = KateImpl::BushImplementation::bush();
    }
    ")
}

#[test]
fn impl_impl_via_definition() {
    test_transform!(find_references, r#"
    trait Bush {
        fn bush() -> felt252;
    }

    trait BushTrait {
        impl BushImplementation: Bush;
    }

    impl GeorgeBush of Bush {
        fn bush() -> felt252 {
            911
        }
    }

    impl KateBush of Bush {
        fn bush() -> felt252 {
            1978
        }
    }

    impl GeorgeImpl of BushTrait {
        impl BushImplementation = GeorgeBush;
    }

    impl KateImpl of BushTrait {
        impl BushImplem<caret>entation = KateBush;
    }


    fn main() {
        let v1 = GeorgeImpl::BushImplementation::bush();
        let v2 = GeorgeImpl::BushImplementation::bush();
        let v3 = KateImpl::BushImplementation::bush();
        let v4 = KateImpl::BushImplementation::bush();
    }
    "#, @r"
    trait Bush {
        fn bush() -> felt252;
    }

    trait BushTrait {
        impl BushImplementation: Bush;
    }

    impl GeorgeBush of Bush {
        fn bush() -> felt252 {
            911
        }
    }

    impl KateBush of Bush {
        fn bush() -> felt252 {
            1978
        }
    }

    impl GeorgeImpl of BushTrait {
        impl BushImplementation = GeorgeBush;
    }

    impl KateImpl of BushTrait {
        impl <sel=declaration>BushImplementation</sel> = KateBush;
    }


    fn main() {
        let v1 = GeorgeImpl::BushImplementation::bush();
        let v2 = GeorgeImpl::BushImplementation::bush();
        let v3 = KateImpl::<sel>BushImplementation</sel>::bush();
        let v4 = KateImpl::<sel>BushImplementation</sel>::bush();
    }
    ")
}

#[test]
fn impl_impl_via_trait() {
    test_transform!(find_references, r#"
    trait Bush {
        fn bush() -> felt252;
    }

    trait BushTrait {
        impl BushImple<caret>mentation: Bush;
    }

    impl GeorgeBush of Bush {
        fn bush() -> felt252 {
            911
        }
    }

    impl KateBush of Bush {
        fn bush() -> felt252 {
            1978
        }
    }

    impl GeorgeImpl of BushTrait {
        impl BushImplementation = GeorgeBush;
    }

    impl KateImpl of BushTrait {
        impl BushImplementation = KateBush;
    }

    fn main() {
        let v1 = GeorgeImpl::BushImplementation::bush();
        let v2 = GeorgeImpl::BushImplementation::bush();
        let v3 = KateImpl::BushImplementation::bush();
        let v4 = KateImpl::BushImplementation::bush();
    }
    "#, @r"
    trait Bush {
        fn bush() -> felt252;
    }

    trait BushTrait {
        impl <sel=declaration>BushImplementation</sel>: Bush;
    }

    impl GeorgeBush of Bush {
        fn bush() -> felt252 {
            911
        }
    }

    impl KateBush of Bush {
        fn bush() -> felt252 {
            1978
        }
    }

    impl GeorgeImpl of BushTrait {
        impl <sel>BushImplementation</sel> = GeorgeBush;
    }

    impl KateImpl of BushTrait {
        impl <sel>BushImplementation</sel> = KateBush;
    }

    fn main() {
        let v1 = GeorgeImpl::<sel>BushImplementation</sel>::bush();
        let v2 = GeorgeImpl::<sel>BushImplementation</sel>::bush();
        let v3 = KateImpl::<sel>BushImplementation</sel>::bush();
        let v4 = KateImpl::<sel>BushImplementation</sel>::bush();
    }
    ")
}

#[test]
fn associated_impl_member_const_usage() {
    test_transform!(find_references, r#"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl ValueCarrier: ConstCarryingTrait;
    }

    impl Carry123 of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl Carry456 of ConstCarryingTrait {
        const val<caret>ue: felt252 = 456;
    }

    impl Foo123 of FooTrait {
        impl ValueCarrier = Carry123;
    }

    impl Bar456 of FooTrait {
        impl ValueCarrier = Carry456;
    }

    fn main() {
        let _a = Bar456::ValueCarrier::value;
        let _b = Bar456::ValueCarrier::value;
    }
    "#, @r"
    trait ConstCarryingTrait {
        const value: felt252;
    }
    trait FooTrait {
        impl ValueCarrier: ConstCarryingTrait;
    }

    impl Carry123 of ConstCarryingTrait {
        const value: felt252 = 123;
    }
    impl Carry456 of ConstCarryingTrait {
        const <sel=declaration>value</sel>: felt252 = 456;
    }

    impl Foo123 of FooTrait {
        impl ValueCarrier = Carry123;
    }

    impl Bar456 of FooTrait {
        impl ValueCarrier = Carry456;
    }

    fn main() {
        let _a = Bar456::ValueCarrier::<sel>value</sel>;
        let _b = Bar456::ValueCarrier::<sel>value</sel>;
    }
    ")
}

#[test]
fn associated_impl_function_usage() {
    test_transform!(find_references, r#"
    trait FunctionImplementingTrait {
        fn function() -> felt252;
    }
    trait FooTrait {
        impl FunctionImplementer: FunctionImplementingTrait;
    }

    impl Func123 of FunctionImplementingTrait {
        fn function() -> felt252 {
            123
        }
    }
    impl Func456 of FunctionImplementingTrait {
        fn fun<caret>ction() -> felt252 {
            456
        }
    }

    impl Foo123 of FooTrait {
        impl FunctionImplementer = Func123;
    }

    impl Bar456 of FooTrait {
        impl FunctionImplementer = Func456;
    }

    fn main() {
        let _a = Bar456::FunctionImplementer::function();
        let _b = Bar456::FunctionImplementer::function();
    }
    "#, @r"
    trait FunctionImplementingTrait {
        fn function() -> felt252;
    }
    trait FooTrait {
        impl FunctionImplementer: FunctionImplementingTrait;
    }

    impl Func123 of FunctionImplementingTrait {
        fn function() -> felt252 {
            123
        }
    }
    impl Func456 of FunctionImplementingTrait {
        fn <sel=declaration>function</sel>() -> felt252 {
            456
        }
    }

    impl Foo123 of FooTrait {
        impl FunctionImplementer = Func123;
    }

    impl Bar456 of FooTrait {
        impl FunctionImplementer = Func456;
    }

    fn main() {
        let _a = Bar456::FunctionImplementer::<sel>function</sel>();
        let _b = Bar456::FunctionImplementer::<sel>function</sel>();
    }
    ")
}

#[test]
fn associated_impl_type_usage() {
    test_transform!(find_references, r#"
    trait TypeCarryingTrait {
        type Numeric;
    }
    trait FooTrait {
        impl TypeCarrier: TypeCarryingTrait;
    }

    impl CarryFelt252 of TypeCarryingTrait {
        type Numeric = felt252;
    }
    impl CarryU256 of TypeCarryingTrait {
        type Num<caret>eric = u256;
    }

    impl Foo123 of FooTrait {
        impl TypeCarrier = CarryFelt252;
    }

    impl Bar456 of FooTrait {
        impl TypeCarrier = CarryU256;
    }

    fn main() {
        let _a: Bar456::TypeCarrier::Numeric = 123;
        let _b: Bar456::TypeCarrier::Numeric = 123;
    }
    "#, @r"
    trait TypeCarryingTrait {
        type Numeric;
    }
    trait FooTrait {
        impl TypeCarrier: TypeCarryingTrait;
    }

    impl CarryFelt252 of TypeCarryingTrait {
        type Numeric = felt252;
    }
    impl CarryU256 of TypeCarryingTrait {
        type <sel=declaration>Numeric</sel> = u256;
    }

    impl Foo123 of FooTrait {
        impl TypeCarrier = CarryFelt252;
    }

    impl Bar456 of FooTrait {
        impl TypeCarrier = CarryU256;
    }

    fn main() {
        let _a: Bar456::TypeCarrier::<sel>Numeric</sel> = 123;
        let _b: Bar456::TypeCarrier::<sel>Numeric</sel> = 123;
    }
    ")
}

#[test]
fn type_bound() {
    test_transform!(find_references, r"
    fn foo<T, +Dro<caret>p<T>>() {}
    ", @r"
    // found several references in the core crate
    fn foo<T, +<sel>Drop</sel><T>>() {}
    ")
}

#[test]
fn negative_type_bound() {
    test_transform!(find_references, r"
    trait Trait<T> {}
    impl Impl<T, -Dro<caret>p<T>> of Trait<T> {}
    ", @r"
    // found several references in the core crate
    trait Trait<T> {}
    impl Impl<T, -<sel>Drop</sel><T>> of Trait<T> {}
    ")
}

#[test]
fn type_bound_user_trait() {
    test_transform!(find_references, r"
    trait Traicik<T> {}
    fn foo<T, +Traicik<caret><T>>() {}
    ", @r"
    trait <sel=declaration>Traicik</sel><T> {}
    fn foo<T, +<sel>Traicik</sel><T>>() {}
    ")
}

#[test]
fn impl_bound() {
    test_transform!(find_references, r"
    fn foo<T, impl Impl: Dro<caret>p<T>>() {}
    ", @r"
    // found several references in the core crate
    fn foo<T, impl Impl: <sel>Drop</sel><T>>() {}
    ")
}
