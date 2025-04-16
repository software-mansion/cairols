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

// FIXME(#170): Function usages should be selected here as well
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
        let y = FooTrait::area(foo);
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
        let x = foo.area();
        let y = FooTrait::area(foo);
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
        let y_bar = FooTrait::area(bar);
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
        let y_bar = FooTrait::<sel>area</sel>(bar);
    }
    ")
}

// FIXME(#170)
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
    "#, @"none response")
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
