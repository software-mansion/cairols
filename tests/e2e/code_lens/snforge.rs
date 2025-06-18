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
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 0
    command = "▶ Run test"
    index = 1
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 0
    command = "▶ Run test"
    index = 2
    file_path = "src/lib.cairo"
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
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 0
    command = "▶ Run tests"
    index = 1
    file_path = "src/lib.cairo"
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
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 0
    command = "▶ Run tests"
    index = 1
    file_path = "src/lib.cairo"

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
    line = 0
    command = "▶ Run test"
    index = 0
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 0
    command = "▶ Run test"
    index = 1
    file_path = "src/lib.cairo"

    [[lenses]]
    line = 4
    command = "▶ Run tests"
    index = 2
    file_path = "src/lib.cairo"
    "#)
}
