use crate::macros::{fixtures::ProjectWithCairoProjectToml, test_macro_expansion_and_diagnostics};

#[test]
fn inline() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCairoProjectToml,
        r#"
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
        "#
    );
}

#[test]
fn inline_assert_macro() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCairoProjectToml,
        r#"
        #[cfg(test)]
        mod test {
            #[test]
            fn test() {
                assert!(1 == 1, "Failure message");
            }
        }
        "#
    );
}

#[test]
fn attribute_with_inline() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCairoProjectToml,
        r#"
        struct A {
            a: felt252
        }

        #[genera<caret>te_trait]
        impl ImplA of A<caret>Trait {
            fn c(self: A) -> felt252 {
                print<caret>ln!("a");<caret>
                self.a + 1
            }
        }
        "#
    );
}

#[test]
fn derive() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCairoProjectToml,
        r#"
        #[d<caret>erive(Drop,<caret> Serde)]<caret>
        struct <caret>A {
            a: felt252
        }
        "#
    );
}

#[test]
fn starknet_interface() {
    test_macro_expansion_and_diagnostics!(
        ProjectWithCairoProjectToml,
        r#"
        #[interface]<caret>
        pub trait Contract<State> {
            fn foo<caret>(self: @State);<caret>
        }
        "#
    );
}
