use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn const_item_via_declaration() {
    test_transform!(rename, r#"
    const FO<caret>O: u32 = 42;
    "#, @"const RENAMED: u32 = 42;");
}

#[test]
fn const_item_via_expr() {
    test_transform!(rename, r#"
    const FOO: u32 = 42;
    fn main() { let _ = FO<caret>O; }
    "#, @r"
    const RENAMED: u32 = 42;
    fn main() { let _ = RENAMED; }
    ");
}

#[test]
fn const_item_via_other_const_expr() {
    test_transform!(rename, r#"
    const FOO: u32 = 42;
    const BAR: u32 = FO<caret>O * 2;
    "#, @r"
    const RENAMED: u32 = 42;
    const BAR: u32 = RENAMED * 2;
    ");
}

// FIXME(#589): Const usage should also be renamed here
#[test]
fn associated_const_via_trait_declaration() {
    test_transform!(rename, r#"
    trait Shape<T> { const SIDE<caret>S: u32; }

    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }

    fn main() {
        let tri = Triangle {};
        assert!(tri::SIDES == 3, 'lul');
    }
    "#, @r"
    trait Shape<T> { const RENAMED: u32; }

    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const RENAMED: u32 = 3; }

    fn main() {
        let tri = Triangle {};
        assert!(tri::SIDES == 3, 'lul');
    }
    ");
}

#[test]
fn associated_const_via_impl_definition() {
    test_transform!(rename, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDE<caret>S: u32 = 3; }
    "#, @r"
    trait Shape<T> { const RENAMED: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const RENAMED: u32 = 3; }
    ");
}

#[test]
fn associated_const_via_expr_use() {
    test_transform!(rename, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::SIDE<caret>S;
    }
    "#, @r"
    trait Shape<T> { const RENAMED: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const RENAMED: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::RENAMED;
    }
    ");
}
