use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn simple_struct() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {
        bar: felt252
    }

    fn test() {
        let foo = Foo {
            bar: 123
        };

        foo.<caret>
    }
    ",
    @r#"
    caret = """
        foo.<caret>
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    detail = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
    "#);
}

#[test]
fn simple_struct_semicolon() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {
        bar: felt252
    }

    fn test() {
        let foo = Foo {
            bar: 123
        };

        foo.<caret>;
    }
    ",
    @r#"
    caret = """
        foo.<caret>;
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    detail = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
    "#);
}

#[test]
fn with_deref() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {
        bar: felt252
    }

    struct Baz {
        foofoo: felt252
    }

    impl FooDeref of Deref<Foo> {
        type Target = Baz;

        fn deref(self: Foo) -> T {
            Baz {
                foofoo: self.bar,
            }
        }
    }

    fn test() {
        let foo = Foo {
            bar: 123
        };

        foo.<caret>;
    }
    ",
    @r#"
    caret = """
        foo.<caret>;
    """

    [[completions]]
    completion_label = "bar"
    detail = "felt252"

    [[completions]]
    completion_label = "deref()"
    detail = "fn(self: T) -> Self::Target"
    insert_text = "deref()"

    [[completions]]
    completion_label = "foofoo"
    detail = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    detail = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
    "#);
}

#[test]
fn with_deref_starknet() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    #[starknet::contract]
    pub mod SpyEventsChecker {
        #[storage]
        struct Storage {
            x: u8,
            y: felt252,
        }

        #[derive(Drop, starknet::Event)]
        pub struct FirstEvent {
            pub some_data: felt252,
        }

        #[external(v0)]
        pub fn emit_one_event(ref self: ContractState, some_data: felt252) {
            self.<caret>
        }

        #[generate_trait]
        impl InternalImpl of InternalTrait {
            fn internal_func(ref self: ContractState) {
                self.emit_one_event(5);
            }
        }
    }
    ",
    @r#"
    caret = """
            self.<caret>
    """

    [[completions]]
    completion_label = "clone()"
    detail = "fn(self: @T) -> T"
    insert_text = "clone()"

    [[completions]]
    completion_label = "deref()"
    detail = "fn(self: T) -> Self::Target"
    insert_text = "deref()"

    [[completions]]
    completion_label = "deref_mut()"
    detail = "fn(ref self: T) -> Self::Target"
    insert_text = "deref_mut()"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "destruct()"
    detail = "fn(self: T) -> () nopanic"
    insert_text = "destruct()"

    [[completions]]
    completion_label = "emit()"
    detail = "fn(ref self: T, event: S) -> ()"
    insert_text = "emit(${1:event})"
    text_edits = ["""
    use starknet::event::EventEmitter;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "internal_func()"
    detail = "fn(ref self: ContractState) -> ()"
    insert_text = "internal_func()"

    [[completions]]
    completion_label = "into()"
    detail = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "panic_destruct()"
    detail = "fn(self: T, ref panic: Panic) -> () nopanic"
    insert_text = "panic_destruct(${1:panic})"

    [[completions]]
    completion_label = "storage_mut()"
    detail = "fn(self: FlattenedStorage<Mutable<T>>) -> Self::BaseType"
    insert_text = "storage_mut()"
    text_edits = ["""
    use starknet::storage::StorageTraitMut;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"

    [[completions]]
    completion_label = "x"
    detail = "starknet::storage::StorageBase<starknet::storage::Mutable<u8>>"

    [[completions]]
    completion_label = "y"
    detail = "starknet::storage::StorageBase<starknet::storage::Mutable<felt252>>"
    "#);
}

#[test]
fn with_mutable_self() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    #[starknet::interface]
    pub trait IHelloStarknet<TContractState> {
        fn increase_balance(ref self: TContractState, amount: felt252);
        fn get_balance(self: @TContractState) -> felt252;
    }

    #[starknet::contract]
    mod HelloStarknet {
        use crate::IHelloStarknet;
        use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};

        #[storage]
        struct Storage {
            balance: felt252,
        }

        #[abi(embed_v0)]
        impl HelloStarknetImpl of super::IHelloStarknet<ContractState> {
            fn increase_balance(ref self: ContractState, amount: felt252) {
                self.ge<caret>
                assert(amount != 0, 'Amount cannot be 0');
                self.balance.write(self.balance.read() + amount);
            }

            fn get_balance(self: @ContractState) -> felt252 {
                self.balance.read()
            }
        }
    }
    ",
    @r#"
    caret = """
                self.ge<caret>
    """

    [[completions]]
    completion_label = "balance"
    detail = "starknet::storage::StorageBase<starknet::storage::Mutable<felt252>>"

    [[completions]]
    completion_label = "get_balance()"
    detail = "fn(self: @TContractState) -> felt252"
    insert_text = "get_balance()"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"
    "#);
}
