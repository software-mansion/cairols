use crate::code_actions::{quick_fix, quick_fix_with_macros};
use crate::support::insta::test_transform;

fn quick_fix_struct_simple(cairo_code: &str) -> String {
    let code = format!(
        r#"
            pub struct Struct {{
                x: u32,
                pub y: felt252,
                pub z: i16
            }}
            {cairo_code}
        }}
        "#
    );

    quick_fix(code.as_str())
}

// Structured version of the test (in terms of modules)
fn quick_fix_struct_with_modules(cairo_code: &str) -> String {
    let code = format!(
        r#"
        mod struct_module {{
            pub struct Struct {{
                x: u32,
                pub y: felt252,
                pub z: i16
            }}
        }}
        mod other_module {{
            {cairo_code}
        }}
        "#
    );

    quick_fix(code.as_str())
}

#[test]
fn fill_whole_struct() {
    test_transform!(quick_fix_struct_simple, r#"
        fn build_struct() {
            let _a = Struct { <caret> };
        }
    "#, @r#"
    Title: Fill struct fields
    Add new text: " x: (), y: (), z: ()"
    At: Range { start: Position { line: 6, character: 21 }, end: Position { line: 6, character: 21 } }
    "#
    )
}

#[test]
fn fill_two_fields() {
    test_transform!(quick_fix_struct_simple, r#"
        fn build_struct() {
            let _b = Struct { x: 0x0, <caret> };
        }
    "#, @r#"
    Title: Fill struct fields
    Add new text: " y: (), z: ()"
    At: Range { start: Position { line: 6, character: 29 }, end: Position { line: 6, character: 29 } }
    "#
    )
}

#[test]
fn no_fill_splat_operator() {
    test_transform!(quick_fix_struct_simple, r#"
        fn build_struct() {
            let s = Struct {
                x: 0x0,
                y: 0x0,
                z: 0x0
            };
            let s = Struct { <caret>..s };
        }
    "#, @"No code actions."
    )
}

#[test]
fn imported_struct_fill_all() {
    test_transform!(quick_fix_struct_with_modules, r#"
        use super::struct_module::Struct;

        fn build_struct() {
            let s = Struct { <caret> };
        }
    "#, @r#"
    Title: Fill struct fields
    Add new text: " y: (), z: ()"
    At: Range { start: Position { line: 11, character: 20 }, end: Position { line: 11, character: 20 } }
    "#
    )
}

#[test]
fn imported_struct_fill_two() {
    test_transform!(quick_fix_struct_with_modules, r#"
        use super::struct_module::Struct;

        fn build_struct() {
            let s = Struct { x: 0x0, <caret> };
        }
    "#, @r#"
    Title: Fill struct fields
    Add new text: " y: (), z: ()"
    At: Range { start: Position { line: 11, character: 28 }, end: Position { line: 11, character: 28 } }
    "#
    )
}

#[test]
fn imported_struct_fill_one() {
    test_transform!(quick_fix_struct_with_modules, r#"
        use super::struct_module::Struct;

        fn build_struct() {
            let s = Struct { x: 0x0, y: 0x0, <caret> };
        }
    "#, @r#"
    Title: Fill struct fields
    Add new text: " z: ()"
    At: Range { start: Position { line: 11, character: 36 }, end: Position { line: 11, character: 36 } }
    "#
    )
}

#[test]
fn no_fill_nonexistent_struct() {
    test_transform!(quick_fix_struct_with_modules, r#"
        fn build_struct() {
            let s = ImaginaryStruct { x: 0x0, y: 0x0, <caret> };
        }
    "#, @"No code actions."
    )
}

#[test]
fn fill_struct_in_macro_controlled_code() {
    test_transform!(quick_fix_with_macros, r#"
     pub struct Struct {
        x: u32,
        pub y: felt252,
        pub z: i16
     }

    #[test]
    fn test_fn() {
        let s = Struct { <caret> };
    }
    "#, @r#"
    Title: Fill struct fields
    Add new text: " x: (), y: (), z: ()"
    At: Range { start: Position { line: 8, character: 20 }, end: Position { line: 8, character: 20 } }
    "#)
}
