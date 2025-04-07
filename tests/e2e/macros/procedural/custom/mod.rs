use crate::macros::fixtures::ProjectWithMultipleCrates;
use crate::macros::test_macro_expansion_and_diagnostics;
use indoc::indoc;

mod v1;
mod v1_and_v2;
mod v2;

#[test]
fn macros_with_equal_names_in_two_separate_packages() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithMultipleCrates,
        cwd = "workspace/package_a",
        files {
            "workspace/package_a/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _name = which_<caret>macro_package!();
                }
            "#),
            "workspace/package_b/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _name = which_<caret>macro_package!();
                }
            "#)
        }
    );
}
