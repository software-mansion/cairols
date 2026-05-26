use crate::code_lens::{test_code_lens_snforge, test_code_lens_snforge_wrong_debug_config};
use crate::support::insta::test_transform;

#[test]
fn only_functions() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 6
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 6
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[execute_in_terminal]]
    command = "snforge test hello::b --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::b --exact"
    cwd = "./"
    "#)
}

#[test]
fn fn_in_mod() {
    test_transform!(test_code_lens_snforge, r#"
    mod b {
        #[test]<caret>
        fn a() {}
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
    command = "snforge test hello::b::a --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::b::a --exact"
    cwd = "./"
    "#)
}

#[test]
fn run_for_mod() {
    test_transform!(test_code_lens_snforge, r#"
    mod b {<caret>
        #[test]
        fn a() {}
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
    command = "snforge test hello::b"
    cwd = "./"
    "#)
}

#[test]
fn mod_without_test() {
    test_transform!(test_code_lens_snforge, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex() {
    test_transform!(test_code_lens_snforge, r#"
    mod b {
        fn a() {}
    }

    mod c {
        #[test]
        fn d() {}

        fn e() {}
    }

    #[test]
    fn f() {}

    fn f() {}<caret>
    "#, @r#"
    [[lenses]]
    line = 4
    command = "▶ Run tests"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 5
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 5
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 11
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 11
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0
    "#)
}

#[test]
fn test_case_1() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]
    #[test_case(1)]<caret>
    #[test_case(2)]
    fn a(_a: felt252) {}
    "#, @r#"
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

    [[lenses]]
    line = 2
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[execute_in_terminal]]
    command = "snforge test hello::a_1 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::a_1 --exact"
    cwd = "./"
    "#)
}

#[test]
fn test_case_2() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]
    #[test_case(1)]
    #[test_case(2)]<caret>
    fn a(_a: felt252) {}
    "#, @r#"
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

    [[lenses]]
    line = 2
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[execute_in_terminal]]
    command = "snforge test hello::a_2 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::a_2 --exact"
    cwd = "./"
    "#)
}

#[test]
fn test_case_with_fuzzer() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]
    #[fuzzer]
    #[test_case(1)]<caret>
    #[test_case(2)]
    fn a(_a: felt252) {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 3
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 3
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[execute_in_terminal]]
    command = "snforge test hello::a_1 --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::a_1 --exact"
    cwd = "./"
    "#)
}

#[test]
fn fuzzer_with_test_case() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]<caret>
    #[fuzzer]
    #[test_case(1)]
    #[test_case(2)]
    fn a(_a: felt252) {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 3

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 3
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 3
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact"
    cwd = "./"
    "#)
}

/// Regression test for https://github.com/software-mansion/cairols/issues/1245
/// A function with both `#[fuzzer]` and `#[test_case]` should have:
/// - both `▶ Run test` and `▶ Debug test` next to `#[test]`
/// - both `▶ Run test` and `▶ Debug test` next to each `#[test_case]`
#[test]
fn fuzzer_with_test_case_has_debug_lens_on_test_cases() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]<caret>
    #[fuzzer]
    #[test_case(1)]
    #[test_case(2)]
    #[test_case(3)]
    fn a(_a: felt252) {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 4

    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 5

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 3
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 6

    [[lenses]]
    line = 3
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [[lenses]]
    line = 4
    command = "▶ Debug test"
    file_path = "src/lib.cairo"
    index = 7

    [[lenses]]
    line = 4
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 3

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact --launch-debugger"
    cwd = "./"

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact"
    cwd = "./"
    "#)
}

#[test]
fn fuzzer_without_test_case() {
    test_transform!(test_code_lens_snforge, r#"
    #[test]<caret>
    #[fuzzer]
    fn a(_a: felt252) {}
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact"
    cwd = "./"
    "#)
}

#[test]
fn fuzzer_before_test() {
    test_transform!(test_code_lens_snforge, r#"
    #[fuzzer]
    #[test]<caret>
    fn a(_a: felt252) {}
    "#, @r#"
    [[lenses]]
    line = 1
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact"
    cwd = "./"
    "#)
}

#[test]
fn debug_with_incorrect_compiler_config() {
    test_transform!(test_code_lens_snforge_wrong_debug_config, r#"
    #[test]<caret>
    fn a() {}
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

    [[show_messages]]
    typ = "Error"
    message = """
    Cannot launch debugger: the Cairo compiler is not configured for debugging.
    Add the following key-value pairs your Scarb.toml to `[profile.dev.cairo]` section:

    unstable-add-statements-code-locations-debug-info = true
    unstable-add-statements-functions-debug-info = true
    add-functions-debug-info = true
    skip-optimizations = true
    """

    [[execute_in_terminal]]
    command = "snforge test hello::a --exact"
    cwd = "./"
    "#)
}
