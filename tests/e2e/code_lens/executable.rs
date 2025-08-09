use crate::code_lens::test_code_lens_scarb_execute;
use indoc::indoc;

#[test]
fn test_one_executable_whole_package() {
    insta::assert_snapshot!(test_code_lens_scarb_execute((
        r#"
        #[executable]<caret>
        fn main(){
            // Do stuff
        }
        "#,
        indoc!(
            r#"
            [package]
            name = "indor"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            cairo_execute = "2.9.0"

            [executable]
            "#
        )
    )));
}

#[test]
fn test_two_executables_whole_package() {
    insta::assert_snapshot!(test_code_lens_scarb_execute((
        r#"
        #[executable]<caret>
        fn main(){
            // Do stuff
        }
        #[executable]
        fn main2(){
            // Do stuff
        }
        "#,
        indoc!(
            r#"
            [package]
            name = "indor"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            cairo_execute = "2.9.0"

            [executable]
            "#
        )
    )));
}

#[test]
fn test_two_executables_one_precise_def() {
    insta::assert_snapshot!(test_code_lens_scarb_execute((
        r#"
        #[executable]<caret>
        fn main(){
            // Do stuff
        }
        #[executable]
        fn main2(){
            // Do stuff
        }
        "#,
        indoc!(
            r#"
            [package]
            name = "indor"
            version = "0.1.0"
            edition = "2024_07"

            [dependencies]
            cairo_execute = "2.9.0"

            [[target.executable]]
            function = "indor::main2"
            "#
        )
    )));
}
