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

// FIXME(#404)
#[test]
fn associated_const_via_trait_declaration() {
    test_transform!(find_references, r#"
    trait Shape<T> { const SIDE<caret>S: u32; }
    "#, @"none response");
}

// FIXME(#404)
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

// FIXME(#405)
#[test]
fn associated_const_via_expr_use() {
    test_transform!(find_references, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::SIDE<caret>S;
    }
    "#, @"none response");
}
