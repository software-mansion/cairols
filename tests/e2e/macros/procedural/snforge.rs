use crate::{
    macros::{MacroTestFixture, fixtures::SnforgeUnitTestProject},
    support::insta::test_transform,
};

#[test]
fn simple_test_function() {
    test_transform!(
        SnforgeUnitTestProject::test_macro_expansion_and_diagnostics,

        r##"
        #[cfg(test)]
        mod tests {
            #[test]<caret>
            fn test_nothing() {
                <caret>assert(1 == 1, 'Who knows');
            }
        }
        "##,

        @r#"
        [[expansions]]
        analyzed_lines = """
            #[test]<caret>
                <caret>assert(1 == 1, 'Who knows');
        """
        generated_code = """
        // lib.cairo
        // ---------

        #[test]
        fn test_nothing() {
            assert(1 == 1, 'Who knows');
        }

        // proc_macro_test
        // ---------------

        #[snforge_internal_test_executable]
        #[__internal_config_statement]
        fn test_nothing() {
            assert(1 == 1, 'Who knows');
        }

        // proc_macro___internal_config_statement
        // --------------------------------------

        #[snforge_internal_test_executable]
        fn test_nothing() {
            if snforge_std::_internals::_is_config_run() {
                return;
            }

            assert(1 == 1, 'Who knows');
        }
        """
        "#
    )
}
