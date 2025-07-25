use lsp_types::Hover;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn let_mut() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut x<caret> = 5;
    }
    "#,@r#"
    source_context = """
        let mut x<caret> = 5;
    """
    highlight = """
        let mut <sel>x</sel> = 5;
    """
    popover = """
    ```cairo
    let mut x: felt252
    ```
    """
    "#)
}

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
fn assign_lhs() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut x = 5;
        x<caret> = add_two(x);;
    }

    /// `add_two` documentation.
    fn add_two(x: u32) -> u32 {
        //! Adds 2 to an unsigned argument.
        x + 2
    }
    "#,@r#"
    source_context = """
        x<caret> = add_two(x);;
    """
    highlight = """
        <sel>x</sel> = add_two(x);;
    """
    popover = """
    ```cairo
    let mut x: u32
    ```
    """
    "#)
}

#[test]
fn assign_rhs_before() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut x = 5;
        x = <caret>add_two(x);
    }

    /// `add_two` documentation.
    fn add_two(x: u32) -> u32 {
        //! Adds 2 to an unsigned argument.
        x + 2
    }
    "#,@r#"
    source_context = """
        x = <caret>add_two(x);
    """
    highlight = """
        x = <sel>add_two</sel>(x);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn add_two(x: u32) -> u32
    ```
    ---
    `add_two` documentation. Adds 2 to an unsigned argument."""
    "#)
}

#[test]
fn assign_rhs_on_fn_name() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut x = 5;
        x = add<caret>_two(x);
    }

    /// `add_two` documentation.
    fn add_two(x: u32) -> u32 {
        //! Adds 2 to an unsigned argument.
        x + 2
    }
    "#,@r#"
    source_context = """
        x = add<caret>_two(x);
    """
    highlight = """
        x = <sel>add_two</sel>(x);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn add_two(x: u32) -> u32
    ```
    ---
    `add_two` documentation. Adds 2 to an unsigned argument."""
    "#)
}

#[test]
fn assign_rhs_after_fn_name() {
    test_transform_plain!(Hover,r#"
    fn main() {
        let mut x = 5;
        x = add_two<caret>(x);
    }

    /// `add_two` documentation.
    fn add_two(x: u32) -> u32 {
        //! Adds 2 to an unsigned argument.
        x + 2
    }
    "#,@r#"
    source_context = """
        x = add_two<caret>(x);
    """
    highlight = """
        x = <sel>add_two</sel>(x);
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn add_two(x: u32) -> u32
    ```
    ---
    `add_two` documentation. Adds 2 to an unsigned argument."""
    "#)
}

#[test]
fn enum_name() {
    test_transform_plain!(Hover,r#"
    enum Co<caret>in {
        Penny,
    }
    "#,@r#"
    source_context = """
    enum Co<caret>in {
    """
    highlight = """
    enum <sel>Coin</sel> {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    enum Coin {
        Penny,
    }
    ```
    """
    "#)
}

#[test]
fn enum_name_macro() {
    test_transform_with_macros!(Hover,r#"
    #[mod_attribute_macro_v2]
    enum Co<caret>in {
        Penny,
    }
    "#,@r#"
    source_context = """
    enum Co<caret>in {
    """
    highlight = """
    enum <sel>Coin</sel> {
    """
    popover = """
    ```cairo
    hello::modzik
    ```
    ```cairo
    enum Coin {
        Penny,
    }
    ```
    """
    "#)
}

#[test]
fn enum_member() {
    test_transform_plain!(Hover,r#"
    enum Coin {
        Penn<caret>y,
    }
    "#,@r#"
    source_context = """
        Penn<caret>y,
    """
    highlight = """
        <sel>Penny</sel>,
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    enum Coin {
        Penny,
    }
    ```
    """
    "#)
}

#[test]
fn fn_name() {
    test_transform_plain!(Hover,r#"
    enum Coin {
        Penny,
    }

    fn value_i<caret>n_cents(coin: Coin) -> felt252 {
        match coin {
            Coin::Penny => 1,
        }
    }
    "#,@r#"
    source_context = """
    fn value_i<caret>n_cents(coin: Coin) -> felt252 {
    """
    highlight = """
    fn <sel>value_in_cents</sel>(coin: Coin) -> felt252 {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    fn value_in_cents(coin: Coin) -> felt252
    ```
    """
    "#)
}

#[test]
fn param_name() {
    test_transform_plain!(Hover,r#"
    enum Coin {
        Penny,
    }

    fn value_in_cents(co<caret>in: Coin) -> felt252 {
        match coin {
            Coin::Penny => 1,
        }
    }
    "#,@r#"
    source_context = """
    fn value_in_cents(co<caret>in: Coin) -> felt252 {
    """
    highlight = """
    fn value_in_cents(<sel>coin</sel>: Coin) -> felt252 {
    """
    popover = """
    ```cairo
    coin: Coin
    ```
    """
    "#)
}

#[test]
fn param_type() {
    test_transform_plain!(Hover,r#"
    enum Coin {
        Penny,
    }

    fn value_in_cents(coin: C<caret>oin) -> felt252 {
        match coin {
            Coin::Penny => 1,
        }
    }
    "#,@r#"
    source_context = """
    fn value_in_cents(coin: C<caret>oin) -> felt252 {
    """
    highlight = """
    fn value_in_cents(coin: <sel>Coin</sel>) -> felt252 {
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    enum Coin {
        Penny,
    }
    ```
    """
    "#)
}

#[test]
fn enum_value() {
    test_transform_plain!(Hover,r#"
    enum Coin {
        Penny,
    }

    fn value_in_cents(coin: Coin) -> felt252 {
        match co<caret>in {
            Coin::Penny => 1,
        }
    }
    "#,@r#"
    source_context = """
        match co<caret>in {
    """
    highlight = """
        match <sel>coin</sel> {
    """
    popover = """
    ```cairo
    coin: Coin
    ```
    """
    "#)
}

#[test]
fn enum_member_in_match() {
    test_transform_plain!(Hover,r#"
    enum Coin {
        Penny,
    }

    fn value_in_cents(coin: Coin) -> felt252 {
        match coin {
            Coin::P<caret>enny => 1,
        }
    }
    "#,@r#"
    source_context = """
            Coin::P<caret>enny => 1,
    """
    highlight = """
            Coin::<sel>Penny</sel> => 1,
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    enum Coin {
        Penny,
    }
    ```
    """
    "#)
}
