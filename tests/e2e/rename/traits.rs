use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn trait_via_definition() {
    test_transform!(rename, r#"
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
    pub trait RENAMED<T> {
        fn area(self: T) -> u64;
    }
    mod rectangle {
        use super::RENAMED;
        #[derive(Copy, Drop)]
        pub struct Rectangle {}
        impl RectangleGeometry of RENAMED<Rectangle> {
            fn area(self: Rectangle) -> u64 { 0 }
        }
    }
    use rectangle::Rectangle;
    fn main() {
        let rect = Rectangle {};
        let area = RENAMED::area(rect);
    }
    ")
}

#[test]
fn trait_method_via_definition() {
    test_transform!(rename, r#"
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
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
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
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(foo);
    }
    ")
}

#[test]
fn trait_method_via_dot_call() {
    test_transform!(rename, r#"
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
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
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
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(foo);
    }
    ")
}

#[test]
fn trait_method_via_path_call() {
    test_transform!(rename, r#"
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
        fn RENAMED(self: @Foo) -> u64;
    }
    impl FooImpl of FooTrait {
        fn RENAMED(self: @Foo) -> u64 { 0 }
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
        let x = foo.RENAMED();
        let y = FooTrait::RENAMED(foo);
    }
    ")
}

// FIXME: (#170) Usages should be renamed here as well
#[test]
fn impl_method_via_definition() {
    test_transform!(rename, r#"
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
        fn RENAMED(self: @Foo) -> u64 { 0 }
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
