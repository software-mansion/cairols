use crate::{code_lens::test_code_lens_no_runner, support::insta::test_transform};

#[test]
fn only_functions() {
    test_transform!(test_code_lens_no_runner, r#"
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
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn only_functions_1() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_2() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_3() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_4() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_5() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_6() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_7() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_1() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_2() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_3() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_4() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_5() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_6() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_7() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_1() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_2() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_3() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_4() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_5() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_6() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_7() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_1() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_2() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_3() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_4() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_5() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_6() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_7() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex_1() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_2() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_3() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_4() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_5() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_6() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_7() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn only_functions_8() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_9() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_10() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_11() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_12() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_8() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_9() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_10() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_11() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_12() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_8() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_9() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_10() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_11() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_12() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_8() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_9() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_10() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_11() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_12() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex_8() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_9() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_10() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_11() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_12() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn only_functions_13() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_14() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_15() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_16() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn only_functions_17() {
    test_transform!(test_code_lens_no_runner, r#"
    #[test]
    fn a() {}
    #[test]<caret>
    fn b() {}


    #[test]
    fn c() {}
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_13() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_14() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_15() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_16() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn fn_in_mod_17() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {
        #[test]<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_13() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_14() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_15() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_16() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn run_for_mod_17() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        #[test]
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_13() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_14() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_15() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_16() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn mod_without_test_17() {
    test_transform!(test_code_lens_no_runner, r#"
    mod b {<caret>
        fn a() {}
    }
    "#, @"lenses = []")
}

#[test]
fn complex_13() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_14() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_15() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_16() {
    test_transform!(test_code_lens_no_runner, r#"
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

#[test]
fn complex_17() {
    test_transform!(test_code_lens_no_runner, r#"
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
