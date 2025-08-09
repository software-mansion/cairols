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
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 3
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

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
    file_path = "src/lib.cairo"
    index = 0
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
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 3
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

    [execute_in_terminal]
    command = "scarb cairo-test --filter hello::baz"
    cwd = "./"
    "#)
}
