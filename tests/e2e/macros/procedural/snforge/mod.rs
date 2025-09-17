use indoc::indoc;

use crate::macros::{
    fixtures::{ProjectWithSnforgeIntegrationTest, ProjectWithSnforgeUnitTest},
    test_macro_expansion_and_diagnostics,
};

#[test]
fn unit_test_simple() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithSnforgeUnitTest,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[cfg(test)]
                mod tests {
                    #[test]<caret>
                    fn test_nothing() {
                        <caret>assert(1 == 1, 'Who knows');
                    }
                }
            "#)
        }
    );
}

#[test]
fn unit_test_with_assert_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithSnforgeUnitTest,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[cfg(test)]
                mod tests {
                    #[test]<caret>
                    fn test_nothing() {
                        <caret>assert_eq!(1, 1, "Who knows");
                    }
                }
            "#)
        }
    );
}

#[test]
fn integration_test_with_fork_and_fuzzer() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithSnforgeIntegrationTest,
        cwd = "test_package",
        files {
            "test_package/tests/test.cairo" => indoc!(r#"
                #[test]<caret>
                #[fork(<caret>"SEPOLIA_LATEST")]
                #[fuzzer(runs: 100,<caret> seed: 0x1234)]
                fn test_nothing(x: felt252) {
                    <caret>assert(x == x, 'Who knows');
                }
            "#)
        }
    );
}
