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
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "core::circuit::CircuitInputs"
    insert_text = "new_inputs($1)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
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
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "core::circuit::CircuitInputs"
    insert_text = "new_inputs($1)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
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
    detail = "core::ops::deref::Deref"
    insert_text = "deref($1)"

    [[completions]]
    completion_label = "foofoo"
    detail = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "core::circuit::CircuitInputs"
    insert_text = "new_inputs($1)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
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
    detail = "core::clone::Clone"
    insert_text = "clone($1)"

    [[completions]]
    completion_label = "deref()"
    detail = "core::ops::deref::Deref"
    insert_text = "deref($1)"

    [[completions]]
    completion_label = "deref_mut()"
    detail = "core::ops::deref::DerefMut"
    insert_text = "deref_mut($1)"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "destruct()"
    detail = "core::traits::Destruct"
    insert_text = "destruct($1)"

    [[completions]]
    completion_label = "emit()"
    detail = "core::starknet::event::EventEmitter"
    insert_text = "emit($1)"
    text_edits = ["""
    use starknet::event::EventEmitter;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "internal_func()"
    detail = "hello::SpyEventsChecker::InternalTrait"
    insert_text = "internal_func($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "new_inputs()"
    detail = "core::circuit::CircuitInputs"
    insert_text = "new_inputs($1)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "panic_destruct()"
    detail = "core::traits::PanicDestruct"
    insert_text = "panic_destruct($1)"

    [[completions]]
    completion_label = "storage_mut()"
    detail = "core::starknet::storage::storage_base::StorageTraitMut"
    insert_text = "storage_mut($1)"
    text_edits = ["""
    use starknet::storage::StorageTraitMut;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"

    [[completions]]
    completion_label = "x"
    detail = "starknet::storage::StorageBase<starknet::storage::Mutable<u8>>"

    [[completions]]
    completion_label = "y"
    detail = "starknet::storage::StorageBase<starknet::storage::Mutable<felt252>>"
    "#);
}
