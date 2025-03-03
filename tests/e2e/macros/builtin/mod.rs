use indoc::indoc;

use crate::macros::{fixtures::ProjectWithCairoProjectToml, test_macro_expansion_and_diagnostics};

#[test]
fn no_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCairoProjectToml,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() { <caret> }
            "#)
        }
    );
}

#[test]
fn inline() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCairoProjectToml,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn fib<caret>(mut n: u32) -> u32 {
                   <caret>  pr<caret>intln!<caret>("some text")<caret>;<caret>
                    let mut a: u32 = 0;
                    let mut b: u32 = 1;
                    while n != 0<caret> {
                        n = n - 1;
                        let temp = b;
                        b = a + b;
                        a = t<caret>emp;
                    };
                    a
                }
            "#)
        }
    );
}

#[test]
fn inline_assert_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCairoProjectToml,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[cfg(test)]
                mod tests {
                    #[test]
                    fn test_nothing() {
                        assert<caret>!(1 == 1, "Failure message");
                    }
                }
            "#)
        }
    );
}

#[test]
fn attribute_with_inline() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCairoProjectToml,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive(Drop)]
                struct A {
                    a: felt252
                }

                #[genera<caret>te_trait]
                impl ImplA of A<caret>Trait {
                    fn c(self: A) {
                        print<caret>ln!("a");<caret>
                        self.a + 1;
                    }
                }
            "#)
        }
    );
}

#[test]
fn derive() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCairoProjectToml,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[d<caret>erive(Drop,<caret> Serde)]<caret>
                struct <caret>A {
                    a: felt252
                }
            "#)
        }
    );
}
