use lsp_types::Hover;

use crate::support::insta::test_transform_plain;

// Sanity check: function parameter hovers work correctly.
#[test]
fn function_param_def() {
    test_transform_plain!(Hover, r#"
    fn foo(ab<caret>c: felt252) -> felt252 { abc * 2 }
    "#, @r#"
    source_context = """
    fn foo(ab<caret>c: felt252) -> felt252 { abc * 2 }
    """
    highlight = """
    fn foo(<sel>abc</sel>: felt252) -> felt252 { abc * 2 }
    """
    popover = """
    ```cairo
    abc: felt252
    ```
    """
    "#)
}

#[test]
fn function_param_usage() {
    test_transform_plain!(Hover, r#"
    fn foo(abc: felt252) -> felt252 { ab<caret>c * 2 }
    "#, @r#"
    source_context = """
    fn foo(abc: felt252) -> felt252 { ab<caret>c * 2 }
    """
    highlight = """
    fn foo(abc: felt252) -> felt252 { <sel>abc</sel> * 2 }
    """
    popover = """
    ```cairo
    abc: felt252
    ```
    """
    "#)
}

// Regression tests for https://github.com/software-mansion/cairols/issues/120
// Closure parameters are not in LookupItemId, so hover falls back to the enclosing closure
// expression instead of the parameter itself.
#[test]
fn closure_param_def() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |ab<caret>c: felt252| abc * 2;
    }
    "#, @r#"
    source_context = """
        let closure = |ab<caret>c: felt252| abc * 2;
    """
    highlight = """
        let closure = |<sel>abc</sel>: felt252| abc * 2;
    """
    popover = """
    ```cairo
    abc: felt252
    ```
    """
    "#)
}

#[test]
fn closure_param_usage() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |abc: felt252| ab<caret>c * 2;
    }
    "#, @r#"
    source_context = """
        let closure = |abc: felt252| ab<caret>c * 2;
    """
    highlight = """
        let closure = |abc: felt252| <sel>abc</sel> * 2;
    """
    popover = """
    ```cairo
    abc: felt252
    ```
    """
    "#)
}
