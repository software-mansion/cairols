use crate::{code_lens::test_code_lens_snforge, support::insta::test_transform};

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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 6
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [execute_in_terminal]
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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [execute_in_terminal]
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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [execute_in_terminal]
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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [execute_in_terminal]
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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [execute_in_terminal]
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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 3
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [execute_in_terminal]
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
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 0

    [[lenses]]
    line = 2
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 1

    [[lenses]]
    line = 3
    command = "▶ Run test"
    file_path = "src/lib.cairo"
    index = 2

    [execute_in_terminal]
    command = "snforge test hello::a --exact"
    cwd = "./"
    "#)
}
