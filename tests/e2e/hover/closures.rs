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

#[test]
fn closure_param_inferred_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |ab<caret>c| abc * 2;
    }
    "#, @r#"
    source_context = """
        let closure = |ab<caret>c| abc * 2;
    """
    highlight = """
        let closure = |<sel>abc</sel>| abc * 2;
    """
    popover = """
    ```cairo
    abc: felt252
    ```
    """
    "#)
}

#[test]
fn closure_param_inferred_type_usage() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |abc| ab<caret>c * 2;
    }
    "#, @r#"
    source_context = """
        let closure = |abc| ab<caret>c * 2;
    """
    highlight = """
        let closure = |abc| <sel>abc</sel> * 2;
    """
    popover = """
    ```cairo
    abc: felt252
    ```
    """
    "#)
}

#[test]
fn closure_multi_param() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |a: felt252, b<caret>: felt252| a + b;
    }
    "#, @r#"
    source_context = """
        let closure = |a: felt252, b<caret>: felt252| a + b;
    """
    highlight = """
        let closure = |a: felt252, <sel>b</sel>: felt252| a + b;
    """
    popover = """
    ```cairo
    b: felt252
    ```
    """
    "#)
}

#[test]
fn closure_multi_param_usage() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |a: felt252, b: felt252| a + b<caret>;
    }
    "#, @r#"
    source_context = """
        let closure = |a: felt252, b: felt252| a + b<caret>;
    """
    highlight = """
        let closure = |a: felt252, b: felt252| a + <sel>b</sel>;
    """
    popover = """
    ```cairo
    b: felt252
    ```
    """
    "#)
}

#[test]
fn closure_nested_inner_param() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _ = |a: felt252| |b<caret>: felt252| a + b;
    }
    "#, @r#"
    source_context = """
        let _ = |a: felt252| |b<caret>: felt252| a + b;
    """
    highlight = """
        let _ = |a: felt252| |<sel>b</sel>: felt252| a + b;
    """
    popover = """
    ```cairo
    b: felt252
    ```
    """
    "#)
}

#[test]
fn closure_nested_inner_param_usage() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _ = |a: felt252| |b: felt252| a + b<caret>;
    }
    "#, @r#"
    source_context = """
        let _ = |a: felt252| |b: felt252| a + b<caret>;
    """
    highlight = """
        let _ = |a: felt252| |b: felt252| a + <sel>b</sel>;
    """
    popover = """
    ```cairo
    b: felt252
    ```
    """
    "#)
}

#[test]
fn closure_param_u32_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |ab<caret>c: u32| abc + 1;
    }
    "#, @r#"
    source_context = """
        let closure = |ab<caret>c: u32| abc + 1;
    """
    highlight = """
        let closure = |<sel>abc</sel>: u32| abc + 1;
    """
    popover = """
    ```cairo
    abc: u32
    ```
    """
    "#)
}

#[test]
fn closure_param_bool_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |ab<caret>c: bool| abc;
    }
    "#, @r#"
    source_context = """
        let closure = |ab<caret>c: bool| abc;
    """
    highlight = """
        let closure = |<sel>abc</sel>: bool| abc;
    """
    popover = """
    ```cairo
    abc: bool
    ```
    """
    "#)
}

#[test]
fn closure_param_snapshot_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let closure = |ab<caret>c: @felt252| *abc + 1;
    }
    "#, @r#"
    source_context = """
        let closure = |ab<caret>c: @felt252| *abc + 1;
    """
    highlight = """
        let closure = |<sel>abc</sel>: @felt252| *abc + 1;
    """
    popover = """
    ```cairo
    abc: @felt252
    ```
    """
    "#)
}

#[test]
fn closure_param_struct_type() {
    test_transform_plain!(Hover, r#"
    #[derive(Drop)]
    struct Point {
        x: felt252,
    }
    fn main() {
        let closure = |p<caret>oint: Point| point.x;
    }
    "#, @r#"
    source_context = """
        let closure = |p<caret>oint: Point| point.x;
    """
    highlight = """
        let closure = |<sel>point</sel>: Point| point.x;
    """
    popover = """
    ```cairo
    point: Point
    ```
    """
    "#)
}
