use crate::code_actions::{quick_fix, quick_fix_without_visibility_constraints};
use crate::support::insta::test_transform;

#[test]
fn simple() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl of ATrait<felt252> {
        fn some_method(self: @felt252) {}
    }

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>thod();
        }
    }
    ", @r#"
    Title: Import crate::ATrait
    Add new text: "use crate::ATrait;
    "
    At: Range { start: Position { line: 8, character: 4 }, end: Position { line: 8, character: 4 } }
    "#);
}

#[test]
fn simple_after_method_name() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl of ATrait<felt252> {
        fn some_method(self: @felt252) {}
    }

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_method(<caret>);
        }
    }
    ", @"No code actions.");
}

#[test]
fn no_suitable_impl() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>thod();
        }
    }
    ", @"No code actions.");
}

#[test]
fn no_suitable_impl_after_method_name() {
    test_transform!(quick_fix, "
    trait ATrait<T> {
        fn some_method(self: @T);
    }

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_method(<caret>);
        }
    }
    ", @"No code actions.");
}

#[test]
fn two_options() {
    test_transform!(quick_fix, "
    trait ATrait1<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl1 of ATrait1<felt252> {
        fn some_method(self: @felt252) {}
    }
    trait ATrait2<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl2 of ATrait2<felt252> {
        fn some_method(self: @felt252) {}
    }

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>thod();
        }
    }
    ", @r#"
    Title: Import crate::ATrait1
    Add new text: "use crate::ATrait1;
    "
    At: Range { start: Position { line: 14, character: 4 }, end: Position { line: 14, character: 4 } }
    Title: Import crate::ATrait2
    Add new text: "use crate::ATrait2;
    "
    At: Range { start: Position { line: 14, character: 4 }, end: Position { line: 14, character: 4 } }
    "#);
}

#[test]
fn two_options_after_method_name() {
    test_transform!(quick_fix, "
    trait ATrait1<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl1 of ATrait1<felt252> {
        fn some_method(self: @felt252) {}
    }
    trait ATrait2<T> {
        fn some_method(self: @T);
    }
    impl Felt252ATraitImpl2 of ATrait2<felt252> {
        fn some_method(self: @felt252) {}
    }

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_method(<caret>);
        }
    }
    ", @"No code actions.");
}

#[test]
fn non_directly_visible_trait() {
    test_transform!(quick_fix, "
    mod hidden_trait {
        pub trait ATrait1<T> {
            fn some_method(self: @T);
        }
        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }
    use hidden_trait::ATrait1;
    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>thod();
        }
    }
    ", @r#"
    Title: Import crate::ATrait1
    Add new text: "use crate::ATrait1;
    "
    At: Range { start: Position { line: 10, character: 4 }, end: Position { line: 10, character: 4 } }
    "#);
}

#[test]
fn non_directly_visible_trait_after_method_name() {
    test_transform!(quick_fix, "
    mod hidden_trait {
        pub trait ATrait1<T> {
            fn some_method(self: @T);
        }
        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }
    use hidden_trait::ATrait1;
    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_method(<caret>);
        }
    }
    ", @"No code actions.");
}

#[test]
fn from_corelib() {
    test_transform!(quick_fix, "
    fn main() {
        let x = core::pedersen::PedersenTrait::new(5_felt252);
        let _y = x.upda<caret>te(3_felt252);
    }
    ", @r#"
    Title: Import core::hash::HashStateTrait
    Add new text: "use core::hash::HashStateTrait;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}

#[test]
fn from_corelib_after_method_name() {
    test_transform!(quick_fix, "
    fn main() {
        let x = core::pedersen::PedersenTrait::new(5_felt252);
        let _y = x.update(3_felt252<caret>);
    }
    ", @"No code actions.");
}

#[test]
fn from_starknet() {
    test_transform!(quick_fix, "
    #[starknet::interface]
    trait HelloStarknetTrait<TContractState> {
        // Returns the current balance.
        fn get_balance(self: @TContractState) -> usize;
    }

    #[starknet::contract]
    mod hello_starknet {
        #[storage]
        struct Storage {
            balance: usize,
        }

        #[abi(embed_v0)]
        impl HelloStarknetImpl of super::HelloStarknetTrait<ContractState> {
            fn get_balance(self: @ContractState) -> usize {
                self.balance.rea<caret>d()
            }
        }
    }
    ", @r#"
    Title: Import starknet::storage::StoragePointerReadAccess
    Add new text: "use starknet::storage::StoragePointerReadAccess;
    "
    At: Range { start: Position { line: 8, character: 4 }, end: Position { line: 8, character: 4 } }
    "#);
}

#[test]
fn from_starknet_after_method_name() {
    test_transform!(quick_fix, "
    #[starknet::interface]
    trait HelloStarknetTrait<TContractState> {
        // Returns the current balance.
        fn get_balance(self: @TContractState) -> usize;
    }

    #[starknet::contract]
    mod hello_starknet {
        #[storage]
        struct Storage {
            balance: usize,
        }

        #[abi(embed_v0)]
        impl HelloStarknetImpl of super::HelloStarknetTrait<ContractState> {
            fn get_balance(self: @ContractState) -> usize {
                self.balance.read(<caret>)
            }
        }
    }
    ", @"No code actions.");
}

#[test]
fn visible_only_in_editions_without_visibility_constraints() {
    test_transform!(quick_fix_without_visibility_constraints, "
    mod hidden_trait {
        trait ATrait1<T> {
            fn some_method(self: @T);
        }
        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }
    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>thod();
        }
    }
    ", @r#"
    Title: Import crate::hidden_trait::ATrait1
    Add new text: "use crate::hidden_trait::ATrait1;
    "
    At: Range { start: Position { line: 9, character: 4 }, end: Position { line: 9, character: 4 } }
    "#);
}

#[test]
fn visible_only_in_editions_without_visibility_constraints_after_method_name() {
    test_transform!(quick_fix_without_visibility_constraints, "
    mod hidden_trait {
        trait ATrait1<T> {
            fn some_method(self: @T);
        }
        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }
    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_method(<caret>);
        }
    }
    ", @"No code actions.");
}

#[test]
fn from_corelib_visible_only_in_editions_without_visibility_constraints() {
    test_transform!(quick_fix_without_visibility_constraints, "
    fn func() {
        // This is a method from a trait from `core` that is `pub (crate)`.
        let (_x, _): (u8, bool) = 5_i8.abs_an<caret>d_sign();
    }
    ", @r#"
    Title: Import integer::AbsAndSign
    Add new text: "use integer::AbsAndSign;
    "
    At: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }
    "#);
}

#[test]
fn from_corelib_visible_only_in_editions_without_visibility_constraints_after_method_name() {
    test_transform!(quick_fix_without_visibility_constraints, "
    fn func() {
        // This is a method from a trait from `core` that is `pub (crate)`.
        let (_x, _): (u8, bool) = 5_i8.abs_and_sign(<caret>);
    }
    ", @"No code actions.");
}
