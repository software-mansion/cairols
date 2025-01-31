use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn uninfered_mut_ident() {
    test_transform!(test_hover,r#"
    fn main() {
        let mut xy<caret>z = unknown_function();
    }
    "#,@r#"
    source_context = """
        let mut xy<caret>z = unknown_function();
    """
    highlight = """
        let mut <sel>xyz</sel> = unknown_function();
    """
    popover = """
    ```cairo
    let mut xyz: <missing>
    ```
    """
    "#)
}

#[test]
fn uninfered_value() {
    test_transform!(test_hover,r#"
    fn main() {
        let mut xyz = unkn<caret>own_function();
    }
    "#,@r#"
    source_context = """
        let mut xyz = unkn<caret>own_function();
    """
    highlight = """
        let mut xyz = <sel>unknown_function</sel>();
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn main()
    ```
    """
    "#)
}

#[test]
fn uninfered_usage() {
    test_transform!(test_hover,r#"
    fn main() {
        let mut xyz = unknown_function();
        let y = xy<caret>z * 2;
    }
    "#,@r#"
    source_context = """
        let y = xy<caret>z * 2;
    """
    highlight = """
        let y = <sel>xyz</sel> * 2;
    """
    popover = """
    ```cairo
    let mut xyz: <missing>
    ```
    """
    "#)
}

#[test]
fn missing_type_param() {
    test_transform!(test_hover,r#"
    fn f(ab<caret>c) -> felt252 {
        2 * abc
    }
    "#,@r#"
    source_context = """
    fn f(ab<caret>c) -> felt252 {
    """
    highlight = """
    fn f(<sel>abc</sel>) -> felt252 {
    """
    popover = """
    ```cairo
    abc: <missing>
    ```
    """
    "#)
}

#[test]
fn missing_type_param_usage() {
    test_transform!(test_hover,r#"
    fn f(abc) -> felt252 {
        2 * ab<caret>c
    }
    "#,@r#"
    source_context = """
        2 * ab<caret>c
    """
    highlight = """
        2 * <sel>abc</sel>
    """
    popover = """
    ```cairo
    abc: <missing>
    ```
    """
    "#)
}
