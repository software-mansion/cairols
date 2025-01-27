use super::test_completions_text_edits;
use crate::support::insta::test_transform;

#[test]
fn empty() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let a = Struct { <caret> };
    }
    ",@r#"
    caret = """
        let a = Struct { <caret> };
    """

    [[completions]]
    completion_label = "x"
    detail = "core::integer::u32"

    [[completions]]
    completion_label = "y"
    detail = "core::felt252"

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn after_prop() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let b = Struct { x: 0x0, <caret> };
    }
    ",@r#"
    caret = """
        let b = Struct { x: 0x0, <caret> };
    """

    [[completions]]
    completion_label = "y"
    detail = "core::felt252"

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn after_prop_before_spread() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let c = Struct {
            x: 0x0,
            <caret>
            ..s
        };
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "y"
    detail = "core::felt252"

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn after_prop_before_spread_same_line() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let d = Struct {
            x: 0x0,
            <caret>..s
        };
    }
    ",@r#"
    caret = """
            <caret>..s
    """

    [[completions]]
    completion_label = "core"

    [[completions]]
    completion_label = "hello"

    [[completions]]
    completion_label = "Struct"

    [[completions]]
    completion_label = "build_struct"

    [[completions]]
    completion_label = "array!"

    [[completions]]
    completion_label = "assert!"

    [[completions]]
    completion_label = "consteval_int!"

    [[completions]]
    completion_label = "format!"

    [[completions]]
    completion_label = "panic!"

    [[completions]]
    completion_label = "print!"

    [[completions]]
    completion_label = "println!"

    [[completions]]
    completion_label = "write!"

    [[completions]]
    completion_label = "writeln!"

    [[completions]]
    completion_label = "selector!"

    [[completions]]
    completion_label = "get_dep_component!"

    [[completions]]
    completion_label = "get_dep_component_mut!"

    [[completions]]
    completion_label = "assert_eq!"

    [[completions]]
    completion_label = "assert_ne!"

    [[completions]]
    completion_label = "assert_lt!"

    [[completions]]
    completion_label = "assert_le!"

    [[completions]]
    completion_label = "assert_gt!"

    [[completions]]
    completion_label = "assert_ge!"

    [[completions]]
    completion_label = "s"

    [[completions]]
    completion_label = "d"
    "#);
}

#[test]
fn before_spread_same_line() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    fn build_struct() {
        let s = Struct {
            x: 0x0,
            y: 0x0,
            z: 0x0
        };

        let e = Struct { <caret>..s };
    }
    ",@r#"
    caret = """
        let e = Struct { <caret>..s };
    """

    [[completions]]
    completion_label = "x"
    detail = "core::integer::u32"

    [[completions]]
    completion_label = "y"
    detail = "core::felt252"

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn imported_empty() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    mod happy_cases {
        use super::Struct;

        fn foo() {
            let a = Struct { <caret> };
        }
    }
    ",@r#"
    caret = """
            let a = Struct { <caret> };
    """

    [[completions]]
    completion_label = "x"
    detail = "core::integer::u32"

    [[completions]]
    completion_label = "y"
    detail = "core::felt252"

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn imported_after_prop() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    mod happy_cases {
        use super::Struct;

        fn foo() {
            let b = Struct { y: 0x0, <caret> };
        }
    }
    ",@r#"
    caret = """
            let b = Struct { y: 0x0, <caret> };
    """

    [[completions]]
    completion_label = "x"
    detail = "core::integer::u32"

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn imported_after_two_prop() {
    test_transform!(test_completions_text_edits,"
    pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
    }

    mod happy_cases {
        use super::Struct;

        fn foo() {
            let c = Struct { y: 0x0, x: 0x0, <caret> }
        }
    }
    ",@r#"
    caret = """
            let c = Struct { y: 0x0, x: 0x0, <caret> }
    """

    [[completions]]
    completion_label = "z"
    detail = "core::integer::i16"
    "#);
}

#[test]
fn not_imported() {
    test_transform!(test_completions_text_edits,"
    mod unhappy_cases {
        fn foo() {
            let a = NonexsitentStruct { <caret> };
        }
    }
    ",@r#"
    caret = """
            let a = NonexsitentStruct { <caret> };
    """
    completions = []
    "#);
}
