use lsp_types::Hover;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn generated_element_use() {
    test_transform_plain!(Hover, r#"
    #[generate_trait]
    impl MyTraitImpl<SelfType> of MyTrait<SelfType> {
        fn some_method(ref self: SelfType) { }
    }

    mod nested {
        use super::MyTr<caret>ait;
    }
    "#,@r#"
    source_context = """
        use super::MyTr<caret>ait;
    """
    highlight = """
        use super::<sel>MyTrait</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl<SelfType> of MyTrait<SelfType>;
    trait MyTrait<SelfType>
    ```
    """
    "#)
}

#[test]
fn generated_element_use_macro() {
    test_transform_with_macros!(Hover,r#"
    #[generate_trait]
    #[complex_attribute_macro_v2]
    impl MyTraitImpl<SelfType> of MyTrait<SelfType> {
        fn some_method(ref self: SelfType) { }
    }

    mod nested {
        use super::MyTr<caret>ait;
    }
    "#,@r#"
    source_context = """
        use super::MyTr<caret>ait;
    """
    highlight = """
        use super::<sel>MyTrait</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl<SelfType> of MyTrait<SelfType>;
    trait MyTrait<SelfType>
    ```
    """
    "#)
}

#[test]
fn ref_self() {
    test_transform_plain!(Hover, r#"
    #[generate_trait]
    impl MyTraitImpl<SelfType> of MyTrait<SelfType> {
        fn some_method(ref se<caret>lf: SelfType) { }
    }
    "#,@r#"
    source_context = """
        fn some_method(ref se<caret>lf: SelfType) { }
    """
    highlight = """
        fn some_method(ref <sel>self</sel>: SelfType) { }
    """
    popover = """
    ```cairo
    ref self: SelfType
    ```
    """
    "#)
}

#[test]
fn self_type() {
    test_transform_plain!(Hover, r#"
    struct SelfType {
        a: felt252,
        b: felt252,
    }

    #[generate_trait]
    impl MyTraitImpl of MyTrait {
        fn some_method(ref self: SelfT<caret>ype) { }
    }
    "#,@r#"
    source_context = """
        fn some_method(ref self: SelfT<caret>ype) { }
    """
    highlight = """
        fn some_method(ref self: <sel>SelfType</sel>) { }
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct SelfType {
        a: felt252,
        b: felt252,
    }
    ```
    """
    "#)
}

#[test]
fn self_type_member() {
    test_transform_plain!(Hover, r#"
    struct SelfType {
        aaa: felt252,
        b: felt252,
    }

    #[generate_trait]
    impl MyTraitImpl of MyTrait {
        fn some_method(ref self: SelfType) {
            self.a<caret>aa
        }
    }
    "#,@r#"
    source_context = """
            self.a<caret>aa
    """
    highlight = """
            self.<sel>aaa</sel>
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct SelfType {
        aaa: felt252,
        b: felt252,
    }
    ```
    """
    "#)
}

#[test]
fn self_type_member_method_call() {
    test_transform_plain!(Hover, r#"
    use core::num::traits::One;

    struct SelfType {
        aaa: felt252,
        b: felt252,
    }

    #[generate_trait]
    impl MyTraitImpl of MyTrait {
        fn some_method(ref self: SelfType) {
            self.aaa.is_o<caret>ne();
        }
    }
    "#,@r#"
    source_context = """
            self.aaa.is_o<caret>ne();
    """
    highlight = """
            self.aaa.<sel>is_one</sel>();
    """
    popover = """
    ```cairo
    core::felt_252::Felt252One
    ```
    ```cairo
    pub(crate) impl Felt252One of One<felt252>;
    fn is_one(self: felt252) -> bool
    ```
    """
    "#)
}

#[test]
fn trait_name_generated() {
    test_transform_plain!(Hover, r#"
    #[generate_trait]
    impl MyTraitImpl of MyTra<caret>it {
        fn some_method() { }
    }
    "#,@r#"
    source_context = """
    impl MyTraitImpl of MyTra<caret>it {
    """
    highlight = """
    impl MyTraitImpl of <sel>MyTrait</sel> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl of MyTrait;
    trait MyTrait
    ```
    """
    "#)
}

#[test]
fn trait_name_generic_name_generated() {
    test_transform_plain!(Hover, r#"
    #[generate_trait]
    impl MyTraitImpl<SelfType> of MyTrait<Self<caret>Type> {
        fn some_method() { }
    }
    "#,@r#"
    source_context = """
    impl MyTraitImpl<SelfType> of MyTrait<Self<caret>Type> {
    """
    highlight = """
    impl MyTraitImpl<SelfType> of MyTrait<<sel>SelfType</sel>> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    impl MyTraitImpl<SelfType> of MyTrait<SelfType>;
    ```
    """
    "#)
}

#[test]
fn trait_name_generic_name() {
    test_transform_plain!(Hover, r#"
    struct Ab {}

    trait MyTrait<SelfType> {
        fn some_method(ref self: SelfType);
    }

    impl MyTraitImpl of MyTrait<A<caret>b> {
        fn some_method(ref self: Ab) { }
    }
    "#,@r#"
    source_context = """
    impl MyTraitImpl of MyTrait<A<caret>b> {
    """
    highlight = """
    impl MyTraitImpl of MyTrait<<sel>Ab</sel>> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Ab {}
    ```
    """
    "#)
}

#[test]
fn self_in_trait() {
    test_transform_plain!(Hover, r#"
    pub trait ShapeGeometry<T> {
        type Unit;
        fn area(self: T) -> Se<caret>lf::Unit;
    }
    "#,@r#"
    source_context = """
        fn area(self: T) -> Se<caret>lf::Unit;
    """
    highlight = """
        fn area(self: T) -> <sel>Self</sel>::Unit;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    pub trait ShapeGeometry<T>
    ```
    """
    "#)
}

#[test]
fn self_in_impl() {
    test_transform_plain!(Hover, r#"
    pub trait ShapeGeometry<T> {
        type Unit;
        fn area(self: T) -> Self::Unit;
    }

    mod rectangle {
        use super::ShapeGeometry;
        #[derive(Copy, Drop)]
        pub struct Rectangle { pub a: u64, pub b: u64 }

        impl RectangleGeometry of ShapeGeometry<Rectangle> {
            type Unit = u64;
            fn area(self: Rectangle) -> Se<caret>lf::Unit {
                let retval: Self::Unit = self.a * self.b;
                retval
            }
        }
    }
    "#,@r#"
    source_context = """
            fn area(self: Rectangle) -> Se<caret>lf::Unit {
    """
    highlight = """
            fn area(self: Rectangle) -> <sel>Self</sel>::Unit {
    """
    popover = """
    ```cairo
    hello::rectangle
    ```
    ```cairo
    impl RectangleGeometry of ShapeGeometry<Rectangle>;
    ```
    """
    "#)
}

#[test]
fn type_bound() {
    test_transform_plain!(Hover, r#"
    fn foo<T, +Dr<caret>op<T>>() {}
    "#, @r#"
    source_context = """
    fn foo<T, +Dr<caret>op<T>>() {}
    """
    highlight = """
    fn foo<T, +<sel>Drop</sel><T>>() {}
    """
    popover = """
    ```cairo
    core::traits
    ```
    ```cairo
    pub trait Drop<T>
    ```
    ---
    A trait for types that can be safely dropped.
    Types implementing `Drop` can be automatically discarded when they go out of scope.
    The drop operation is a no-op - it simply indicates to the compiler that this type
    can be safely discarded.
    # Deriving

    This trait can be automatically derived using `#[derive(Drop)]`. All basic types
    implement `Drop` by default, except for `Felt252Dict`.
    # Examples

    Without `Drop`:
    ```cairo
    struct Point {
        x: u128,
        y: u128,
    }

    fn foo(p: Point) {} // Error: `p` cannot be dropped
    ```

    With `Drop`:
    ```cairo
    #[derive(Drop)]
    struct Point {
        x: u128,
        y: u128,
    }

    fn foo(p: Point) {} // OK: `p` is dropped at the end of the function
    ```"""
    "#)
}

#[test]
fn negative_type_bound() {
    test_transform_plain!(Hover, r#"
    trait Trait<T> {}
    impl Impl<T, -Dest<caret>ruct<T>> of Trait<T> {}
    "#, @r#"
    source_context = """
    impl Impl<T, -Dest<caret>ruct<T>> of Trait<T> {}
    """
    highlight = """
    impl Impl<T, -<sel>Destruct</sel><T>> of Trait<T> {}
    """
    popover = """
    ```cairo
    core::traits
    ```
    ```cairo
    pub trait Destruct<T>
    ```
    ---
    A trait that allows for custom destruction behavior of a type.
    In Cairo, values must be explicitly handled - they cannot be silently dropped.
    Types can only go out of scope in two ways:
    1. Implement `Drop` - for types that can be discarded trivially
    2. Implement `Destruct` - for types that need cleanup when destroyed. Typically, any type that
    contains
    a `Felt252Dict` must implement `Destruct`, as the `Felt252Dict` needs to be "squashed" when
    going
    out of scope to ensure a program is sound.

    Generally, `Destruct` does not need to be implemented manually. It can be derived from the
    `Drop` and `Destruct` implementations of the type's fields.
    # Examples

    Here's a simple type that wraps a `Felt252Dict` and needs to be destructed:
    ```cairo
    use core::dict::Felt252Dict;

    // A struct containing a Felt252Dict must implement Destruct
    #[derive(Destruct, Default)]
    struct ResourceManager {
        resources: Felt252Dict<u32>,
        count: u32,
    }

    #[generate_trait]
    impl ResourceManagerImpl of ResourceManagerTrait{
       fn add_resource(ref self: ResourceManager, resource_id: felt252, amount: u32){
           assert!(self.resources.get(resource_id) == 0, "Resource already exists");
           self.resources.insert(resource_id, amount);
           self.count += amount;
       }
    }

    let mut manager = Default::default();

    // Add some resources
    manager.add_resource(1, 100);

    // When manager goes out of scope here, Destruct is automatically called,
    // which ensures the dictionary is properly squashed
    ```"""
    "#)
}

#[test]
fn type_bound_user_trait() {
    test_transform_plain!(Hover, r#"
    /// Doc of Trait.
    trait Trait<T> {}
    fn foo<T, +Tr<caret>ait<T>>() {}
    "#, @r#"
    source_context = """
    fn foo<T, +Tr<caret>ait<T>>() {}
    """
    highlight = """
    fn foo<T, +<sel>Trait</sel><T>>() {}
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait Trait<T>
    ```
    ---
    Doc of Trait."""
    "#)
}

#[test]
fn impl_bound_user_trait() {
    test_transform_plain!(Hover, r#"
    /// Doc of Trait.
    trait Trait<T> {}
    fn foo<T, impl Impl: Tr<caret>ait<T>>() {}
    "#, @r#"
    source_context = """
    fn foo<T, impl Impl: Tr<caret>ait<T>>() {}
    """
    highlight = """
    fn foo<T, impl Impl: <sel>Trait</sel><T>>() {}
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    trait Trait<T>
    ```
    ---
    Doc of Trait."""
    "#)
}
