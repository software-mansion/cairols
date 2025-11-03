use lsp_types::Hover;

use crate::support::insta::test_transform_plain;

#[test]
fn after_mod_kw() {
    test_transform_plain!(Hover,r#"
    mod<caret> missing;
    "#,@r#"
    source_context = """
    mod<caret> missing;
    """
    "#)
}

#[test]
fn mod_name() {
    test_transform_plain!(Hover,r#"
    mod miss<caret>ing;
    "#,@r#"
    source_context = """
    mod miss<caret>ing;
    """
    highlight = """
    mod <sel>missing</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    mod missing
    ```
    """
    "#)
}
