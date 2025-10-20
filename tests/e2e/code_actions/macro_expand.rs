use crate::code_actions::{quick_fix, quick_fix_with_macros};
use crate::support::insta::test_transform;

#[test]
fn derive_ident() {
    test_transform!(quick_fix, "
        #[deri<caret>ve(Drop, Serde)]
        struct A {
            a: felt252
        }
    ", @"Title: Recursively expand macros for item at caret");
}

#[test]
fn derived_struct_ident() {
    test_transform!(quick_fix, "
        #[derive(Drop, Serde)]
        st<caret>ruct A {
            a: felt252
        }
    ", @"No code actions.");
}

#[test]
fn macro_ident() {
    test_transform!(quick_fix,"
        #[generat<caret>e_trait]
        impl ImplA of ATrait {
            fn c(self: A) -> felt252 {
                123
            }
    }
    ", @"Title: Recursively expand macros for item at caret")
}

#[test]
fn macro_affected_impl_ident() {
    test_transform!(quick_fix,"
        #[generate_trait]
        impl Imp<caret>lA of ATrait {
            fn c(self: A) -> felt252 {
                123
            }
    }
    ", @"No code actions.")
}

#[test]
fn macro_affected_impl_after_impl_block() {
    test_transform!(quick_fix,"
        #[generate_trait]
        impl ImplA of ATrait {
            fn c(self: A) -> felt252 {
                123
            }
        }<caret>
    ", @"No code actions.")
}

#[test]
fn inline_macro_inside_ident() {
    test_transform!(quick_fix, r#"
    fn c() {
        print<caret>ln!("a");
    }
    "#, @"Title: Expand macro recursively at caret")
}

#[test]
fn inline_macro_after_ident() {
    test_transform!(quick_fix, r#"
    fn c() {
        println<caret>!("a");
    }
    "#, @"Title: Expand macro recursively at caret")
}

#[test]
fn inline_macro_inside_macro_controlled_code() {
    test_transform!(quick_fix_with_macros, r#"
    #[test]
    fn test_smthing() {
        println<caret>!("a");
    }
    "#, @"Title: Expand macro recursively at caret")
}
