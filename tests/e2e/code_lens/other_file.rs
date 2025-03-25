use crate::{code_lens::test_code_lens_cairo_test, support::insta::test_transform};

#[test]
fn other_file() {
    test_transform!(test_code_lens_cairo_test, r#"
    #[test]
    fn a() {}

    mod foo;<caret>
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 3
    command = "▶ Run tests"
    index = 1
    file_path = "src/lib.cairo"

    [execute_in_terminal]
    command = "scarb cairo-test --filter hello::foo"
    cwd = "./"
    "#)
}

#[test]
fn other_file_no_test() {
    test_transform!(test_code_lens_cairo_test, r#"
    #[test]
    fn a() {}

    mod bar;<caret>
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"
    "#)
}

#[test]
fn other_file_nested_test() {
    test_transform!(test_code_lens_cairo_test, r#"
    #[test]
    fn a() {}

    mod baz;<caret>
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 3
    command = "▶ Run tests"
    index = 1
    file_path = "src/lib.cairo"

    [execute_in_terminal]
    command = "scarb cairo-test --filter hello::baz"
    cwd = "./"
    "#)
}
