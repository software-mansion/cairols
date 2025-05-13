use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn const_item_via_declaration() {
    test_transform!(find_references, r#"
    const FO<caret>O: u32 = 42;
    "#, @"const <sel=declaration>FOO</sel>: u32 = 42;");
}

#[test]
fn const_item_via_expr() {
    test_transform!(find_references, r#"
    const FOO: u32 = 42;
    fn main() { let _ = FO<caret>O; }
    "#, @r"
    const <sel=declaration>FOO</sel>: u32 = 42;
    fn main() { let _ = <sel>FOO</sel>; }
    ");
}

#[test]
fn const_item_via_other_const_expr() {
    test_transform!(find_references, r#"
    const FOO: u32 = 42;
    const BAR: u32 = FO<caret>O * 2;
    "#, @r"
    const <sel=declaration>FOO</sel>: u32 = 42;
    const BAR: u32 = <sel>FOO</sel> * 2;
    ");
}

#[test]
fn associated_const_via_trait_declaration() {
    test_transform!(find_references, r#"
    trait Shape<T> { const SIDE<caret>S: u32; }
    "#, @"trait Shape<T> { const <sel=declaration>SIDES</sel>: u32; }");
}

#[test]
fn associated_const_via_impl_definition() {
    test_transform!(find_references, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDE<caret>S: u32 = 3; }
    "#, @r"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const <sel=declaration>SIDES</sel>: u32 = 3; }
    ");
}

#[test]
fn associated_trait_const_via_usages() {
    test_transform!(find_references, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }

    fn main() {
        let a = TriangleShape::SI<caret>DES == 1;
        let b = TriangleShape::SIDES == 2;
        let c = TriangleShape::SIDES == 3;
    }
    "#, @r"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const <sel=declaration>SIDES</sel>: u32 = 3; }

    fn main() {
        let a = TriangleShape::<sel>SIDES</sel> == 1;
        let b = TriangleShape::<sel>SIDES</sel> == 2;
        let c = TriangleShape::<sel>SIDES</sel> == 3;
    }
    ");
}

#[test]
fn associated_module_const_via_usages() {
    test_transform!(find_references, r#"
    mod TriangleData {
        pub const SIDES: felt252 = 3;
    }

    fn main() {
        let a = TriangleData::SI<caret>DES == 1;
        let b = TriangleData::SIDES == 2;
        let c = TriangleData::SIDES == 3;
    }
    "#, @r"
    mod TriangleData {
        pub const <sel=declaration>SIDES</sel>: felt252 = 3;
    }

    fn main() {
        let a = TriangleData::<sel>SIDES</sel> == 1;
        let b = TriangleData::<sel>SIDES</sel> == 2;
        let c = TriangleData::<sel>SIDES</sel> == 3;
    }
    ");
}

#[test]
fn associated_const_via_expr_use() {
    test_transform!(find_references, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::SIDE<caret>S;
    }
    "#, @r"
    trait Shape<T> { const <sel=declaration>SIDES</sel>: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const <sel>SIDES</sel>: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::<sel>SIDES</sel>;
    }
    ");
}

#[test]
fn dual_trait_const_via_trait_function() {
    test_transform!(find_references, r#"
    trait FooTrait<T> {
        const FOO_CONST<caret>ANT: felt252;
    }
    impl FooImpl of FooTrait<felt252> {
        const FOO_CONSTANT: felt252 = 123;
    }

    impl BarImpl of FooTrait<u256> {
        const FOO_CONSTANT: u256 = 123_u256;
    }

    fn main() {
        let foo: felt252 = FooImpl::FOO_CONSTANT;
        let bar: u256 = BarImpl::FOO_CONSTANT;
    }
    "#, @r"
    trait FooTrait<T> {
        const <sel=declaration>FOO_CONSTANT</sel>: felt252;
    }
    impl FooImpl of FooTrait<felt252> {
        const <sel>FOO_CONSTANT</sel>: felt252 = 123;
    }

    impl BarImpl of FooTrait<u256> {
        const <sel>FOO_CONSTANT</sel>: u256 = 123_u256;
    }

    fn main() {
        let foo: felt252 = FooImpl::<sel>FOO_CONSTANT</sel>;
        let bar: u256 = BarImpl::<sel>FOO_CONSTANT</sel>;
    }
    ")
}
