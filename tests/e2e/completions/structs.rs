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
    detail = "u32"

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
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
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
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
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
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
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
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
    detail = "u32"

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
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
    detail = "u32"

    [[completions]]
    completion_label = "y"
    detail = "felt252"

    [[completions]]
    completion_label = "z"
    detail = "i16"
    "#);
}

#[test]
fn some_field_private() {
    test_transform!(test_completions_text_edits,"
    mod wrapper {
        pub struct Struct {
            x: u32,
            pub y: felt252,
            pub z: i16
        }
    }

    mod struct_is_not_in_ancestor_module {
        use super::wrapper::Struct;

        fn foo() {
            let a = Struct { <caret> };
        }
    }
    ",@r#"
    caret = """
            let a = Struct { <caret> };
    """
    completions = []
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
    detail = "u32"

    [[completions]]
    completion_label = "z"
    detail = "i16"
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
    detail = "i16"
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

#[test]
fn dep_without_visibility_support() {
    test_transform!(test_completions_text_edits,"
    fn a() {
        dep::Foo { // This struct is partially private
            <caret>
        };
    }
    ",@r#"
    caret = """
            <caret>
    """

    [[completions]]
    completion_label = "a"
    detail = "felt252"

    [[completions]]
    completion_label = "b"
    detail = "felt252"
    "#);
}
