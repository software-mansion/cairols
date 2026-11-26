use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn let_type_annotation_empty() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a() {
        let a: <caret> = 123;
    }
    ",@r#"
    caret = """
        let a: <caret> = 123;
    """

    [[completions]]
    completion_label = "Array"

    [[completions]]
    completion_label = "Box"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "NonZero"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "Option"

    [[completions]]
    completion_label = "Panic"

    [[completions]]
    completion_label = "PanicResult"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "Span"

    [[completions]]
    completion_label = "System"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "bytes31"

    [[completions]]
    completion_label = "felt252"

    [[completions]]
    completion_label = "i128"

    [[completions]]
    completion_label = "i16"

    [[completions]]
    completion_label = "i32"

    [[completions]]
    completion_label = "i64"

    [[completions]]
    completion_label = "i8"

    [[completions]]
    completion_label = "u128"

    [[completions]]
    completion_label = "u16"

    [[completions]]
    completion_label = "u256"

    [[completions]]
    completion_label = "u32"

    [[completions]]
    completion_label = "u64"

    [[completions]]
    completion_label = "u8"

    [[completions]]
    completion_label = "usize"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "AccountContractDispatcher"
    completion_label_path = "(use starknet::account::AccountContractDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcher"
    completion_label_path = "(use starknet::account::AccountContractLibraryDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractLibraryDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractLibraryDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeDispatcher"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractSafeDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcher"
    completion_label_path = "(use starknet::account::AccountContractSafeLibraryDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractSafeLibraryDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    completion_label_path = "(use core::circuit::AddInputResult)"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddMod"
    completion_label_path = "(use core::circuit::AddMod)"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "ArrayIter"
    completion_label_path = "(use core::array::ArrayIter)"
    text_edits = ["""
    use core::array::ArrayIter;

    """]

    [[completions]]
    completion_label = "Bitwise"
    completion_label_path = "(use core::integer::Bitwise)"
    text_edits = ["""
    use core::integer::Bitwise;

    """]

    [[completions]]
    completion_label = "BlockInfo"
    completion_label_path = "(use starknet::BlockInfo)"
    text_edits = ["""
    use starknet::BlockInfo;

    """]

    [[completions]]
    completion_label = "BoundedInt"
    completion_label_path = "(use core::internal::bounded_int::BoundedInt)"
    text_edits = ["""
    use core::internal::bounded_int::BoundedInt;

    """]

    [[completions]]
    completion_label = "BuiltinCosts"
    completion_label_path = "(use core::gas::BuiltinCosts)"
    text_edits = ["""
    use core::gas::BuiltinCosts;

    """]

    [[completions]]
    completion_label = "ByteArrayIter"
    completion_label_path = "(use core::byte_array::ByteArrayIter)"
    text_edits = ["""
    use core::byte_array::ByteArrayIter;

    """]

    [[completions]]
    completion_label = "ByteSpan"
    completion_label_path = "(use core::byte_array::ByteSpan)"
    text_edits = ["""
    use core::byte_array::ByteSpan;

    """]

    [[completions]]
    completion_label = "ByteSpanIter"
    completion_label_path = "(use core::byte_array::ByteSpanIter)"
    text_edits = ["""
    use core::byte_array::ByteSpanIter;

    """]

    [[completions]]
    completion_label = "Call"
    completion_label_path = "(use starknet::account::Call)"
    text_edits = ["""
    use starknet::account::Call;

    """]

    [[completions]]
    completion_label = "Circuit"
    completion_label_path = "(use core::circuit::Circuit)"
    text_edits = ["""
    use core::circuit::Circuit;

    """]

    [[completions]]
    completion_label = "CircuitElement"
    completion_label_path = "(use core::circuit::CircuitElement)"
    text_edits = ["""
    use core::circuit::CircuitElement;

    """]

    [[completions]]
    completion_label = "CircuitInput"
    completion_label_path = "(use core::circuit::CircuitInput)"
    text_edits = ["""
    use core::circuit::CircuitInput;

    """]

    [[completions]]
    completion_label = "CircuitModulus"
    completion_label_path = "(use core::circuit::CircuitModulus)"
    text_edits = ["""
    use core::circuit::CircuitModulus;

    """]

    [[completions]]
    completion_label = "ClassHash"
    completion_label_path = "(use starknet::ClassHash)"
    text_edits = ["""
    use starknet::ClassHash;

    """]

    [[completions]]
    completion_label = "ConstOne"
    completion_label_path = "(use core::circuit::ConstOne)"
    text_edits = ["""
    use core::circuit::ConstOne;

    """]

    [[completions]]
    completion_label = "ConstZero"
    completion_label_path = "(use core::circuit::ConstZero)"
    text_edits = ["""
    use core::circuit::ConstZero;

    """]

    [[completions]]
    completion_label = "ContractAddress"
    completion_label_path = "(use starknet::ContractAddress)"
    text_edits = ["""
    use starknet::ContractAddress;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "(use starknet::deployment::DeploymentParams)"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "(use core::internal::DestructWith)"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "(use core::internal::DropWith)"
    text_edits = ["""
    use core::internal::DropWith;

    """]

    [[completions]]
    completion_label = "EcOp"
    completion_label_path = "(use core::ec::EcOp)"
    text_edits = ["""
    use core::ec::EcOp;

    """]

    [[completions]]
    completion_label = "EcPoint"
    completion_label_path = "(use core::ec::EcPoint)"
    text_edits = ["""
    use core::ec::EcPoint;

    """]

    [[completions]]
    completion_label = "EcState"
    completion_label_path = "(use core::ec::EcState)"
    text_edits = ["""
    use core::ec::EcState;

    """]

    [[completions]]
    completion_label = "Error"
    completion_label_path = "(use core::fmt::Error)"
    text_edits = ["""
    use core::fmt::Error;

    """]

    [[completions]]
    completion_label = "EthAddress"
    completion_label_path = "(use starknet::EthAddress)"
    text_edits = ["""
    use starknet::EthAddress;

    """]

    [[completions]]
    completion_label = "ExecutionInfo"
    completion_label_path = "(use starknet::ExecutionInfo)"
    text_edits = ["""
    use starknet::ExecutionInfo;

    """]

    [[completions]]
    completion_label = "Felt252Dict"
    completion_label_path = "(use core::dict::Felt252Dict)"
    text_edits = ["""
    use core::dict::Felt252Dict;

    """]

    [[completions]]
    completion_label = "Felt252DictEntry"
    completion_label_path = "(use core::dict::Felt252DictEntry)"
    text_edits = ["""
    use core::dict::Felt252DictEntry;

    """]

    [[completions]]
    completion_label = "FlattenedStorage"
    completion_label_path = "(use starknet::storage::FlattenedStorage)"
    text_edits = ["""
    use starknet::storage::FlattenedStorage;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]

    [[completions]]
    completion_label = "FromNullableResult"
    completion_label_path = "(use core::nullable::FromNullableResult)"
    text_edits = ["""
    use core::nullable::FromNullableResult;

    """]

    [[completions]]
    completion_label = "GasBuiltin"
    completion_label_path = "(use core::gas::GasBuiltin)"
    text_edits = ["""
    use core::gas::GasBuiltin;

    """]

    [[completions]]
    completion_label = "GasReserve"
    completion_label_path = "(use core::gas::GasReserve)"
    text_edits = ["""
    use core::gas::GasReserve;

    """]

    [[completions]]
    completion_label = "HashState"
    completion_label_path = "(use core::pedersen::HashState)"
    text_edits = ["""
    use core::pedersen::HashState;

    """]

    [[completions]]
    completion_label = "HashState"
    completion_label_path = "(use core::poseidon::HashState)"
    text_edits = ["""
    use core::poseidon::HashState;

    """]

    [[completions]]
    completion_label = "InferDestruct"
    completion_label_path = "(use core::internal::InferDestruct)"
    text_edits = ["""
    use core::internal::InferDestruct;

    """]

    [[completions]]
    completion_label = "InferDrop"
    completion_label_path = "(use core::internal::InferDrop)"
    text_edits = ["""
    use core::internal::InferDrop;

    """]

    [[completions]]
    completion_label = "LoopResult"
    completion_label_path = "(use core::internal::LoopResult)"
    text_edits = ["""
    use core::internal::LoopResult;

    """]

    [[completions]]
    completion_label = "Map"
    completion_label_path = "(use starknet::storage::Map)"
    text_edits = ["""
    use starknet::storage::Map;

    """]

    [[completions]]
    completion_label = "MulMod"
    completion_label_path = "(use core::circuit::MulMod)"
    text_edits = ["""
    use core::circuit::MulMod;

    """]

    [[completions]]
    completion_label = "Mutable"
    completion_label_path = "(use starknet::storage::Mutable)"
    text_edits = ["""
    use starknet::storage::Mutable;

    """]

    [[completions]]
    completion_label = "NonZeroEcPoint"
    completion_label_path = "(use core::ec::NonZeroEcPoint)"
    text_edits = ["""
    use core::ec::NonZeroEcPoint;

    """]

    [[completions]]
    completion_label = "OptionIter"
    completion_label_path = "(use core::option::OptionIter)"
    text_edits = ["""
    use core::option::OptionIter;

    """]

    [[completions]]
    completion_label = "OptionRev"
    completion_label_path = "(use core::internal::OptionRev)"
    text_edits = ["""
    use core::internal::OptionRev;

    """]

    [[completions]]
    completion_label = "Pedersen"
    completion_label_path = "(use core::pedersen::Pedersen)"
    text_edits = ["""
    use core::pedersen::Pedersen;

    """]

    [[completions]]
    completion_label = "PendingStoragePath"
    completion_label_path = "(use starknet::storage::PendingStoragePath)"
    text_edits = ["""
    use starknet::storage::PendingStoragePath;

    """]

    [[completions]]
    completion_label = "Poseidon"
    completion_label_path = "(use core::poseidon::Poseidon)"
    text_edits = ["""
    use core::poseidon::Poseidon;

    """]

    [[completions]]
    completion_label = "Range"
    completion_label_path = "(use core::ops::Range)"
    text_edits = ["""
    use core::ops::Range;

    """]

    [[completions]]
    completion_label = "RangeCheck"
    completion_label_path = "(use core::RangeCheck)"
    text_edits = ["""
    use core::RangeCheck;

    """]

    [[completions]]
    completion_label = "RangeCheck96"
    completion_label_path = "(use core::circuit::RangeCheck96)"
    text_edits = ["""
    use core::circuit::RangeCheck96;

    """]

    [[completions]]
    completion_label = "RangeInclusive"
    completion_label_path = "(use core::ops::RangeInclusive)"
    text_edits = ["""
    use core::ops::RangeInclusive;

    """]

    [[completions]]
    completion_label = "RangeInclusiveIterator"
    completion_label_path = "(use core::ops::RangeInclusiveIterator)"
    text_edits = ["""
    use core::ops::RangeInclusiveIterator;

    """]

    [[completions]]
    completion_label = "RangeIterator"
    completion_label_path = "(use core::ops::RangeIterator)"
    text_edits = ["""
    use core::ops::RangeIterator;

    """]

    [[completions]]
    completion_label = "ResourceBounds"
    completion_label_path = "(use starknet::ResourceBounds)"
    text_edits = ["""
    use starknet::ResourceBounds;

    """]

    [[completions]]
    completion_label = "Secp256k1Point"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Point)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Point;

    """]

    [[completions]]
    completion_label = "Secp256r1Point"
    completion_label_path = "(use starknet::secp256r1::Secp256r1Point)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1Point;

    """]

    [[completions]]
    completion_label = "SegmentArena"
    completion_label_path = "(use core::SegmentArena)"
    text_edits = ["""
    use core::SegmentArena;

    """]

    [[completions]]
    completion_label = "Signature"
    completion_label_path = "(use starknet::secp256_trait::Signature)"
    text_edits = ["""
    use starknet::secp256_trait::Signature;

    """]

    [[completions]]
    completion_label = "SpanIter"
    completion_label_path = "(use core::array::SpanIter)"
    text_edits = ["""
    use core::array::SpanIter;

    """]

    [[completions]]
    completion_label = "SquashedFelt252Dict"
    completion_label_path = "(use core::dict::SquashedFelt252Dict)"
    text_edits = ["""
    use core::dict::SquashedFelt252Dict;

    """]

    [[completions]]
    completion_label = "StorageAddress"
    completion_label_path = "(use starknet::StorageAddress)"
    text_edits = ["""
    use starknet::StorageAddress;

    """]

    [[completions]]
    completion_label = "StorageBase"
    completion_label_path = "(use starknet::storage::StorageBase)"
    text_edits = ["""
    use starknet::storage::StorageBase;

    """]

    [[completions]]
    completion_label = "StorageBaseAddress"
    completion_label_path = "(use starknet::storage_access::StorageBaseAddress)"
    text_edits = ["""
    use starknet::storage_access::StorageBaseAddress;

    """]

    [[completions]]
    completion_label = "StoragePath"
    completion_label_path = "(use starknet::storage::StoragePath)"
    text_edits = ["""
    use starknet::storage::StoragePath;

    """]

    [[completions]]
    completion_label = "StoragePointer"
    completion_label_path = "(use starknet::storage::StoragePointer)"
    text_edits = ["""
    use starknet::storage::StoragePointer;

    """]

    [[completions]]
    completion_label = "StoragePointer0Offset"
    completion_label_path = "(use starknet::storage::StoragePointer0Offset)"
    text_edits = ["""
    use starknet::storage::StoragePointer0Offset;

    """]

    [[completions]]
    completion_label = "SyscallResult"
    completion_label_path = "(use starknet::SyscallResult)"
    text_edits = ["""
    use starknet::SyscallResult;

    """]

    [[completions]]
    completion_label = "TxInfo"
    completion_label_path = "(use starknet::TxInfo)"
    text_edits = ["""
    use starknet::TxInfo;

    """]

    [[completions]]
    completion_label = "U128MulGuarantee"
    completion_label_path = "(use core::integer::U128MulGuarantee)"
    text_edits = ["""
    use core::integer::U128MulGuarantee;

    """]

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]

    [[completions]]
    completion_label = "Vec"
    completion_label_path = "(use starknet::storage::Vec)"
    text_edits = ["""
    use starknet::storage::Vec;

    """]

    [[completions]]
    completion_label = "VecIter"
    completion_label_path = "(use starknet::storage::VecIter)"
    text_edits = ["""
    use starknet::storage::VecIter;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "qm31"
    completion_label_path = "(use core::qm31::qm31)"
    text_edits = ["""
    use core::qm31::qm31;

    """]

    [[completions]]
    completion_label = "u2"
    completion_label_path = "(use core::sha256::u2)"
    text_edits = ["""
    use core::sha256::u2;

    """]

    [[completions]]
    completion_label = "u3"
    completion_label_path = "(use core::sha384::u3)"
    text_edits = ["""
    use core::sha384::u3;

    """]

    [[completions]]
    completion_label = "u384"
    completion_label_path = "(use core::circuit::u384)"
    text_edits = ["""
    use core::circuit::u384;

    """]

    [[completions]]
    completion_label = "u512"
    completion_label_path = "(use core::integer::u512)"
    text_edits = ["""
    use core::integer::u512;

    """]

    [[completions]]
    completion_label = "u96"
    completion_label_path = "(use core::circuit::u96)"
    text_edits = ["""
    use core::circuit::u96;

    """]
    "#);
}

#[test]
fn let_type_annotation_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    fn a() {
        let a: Foo<caret> = 123;
    }
    ",@r#"
    caret = """
        let a: Foo<caret> = 123;
    """

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn let_type_annotation_module_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub struct FooStruct {}
        pub const FooConst: u32 = 0;
        pub fn FooFn() {}
        pub trait FooTrait {}
        pub mod nested {}
    }

    fn a() {
        let a: module::<caret> = 123;
    }
    ",@r#"
    caret = """
        let a: module::<caret> = 123;
    """

    [[completions]]
    completion_label = "FooStruct"
    completion_label_path = "(use module::FooStruct)"
    "#);
}

#[test]
fn return_type_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    fn a() -> Foo<caret> {}
    ",@r#"
    caret = """
    fn a() -> Foo<caret> {}
    """

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn return_type_empty() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a() -> <caret> {}
    ",@r#"
    caret = """
    fn a() -> <caret> {}
    """

    [[completions]]
    completion_label = "Array"

    [[completions]]
    completion_label = "Box"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "NonZero"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "Option"

    [[completions]]
    completion_label = "Panic"

    [[completions]]
    completion_label = "PanicResult"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "Span"

    [[completions]]
    completion_label = "System"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "bytes31"

    [[completions]]
    completion_label = "felt252"

    [[completions]]
    completion_label = "i128"

    [[completions]]
    completion_label = "i16"

    [[completions]]
    completion_label = "i32"

    [[completions]]
    completion_label = "i64"

    [[completions]]
    completion_label = "i8"

    [[completions]]
    completion_label = "u128"

    [[completions]]
    completion_label = "u16"

    [[completions]]
    completion_label = "u256"

    [[completions]]
    completion_label = "u32"

    [[completions]]
    completion_label = "u64"

    [[completions]]
    completion_label = "u8"

    [[completions]]
    completion_label = "usize"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "AccountContractDispatcher"
    completion_label_path = "(use starknet::account::AccountContractDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcher"
    completion_label_path = "(use starknet::account::AccountContractLibraryDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractLibraryDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractLibraryDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractLibraryDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeDispatcher"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractSafeDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcher"
    completion_label_path = "(use starknet::account::AccountContractSafeLibraryDispatcher)"
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcher;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointers"
    completion_label_path = "(use starknet::account::AccountContractSafeLibraryDispatcherSubPointers)"
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointers;

    """]

    [[completions]]
    completion_label = "AccountContractSafeLibraryDispatcherSubPointersMut"
    completion_label_path = "(use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut)"
    text_edits = ["""
    use starknet::account::AccountContractSafeLibraryDispatcherSubPointersMut;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    completion_label_path = "(use core::circuit::AddInputResult)"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddMod"
    completion_label_path = "(use core::circuit::AddMod)"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "ArrayIter"
    completion_label_path = "(use core::array::ArrayIter)"
    text_edits = ["""
    use core::array::ArrayIter;

    """]

    [[completions]]
    completion_label = "Bitwise"
    completion_label_path = "(use core::integer::Bitwise)"
    text_edits = ["""
    use core::integer::Bitwise;

    """]

    [[completions]]
    completion_label = "BlockInfo"
    completion_label_path = "(use starknet::BlockInfo)"
    text_edits = ["""
    use starknet::BlockInfo;

    """]

    [[completions]]
    completion_label = "BoundedInt"
    completion_label_path = "(use core::internal::bounded_int::BoundedInt)"
    text_edits = ["""
    use core::internal::bounded_int::BoundedInt;

    """]

    [[completions]]
    completion_label = "BuiltinCosts"
    completion_label_path = "(use core::gas::BuiltinCosts)"
    text_edits = ["""
    use core::gas::BuiltinCosts;

    """]

    [[completions]]
    completion_label = "ByteArrayIter"
    completion_label_path = "(use core::byte_array::ByteArrayIter)"
    text_edits = ["""
    use core::byte_array::ByteArrayIter;

    """]

    [[completions]]
    completion_label = "ByteSpan"
    completion_label_path = "(use core::byte_array::ByteSpan)"
    text_edits = ["""
    use core::byte_array::ByteSpan;

    """]

    [[completions]]
    completion_label = "ByteSpanIter"
    completion_label_path = "(use core::byte_array::ByteSpanIter)"
    text_edits = ["""
    use core::byte_array::ByteSpanIter;

    """]

    [[completions]]
    completion_label = "Call"
    completion_label_path = "(use starknet::account::Call)"
    text_edits = ["""
    use starknet::account::Call;

    """]

    [[completions]]
    completion_label = "Circuit"
    completion_label_path = "(use core::circuit::Circuit)"
    text_edits = ["""
    use core::circuit::Circuit;

    """]

    [[completions]]
    completion_label = "CircuitElement"
    completion_label_path = "(use core::circuit::CircuitElement)"
    text_edits = ["""
    use core::circuit::CircuitElement;

    """]

    [[completions]]
    completion_label = "CircuitInput"
    completion_label_path = "(use core::circuit::CircuitInput)"
    text_edits = ["""
    use core::circuit::CircuitInput;

    """]

    [[completions]]
    completion_label = "CircuitModulus"
    completion_label_path = "(use core::circuit::CircuitModulus)"
    text_edits = ["""
    use core::circuit::CircuitModulus;

    """]

    [[completions]]
    completion_label = "ClassHash"
    completion_label_path = "(use starknet::ClassHash)"
    text_edits = ["""
    use starknet::ClassHash;

    """]

    [[completions]]
    completion_label = "ConstOne"
    completion_label_path = "(use core::circuit::ConstOne)"
    text_edits = ["""
    use core::circuit::ConstOne;

    """]

    [[completions]]
    completion_label = "ConstZero"
    completion_label_path = "(use core::circuit::ConstZero)"
    text_edits = ["""
    use core::circuit::ConstZero;

    """]

    [[completions]]
    completion_label = "ContractAddress"
    completion_label_path = "(use starknet::ContractAddress)"
    text_edits = ["""
    use starknet::ContractAddress;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "(use starknet::deployment::DeploymentParams)"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "(use core::internal::DestructWith)"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "(use core::internal::DropWith)"
    text_edits = ["""
    use core::internal::DropWith;

    """]

    [[completions]]
    completion_label = "EcOp"
    completion_label_path = "(use core::ec::EcOp)"
    text_edits = ["""
    use core::ec::EcOp;

    """]

    [[completions]]
    completion_label = "EcPoint"
    completion_label_path = "(use core::ec::EcPoint)"
    text_edits = ["""
    use core::ec::EcPoint;

    """]

    [[completions]]
    completion_label = "EcState"
    completion_label_path = "(use core::ec::EcState)"
    text_edits = ["""
    use core::ec::EcState;

    """]

    [[completions]]
    completion_label = "Error"
    completion_label_path = "(use core::fmt::Error)"
    text_edits = ["""
    use core::fmt::Error;

    """]

    [[completions]]
    completion_label = "EthAddress"
    completion_label_path = "(use starknet::EthAddress)"
    text_edits = ["""
    use starknet::EthAddress;

    """]

    [[completions]]
    completion_label = "ExecutionInfo"
    completion_label_path = "(use starknet::ExecutionInfo)"
    text_edits = ["""
    use starknet::ExecutionInfo;

    """]

    [[completions]]
    completion_label = "Felt252Dict"
    completion_label_path = "(use core::dict::Felt252Dict)"
    text_edits = ["""
    use core::dict::Felt252Dict;

    """]

    [[completions]]
    completion_label = "Felt252DictEntry"
    completion_label_path = "(use core::dict::Felt252DictEntry)"
    text_edits = ["""
    use core::dict::Felt252DictEntry;

    """]

    [[completions]]
    completion_label = "FlattenedStorage"
    completion_label_path = "(use starknet::storage::FlattenedStorage)"
    text_edits = ["""
    use starknet::storage::FlattenedStorage;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]

    [[completions]]
    completion_label = "FromNullableResult"
    completion_label_path = "(use core::nullable::FromNullableResult)"
    text_edits = ["""
    use core::nullable::FromNullableResult;

    """]

    [[completions]]
    completion_label = "GasBuiltin"
    completion_label_path = "(use core::gas::GasBuiltin)"
    text_edits = ["""
    use core::gas::GasBuiltin;

    """]

    [[completions]]
    completion_label = "GasReserve"
    completion_label_path = "(use core::gas::GasReserve)"
    text_edits = ["""
    use core::gas::GasReserve;

    """]

    [[completions]]
    completion_label = "HashState"
    completion_label_path = "(use core::pedersen::HashState)"
    text_edits = ["""
    use core::pedersen::HashState;

    """]

    [[completions]]
    completion_label = "HashState"
    completion_label_path = "(use core::poseidon::HashState)"
    text_edits = ["""
    use core::poseidon::HashState;

    """]

    [[completions]]
    completion_label = "InferDestruct"
    completion_label_path = "(use core::internal::InferDestruct)"
    text_edits = ["""
    use core::internal::InferDestruct;

    """]

    [[completions]]
    completion_label = "InferDrop"
    completion_label_path = "(use core::internal::InferDrop)"
    text_edits = ["""
    use core::internal::InferDrop;

    """]

    [[completions]]
    completion_label = "LoopResult"
    completion_label_path = "(use core::internal::LoopResult)"
    text_edits = ["""
    use core::internal::LoopResult;

    """]

    [[completions]]
    completion_label = "Map"
    completion_label_path = "(use starknet::storage::Map)"
    text_edits = ["""
    use starknet::storage::Map;

    """]

    [[completions]]
    completion_label = "MulMod"
    completion_label_path = "(use core::circuit::MulMod)"
    text_edits = ["""
    use core::circuit::MulMod;

    """]

    [[completions]]
    completion_label = "Mutable"
    completion_label_path = "(use starknet::storage::Mutable)"
    text_edits = ["""
    use starknet::storage::Mutable;

    """]

    [[completions]]
    completion_label = "NonZeroEcPoint"
    completion_label_path = "(use core::ec::NonZeroEcPoint)"
    text_edits = ["""
    use core::ec::NonZeroEcPoint;

    """]

    [[completions]]
    completion_label = "OptionIter"
    completion_label_path = "(use core::option::OptionIter)"
    text_edits = ["""
    use core::option::OptionIter;

    """]

    [[completions]]
    completion_label = "OptionRev"
    completion_label_path = "(use core::internal::OptionRev)"
    text_edits = ["""
    use core::internal::OptionRev;

    """]

    [[completions]]
    completion_label = "Pedersen"
    completion_label_path = "(use core::pedersen::Pedersen)"
    text_edits = ["""
    use core::pedersen::Pedersen;

    """]

    [[completions]]
    completion_label = "PendingStoragePath"
    completion_label_path = "(use starknet::storage::PendingStoragePath)"
    text_edits = ["""
    use starknet::storage::PendingStoragePath;

    """]

    [[completions]]
    completion_label = "Poseidon"
    completion_label_path = "(use core::poseidon::Poseidon)"
    text_edits = ["""
    use core::poseidon::Poseidon;

    """]

    [[completions]]
    completion_label = "Range"
    completion_label_path = "(use core::ops::Range)"
    text_edits = ["""
    use core::ops::Range;

    """]

    [[completions]]
    completion_label = "RangeCheck"
    completion_label_path = "(use core::RangeCheck)"
    text_edits = ["""
    use core::RangeCheck;

    """]

    [[completions]]
    completion_label = "RangeCheck96"
    completion_label_path = "(use core::circuit::RangeCheck96)"
    text_edits = ["""
    use core::circuit::RangeCheck96;

    """]

    [[completions]]
    completion_label = "RangeInclusive"
    completion_label_path = "(use core::ops::RangeInclusive)"
    text_edits = ["""
    use core::ops::RangeInclusive;

    """]

    [[completions]]
    completion_label = "RangeInclusiveIterator"
    completion_label_path = "(use core::ops::RangeInclusiveIterator)"
    text_edits = ["""
    use core::ops::RangeInclusiveIterator;

    """]

    [[completions]]
    completion_label = "RangeIterator"
    completion_label_path = "(use core::ops::RangeIterator)"
    text_edits = ["""
    use core::ops::RangeIterator;

    """]

    [[completions]]
    completion_label = "ResourceBounds"
    completion_label_path = "(use starknet::ResourceBounds)"
    text_edits = ["""
    use starknet::ResourceBounds;

    """]

    [[completions]]
    completion_label = "Secp256k1Point"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Point)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Point;

    """]

    [[completions]]
    completion_label = "Secp256r1Point"
    completion_label_path = "(use starknet::secp256r1::Secp256r1Point)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1Point;

    """]

    [[completions]]
    completion_label = "SegmentArena"
    completion_label_path = "(use core::SegmentArena)"
    text_edits = ["""
    use core::SegmentArena;

    """]

    [[completions]]
    completion_label = "Signature"
    completion_label_path = "(use starknet::secp256_trait::Signature)"
    text_edits = ["""
    use starknet::secp256_trait::Signature;

    """]

    [[completions]]
    completion_label = "SpanIter"
    completion_label_path = "(use core::array::SpanIter)"
    text_edits = ["""
    use core::array::SpanIter;

    """]

    [[completions]]
    completion_label = "SquashedFelt252Dict"
    completion_label_path = "(use core::dict::SquashedFelt252Dict)"
    text_edits = ["""
    use core::dict::SquashedFelt252Dict;

    """]

    [[completions]]
    completion_label = "StorageAddress"
    completion_label_path = "(use starknet::StorageAddress)"
    text_edits = ["""
    use starknet::StorageAddress;

    """]

    [[completions]]
    completion_label = "StorageBase"
    completion_label_path = "(use starknet::storage::StorageBase)"
    text_edits = ["""
    use starknet::storage::StorageBase;

    """]

    [[completions]]
    completion_label = "StorageBaseAddress"
    completion_label_path = "(use starknet::storage_access::StorageBaseAddress)"
    text_edits = ["""
    use starknet::storage_access::StorageBaseAddress;

    """]

    [[completions]]
    completion_label = "StoragePath"
    completion_label_path = "(use starknet::storage::StoragePath)"
    text_edits = ["""
    use starknet::storage::StoragePath;

    """]

    [[completions]]
    completion_label = "StoragePointer"
    completion_label_path = "(use starknet::storage::StoragePointer)"
    text_edits = ["""
    use starknet::storage::StoragePointer;

    """]

    [[completions]]
    completion_label = "StoragePointer0Offset"
    completion_label_path = "(use starknet::storage::StoragePointer0Offset)"
    text_edits = ["""
    use starknet::storage::StoragePointer0Offset;

    """]

    [[completions]]
    completion_label = "SyscallResult"
    completion_label_path = "(use starknet::SyscallResult)"
    text_edits = ["""
    use starknet::SyscallResult;

    """]

    [[completions]]
    completion_label = "TxInfo"
    completion_label_path = "(use starknet::TxInfo)"
    text_edits = ["""
    use starknet::TxInfo;

    """]

    [[completions]]
    completion_label = "U128MulGuarantee"
    completion_label_path = "(use core::integer::U128MulGuarantee)"
    text_edits = ["""
    use core::integer::U128MulGuarantee;

    """]

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]

    [[completions]]
    completion_label = "Vec"
    completion_label_path = "(use starknet::storage::Vec)"
    text_edits = ["""
    use starknet::storage::Vec;

    """]

    [[completions]]
    completion_label = "VecIter"
    completion_label_path = "(use starknet::storage::VecIter)"
    text_edits = ["""
    use starknet::storage::VecIter;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "qm31"
    completion_label_path = "(use core::qm31::qm31)"
    text_edits = ["""
    use core::qm31::qm31;

    """]

    [[completions]]
    completion_label = "u2"
    completion_label_path = "(use core::sha256::u2)"
    text_edits = ["""
    use core::sha256::u2;

    """]

    [[completions]]
    completion_label = "u3"
    completion_label_path = "(use core::sha384::u3)"
    text_edits = ["""
    use core::sha384::u3;

    """]

    [[completions]]
    completion_label = "u384"
    completion_label_path = "(use core::circuit::u384)"
    text_edits = ["""
    use core::circuit::u384;

    """]

    [[completions]]
    completion_label = "u512"
    completion_label_path = "(use core::integer::u512)"
    text_edits = ["""
    use core::integer::u512;

    """]

    [[completions]]
    completion_label = "u96"
    completion_label_path = "(use core::circuit::u96)"
    text_edits = ["""
    use core::circuit::u96;

    """]
    "#);
}

#[test]
fn type_alias_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    type MyType = Foo<caret>;
    ",@r#"
    caret = """
    type MyType = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn type_alias_module_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub struct FooStruct {}
        pub const FooConst: u32 = 0;
        pub fn FooFn() {}
        pub trait FooTrait {}
        pub mod nested {}
    }

    type MyType = module::<caret>;
    ",@r#"
    caret = """
    type MyType = module::<caret>;
    """

    [[completions]]
    completion_label = "FooStruct"
    completion_label_path = "(use module::FooStruct)"
    "#);
}

#[test]
fn let_type_annotation_tuple() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    fn a() {
        let a: (u8, Foo<caret>) = 123;
    }
    ",@r#"
    caret = """
        let a: (u8, Foo<caret>) = 123;
    """

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn return_type_tuple() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    fn a() -> (u8, Foo<caret>) {}
    ",@r#"
    caret = """
    fn a() -> (u8, Foo<caret>) {}
    """

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn type_alias_tuple() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    const FooConst: u32 = 0;
    fn FooFn() {}
    trait FooTrait {}

    type MyType = (u8, Foo<caret>);
    ",@r#"
    caret = """
    type MyType = (u8, Foo<caret>);
    """

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]
    "#);
}

#[test]
fn let_type_annotation_no_params_and_variables() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a(foo: u32) {
        let foo_bar = 1;
        let x: fo<caret> = 1;
    }
    ",@r#"
    caret = """
        let x: fo<caret> = 1;
    """
    completions = []
    "#);
}

#[test]
fn let_type_annotation_no_inline_macros() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a() {
        let x: pani<caret> = 1;
    }
    ",@r#"
    caret = """
        let x: pani<caret> = 1;
    """

    [[completions]]
    completion_label = "Panic"

    [[completions]]
    completion_label = "Span"

    [[completions]]
    completion_label = "SpanIter"
    completion_label_path = "(use core::array::SpanIter)"
    text_edits = ["""
    use core::array::SpanIter;

    """]
    "#);
}

#[test]
fn self_in_trait_return_type() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo {
        type Bar;
        fn qux() -> Self::<caret>;
    }
    ",@r#"
    caret = """
        fn qux() -> Self::<caret>;
    """

    [[completions]]
    completion_label = "Bar"
    "#);
}

#[test]
fn self_in_impl_let_type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo {
        type Bar;
        fn qux();
    }

    impl FooImpl of Foo {
        type Bar = u32;
        fn qux() {
            let a: Self::<caret> = 1;
        }
    }
    ",@r#"
    caret = """
            let a: Self::<caret> = 1;
    """

    [[completions]]
    completion_label = "Bar"
    "#);
}

#[test]
fn impl_path_in_let_type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        type Bar;
        const BAZ: u32;
        fn qux();
    }

    impl MyImpl of MyTrait {
        type Bar = u32;
        const BAZ: u32 = 1;
        fn qux() {}
    }

    fn a() {
        let a: MyImpl::<caret> = 1;
    }
    ",@r#"
    caret = """
        let a: MyImpl::<caret> = 1;
    """

    [[completions]]
    completion_label = "Bar"
    "#);
}

#[test]
fn let_type_annotation_no_modules() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod zzqmodule {}

    fn a() {
        let a: zzqm<caret> = 1;
    }
    ",@r#"
    caret = """
        let a: zzqm<caret> = 1;
    """
    completions = []
    "#);
}

#[test]
fn let_type_annotation_crate_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn a() {
        let a: dep::<caret> = 1;
    }
    ",@r#"
    caret = """
        let a: dep::<caret> = 1;
    """

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    "#);
}
