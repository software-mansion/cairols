use crate::{code_lens::test_code_lens_both_runners, support::insta::test_transform};

#[test]
fn only_functions() {
    test_transform!(test_code_lens_both_runners, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod() {
    test_transform!(test_code_lens_both_runners, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod() {
    test_transform!(test_code_lens_both_runners, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test() {
    test_transform!(test_code_lens_both_runners, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex() {
    test_transform!(test_code_lens_both_runners, r#"
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
    "#, @"lenses = []")
}
