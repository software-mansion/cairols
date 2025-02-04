use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn mod_kw() {
    test_transform!(test_hover,r#"
    m<caret>od missing;
    "#,@r#"
    source_context = """
    m<caret>od missing;
    """
    "#)
}

#[test]
fn after_mod_kw() {
    test_transform!(test_hover,r#"
    mod<caret> missing;
    "#,@r#"
    source_context = """
    mod<caret> missing;
    """
    "#)
}

#[test]
fn mod_name() {
    test_transform!(test_hover,r#"
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
