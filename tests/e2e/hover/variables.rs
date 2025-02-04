use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn ident_typed() {
    test_transform!(test_hover,r#"
    fn main() {
        let ab<caret>c: felt252 = 0;
    }
    "#,@r#"
    source_context = """
        let ab<caret>c: felt252 = 0;
    """
    popover = "Type: `core::felt252`"
    "#)
}

#[test]
fn ident() {
    test_transform!(test_hover,r#"
    fn main() {
        let xy<caret>z = 3;
    }
    "#,@r#"
    source_context = """
        let xy<caret>z = 3;
    """
    popover = "Type: `core::felt252`"
    "#)
}

#[test]
fn ident_mut() {
    test_transform!(test_hover,r#"
    fn main() {
        let abc: felt252 = 0;
        let mut de<caret>f = abc * 2;
    }
    "#,@r#"
    source_context = """
        let mut de<caret>f = abc * 2;
    """
    popover = "Type: `core::felt252`"
    "#)
}

#[test]
fn value_mut() {
    test_transform!(test_hover,r#"
    fn main() {
        let abc: felt252 = 0;
        let mut def = ab<caret>c * 2;
    }
    "#,@r#"
    source_context = """
        let mut def = ab<caret>c * 2;
    """
    highlight = """
        let mut def = <sel>abc</sel> * 2;
    """
    popover = """
    ```cairo
    let abc: core::felt252
    ```
    """
    "#)
}

#[test]
fn star_lhs() {
    test_transform!(test_hover,r#"
    fn main() {
        let mut def: felt252 = 0;
        let xyz = 0;
        let _ = de<caret>f * xyz;
    }
    "#,@r#"
    source_context = """
        let _ = de<caret>f * xyz;
    """
    highlight = """
        let _ = <sel>def</sel> * xyz;
    """
    popover = """
    ```cairo
    let mut def: core::felt252
    ```
    """
    "#)
}

#[test]
fn star_rhs() {
    test_transform!(test_hover,r#"
    fn main() {
        let mut def: felt252 = 0;
        let xyz = 0;
        let _ = def * xy<caret>z;
    }
    "#,@r#"
    source_context = """
        let _ = def * xy<caret>z;
    """
    highlight = """
        let _ = def * <sel>xyz</sel>;
    """
    popover = """
    ```cairo
    let xyz: core::felt252
    ```
    """
    "#)
}
