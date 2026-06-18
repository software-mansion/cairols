use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
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
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "foofoo"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "deref()"
    completion_label_type_info = "fn(self: T) -> Self::Target"
    insert_text = "deref()"

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
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
    completion_label = "x"
    completion_label_type_info = "starknet::storage::StorageBase<starknet::storage::Mutable<u8>>"

    [[completions]]
    completion_label = "y"
    completion_label_type_info = "starknet::storage::StorageBase<starknet::storage::Mutable<felt252>>"

    [[completions]]
    completion_label = "clone()"
    completion_label_type_info = "fn(self: @T) -> T"
    insert_text = "clone()"

    [[completions]]
    completion_label = "deref()"
    completion_label_type_info = "fn(self: T) -> Self::Target"
    insert_text = "deref()"

    [[completions]]
    completion_label = "deref_mut()"
    completion_label_type_info = "fn(ref self: T) -> Self::Target"
    insert_text = "deref_mut()"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "destruct()"
    completion_label_type_info = "fn(self: T) -> () nopanic"
    insert_text = "destruct()"

    [[completions]]
    completion_label = "emit()"
    completion_label_type_info = "fn(ref self: T, event: S) -> ()"
    insert_text = "emit(${1:event})"
    text_edits = ["""
    use starknet::event::EventEmitter;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "internal_func()"
    completion_label_type_info = "fn(ref self: ContractState) -> ()"
    insert_text = "internal_func()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "panic_destruct()"
    completion_label_type_info = "fn(self: T, ref panic: Panic) -> () nopanic"
    insert_text = "panic_destruct(${1:panic})"

    [[completions]]
    completion_label = "storage_mut()"
    completion_label_type_info = "fn(self: FlattenedStorage<Mutable<T>>) -> Self::BaseType"
    insert_text = "storage_mut()"
    text_edits = ["""
    use starknet::storage::StorageTraitMut;

    """]

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
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
    completion_label_type_info = "starknet::storage::StorageBase<starknet::storage::Mutable<felt252>>"

    [[completions]]
    completion_label = "get_balance()"
    completion_label_type_info = "fn(self: @TContractState) -> felt252"
    insert_text = "get_balance()"

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"
    "#);
}

// FIXME(#589): This test should yield a literal instead of constant when resolver is fixed
#[test]
fn with_const_parametrized_generic_function() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    mod impler {
        pub struct S {}

        pub struct ConstParamStruct<const CONSTANT: felt252>  {
            pub name: felt252
        }

        trait ConstFunction<T> {
            fn hehe<const name: felt252>(self: T) -> ConstParamStruct<'const    \n\tvalue'>;
        }

        impl SConstFunction of ConstFunction<S> {
            fn hehe<'const    \n\tvalue'>(self: S) -> ConstParamStruct<'const    \n\tvalue'> { 123 }
        }
    }

    use impler::S;

    fn test() {
        let s = S{};
        s.he<caret>
    }
    ",
    @r#"
    caret = """
        s.he<caret>
    """

    [[completions]]
    completion_label = "hehe()"
    completion_label_type_info = "fn(self: T) -> ConstParamStruct<132172156746238226582224867971537073509>"
    insert_text = "hehe()"
    "#);
}

#[test]
fn with_not_imported_return_type() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    mod outer {
        pub mod inner {
            pub struct Long {}
        }
    }

    mod impler {
        pub struct S {}

        use super::outer::inner::Long;

        trait ReturnLong<T> {
            fn make(self: T) -> Long;
        }

        impl SReturnLong of ReturnLong<S> {
            fn make(self: S) -> Long { Long {} }
        }
    }

    use impler::S;

    fn test() {
        let s = S{};
        s.<caret>
    }
    ",
    @r#"
    caret = """
        s.<caret>
    """

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T) -> Long"
    insert_text = "make()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;
    """]

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
    "#);
}

#[test]
fn with_already_typed_parens() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.m<caret>();
    }
    ",
    @r#"
    caret = """
        s.m<caret>();
    """

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"
    "#);
}

#[test]
fn with_already_typed_parens_and_no_method_chars() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.<caret>();
    }
    ",
    @r#"
    caret = """
        s.<caret>();
    """

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
    "#);
}

#[test]
fn with_already_typed_parens_and_caret_inside() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.m(<caret>);
    }
    ",
    @r#"
    caret = """
        s.m(<caret>);
    """
    completions = []
    "#);
}

#[test]
fn with_already_typed_arg_and_caret_inside() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.m(37<caret>);
    }
    ",
    @r#"
    caret = """
        s.m(37<caret>);
    """
    completions = []
    "#);
}

#[test]
fn with_already_typed_parens_and_caret_after() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.m()<caret>;
    }
    ",
    @r#"
    caret = """
        s.m()<caret>;
    """
    completions = []
    "#);
}

#[test]
fn with_already_typed_incomplete_parens_and_caret_before() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.m<caret>(;
    }
    ",
    @r#"
    caret = """
        s.m<caret>(;
    """

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"
    "#);
}

#[test]
fn with_already_typed_incomplete_parens_and_caret_after() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        s.m(<caret>;
    }
    ",
    @r#"
    caret = """
        s.m(<caret>;
    """
    completions = []
    "#);
}

#[test]
fn with_nested_binary_expressions_and_caret_afterc() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    struct S2 {}

    trait ReturnLong2<T> {
        fn make(self: T, long: Long) -> Long;
    }

    impl SReturnLong2 of ReturnLong2<S2> {
        fn make(self: S2, long: Long) -> Long { Long {} }
    }

    fn test() {
        let s = S{};
        let s2 = S2{};
        s2.make(s.m<caret>());
    }
    ",
    @r#"
    caret = """
        s2.make(s.m<caret>());
    """

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"
    "#);
}

#[test]
fn with_already_typed_parens_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        s.m<caret>();
    }
    ",
    @r#"
    caret = """
        s.m<caret>();
    """

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"
    "#);
}

#[test]
fn with_already_typed_parens_and_caret_inside_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        s.m(<caret>);
    }
    ",
    @r#"
    caret = """
        s.m(<caret>);
    """
    completions = []
    "#);
}

#[test]
fn with_already_typed_arg_and_caret_inside_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        s.m(37<caret>);
    }
    ",
    @r#"
    caret = """
        s.m(37<caret>);
    """
    completions = []
    "#);
}

#[test]
fn with_already_typed_parens_and_caret_after_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        s.m()<caret>;
    }
    ",
    @r#"
    caret = """
        s.m()<caret>;
    """
    completions = []
    "#);
}

#[test]
fn with_already_typed_incomplete_parens_and_caret_before_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        s.m<caret>(;
    }
    ",
    @r#"
    caret = """
        s.m<caret>(;
    """

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"
    "#);
}

#[test]
fn with_already_typed_incomplete_parens_and_caret_after_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        s.m(<caret>;
    }
    ",
    @r#"
    caret = """
        s.m(<caret>;
    """
    completions = []
    "#);
}

#[test]
fn with_nested_binary_expressions_and_caret_after_with_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),
    "
    struct Long {}

    struct S {}

    trait ReturnLong<T> {
        fn make(self: T, a: u32) -> Long;
    }

    impl SReturnLong of ReturnLong<S> {
        fn make(self: S, a: u32) -> Long { Long {} }
    }

    struct S2 {}

    trait ReturnLong2<T> {
        fn make(self: T, long: Long) -> Long;
    }

    impl SReturnLong2 of ReturnLong2<S2> {
        fn make(self: S2, long: Long) -> Long { Long {} }
    }

    #[complex_attribute_macro_v2]
    fn test() {
        let s = S{};
        let s2 = S2{};
        s2.make(s.m<caret>());
    }
    ",
    @r#"
    caret = """
        s2.make(s.m<caret>());
    """

    [[completions]]
    completion_label = "make()"
    completion_label_type_info = "fn(self: T, a: u32) -> Long"
    insert_text = "make(${1:a})"
    "#);
}


// Regression tests for https://github.com/software-mansion/cairols/issues/1314

#[test]
fn pub_impl_method_dot_completion() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {}

    trait Greet<T> {
        fn hello(self: T) -> felt252;
    }

    pub impl FooGreet of Greet<Foo> {
        fn hello(self: Foo) -> felt252 { 0 }
    }

    fn test() {
        let f = Foo {};
        f.he<caret>
    }
    ",
    @r#"
    caret = """
        f.he<caret>
    """

    [[completions]]
    completion_label = "hello()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "hello()"
    "#);
}

#[test]
fn pub_impl_method_dot_completion_from_submodule() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    mod impls {
        pub struct Foo {}

        pub trait Greet<T> {
            fn hello(self: T) -> felt252;
        }

        pub impl FooGreet of Greet<Foo> {
            fn hello(self: Foo) -> felt252 { 0 }
        }
    }

    fn test() {
        let f = impls::Foo {};
        f.he<caret>
    }
    ",
    @r#"
    caret = """
        f.he<caret>
    """

    [[completions]]
    completion_label = "hello()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "hello()"
    text_edits = ["""
    use impls::Greet;

    """]
    "#);
}

#[test]
fn pub_impl_method_dot_completion_multiple_methods() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Counter {}

    trait CounterTrait<T> {
        fn increment(self: T) -> felt252;
        fn decrement(self: T) -> felt252;
        fn reset(self: T) -> felt252;
    }

    pub impl CounterImpl of CounterTrait<Counter> {
        fn increment(self: Counter) -> felt252 { 1 }
        fn decrement(self: Counter) -> felt252 { 2 }
        fn reset(self: Counter) -> felt252 { 0 }
    }

    fn test() {
        let c = Counter {};
        c.<caret>
    }
    ",
    @r#"
    caret = """
        c.<caret>
    """

    [[completions]]
    completion_label = "decrement()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "decrement()"

    [[completions]]
    completion_label = "get_descriptor()"
    completion_label_type_info = "fn(self: CES) -> CircuitDescriptor<CD::CircuitType>"
    insert_text = "get_descriptor()"

    [[completions]]
    completion_label = "increment()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "increment()"

    [[completions]]
    completion_label = "into()"
    completion_label_type_info = "fn(self: T) -> S"
    insert_text = "into()"

    [[completions]]
    completion_label = "new_inputs()"
    completion_label_type_info = "fn(self: CES) -> AddInputResult<CD::CircuitType>"
    insert_text = "new_inputs()"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "reset()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "reset()"

    [[completions]]
    completion_label = "try_into()"
    completion_label_type_info = "fn(self: T) -> Option<S>"
    insert_text = "try_into()"
    "#);
}

#[test]
fn pub_impl_generic_with_constraints_dot_completion() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    trait Sortable<T> {
        fn sort(self: T) -> felt252;
    }

    pub impl SortImpl<T, +Copy<T>, +Drop<T>> of Sortable<T> {
        fn sort(self: T) -> felt252 { 0 }
    }

    fn test() {
        let x: felt252 = 5;
        x.so<caret>
    }
    ",
    @r#"
    caret = """
        x.so<caret>
    """

    [[completions]]
    completion_label = "is_non_one()"
    completion_label_type_info = "fn(self: @T) -> bool"
    insert_text = "is_non_one()"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    completion_label_type_info = "fn(self: @T) -> bool"
    insert_text = "is_non_zero()"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    completion_label_type_info = "fn(self: T) -> bool"
    insert_text = "is_non_zero()"

    [[completions]]
    completion_label = "is_one()"
    completion_label_type_info = "fn(self: @T) -> bool"
    insert_text = "is_one()"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "sort()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "sort()"
    "#);
}

#[test]
fn impl_generic_with_constraints_dot_completion() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    trait Sortable<T> {
        fn sort(self: T) -> felt252;
    }

    impl SortImpl<T, +Copy<T>, +Drop<T>> of Sortable<T> {
        fn sort(self: T) -> felt252 { 0 }
    }

    fn test() {
        let x: felt252 = 5;
        x.so<caret>
    }
    ",
    @r#"
    caret = """
        x.so<caret>
    """

    [[completions]]
    completion_label = "is_non_one()"
    completion_label_type_info = "fn(self: @T) -> bool"
    insert_text = "is_non_one()"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    completion_label_type_info = "fn(self: @T) -> bool"
    insert_text = "is_non_zero()"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    completion_label_type_info = "fn(self: T) -> bool"
    insert_text = "is_non_zero()"

    [[completions]]
    completion_label = "is_one()"
    completion_label_type_info = "fn(self: @T) -> bool"
    insert_text = "is_one()"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "sort()"
    completion_label_type_info = "fn(self: T) -> felt252"
    insert_text = "sort()"
    "#);
}

#[test]
fn pub_impl_concrete_dot_completion() {
    test_transform_plain!(Completion, completion_fixture(),
    "
    struct Foo {}

    trait Sortable {
        fn sort(self: Foo) -> felt252;
    }

    pub impl SortImpl of Sortable {
        fn sort(self: Foo) -> felt252 { 0 }
    }

    fn test() {
        let x = Foo {};
        x.so<caret>
    }
    ",
    @r#"
    caret = """
        x.so<caret>
    """

    [[completions]]
    completion_label = "sort()"
    completion_label_type_info = "fn(self: Foo) -> felt252"
    insert_text = "sort()"
    "#);
}
