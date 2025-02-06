use crate::hover::test_hover;
use crate::support::insta::test_transform;

#[test]
fn member_definition_name() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member<caret>1: felt252,
        member2: u256
    }
    ",@r#"
    source_context = """
        member<caret>1: felt252,
    """
    highlight = """
        <sel>member1</sel>: felt252,
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Struct {
        member1: felt252,
        member2: u256,
    }
    ```
    ---
    Docstring of member1."""
    "#)
}

#[test]
fn struct_init_first_member_name() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member1: felt252,
        member2: u256
    }

    mod happy_cases {
        use super::Struct;

        fn constructor() {
            let _s = Struct {  member1<caret>: 0, member2: 0 };
        }
    }
    ",@r#"
    source_context = """
            let _s = Struct {  member1<caret>: 0, member2: 0 };
    """
    highlight = """
            let _s = Struct {  <sel>member1</sel>: 0, member2: 0 };
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Struct {
        member1: felt252,
        member2: u256,
    }
    ```
    ---
    Docstring of member1."""
    "#)
}

#[test]
fn struct_init_second_member_name() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member1: felt252,
        member2: u256
    }

    mod happy_cases {
        use super::Struct;

        fn constructor() {
            let _s = Struct {  member1: 0, mem<caret>ber2: 0 };
        }
    }
    ",@r#"
    source_context = """
            let _s = Struct {  member1: 0, mem<caret>ber2: 0 };
    """
    highlight = """
            let _s = Struct {  member1: 0, <sel>member2</sel>: 0 };
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Struct {
        member1: felt252,
        member2: u256,
    }
    ```
    """
    "#)
}

#[test]
fn struct_init_first_member_shorthand() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member1: felt252,
        member2: u256
    }

    mod happy_cases {
        use super::Struct;

        fn constructor() {
            let member1 = 0;
            let member2 = 0;
            let _s = Struct { mem<caret>ber1, member2 };
        }
    }
    ",@r#"
    source_context = """
            let _s = Struct { mem<caret>ber1, member2 };
    """
    highlight = """
            let _s = Struct { <sel>member1</sel>, member2 };
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Struct {
        member1: felt252,
        member2: u256,
    }
    ```
    ---
    Docstring of member1."""
    "#)
}
#[test]
fn struct_init_second_member_shorthand_after() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member1: felt252,
        member2: u256
    }

    mod happy_cases {
        use super::Struct;

        fn constructor() {
            let member1 = 0;
            let member2 = 0;
            let _s = Struct { member1, member2<caret> };
        }
    }
    ",@r#"
    source_context = """
            let _s = Struct { member1, member2<caret> };
    """
    highlight = """
            let _s = Struct { member1, <sel>member2</sel> };
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Struct {
        member1: felt252,
        member2: u256,
    }
    ```
    """
    "#)
}

#[test]
fn struct_member_access_after_name() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member1: felt252,
        member2: u256
    }

    mod happy_cases {
        use super::Struct;

        fn member_access() {
            let s = Struct {  member1: 0, member2: 0 };
            let _ = s.member1<caret>;
        }
    }
    ",@r#"
    source_context = """
            let _ = s.member1<caret>;
    """
    highlight = """
            let _ = s.<sel>member1</sel>;
    """
    popover = """
    ```cairo
    hello
    ```
    ```cairo
    struct Struct {
        member1: felt252,
        member2: u256,
    }
    ```
    ---
    Docstring of member1."""
    "#)
}

#[test]
fn non_existent_struct_member_init() {
    test_transform!(test_hover,"
    /// Docstring of Struct.
    struct Struct {
        /// Docstring of member1.
        member1: felt252,
        member2: u256
    }

    mod unhappy_cases {
        fn non_existent_struct {
            let _ = NonExistentStruct { mem<caret>ber: 0 };
        }
    }
    ",@r#"
    source_context = """
            let _ = NonExistentStruct { mem<caret>ber: 0 };
    """
    highlight = """
            let _ = NonExistentStruct { <sel>member</sel>: 0 };
    """
    popover = """
    ```cairo
    hello::unhappy_cases
    ```
    ```cairo
    fn non_existent_struct
    ```
    """
    "#)
}
