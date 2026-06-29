use crate::code_lens::test_code_lens_snforge_with_macros;
use crate::support::insta::test_transform;

#[test]
fn attribute_macro_generates_test() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    #[test_generating_attribute_macro_v2]<caret>
    fn carrier() {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_v2 --exact"
    cwd = "./"
    "#)
}

#[test]
fn inline_macro_generates_test() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    test_generating_inline_macro_v2!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_v2 --exact"
    cwd = "./"
    "#)
}

#[test]
fn handwritten_module_aggregates_generated_test() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    mod tests {<caret>
        test_generating_inline_macro_v2!();
    }
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 1
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 1
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::tests"
    cwd = "./"
    "#)
}

// CAVEAT: *Generated* module's lens cannot be executed -
// the click cannot be resolved back to a macro-generated `mod` item.
#[test]
fn inline_macro_generates_test_module() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    test_module_generating_inline_macro_v2!();

    fn not_a_test() {}<caret>
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 0
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1
    "#)
}

#[test]
fn attribute_macro_generates_test_in_handwritten_module() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    mod tests {
        #[test_generating_attribute_macro_v2]<caret>
        fn carrier() {}
    }
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 1
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 1
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::tests::generated_test_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::tests::generated_test_v2 --exact"
    cwd = "./"
    "#)
}

#[test]
fn attribute_macro_generates_multiple_tests() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    #[multiple_tests_generating_attribute_macro_v2]<caret>
    fn carrier() {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_1_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_2_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_3_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_1_v2 --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_2_v2 --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generated_test_3_v2 --exact"
    cwd = "./"
    "#)
}

#[test]
fn inline_macro_generates_multiple_tests() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    multiple_tests_generating_inline_macro_v2!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_1_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_2_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_3_v2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_1_v2 --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_2_v2 --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::inline_generated_test_3_v2 --exact"
    cwd = "./"
    "#)
}

// CAVEAT: *Generated* module's lens cannot be executed -
// the click cannot be resolved back to a macro-generated `mod` item.
#[test]
fn inline_macro_generates_test_module_with_multiple_tests() {
    test_transform!(test_code_lens_snforge_with_macros, r#"
    multiple_tests_module_generating_inline_macro_v2!();

    fn not_a_test() {}<caret>
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 6

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 0
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 3
    "#)
}
