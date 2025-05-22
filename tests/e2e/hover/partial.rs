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
    "#)
}
