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
    <sel=declaration>pub trait <sel>ShapeGeometry</sel><T> {
        fn area(self: T) -> u64;
    }</sel>
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
        let x = foo.<sel>area</sel>();
        let y = FooTrait::<sel>area</sel>(foo);
    }
    ")
}

// FIXME(#170): Does not work as expected.
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
    <sel=declaration>impl <sel>FooImpl</sel> of FooTrait {
        fn area(self: @Foo) -> u64 { 0 }
    }</sel>
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
