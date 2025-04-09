use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn const_item_via_declaration() {
    test_transform!(test_hover, r#"
    const FO<caret>O: u32 = 42;
    "#, @r#"
    source_context = """
    const FO<caret>O: u32 = 42;
    """
    highlight = """
    const <sel>FOO</sel>: u32 = 42;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    const FOO: u32 = 42;
    ```
    """
    "#);
}

#[test]
fn const_item_via_expr() {
    test_transform!(test_hover, r#"
    const FOO: u32 = 42;
    fn main() { let _ = FO<caret>O; }
    "#, @r#"
    source_context = """
    fn main() { let _ = FO<caret>O; }
    """
    highlight = """
    fn main() { let _ = <sel>FOO</sel>; }
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    const FOO: u32 = 42;
    ```
    """
    "#);
}

#[test]
fn const_item_via_other_const_expr() {
    test_transform!(test_hover, r#"
    const FOO: u32 = 42;
    const BAR: u32 = FO<caret>O * 2;
    "#, @r#"
    source_context = """
    const BAR: u32 = FO<caret>O * 2;
    """
    highlight = """
    const BAR: u32 = <sel>FOO</sel> * 2;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    const FOO: u32 = 42;
    ```
    """
    "#);
}

#[test]
fn associated_const_via_trait_declaration() {
    test_transform!(test_hover, r#"
    trait Shape<T> { const SIDE<caret>S: u32; }
    "#, @r#"
    source_context = """
    trait Shape<T> { const SIDE<caret>S: u32; }
    """
    highlight = """
    trait Shape<T> { const <sel>SIDES</sel>: u32; }
    """
    popover = """
    ```cairo
    hello::Shape
    ```
    ```cairo
    trait Shape<T>
    const SIDES: u32;
    ```
    """
    "#);
}

#[test]
fn associated_const_via_impl_definition() {
    test_transform!(test_hover, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDE<caret>S: u32 = 3; }
    "#, @r#"
    source_context = """
    impl TriangleShape of Shape<Triangle> { const SIDE<caret>S: u32 = 3; }
    """
    highlight = """
    impl TriangleShape of Shape<Triangle> { const <sel>SIDES</sel>: u32 = 3; }
    """
    popover = """
    ```cairo
    hello::TriangleShape
    ```
    ```cairo
    impl TriangleShape of Shape<Triangle>;
    const SIDES: u32 = 3;
    ```
    """
    "#);
}

// FIXME: https://github.com/software-mansion/cairols/issues/405
//        https://github.com/software-mansion/cairols/issues/170
#[test]
fn associated_const_via_expr_use() {
    test_transform!(test_hover, r#"
    trait Shape<T> { const SIDES: u32; }
    struct Triangle {}
    impl TriangleShape of Shape<Triangle> { const SIDES: u32 = 3; }
    fn print_shape_info<T, impl ShapeImpl: Shape<T>>() {
        let _ = ShapeImpl::SIDE<caret>S;
    }
    "#, @r#"
    source_context = """
        let _ = ShapeImpl::SIDE<caret>S;
    """
    highlight = """
        let _ = ShapeImpl::<sel>SIDES</sel>;
    """
    popover = """
    ```cairo
    hello::Shape
    ```
    ```cairo
    trait Shape<T>
    const SIDES: u32;
    ```
    """
    "#);
}
