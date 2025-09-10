use lsp_types::Hover;

use crate::support::insta::test_transform_plain;

#[test]
fn println() {
    test_transform_plain!(Hover,r#"
    fn main() {
        p<caret>rintln!("The value of x is: {}", x);
    }
    "#,@r#"
    source_context = """
        p<caret>rintln!("The value of x is: {}", x);
    """
    highlight = """
        <sel>println</sel>!("The value of x is: {}", x);
    """
    popover = '''
    ```cairo
    println
    ```
    ---
    Prints to the standard output, with a newline.
    This macro uses the same syntax as `format!`, but writes to the standard output instead.

    # Panics
    Panics if any of the formatting of arguments fails.

    # Examples
    ```cairo
    println!(); // Prints an empty line.
    println!(\"hello\"); // Prints "hello".
    let world: ByteArray = "world";
    println!("hello {}", world_ba); // Prints "hello world".
    println!("hello {world_ba}"); // Prints "hello world".
    ```
    '''
    "#)
}

#[test]
fn declarative_macro_on_definition() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let x = inc!(0);
    }

    /// Increment number by one.
    pub macro i<caret>nc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    source_context = """
    pub macro i<caret>nc {
    """
    highlight = """
    pub macro <sel>inc</sel> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    macro inc {
        ($x:expr) => { ... };
    } => { ... }
    ```
    ---
    Increment number by one."""
    "#)
}

#[test]
fn declarative_macro_on_usage() {
    test_transform_plain!(Hover, r#"
    fn main() {
        let x = i<caret>nc!(0);
    }

    /// Increment number by one.
    pub macro inc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    source_context = """
        let x = i<caret>nc!(0);
    """
    highlight = """
        let x = <sel>inc</sel>!(0);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    macro inc {
        ($x:expr) => { ... };
    } => { ... }
    ```
    ---
    Increment number by one."""
    "#)
}

#[test]
fn declarative_macro_on_usage_with_macros() {
    test_transform_plain!(Hover, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        let x = i<caret>nc!(0);
    }

    /// Increment number by one.
    pub macro inc {
        ($x:expr) => { $x + 1 };
    }
    "#, @r#"
    source_context = """
        let x = i<caret>nc!(0);
    """
    highlight = """
        let x = <sel>inc</sel>!(0);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    macro inc {
        ($x:expr) => { ... };
    } => { ... }
    ```
    ---
    Increment number by one."""
    "#)
}
