use crate::code_lens::test_code_lens_no_scarb_package;
use crate::support::insta::test_transform;

#[test]
fn run_test() {
    test_transform!(test_code_lens_no_scarb_package, r#"
    #[test]<caret>
    fn a() {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[show_messages]]
    typ = "Error"
    message = "Cannot run tests: could not find the Scarb package for this file."
    "#)
}
