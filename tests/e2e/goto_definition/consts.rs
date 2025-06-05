use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn const_item_via_declaration() {
    test_transform_plain!(GotoDefinition, r#"
    const FO<caret>O: u32 = 42;
    "#, @r"
    const <sel>FOO</sel>: u32 = 42;
    ");
}

#[test]
fn const_item_via_expr() {
    test_transform_plain!(GotoDefinition, r#"
    const FOO: u32 = 42;
    fn main() { let _ = FO<caret>O; }
    "#, @r"
    const <sel>FOO</sel>: u32 = 42;
    fn main() { let _ = FOO; }
    ");
}

#[test]
fn const_item_via_other_const_expr() {
    test_transform_plain!(GotoDefinition, r#"
    const FOO: u32 = 42;
    const BAR: u32 = FO<caret>O * 2;
    "#, @r"
    const <sel>FOO</sel>: u32 = 42;
    const BAR: u32 = FOO * 2;
    ");
}

#[test]
fn associated_const_via_trait_declaration() {
    test_transform_plain!(GotoDefinition, r#"
    trait Shape<T> { const SIDE<caret>S: u32; }
    "#, @"trait Shape<T> { const <sel>SIDES</sel>: u32; }");
}

#[test]
fn associated_const_via_expr_use() {
    test_transform_plain!(GotoDefinition, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::SIDE<caret>S;
    }
    "#, @r"
    trait Shape<T> { const <sel>SIDES</sel>: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::SIDES;
    }
    ");
}

#[test]
fn const_item_via_expr_with_macros() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    const FOO: u32 = 42;
    #[complex_attribute_macro_v2]
    fn main() { let _ = FO<caret>O; }
    "#, @r"
    #[complex_attribute_macro_v2]
    const <sel>FOO</sel>: u32 = 42;
    #[complex_attribute_macro_v2]
    fn main() { let _ = FOO; }
    ");
}
