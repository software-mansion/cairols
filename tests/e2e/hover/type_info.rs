use lsp_types::Hover;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

/// Hover on binary `+` operator shows type of the binary expression.
#[test]
fn binary_plus_felt252() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _ = 1 <caret>+ 2;
    }
    "#, @r#"
    source_context = """
        let _ = 1 <caret>+ 2;
    """
    highlight = """
        let _ = <sel>1 + 2</sel>;
    """
    popover = """
    ```cairo
    felt252
    ```
    """
    "#)
}

/// Hover on binary `*` operator shows type of the binary expression.
#[test]
fn binary_mul_u32() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _ = 1_u32 <caret>* 2_u32;
    }
    "#, @r#"
    source_context = """
        let _ = 1_u32 <caret>* 2_u32;
    """
    highlight = """
        let _ = <sel>1_u32 * 2_u32</sel>;
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}

/// Hover on closing paren of a function call shows the return type.
#[test]
fn function_call_return_type() {
    test_transform_plain!(Hover, r#"
    fn get_num() -> u32 {
        1_u32
    }

    fn main() {
        let _ = get_num(<caret>);
    }
    "#, @r#"
    source_context = """
        let _ = get_num(<caret>);
    """
    highlight = """
        let _ = <sel>get_num()</sel>;
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}

/// Hover on unary `!` operator shows the type of the unary expression.
#[test]
fn unary_bool_not() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let x: bool = true;
        let _ = <caret>!x;
    }
    "#, @r#"
    source_context = """
        let _ = <caret>!x;
    """
    highlight = """
        let _ = <sel>!x</sel>;
    """
    popover = """
    ```cairo
    bool
    ```
    """
    "#)
}

/// Hover on unary `-` operator shows the type of the negated expression.
#[test]
fn unary_minus_i32() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let x: i32 = 5_i32;
        let _ = <caret>-x;
    }
    "#, @r#"
    source_context = """
        let _ = <caret>-x;
    """
    highlight = """
        let _ = <sel>-x</sel>;
    """
    popover = """
    ```cairo
    i32
    ```
    """
    "#)
}

/// Hover on opening paren of a tuple shows the type of the whole tuple.
#[test]
fn tuple_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _ = <caret>(1_u32, 2_u64);
    }
    "#, @r#"
    source_context = """
        let _ = <caret>(1_u32, 2_u64);
    """
    highlight = """
        let _ = <sel>(1_u32, 2_u64)</sel>;
    """
    popover = """
    ```cairo
    (u32, u64)
    ```
    """
    "#)
}

/// Hover on struct constructor brace shows the full type of the constructed struct.
#[test]
fn struct_ctor_type() {
    test_transform_plain!(Hover, r#"
    struct Foo {
        x: u32,
    }

    fn main() {
        let _ = Foo <caret>{ x: 1_u32 };
    }
    "#, @r#"
    source_context = """
        let _ = Foo <caret>{ x: 1_u32 };
    """
    highlight = """
        let _ = <sel>Foo { x: 1_u32 }</sel>;
    """
    popover = """
    ```cairo
    struct Foo {
        x: u32,
    }
    ```
    """
    "#)
}

/// Hover on a block expression that returns a user-defined struct shows the full struct type.
#[test]
fn block_expr_returning_struct_type() {
    test_transform_plain!(Hover, r#"
    struct Foo {
        x: u32,
    }

    fn make_foo() -> Foo {
        Foo { x: 1_u32 }
    }

    fn main() {
        let _ = if true <caret>{ make_foo() } else { make_foo() };
    }
    "#, @r#"
    source_context = """
        let _ = if true <caret>{ make_foo() } else { make_foo() };
    """
    highlight = """
        let _ = if true <sel>{ make_foo() }</sel> else { make_foo() };
    """
    popover = """
    ```cairo
    struct Foo {
        x: u32,
    }
    ```
    """
    "#)
}

/// Hover on the opening brace of a block expression shows the type of the block.
#[test]
fn block_expr_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _: u32 = if true <caret>{ 1_u32 } else { 2_u32 };
    }
    "#, @r#"
    source_context = """
        let _: u32 = if true <caret>{ 1_u32 } else { 2_u32 };
    """
    highlight = """
        let _: u32 = if true <sel>{ 1_u32 }</sel> else { 2_u32 };
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}

/// Hover on snapshot expression (`@`) shows type of the snapshotted expression.
#[test]
fn snapshot_expr_type() {
    test_transform_plain!(Hover, r#"
    fn takes_snap(x: @u32) {}

    fn main() {
        let x: u32 = 1_u32;
        takes_snap(<caret>@x);
    }
    "#, @r#"
    source_context = """
        takes_snap(<caret>@x);
    """
    highlight = """
        takes_snap(<sel>@x</sel>);
    """
    popover = """
    ```cairo
    @u32
    ```
    """
    "#)
}

/// Hover on desnap operator `*` when used as desnap (not multiply) shows the type.
#[test]
fn desnap_expr_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let x: u32 = 1_u32;
        let snap: @u32 = @x;
        let _ = <caret>*snap;
    }
    "#, @r#"
    source_context = """
        let _ = <caret>*snap;
    """
    highlight = """
        let _ = <sel>*snap</sel>;
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}

/// Hover on a method call closing paren shows the return type of the method.
#[test]
fn method_call_return_type() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let x: u32 = 0_u32;
        let _: u64 = x.into(<caret>);
    }
    "#, @r#"
    source_context = """
        let _: u64 = x.into(<caret>);
    """
    highlight = """
        let _: u64 = <sel>x.into()</sel>;
    """
    popover = """
    ```cairo
    u64
    ```
    """
    "#)
}

#[test]
fn type_info_inside_proc_macro_inline_macro() {
    test_transform_with_macros!(Hover, r#"
    fn main() {
        let _: u32 = simple_inline_macro_v2!(1_u32 <caret>+ 2_u32);
    }
    "#, @r#"
    source_context = """
        let _: u32 = simple_inline_macro_v2!(1_u32 <caret>+ 2_u32);
    """
    highlight = """
        let _: u32 = simple_inline_macro_v2!(<sel>1_u32 + 2_u32</sel>);
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}

#[test]
fn type_info_inside_attribute_macro() {
    test_transform_with_macros!(Hover, r#"
    #[simple_attribute_macro_v2]
    fn main() {
        let _ = 1_u32 <caret>+ 2_u32;
    }
    "#, @r#"
    source_context = """
        let _ = 1_u32 <caret>+ 2_u32;
    """
    highlight = """
        let _ = <sel>1_u32 + 2_u32</sel>;
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}

/// Hover on an operator inside a built-in inline macro shows no type info — built-in macros use
/// PatchBuilder::add_node which emits a single CodeOrigin::Start mapping per argument node rather
/// than per-token mappings, so source tokens inside the token tree have no expression resultants
/// to walk up from.
#[test]
fn no_type_info_inside_builtin_inline_macro() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let _ = array![1_u32 <caret>+ 2_u32];
    }
    "#, @r#"
    source_context = """
        let _ = array![1_u32 <caret>+ 2_u32];
    """
    "#)
}

/// Hover on an operator in a const initializer shows no type info — const items have no function
/// body, so expression types cannot be looked up.
#[test]
fn no_type_info_outside_function_body() {
    test_transform_plain!(Hover, r#"
    const X: u32 = 1_u32 <caret>+ 2_u32;
    "#, @r#"
    source_context = """
    const X: u32 = 1_u32 <caret>+ 2_u32;
    """
    "#)
}

/// Hover inside a Cairo test function shows expression type info like a normal function body.
#[test]
fn type_info_inside_test_function() {
    test_transform_with_macros!(Hover, r#"
    #[complex_attribute_macro_v2]
    fn it_works() {
        let _ = 1_u32 <caret>+ 2_u32;
    }
    "#, @r#"
    source_context = """
        let _ = 1_u32 <caret>+ 2_u32;
    """
    highlight = """
        let _ = <sel>1_u32 + 2_u32</sel>;
    """
    popover = """
    ```cairo
    u32
    ```
    """
    "#)
}
