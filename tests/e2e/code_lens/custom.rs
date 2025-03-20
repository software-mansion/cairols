use crate::{code_lens::test_code_lens_custom_runner, support::insta::test_transform};

#[test]
fn only_functions() {
    test_transform!(test_code_lens_custom_runner, r#"
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
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 2
    command = "▶ Run test"
    index = 1
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 6
    command = "▶ Run test"
    index = 2
    file_path = "src/lib.cairo"

    [execute_in_terminal]
    command = "some random template ->hello::b<- string"
    cwd = "./"
    "#)
}

#[test]
fn fn_in_mod() {
    test_transform!(test_code_lens_custom_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run tests"
    index = 1
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 1
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [execute_in_terminal]
    command = "some random template ->hello::b::a<- string"
    cwd = "./"
    "#)
}

#[test]
fn run_for_mod() {
    test_transform!(test_code_lens_custom_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @r#"
    [[lenses]]
    line = 0
    command = "▶ Run tests"
    index = 1
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 1
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [execute_in_terminal]
    command = "some random template ->hello::b<- string"
    cwd = "./"
    "#)
}

#[test]
fn mod_without_test() {
    test_transform!(test_code_lens_custom_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex() {
    test_transform!(test_code_lens_custom_runner, r#"
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
    index = 1
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 5
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 11
    command = "▶ Run test"
    index = 2
    file_path = "src/lib.cairo"
    "#)
}
