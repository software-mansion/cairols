use crate::code_lens::test_code_lens_snforge_with_user_defined_macros;
use crate::support::insta::test_transform;

// FIXME(#1360):
// `ModuleItemId::full_path` splices the raw macro-invocation text in as path segments,
// so the command is `snforge test hello::generate_test!();::expose! { … }::generated_test --exact`,
// not a clean `hello::generated_test`. This is deliberate — it is exactly the name snforge collects the test
// under (a compiler naming convention), so these verbatim commands actually run the tests.

#[test]
fn declarative_macro_generates_test() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_test {
        () => {
            expose! {
                #[test]
                fn generated_test() {}
            }
        };
    }

    generate_test!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 9
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 9
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test!_#126::expose!_#0::generated_test --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test!_#126::expose!_#0::generated_test --exact"
    cwd = "./"
    "#)
}

#[test]
fn handwritten_module_aggregates_generated_test() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_test {
        () => {
            expose! {
                #[test]
                fn generated_test() {}
            }
        };
    }

    mod tests {<caret>
        use super::generate_test;

        generate_test!();
    }
    "#, @r#"
    [[lenses]]
    line = 9
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 12
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 12
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::tests"
    cwd = "./"
    "#)
}

#[test]
fn declarative_macro_generates_test_module() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_test_module {
        () => {
            expose! {
                mod generated_test_mod {
                    #[test]
                    fn test_in_generated_mod() {}
                }
            }
        };
    }

    generate_test_module!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 11
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 11
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 11
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test_module!_#199::expose!_#0::generated_test_mod::test_in_generated_mod --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test_module!_#199::expose!_#0::generated_test_mod::test_in_generated_mod --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test_module!_#199::expose!_#0::generated_test_mod"
    cwd = "./"
    "#)
}

#[test]
fn declarative_macro_generates_multiple_tests() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_multiple_tests {
        () => {
            expose! {
                #[test]
                fn generated_test_1() {}

                #[test]
                fn generated_test_2() {}

                #[test]
                fn generated_test_3() {}
            }
        };
    }

    generate_multiple_tests!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 15
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 15
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 15
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 15
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 15
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 15
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_tests!_#254::expose!_#0::generated_test_1 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_tests!_#254::expose!_#0::generated_test_2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_tests!_#254::expose!_#0::generated_test_3 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_tests!_#254::expose!_#0::generated_test_1 --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_tests!_#254::expose!_#0::generated_test_2 --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_tests!_#254::expose!_#0::generated_test_3 --exact"
    cwd = "./"
    "#)
}

#[test]
fn declarative_macro_generates_multiple_test_modules() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_multiple_test_modules {
        () => {
            expose! {
                mod first_generated_test_mod {
                    #[test]
                    fn test_in_first_generated_mod() {}
                }

                mod second_generated_test_mod {
                    #[test]
                    fn test_in_second_generated_mod() {}
                }
            }
        };
    }

    generate_multiple_test_modules!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 16
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 16
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 16
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 16
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 16
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 16
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 3

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_test_modules!_#357::expose!_#0::first_generated_test_mod::test_in_first_generated_mod --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_test_modules!_#357::expose!_#0::second_generated_test_mod::test_in_second_generated_mod --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_test_modules!_#357::expose!_#0::first_generated_test_mod::test_in_first_generated_mod --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_test_modules!_#357::expose!_#0::second_generated_test_mod::test_in_second_generated_mod --exact"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_test_modules!_#357::expose!_#0::first_generated_test_mod"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_multiple_test_modules!_#357::expose!_#0::second_generated_test_mod"
    cwd = "./"
    "#)
}

#[test]
fn declarative_macro_generates_test_module_with_multiple_tests() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_tests_module {
        () => {
            expose! {
                mod generated_tests_mod {
                    #[test]
                    fn test_1_in_generated_mod() {}

                    #[test]
                    fn test_2_in_generated_mod() {}

                    #[test]
                    fn test_3_in_generated_mod() {}
                }
            }
        };
    }

    generate_tests_module!();

    fn not_a_test() {}<caret>
    "#, @r#"
    [[lenses]]
    line = 17
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 17
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 17
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 6

    [[lenses]]
    line = 17
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 17
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 17
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 17
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 3
    "#)
}

#[test]
fn two_declarative_macro_invocations_generate_tests() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_test {
        ($name:ident) => {
            expose! {
                #[test]
                fn $name() {}
            }
        };
    }

    generate_test!(first_generated_test)<caret>;
    generate_test!(second_generated_test);
    "#, @r#"
    [[lenses]]
    line = 9
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 9
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 10
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 10
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test!_#128::expose!_#0::first_generated_test --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test!_#128::expose!_#0::first_generated_test --exact"
    cwd = "./"
    "#)
}

#[test]
fn handwritten_and_generated_tests_coexist() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_test {
        () => {
            expose! {
                #[test]
                fn generated_test() {}
            }
        };
    }

    #[test]
    fn handwritten_test() {}

    generate_test!();

    fn not_a_test() {}<caret>
    "#, @r#"
    [[lenses]]
    line = 9
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 9
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 12
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 12
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1
    "#)
}

#[test] // FIXME(#1361)
fn identical_invocations_produce_colliding_run_commands() {
    test_transform!(test_code_lens_snforge_with_user_defined_macros, r#"
    macro generate_test {
        () => {
            #[test]
            fn generated_test() {}
        };
    }

    generate_test!();
    generate_test!()<caret>;
    "#, @r#"
    [[lenses]]
    line = 7
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 7
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 8
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 8
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test!_#109::generated_test --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::generate_test!_#109::generated_test --exact"
    cwd = "./"
    "#)
}
