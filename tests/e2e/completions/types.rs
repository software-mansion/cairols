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
    completion_label = "dep"

    [[completions]]
    completion_label = "hello"

    [[completions]]
    completion_label = "Add"

    [[completions]]
    completion_label = "Array"

    [[completions]]
    completion_label = "ArrayTrait"

    [[completions]]
    completion_label = "Box"

    [[completions]]
    completion_label = "BoxTrait"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "ByteArrayTrait"

    [[completions]]
    completion_label = "Bytes31Trait"

    [[completions]]
    completion_label = "Clone"

    [[completions]]
    completion_label = "Copy"

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "Deref"

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "Div"

    [[completions]]
    completion_label = "DivRem"

    [[completions]]
    completion_label = "Drop"

    [[completions]]
    completion_label = "Felt252DictTrait"

    [[completions]]
    completion_label = "Felt252DictValue"

    [[completions]]
    completion_label = "FromIterator"

    [[completions]]
    completion_label = "Into"

    [[completions]]
    completion_label = "IntoIterator"

    [[completions]]
    completion_label = "Iterator"

    [[completions]]
    completion_label = "Mul"

    [[completions]]
    completion_label = "Neg"

    [[completions]]
    completion_label = "NonZero"

    [[completions]]
    completion_label = "Not"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "NullableTrait"

    [[completions]]
    completion_label = "Option"

    [[completions]]
    completion_label = "OptionTrait"

    [[completions]]
    completion_label = "Panic"

    [[completions]]
    completion_label = "PanicDestruct"

    [[completions]]
    completion_label = "PanicResult"

    [[completions]]
    completion_label = "PartialEq"

    [[completions]]
    completion_label = "PartialOrd"

    [[completions]]
    completion_label = "Rem"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "ResultTrait"

    [[completions]]
    completion_label = "Serde"

    [[completions]]
    completion_label = "Span"

    [[completions]]
    completion_label = "SpanTrait"

    [[completions]]
    completion_label = "Sub"

    [[completions]]
    completion_label = "System"

    [[completions]]
    completion_label = "ToSpanTrait"

    [[completions]]
    completion_label = "TryInto"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "bytes31"

    [[completions]]
    completion_label = "core"

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
    completion_label = "starknet"

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
    completion_label = "AccountContract"
    completion_label_path = "(use starknet::AccountContract)"
    text_edits = ["""
    use starknet::AccountContract;

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
    completion_label = "AccountContractDispatcherTrait"
    completion_label_path = "(use starknet::account::AccountContractDispatcherTrait)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcherTrait;

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
    completion_label = "AccountContractSafeDispatcherTrait"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcherTrait)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcherTrait;

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
    completion_label = "AddAssign"
    completion_label_path = "(use core::ops::AddAssign)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "AddEq"
    completion_label_path = "(use core::traits::AddEq)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "AddHelper"
    completion_label_path = "(use core::internal::bounded_int::AddHelper)"
    text_edits = ["""
    use core::internal::bounded_int::AddHelper;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    completion_label_path = "(use core::circuit::AddInputResult)"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddInputResultImpl"
    completion_label_path = "(use core::circuit::AddInputResultImpl)"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "AddInputResultTrait"
    completion_label_path = "(use core::circuit::AddInputResultTrait)"
    text_edits = ["""
    use core::circuit::AddInputResultTrait;

    """]

    [[completions]]
    completion_label = "AddMod"
    completion_label_path = "(use core::circuit::AddMod)"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "AppendFormattedToByteArray"
    completion_label_path = "(use core::to_byte_array::AppendFormattedToByteArray)"
    text_edits = ["""
    use core::to_byte_array::AppendFormattedToByteArray;

    """]

    [[completions]]
    completion_label = "ArrayImpl"
    completion_label_path = "(use core::array::ArrayImpl)"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayIter"
    completion_label_path = "(use core::array::ArrayIter)"
    text_edits = ["""
    use core::array::ArrayIter;

    """]

    [[completions]]
    completion_label = "BitAnd"
    completion_label_path = "(use core::traits::BitAnd)"
    text_edits = ["""
    use core::traits::BitAnd;

    """]

    [[completions]]
    completion_label = "BitNot"
    completion_label_path = "(use core::traits::BitNot)"
    text_edits = ["""
    use core::traits::BitNot;

    """]

    [[completions]]
    completion_label = "BitOr"
    completion_label_path = "(use core::traits::BitOr)"
    text_edits = ["""
    use core::traits::BitOr;

    """]

    [[completions]]
    completion_label = "BitSize"
    completion_label_path = "(use core::num::traits::BitSize)"
    text_edits = ["""
    use core::num::traits::BitSize;

    """]

    [[completions]]
    completion_label = "BitXor"
    completion_label_path = "(use core::traits::BitXor)"
    text_edits = ["""
    use core::traits::BitXor;

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
    completion_label = "BoolImpl"
    completion_label_path = "(use core::boolean::BoolImpl)"
    text_edits = ["""
    use core::boolean::BoolImpl;

    """]

    [[completions]]
    completion_label = "BoolTrait"
    completion_label_path = "(use core::boolean::BoolTrait)"
    text_edits = ["""
    use core::boolean::BoolTrait;

    """]

    [[completions]]
    completion_label = "Bounded"
    completion_label_path = "(use core::num::traits::Bounded)"
    text_edits = ["""
    use core::num::traits::Bounded;

    """]

    [[completions]]
    completion_label = "BoundedInt"
    completion_label_path = "(use core::integer::BoundedInt)"
    text_edits = ["""
    use core::integer::BoundedInt;

    """]

    [[completions]]
    completion_label = "BoundedInt"
    completion_label_path = "(use core::internal::bounded_int::BoundedInt)"
    text_edits = ["""
    use core::internal::bounded_int::BoundedInt;

    """]

    [[completions]]
    completion_label = "BoxImpl"
    completion_label_path = "(use core::box::BoxImpl)"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "BuiltinCosts"
    completion_label_path = "(use core::gas::BuiltinCosts)"
    text_edits = ["""
    use core::gas::BuiltinCosts;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl"
    completion_label_path = "(use core::byte_array::ByteArrayImpl)"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

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
    completion_label = "ByteSpanImpl"
    completion_label_path = "(use core::byte_array::ByteSpanImpl)"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanIter"
    completion_label_path = "(use core::byte_array::ByteSpanIter)"
    text_edits = ["""
    use core::byte_array::ByteSpanIter;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait"
    completion_label_path = "(use core::byte_array::ByteSpanTrait)"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "Bytes31Impl"
    completion_label_path = "(use core::bytes_31::Bytes31Impl)"
    text_edits = ["""
    use core::bytes_31::Bytes31Impl;

    """]

    [[completions]]
    completion_label = "Call"
    completion_label_path = "(use starknet::account::Call)"
    text_edits = ["""
    use starknet::account::Call;

    """]

    [[completions]]
    completion_label = "CheckedAdd"
    completion_label_path = "(use core::num::traits::CheckedAdd)"
    text_edits = ["""
    use core::num::traits::CheckedAdd;

    """]

    [[completions]]
    completion_label = "CheckedMul"
    completion_label_path = "(use core::num::traits::CheckedMul)"
    text_edits = ["""
    use core::num::traits::CheckedMul;

    """]

    [[completions]]
    completion_label = "CheckedSub"
    completion_label_path = "(use core::num::traits::CheckedSub)"
    text_edits = ["""
    use core::num::traits::CheckedSub;

    """]

    [[completions]]
    completion_label = "Circuit"
    completion_label_path = "(use core::circuit::Circuit)"
    text_edits = ["""
    use core::circuit::Circuit;

    """]

    [[completions]]
    completion_label = "CircuitDefinition"
    completion_label_path = "(use core::circuit::CircuitDefinition)"
    text_edits = ["""
    use core::circuit::CircuitDefinition;

    """]

    [[completions]]
    completion_label = "CircuitElement"
    completion_label_path = "(use core::circuit::CircuitElement)"
    text_edits = ["""
    use core::circuit::CircuitElement;

    """]

    [[completions]]
    completion_label = "CircuitElementCopy"
    completion_label_path = "(use core::circuit::CircuitElementCopy)"
    text_edits = ["""
    use core::circuit::CircuitElementCopy;

    """]

    [[completions]]
    completion_label = "CircuitElementDrop"
    completion_label_path = "(use core::circuit::CircuitElementDrop)"
    text_edits = ["""
    use core::circuit::CircuitElementDrop;

    """]

    [[completions]]
    completion_label = "CircuitElementTrait"
    completion_label_path = "(use core::circuit::CircuitElementTrait)"
    text_edits = ["""
    use core::circuit::CircuitElementTrait;

    """]

    [[completions]]
    completion_label = "CircuitInput"
    completion_label_path = "(use core::circuit::CircuitInput)"
    text_edits = ["""
    use core::circuit::CircuitInput;

    """]

    [[completions]]
    completion_label = "CircuitInputs"
    completion_label_path = "(use core::circuit::CircuitInputs)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "CircuitInputsImpl"
    completion_label_path = "(use core::circuit::CircuitInputsImpl)"
    text_edits = ["""
    use core::circuit::CircuitInputsImpl;

    """]

    [[completions]]
    completion_label = "CircuitModulus"
    completion_label_path = "(use core::circuit::CircuitModulus)"
    text_edits = ["""
    use core::circuit::CircuitModulus;

    """]

    [[completions]]
    completion_label = "CircuitOutputsTrait"
    completion_label_path = "(use core::circuit::CircuitOutputsTrait)"
    text_edits = ["""
    use core::circuit::CircuitOutputsTrait;

    """]

    [[completions]]
    completion_label = "ClassHash"
    completion_label_path = "(use starknet::ClassHash)"
    text_edits = ["""
    use starknet::ClassHash;

    """]

    [[completions]]
    completion_label = "ClassHashIntoFelt252"
    completion_label_path = "(use starknet::class_hash::ClassHashIntoFelt252)"
    text_edits = ["""
    use starknet::class_hash::ClassHashIntoFelt252;

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
    completion_label = "ConstrainHelper"
    completion_label_path = "(use core::internal::bounded_int::ConstrainHelper)"
    text_edits = ["""
    use core::internal::bounded_int::ConstrainHelper;

    """]

    [[completions]]
    completion_label = "ContractAddress"
    completion_label_path = "(use starknet::ContractAddress)"
    text_edits = ["""
    use starknet::ContractAddress;

    """]

    [[completions]]
    completion_label = "ContractAddressIntoFelt252"
    completion_label_path = "(use starknet::contract_address::ContractAddressIntoFelt252)"
    text_edits = ["""
    use starknet::contract_address::ContractAddressIntoFelt252;

    """]

    [[completions]]
    completion_label = "Debug"
    completion_label_path = "(use core::fmt::Debug)"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::DebugImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "(use starknet::deployment::DeploymentParams)"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DerefMut"
    completion_label_path = "(use core::ops::DerefMut)"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    completion_label_path = "(use core::circuit::DestructFailureGuarantee)"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    completion_label_path = "(use core::option::DestructOption)"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "(use core::internal::DestructWith)"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    completion_label_path = "(use core::fmt::Display)"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "(use core::ops::DivAssign)"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    completion_label_path = "(use core::traits::DivEq)"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "(use core::num::traits::DivRem)"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    completion_label_path = "(use core::internal::bounded_int::DivRemHelper)"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

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
    completion_label = "EcPointImpl"
    completion_label_path = "(use core::ec::EcPointImpl)"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait"
    completion_label_path = "(use core::ec::EcPointTrait)"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]

    [[completions]]
    completion_label = "EcState"
    completion_label_path = "(use core::ec::EcState)"
    text_edits = ["""
    use core::ec::EcState;

    """]

    [[completions]]
    completion_label = "EcStateImpl"
    completion_label_path = "(use core::ec::EcStateImpl)"
    text_edits = ["""
    use core::ec::EcStateImpl;

    """]

    [[completions]]
    completion_label = "EcStateTrait"
    completion_label_path = "(use core::ec::EcStateTrait)"
    text_edits = ["""
    use core::ec::EcStateTrait;

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
    completion_label = "EthAddressIntoFelt252"
    completion_label_path = "(use starknet::eth_address::EthAddressIntoFelt252)"
    text_edits = ["""
    use starknet::eth_address::EthAddressIntoFelt252;

    """]

    [[completions]]
    completion_label = "EthAddressPrintImpl"
    completion_label_path = "(use starknet::eth_address::EthAddressPrintImpl)"
    text_edits = ["""
    use starknet::eth_address::EthAddressPrintImpl;

    """]

    [[completions]]
    completion_label = "EthAddressSerde"
    completion_label_path = "(use starknet::eth_address::EthAddressSerde)"
    text_edits = ["""
    use starknet::eth_address::EthAddressSerde;

    """]

    [[completions]]
    completion_label = "EvalCircuitImpl"
    completion_label_path = "(use core::circuit::EvalCircuitImpl)"
    text_edits = ["""
    use core::circuit::EvalCircuitImpl;

    """]

    [[completions]]
    completion_label = "EvalCircuitTrait"
    completion_label_path = "(use core::circuit::EvalCircuitTrait)"
    text_edits = ["""
    use core::circuit::EvalCircuitTrait;

    """]

    [[completions]]
    completion_label = "Event"
    completion_label_path = "(use starknet::Event)"
    text_edits = ["""
    use starknet::Event;

    """]

    [[completions]]
    completion_label = "EventEmitter"
    completion_label_path = "(use starknet::event::EventEmitter)"
    text_edits = ["""
    use starknet::event::EventEmitter;

    """]

    [[completions]]
    completion_label = "ExecutionInfo"
    completion_label_path = "(use starknet::ExecutionInfo)"
    text_edits = ["""
    use starknet::ExecutionInfo;

    """]

    [[completions]]
    completion_label = "Extend"
    completion_label_path = "(use core::iter::Extend)"
    text_edits = ["""
    use core::iter::Extend;

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
    completion_label = "Felt252DictEntryTrait"
    completion_label_path = "(use core::dict::Felt252DictEntryTrait)"
    text_edits = ["""
    use core::dict::Felt252DictEntryTrait;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoClassHash"
    completion_label_path = "(use starknet::class_hash::Felt252TryIntoClassHash)"
    text_edits = ["""
    use starknet::class_hash::Felt252TryIntoClassHash;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoEthAddress"
    completion_label_path = "(use starknet::eth_address::Felt252TryIntoEthAddress)"
    text_edits = ["""
    use starknet::eth_address::Felt252TryIntoEthAddress;

    """]

    [[completions]]
    completion_label = "FlattenedStorage"
    completion_label_path = "(use starknet::storage::FlattenedStorage)"
    text_edits = ["""
    use starknet::storage::FlattenedStorage;

    """]

    [[completions]]
    completion_label = "Fn"
    completion_label_path = "(use core::ops::Fn)"
    text_edits = ["""
    use core::ops::Fn;

    """]

    [[completions]]
    completion_label = "FnOnce"
    completion_label_path = "(use core::ops::FnOnce)"
    text_edits = ["""
    use core::ops::FnOnce;

    """]

    [[completions]]
    completion_label = "FormatAsByteArray"
    completion_label_path = "(use core::to_byte_array::FormatAsByteArray)"
    text_edits = ["""
    use core::to_byte_array::FormatAsByteArray;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]

    [[completions]]
    completion_label = "ForwardingClassHash"
    completion_label_path = "(use starknet::ForwardingClassHash)"
    text_edits = ["""
    use starknet::ForwardingClassHash;

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
    completion_label = "Get"
    completion_label_path = "(use core::ops::Get)"
    text_edits = ["""
    use core::ops::Get;

    """]

    [[completions]]
    completion_label = "Hash"
    completion_label_path = "(use core::hash::Hash)"
    text_edits = ["""
    use core::hash::Hash;

    """]

    [[completions]]
    completion_label = "HashImpl"
    completion_label_path = "(use core::hash::into_felt252_based::HashImpl)"
    text_edits = ["""
    use core::hash::into_felt252_based::HashImpl;

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
    completion_label = "HashStateExTrait"
    completion_label_path = "(use core::hash::HashStateExTrait)"
    text_edits = ["""
    use core::hash::HashStateExTrait;

    """]

    [[completions]]
    completion_label = "HashStateTrait"
    completion_label_path = "(use core::hash::HashStateTrait)"
    text_edits = ["""
    use core::hash::HashStateTrait;

    """]

    [[completions]]
    completion_label = "Index"
    completion_label_path = "(use core::ops::Index)"
    text_edits = ["""
    use core::ops::Index;

    """]

    [[completions]]
    completion_label = "Index"
    completion_label_path = "(use core::traits::Index)"
    text_edits = ["""
    use core::traits::Index;

    """]

    [[completions]]
    completion_label = "IndexView"
    completion_label_path = "(use core::ops::IndexView)"
    text_edits = ["""
    use core::ops::IndexView;

    """]

    [[completions]]
    completion_label = "IndexView"
    completion_label_path = "(use core::traits::IndexView)"
    text_edits = ["""
    use core::traits::IndexView;

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
    completion_label = "IntoIterRange"
    completion_label_path = "(use starknet::storage::IntoIterRange)"
    text_edits = ["""
    use starknet::storage::IntoIterRange;

    """]

    [[completions]]
    completion_label = "LegacyHash"
    completion_label_path = "(use core::hash::LegacyHash)"
    text_edits = ["""
    use core::hash::LegacyHash;

    """]

    [[completions]]
    completion_label = "LoopResult"
    completion_label_path = "(use core::internal::LoopResult)"
    text_edits = ["""
    use core::internal::LoopResult;

    """]

    [[completions]]
    completion_label = "LowerHex"
    completion_label_path = "(use core::fmt::LowerHex)"
    text_edits = ["""
    use core::fmt::LowerHex;

    """]

    [[completions]]
    completion_label = "LowerHexImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::LowerHexImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::LowerHexImpl;

    """]

    [[completions]]
    completion_label = "Map"
    completion_label_path = "(use starknet::storage::Map)"
    text_edits = ["""
    use starknet::storage::Map;

    """]

    [[completions]]
    completion_label = "MulAssign"
    completion_label_path = "(use core::ops::MulAssign)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "MulEq"
    completion_label_path = "(use core::traits::MulEq)"
    text_edits = ["""
    use core::traits::MulEq;

    """]

    [[completions]]
    completion_label = "MulHelper"
    completion_label_path = "(use core::internal::bounded_int::MulHelper)"
    text_edits = ["""
    use core::internal::bounded_int::MulHelper;

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
    completion_label = "MutableVecTrait"
    completion_label_path = "(use starknet::storage::MutableVecTrait)"
    text_edits = ["""
    use starknet::storage::MutableVecTrait;

    """]

    [[completions]]
    completion_label = "NegateHelper"
    completion_label_path = "(use core::internal::bounded_int::NegateHelper)"
    text_edits = ["""
    use core::internal::bounded_int::NegateHelper;

    """]

    [[completions]]
    completion_label = "NonZeroEcPoint"
    completion_label_path = "(use core::ec::NonZeroEcPoint)"
    text_edits = ["""
    use core::ec::NonZeroEcPoint;

    """]

    [[completions]]
    completion_label = "NullableImpl"
    completion_label_path = "(use core::nullable::NullableImpl)"
    text_edits = ["""
    use core::nullable::NullableImpl;

    """]

    [[completions]]
    completion_label = "NumericLiteral"
    completion_label_path = "(use core::integer::NumericLiteral)"
    text_edits = ["""
    use core::integer::NumericLiteral;

    """]

    [[completions]]
    completion_label = "One"
    completion_label_path = "(use core::num::traits::One)"
    text_edits = ["""
    use core::num::traits::One;

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
    completion_label = "OptionTraitImpl"
    completion_label_path = "(use core::option::OptionTraitImpl)"
    text_edits = ["""
    use core::option::OptionTraitImpl;

    """]

    [[completions]]
    completion_label = "OverflowingAdd"
    completion_label_path = "(use core::num::traits::OverflowingAdd)"
    text_edits = ["""
    use core::num::traits::OverflowingAdd;

    """]

    [[completions]]
    completion_label = "OverflowingMul"
    completion_label_path = "(use core::num::traits::OverflowingMul)"
    text_edits = ["""
    use core::num::traits::OverflowingMul;

    """]

    [[completions]]
    completion_label = "OverflowingSub"
    completion_label_path = "(use core::num::traits::OverflowingSub)"
    text_edits = ["""
    use core::num::traits::OverflowingSub;

    """]

    [[completions]]
    completion_label = "Pedersen"
    completion_label_path = "(use core::pedersen::Pedersen)"
    text_edits = ["""
    use core::pedersen::Pedersen;

    """]

    [[completions]]
    completion_label = "PedersenImpl"
    completion_label_path = "(use core::pedersen::PedersenImpl)"
    text_edits = ["""
    use core::pedersen::PedersenImpl;

    """]

    [[completions]]
    completion_label = "PedersenTrait"
    completion_label_path = "(use core::pedersen::PedersenTrait)"
    text_edits = ["""
    use core::pedersen::PedersenTrait;

    """]

    [[completions]]
    completion_label = "PeekableTrait"
    completion_label_path = "(use core::iter::PeekableTrait)"
    text_edits = ["""
    use core::iter::PeekableTrait;

    """]

    [[completions]]
    completion_label = "PendingStoragePath"
    completion_label_path = "(use starknet::storage::PendingStoragePath)"
    text_edits = ["""
    use starknet::storage::PendingStoragePath;

    """]

    [[completions]]
    completion_label = "PendingStoragePathTrait"
    completion_label_path = "(use starknet::storage::PendingStoragePathTrait)"
    text_edits = ["""
    use starknet::storage::PendingStoragePathTrait;

    """]

    [[completions]]
    completion_label = "Poseidon"
    completion_label_path = "(use core::poseidon::Poseidon)"
    text_edits = ["""
    use core::poseidon::Poseidon;

    """]

    [[completions]]
    completion_label = "PoseidonImpl"
    completion_label_path = "(use core::poseidon::PoseidonImpl)"
    text_edits = ["""
    use core::poseidon::PoseidonImpl;

    """]

    [[completions]]
    completion_label = "PoseidonTrait"
    completion_label_path = "(use core::poseidon::PoseidonTrait)"
    text_edits = ["""
    use core::poseidon::PoseidonTrait;

    """]

    [[completions]]
    completion_label = "Pow"
    completion_label_path = "(use core::num::traits::Pow)"
    text_edits = ["""
    use core::num::traits::Pow;

    """]

    [[completions]]
    completion_label = "Product"
    completion_label_path = "(use core::iter::Product)"
    text_edits = ["""
    use core::iter::Product;

    """]

    [[completions]]
    completion_label = "QM31Trait"
    completion_label_path = "(use core::qm31::QM31Trait)"
    text_edits = ["""
    use core::qm31::QM31Trait;

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
    completion_label = "RangeInclusiveTrait"
    completion_label_path = "(use core::ops::RangeInclusiveTrait)"
    text_edits = ["""
    use core::ops::RangeInclusiveTrait;

    """]

    [[completions]]
    completion_label = "RangeIterator"
    completion_label_path = "(use core::ops::RangeIterator)"
    text_edits = ["""
    use core::ops::RangeIterator;

    """]

    [[completions]]
    completion_label = "RangeTrait"
    completion_label_path = "(use core::ops::RangeTrait)"
    text_edits = ["""
    use core::ops::RangeTrait;

    """]

    [[completions]]
    completion_label = "RemAssign"
    completion_label_path = "(use core::ops::RemAssign)"
    text_edits = ["""
    use core::ops::RemAssign;

    """]

    [[completions]]
    completion_label = "RemEq"
    completion_label_path = "(use core::traits::RemEq)"
    text_edits = ["""
    use core::traits::RemEq;

    """]

    [[completions]]
    completion_label = "ResourceBounds"
    completion_label_path = "(use starknet::ResourceBounds)"
    text_edits = ["""
    use starknet::ResourceBounds;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl"
    completion_label_path = "(use core::result::ResultTraitImpl)"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "SaturatingAdd"
    completion_label_path = "(use core::num::traits::SaturatingAdd)"
    text_edits = ["""
    use core::num::traits::SaturatingAdd;

    """]

    [[completions]]
    completion_label = "SaturatingMul"
    completion_label_path = "(use core::num::traits::SaturatingMul)"
    text_edits = ["""
    use core::num::traits::SaturatingMul;

    """]

    [[completions]]
    completion_label = "SaturatingSub"
    completion_label_path = "(use core::num::traits::SaturatingSub)"
    text_edits = ["""
    use core::num::traits::SaturatingSub;

    """]

    [[completions]]
    completion_label = "Secp256PointTrait"
    completion_label_path = "(use starknet::secp256_trait::Secp256PointTrait)"
    text_edits = ["""
    use starknet::secp256_trait::Secp256PointTrait;

    """]

    [[completions]]
    completion_label = "Secp256Trait"
    completion_label_path = "(use starknet::secp256_trait::Secp256Trait)"
    text_edits = ["""
    use starknet::secp256_trait::Secp256Trait;

    """]

    [[completions]]
    completion_label = "Secp256k1Impl"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Impl)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Impl;

    """]

    [[completions]]
    completion_label = "Secp256k1Point"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Point)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Point;

    """]

    [[completions]]
    completion_label = "Secp256k1PointImpl"
    completion_label_path = "(use starknet::secp256k1::Secp256k1PointImpl)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1PointImpl;

    """]

    [[completions]]
    completion_label = "Secp256r1Impl"
    completion_label_path = "(use starknet::secp256r1::Secp256r1Impl)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1Impl;

    """]

    [[completions]]
    completion_label = "Secp256r1Point"
    completion_label_path = "(use starknet::secp256r1::Secp256r1Point)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1Point;

    """]

    [[completions]]
    completion_label = "Secp256r1PointImpl"
    completion_label_path = "(use starknet::secp256r1::Secp256r1PointImpl)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1PointImpl;

    """]

    [[completions]]
    completion_label = "SegmentArena"
    completion_label_path = "(use core::SegmentArena)"
    text_edits = ["""
    use core::SegmentArena;

    """]

    [[completions]]
    completion_label = "SerdeImpl"
    completion_label_path = "(use core::serde::into_felt252_based::SerdeImpl)"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

    """]

    [[completions]]
    completion_label = "Signature"
    completion_label_path = "(use starknet::secp256_trait::Signature)"
    text_edits = ["""
    use starknet::secp256_trait::Signature;

    """]

    [[completions]]
    completion_label = "SpanImpl"
    completion_label_path = "(use core::array::SpanImpl)"
    text_edits = ["""
    use core::array::SpanImpl;

    """]

    [[completions]]
    completion_label = "SpanIndex"
    completion_label_path = "(use core::array::SpanIndex)"
    text_edits = ["""
    use core::array::SpanIndex;

    """]

    [[completions]]
    completion_label = "SpanIter"
    completion_label_path = "(use core::array::SpanIter)"
    text_edits = ["""
    use core::array::SpanIter;

    """]

    [[completions]]
    completion_label = "Sqrt"
    completion_label_path = "(use core::num::traits::Sqrt)"
    text_edits = ["""
    use core::num::traits::Sqrt;

    """]

    [[completions]]
    completion_label = "SquashedFelt252Dict"
    completion_label_path = "(use core::dict::SquashedFelt252Dict)"
    text_edits = ["""
    use core::dict::SquashedFelt252Dict;

    """]

    [[completions]]
    completion_label = "SquashedFelt252DictImpl"
    completion_label_path = "(use core::dict::SquashedFelt252DictImpl)"
    text_edits = ["""
    use core::dict::SquashedFelt252DictImpl;

    """]

    [[completions]]
    completion_label = "SquashedFelt252DictTrait"
    completion_label_path = "(use core::dict::SquashedFelt252DictTrait)"
    text_edits = ["""
    use core::dict::SquashedFelt252DictTrait;

    """]

    [[completions]]
    completion_label = "StorableStoragePointerReadAccess"
    completion_label_path = "(use starknet::storage::StorableStoragePointerReadAccess)"
    text_edits = ["""
    use starknet::storage::StorableStoragePointerReadAccess;

    """]

    [[completions]]
    completion_label = "StorageAddress"
    completion_label_path = "(use starknet::StorageAddress)"
    text_edits = ["""
    use starknet::StorageAddress;

    """]

    [[completions]]
    completion_label = "StorageAsPath"
    completion_label_path = "(use starknet::storage::StorageAsPath)"
    text_edits = ["""
    use starknet::storage::StorageAsPath;

    """]

    [[completions]]
    completion_label = "StorageAsPointer"
    completion_label_path = "(use starknet::storage::StorageAsPointer)"
    text_edits = ["""
    use starknet::storage::StorageAsPointer;

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
    completion_label = "StorageMapReadAccess"
    completion_label_path = "(use starknet::storage::StorageMapReadAccess)"
    text_edits = ["""
    use starknet::storage::StorageMapReadAccess;

    """]

    [[completions]]
    completion_label = "StorageMapWriteAccess"
    completion_label_path = "(use starknet::storage::StorageMapWriteAccess)"
    text_edits = ["""
    use starknet::storage::StorageMapWriteAccess;

    """]

    [[completions]]
    completion_label = "StorageNode"
    completion_label_path = "(use starknet::storage::StorageNode)"
    text_edits = ["""
    use starknet::storage::StorageNode;

    """]

    [[completions]]
    completion_label = "StorageNodeDeref"
    completion_label_path = "(use starknet::storage::StorageNodeDeref)"
    text_edits = ["""
    use starknet::storage::StorageNodeDeref;

    """]

    [[completions]]
    completion_label = "StorageNodeMut"
    completion_label_path = "(use starknet::storage::StorageNodeMut)"
    text_edits = ["""
    use starknet::storage::StorageNodeMut;

    """]

    [[completions]]
    completion_label = "StorageNodeMutDeref"
    completion_label_path = "(use starknet::storage::StorageNodeMutDeref)"
    text_edits = ["""
    use starknet::storage::StorageNodeMutDeref;

    """]

    [[completions]]
    completion_label = "StoragePath"
    completion_label_path = "(use starknet::storage::StoragePath)"
    text_edits = ["""
    use starknet::storage::StoragePath;

    """]

    [[completions]]
    completion_label = "StoragePathEntry"
    completion_label_path = "(use starknet::storage::StoragePathEntry)"
    text_edits = ["""
    use starknet::storage::StoragePathEntry;

    """]

    [[completions]]
    completion_label = "StoragePathMutableConversion"
    completion_label_path = "(use starknet::storage::StoragePathMutableConversion)"
    text_edits = ["""
    use starknet::storage::StoragePathMutableConversion;

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
    completion_label = "StoragePointerReadAccess"
    completion_label_path = "(use starknet::storage::StoragePointerReadAccess)"
    text_edits = ["""
    use starknet::storage::StoragePointerReadAccess;

    """]

    [[completions]]
    completion_label = "StoragePointerWriteAccess"
    completion_label_path = "(use starknet::storage::StoragePointerWriteAccess)"
    text_edits = ["""
    use starknet::storage::StoragePointerWriteAccess;

    """]

    [[completions]]
    completion_label = "StorageTrait"
    completion_label_path = "(use starknet::storage::StorageTrait)"
    text_edits = ["""
    use starknet::storage::StorageTrait;

    """]

    [[completions]]
    completion_label = "StorageTraitMut"
    completion_label_path = "(use starknet::storage::StorageTraitMut)"
    text_edits = ["""
    use starknet::storage::StorageTraitMut;

    """]

    [[completions]]
    completion_label = "Store"
    completion_label_path = "(use starknet::Store)"
    text_edits = ["""
    use starknet::Store;

    """]

    [[completions]]
    completion_label = "StorePacking"
    completion_label_path = "(use starknet::storage_access::StorePacking)"
    text_edits = ["""
    use starknet::storage_access::StorePacking;

    """]

    [[completions]]
    completion_label = "StringLiteral"
    completion_label_path = "(use core::string::StringLiteral)"
    text_edits = ["""
    use core::string::StringLiteral;

    """]

    [[completions]]
    completion_label = "SubAssign"
    completion_label_path = "(use core::ops::SubAssign)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]

    [[completions]]
    completion_label = "SubEq"
    completion_label_path = "(use core::traits::SubEq)"
    text_edits = ["""
    use core::traits::SubEq;

    """]

    [[completions]]
    completion_label = "SubHelper"
    completion_label_path = "(use core::internal::bounded_int::SubHelper)"
    text_edits = ["""
    use core::internal::bounded_int::SubHelper;

    """]

    [[completions]]
    completion_label = "SubPointers"
    completion_label_path = "(use starknet::storage::SubPointers)"
    text_edits = ["""
    use starknet::storage::SubPointers;

    """]

    [[completions]]
    completion_label = "SubPointersDeref"
    completion_label_path = "(use starknet::storage::SubPointersDeref)"
    text_edits = ["""
    use starknet::storage::SubPointersDeref;

    """]

    [[completions]]
    completion_label = "SubPointersForward"
    completion_label_path = "(use starknet::storage::SubPointersForward)"
    text_edits = ["""
    use starknet::storage::SubPointersForward;

    """]

    [[completions]]
    completion_label = "SubPointersMut"
    completion_label_path = "(use starknet::storage::SubPointersMut)"
    text_edits = ["""
    use starknet::storage::SubPointersMut;

    """]

    [[completions]]
    completion_label = "SubPointersMutDeref"
    completion_label_path = "(use starknet::storage::SubPointersMutDeref)"
    text_edits = ["""
    use starknet::storage::SubPointersMutDeref;

    """]

    [[completions]]
    completion_label = "SubPointersMutForward"
    completion_label_path = "(use starknet::storage::SubPointersMutForward)"
    text_edits = ["""
    use starknet::storage::SubPointersMutForward;

    """]

    [[completions]]
    completion_label = "Sum"
    completion_label_path = "(use core::iter::Sum)"
    text_edits = ["""
    use core::iter::Sum;

    """]

    [[completions]]
    completion_label = "SyscallResult"
    completion_label_path = "(use starknet::SyscallResult)"
    text_edits = ["""
    use starknet::SyscallResult;

    """]

    [[completions]]
    completion_label = "SyscallResultTrait"
    completion_label_path = "(use starknet::SyscallResultTrait)"
    text_edits = ["""
    use starknet::SyscallResultTrait;

    """]

    [[completions]]
    completion_label = "ToByteSpanTrait"
    completion_label_path = "(use core::byte_array::ToByteSpanTrait)"
    text_edits = ["""
    use core::byte_array::ToByteSpanTrait;

    """]

    [[completions]]
    completion_label = "TrimMaxHelper"
    completion_label_path = "(use core::internal::bounded_int::TrimMaxHelper)"
    text_edits = ["""
    use core::internal::bounded_int::TrimMaxHelper;

    """]

    [[completions]]
    completion_label = "TrimMinHelper"
    completion_label_path = "(use core::internal::bounded_int::TrimMinHelper)"
    text_edits = ["""
    use core::internal::bounded_int::TrimMinHelper;

    """]

    [[completions]]
    completion_label = "TxInfo"
    completion_label_path = "(use starknet::TxInfo)"
    text_edits = ["""
    use starknet::TxInfo;

    """]

    [[completions]]
    completion_label = "TypeEqual"
    completion_label_path = "(use core::metaprogramming::TypeEqual)"
    text_edits = ["""
    use core::metaprogramming::TypeEqual;

    """]

    [[completions]]
    completion_label = "U128MulGuarantee"
    completion_label_path = "(use core::integer::U128MulGuarantee)"
    text_edits = ["""
    use core::integer::U128MulGuarantee;

    """]

    [[completions]]
    completion_label = "U256IntoEthAddress"
    completion_label_path = "(use starknet::eth_address::U256IntoEthAddress)"
    text_edits = ["""
    use starknet::eth_address::U256IntoEthAddress;

    """]

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]

    [[completions]]
    completion_label = "ValidStorageTypeTrait"
    completion_label_path = "(use starknet::storage::ValidStorageTypeTrait)"
    text_edits = ["""
    use starknet::storage::ValidStorageTypeTrait;

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
    completion_label = "VecTrait"
    completion_label_path = "(use starknet::storage::VecTrait)"
    text_edits = ["""
    use starknet::storage::VecTrait;

    """]

    [[completions]]
    completion_label = "WideMul"
    completion_label_path = "(use core::num::traits::WideMul)"
    text_edits = ["""
    use core::num::traits::WideMul;

    """]

    [[completions]]
    completion_label = "WideSquare"
    completion_label_path = "(use core::num::traits::WideSquare)"
    text_edits = ["""
    use core::num::traits::WideSquare;

    """]

    [[completions]]
    completion_label = "WrappingAdd"
    completion_label_path = "(use core::num::traits::WrappingAdd)"
    text_edits = ["""
    use core::num::traits::WrappingAdd;

    """]

    [[completions]]
    completion_label = "WrappingMul"
    completion_label_path = "(use core::num::traits::WrappingMul)"
    text_edits = ["""
    use core::num::traits::WrappingMul;

    """]

    [[completions]]
    completion_label = "WrappingSub"
    completion_label_path = "(use core::num::traits::WrappingSub)"
    text_edits = ["""
    use core::num::traits::WrappingSub;

    """]

    [[completions]]
    completion_label = "Zero"
    completion_label_path = "(use core::num::traits::Zero)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "account"
    completion_label_path = "(use starknet::account)"
    text_edits = ["""
    use starknet::account;

    """]

    [[completions]]
    completion_label = "array"
    completion_label_path = "(use core::array)"
    text_edits = ["""
    use core::array;

    """]

    [[completions]]
    completion_label = "bit_size"
    completion_label_path = "(use core::num::traits::bit_size)"
    text_edits = ["""
    use core::num::traits::bit_size;

    """]

    [[completions]]
    completion_label = "blake"
    completion_label_path = "(use core::blake)"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "boolean"
    completion_label_path = "(use core::boolean)"
    text_edits = ["""
    use core::boolean;

    """]

    [[completions]]
    completion_label = "bounded_int"
    completion_label_path = "(use core::internal::bounded_int)"
    text_edits = ["""
    use core::internal::bounded_int;

    """]

    [[completions]]
    completion_label = "box"
    completion_label_path = "(use core::box)"
    text_edits = ["""
    use core::box;

    """]

    [[completions]]
    completion_label = "byte_array"
    completion_label_path = "(use core::byte_array)"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "bytes_31"
    completion_label_path = "(use core::bytes_31)"
    text_edits = ["""
    use core::bytes_31;

    """]

    [[completions]]
    completion_label = "checked"
    completion_label_path = "(use core::num::traits::ops::checked)"
    text_edits = ["""
    use core::num::traits::ops::checked;

    """]

    [[completions]]
    completion_label = "circuit"
    completion_label_path = "(use core::circuit)"
    text_edits = ["""
    use core::circuit;

    """]

    [[completions]]
    completion_label = "class_hash"
    completion_label_path = "(use starknet::class_hash)"
    text_edits = ["""
    use starknet::class_hash;

    """]

    [[completions]]
    completion_label = "clone"
    completion_label_path = "(use core::clone)"
    text_edits = ["""
    use core::clone;

    """]

    [[completions]]
    completion_label = "cmp"
    completion_label_path = "(use core::cmp)"
    text_edits = ["""
    use core::cmp;

    """]

    [[completions]]
    completion_label = "contract_address"
    completion_label_path = "(use starknet::contract_address)"
    text_edits = ["""
    use starknet::contract_address;

    """]

    [[completions]]
    completion_label = "debug"
    completion_label_path = "(use core::debug)"
    text_edits = ["""
    use core::debug;

    """]

    [[completions]]
    completion_label = "deployment"
    completion_label_path = "(use starknet::deployment)"
    text_edits = ["""
    use starknet::deployment;

    """]

    [[completions]]
    completion_label = "dict"
    completion_label_path = "(use core::dict)"
    text_edits = ["""
    use core::dict;

    """]

    [[completions]]
    completion_label = "divrem"
    completion_label_path = "(use core::num::traits::ops::divrem)"
    text_edits = ["""
    use core::num::traits::ops::divrem;

    """]

    [[completions]]
    completion_label = "ec"
    completion_label_path = "(use core::ec)"
    text_edits = ["""
    use core::ec;

    """]

    [[completions]]
    completion_label = "ecdsa"
    completion_label_path = "(use core::ecdsa)"
    text_edits = ["""
    use core::ecdsa;

    """]

    [[completions]]
    completion_label = "eth_address"
    completion_label_path = "(use starknet::eth_address)"
    text_edits = ["""
    use starknet::eth_address;

    """]

    [[completions]]
    completion_label = "eth_signature"
    completion_label_path = "(use starknet::eth_signature)"
    text_edits = ["""
    use starknet::eth_signature;

    """]

    [[completions]]
    completion_label = "event"
    completion_label_path = "(use starknet::event)"
    text_edits = ["""
    use starknet::event;

    """]

    [[completions]]
    completion_label = "fmt"
    completion_label_path = "(use core::fmt)"
    text_edits = ["""
    use core::fmt;

    """]

    [[completions]]
    completion_label = "gas"
    completion_label_path = "(use core::gas)"
    text_edits = ["""
    use core::gas;

    """]

    [[completions]]
    completion_label = "get"
    completion_label_path = "(use core::ops::get)"
    text_edits = ["""
    use core::ops::get;

    """]

    [[completions]]
    completion_label = "hash"
    completion_label_path = "(use core::hash)"
    text_edits = ["""
    use core::hash;

    """]

    [[completions]]
    completion_label = "index"
    completion_label_path = "(use core::ops::index)"
    text_edits = ["""
    use core::ops::index;

    """]

    [[completions]]
    completion_label = "integer"
    completion_label_path = "(use core::integer)"
    text_edits = ["""
    use core::integer;

    """]

    [[completions]]
    completion_label = "internal"
    completion_label_path = "(use core::internal)"
    text_edits = ["""
    use core::internal;

    """]

    [[completions]]
    completion_label = "into_felt252_based"
    completion_label_path = "(use core::fmt::into_felt252_based)"
    text_edits = ["""
    use core::fmt::into_felt252_based;

    """]

    [[completions]]
    completion_label = "into_felt252_based"
    completion_label_path = "(use core::hash::into_felt252_based)"
    text_edits = ["""
    use core::hash::into_felt252_based;

    """]

    [[completions]]
    completion_label = "into_felt252_based"
    completion_label_path = "(use core::serde::into_felt252_based)"
    text_edits = ["""
    use core::serde::into_felt252_based;

    """]

    [[completions]]
    completion_label = "iter"
    completion_label_path = "(use core::iter)"
    text_edits = ["""
    use core::iter;

    """]

    [[completions]]
    completion_label = "keccak"
    completion_label_path = "(use core::keccak)"
    text_edits = ["""
    use core::keccak;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "m31_ops"
    completion_label_path = "(use core::qm31::m31_ops)"
    text_edits = ["""
    use core::qm31::m31_ops;

    """]

    [[completions]]
    completion_label = "math"
    completion_label_path = "(use core::math)"
    text_edits = ["""
    use core::math;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
    text_edits = ["""
    use core::metaprogramming;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "nullable"
    completion_label_path = "(use core::nullable)"
    text_edits = ["""
    use core::nullable;

    """]

    [[completions]]
    completion_label = "num"
    completion_label_path = "(use core::num)"
    text_edits = ["""
    use core::num;

    """]

    [[completions]]
    completion_label = "one"
    completion_label_path = "(use core::num::traits::one)"
    text_edits = ["""
    use core::num::traits::one;

    """]

    [[completions]]
    completion_label = "ops"
    completion_label_path = "(use core::num::traits::ops)"
    text_edits = ["""
    use core::num::traits::ops;

    """]

    [[completions]]
    completion_label = "ops"
    completion_label_path = "(use core::ops)"
    text_edits = ["""
    use core::ops;

    """]

    [[completions]]
    completion_label = "option"
    completion_label_path = "(use core::option)"
    text_edits = ["""
    use core::option;

    """]

    [[completions]]
    completion_label = "overflowing"
    completion_label_path = "(use core::num::traits::ops::overflowing)"
    text_edits = ["""
    use core::num::traits::ops::overflowing;

    """]

    [[completions]]
    completion_label = "panics"
    completion_label_path = "(use core::panics)"
    text_edits = ["""
    use core::panics;

    """]

    [[completions]]
    completion_label = "pedersen"
    completion_label_path = "(use core::pedersen)"
    text_edits = ["""
    use core::pedersen;

    """]

    [[completions]]
    completion_label = "poseidon"
    completion_label_path = "(use core::poseidon)"
    text_edits = ["""
    use core::poseidon;

    """]

    [[completions]]
    completion_label = "pow"
    completion_label_path = "(use core::num::traits::ops::pow)"
    text_edits = ["""
    use core::num::traits::ops::pow;

    """]

    [[completions]]
    completion_label = "qm31"
    completion_label_path = "(use core::qm31)"
    text_edits = ["""
    use core::qm31;

    """]

    [[completions]]
    completion_label = "qm31"
    completion_label_path = "(use core::qm31::qm31)"
    text_edits = ["""
    use core::qm31::qm31;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]

    [[completions]]
    completion_label = "saturating"
    completion_label_path = "(use core::num::traits::ops::saturating)"
    text_edits = ["""
    use core::num::traits::ops::saturating;

    """]

    [[completions]]
    completion_label = "secp256_trait"
    completion_label_path = "(use starknet::secp256_trait)"
    text_edits = ["""
    use starknet::secp256_trait;

    """]

    [[completions]]
    completion_label = "secp256k1"
    completion_label_path = "(use starknet::secp256k1)"
    text_edits = ["""
    use starknet::secp256k1;

    """]

    [[completions]]
    completion_label = "secp256r1"
    completion_label_path = "(use starknet::secp256r1)"
    text_edits = ["""
    use starknet::secp256r1;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "sha256"
    completion_label_path = "(use core::sha256)"
    text_edits = ["""
    use core::sha256;

    """]

    [[completions]]
    completion_label = "sha384"
    completion_label_path = "(use core::sha384)"
    text_edits = ["""
    use core::sha384;

    """]

    [[completions]]
    completion_label = "sha512"
    completion_label_path = "(use core::sha512)"
    text_edits = ["""
    use core::sha512;

    """]

    [[completions]]
    completion_label = "stark_curve"
    completion_label_path = "(use core::ec::stark_curve)"
    text_edits = ["""
    use core::ec::stark_curve;

    """]

    [[completions]]
    completion_label = "storage"
    completion_label_path = "(use starknet::storage)"
    text_edits = ["""
    use starknet::storage;

    """]

    [[completions]]
    completion_label = "storage_access"
    completion_label_path = "(use starknet::storage_access)"
    text_edits = ["""
    use starknet::storage_access;

    """]

    [[completions]]
    completion_label = "string"
    completion_label_path = "(use core::string)"
    text_edits = ["""
    use core::string;

    """]

    [[completions]]
    completion_label = "syscalls"
    completion_label_path = "(use starknet::syscalls)"
    text_edits = ["""
    use starknet::syscalls;

    """]

    [[completions]]
    completion_label = "testing"
    completion_label_path = "(use core::testing)"
    text_edits = ["""
    use core::testing;

    """]

    [[completions]]
    completion_label = "testing"
    completion_label_path = "(use starknet::testing)"
    text_edits = ["""
    use starknet::testing;

    """]

    [[completions]]
    completion_label = "to_byte_array"
    completion_label_path = "(use core::to_byte_array)"
    text_edits = ["""
    use core::to_byte_array;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "(use core::num::traits)"
    text_edits = ["""
    use core::num::traits;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "(use core::traits)"
    text_edits = ["""
    use core::traits;

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

    [[completions]]
    completion_label = "wrapping"
    completion_label_path = "(use core::num::traits::ops::wrapping)"
    text_edits = ["""
    use core::num::traits::ops::wrapping;

    """]

    [[completions]]
    completion_label = "zero"
    completion_label_path = "(use core::num::traits::zero)"
    text_edits = ["""
    use core::num::traits::zero;

    """]

    [[completions]]
    completion_label = "zeroable"
    completion_label_path = "(use core::zeroable)"
    text_edits = ["""
    use core::zeroable;

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
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

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

    [[completions]]
    completion_label = "FooTrait"
    completion_label_path = "(use module::FooTrait)"

    [[completions]]
    completion_label = "nested"
    completion_label_path = "(use module::nested)"
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
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

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
    completion_label = "dep"

    [[completions]]
    completion_label = "hello"

    [[completions]]
    completion_label = "Add"

    [[completions]]
    completion_label = "Array"

    [[completions]]
    completion_label = "ArrayTrait"

    [[completions]]
    completion_label = "Box"

    [[completions]]
    completion_label = "BoxTrait"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "ByteArrayTrait"

    [[completions]]
    completion_label = "Bytes31Trait"

    [[completions]]
    completion_label = "Clone"

    [[completions]]
    completion_label = "Copy"

    [[completions]]
    completion_label = "Default"

    [[completions]]
    completion_label = "Deref"

    [[completions]]
    completion_label = "Destruct"

    [[completions]]
    completion_label = "Div"

    [[completions]]
    completion_label = "DivRem"

    [[completions]]
    completion_label = "Drop"

    [[completions]]
    completion_label = "Felt252DictTrait"

    [[completions]]
    completion_label = "Felt252DictValue"

    [[completions]]
    completion_label = "FromIterator"

    [[completions]]
    completion_label = "Into"

    [[completions]]
    completion_label = "IntoIterator"

    [[completions]]
    completion_label = "Iterator"

    [[completions]]
    completion_label = "Mul"

    [[completions]]
    completion_label = "Neg"

    [[completions]]
    completion_label = "NonZero"

    [[completions]]
    completion_label = "Not"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "NullableTrait"

    [[completions]]
    completion_label = "Option"

    [[completions]]
    completion_label = "OptionTrait"

    [[completions]]
    completion_label = "Panic"

    [[completions]]
    completion_label = "PanicDestruct"

    [[completions]]
    completion_label = "PanicResult"

    [[completions]]
    completion_label = "PartialEq"

    [[completions]]
    completion_label = "PartialOrd"

    [[completions]]
    completion_label = "Rem"

    [[completions]]
    completion_label = "Result"

    [[completions]]
    completion_label = "ResultTrait"

    [[completions]]
    completion_label = "Serde"

    [[completions]]
    completion_label = "Span"

    [[completions]]
    completion_label = "SpanTrait"

    [[completions]]
    completion_label = "Sub"

    [[completions]]
    completion_label = "System"

    [[completions]]
    completion_label = "ToSpanTrait"

    [[completions]]
    completion_label = "TryInto"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "bytes31"

    [[completions]]
    completion_label = "core"

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
    completion_label = "starknet"

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
    completion_label = "AccountContract"
    completion_label_path = "(use starknet::AccountContract)"
    text_edits = ["""
    use starknet::AccountContract;

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
    completion_label = "AccountContractDispatcherTrait"
    completion_label_path = "(use starknet::account::AccountContractDispatcherTrait)"
    text_edits = ["""
    use starknet::account::AccountContractDispatcherTrait;

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
    completion_label = "AccountContractSafeDispatcherTrait"
    completion_label_path = "(use starknet::account::AccountContractSafeDispatcherTrait)"
    text_edits = ["""
    use starknet::account::AccountContractSafeDispatcherTrait;

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
    completion_label = "AddAssign"
    completion_label_path = "(use core::ops::AddAssign)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "AddEq"
    completion_label_path = "(use core::traits::AddEq)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "AddHelper"
    completion_label_path = "(use core::internal::bounded_int::AddHelper)"
    text_edits = ["""
    use core::internal::bounded_int::AddHelper;

    """]

    [[completions]]
    completion_label = "AddInputResult"
    completion_label_path = "(use core::circuit::AddInputResult)"
    text_edits = ["""
    use core::circuit::AddInputResult;

    """]

    [[completions]]
    completion_label = "AddInputResultImpl"
    completion_label_path = "(use core::circuit::AddInputResultImpl)"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "AddInputResultTrait"
    completion_label_path = "(use core::circuit::AddInputResultTrait)"
    text_edits = ["""
    use core::circuit::AddInputResultTrait;

    """]

    [[completions]]
    completion_label = "AddMod"
    completion_label_path = "(use core::circuit::AddMod)"
    text_edits = ["""
    use core::circuit::AddMod;

    """]

    [[completions]]
    completion_label = "AppendFormattedToByteArray"
    completion_label_path = "(use core::to_byte_array::AppendFormattedToByteArray)"
    text_edits = ["""
    use core::to_byte_array::AppendFormattedToByteArray;

    """]

    [[completions]]
    completion_label = "ArrayImpl"
    completion_label_path = "(use core::array::ArrayImpl)"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "ArrayIter"
    completion_label_path = "(use core::array::ArrayIter)"
    text_edits = ["""
    use core::array::ArrayIter;

    """]

    [[completions]]
    completion_label = "BitAnd"
    completion_label_path = "(use core::traits::BitAnd)"
    text_edits = ["""
    use core::traits::BitAnd;

    """]

    [[completions]]
    completion_label = "BitNot"
    completion_label_path = "(use core::traits::BitNot)"
    text_edits = ["""
    use core::traits::BitNot;

    """]

    [[completions]]
    completion_label = "BitOr"
    completion_label_path = "(use core::traits::BitOr)"
    text_edits = ["""
    use core::traits::BitOr;

    """]

    [[completions]]
    completion_label = "BitSize"
    completion_label_path = "(use core::num::traits::BitSize)"
    text_edits = ["""
    use core::num::traits::BitSize;

    """]

    [[completions]]
    completion_label = "BitXor"
    completion_label_path = "(use core::traits::BitXor)"
    text_edits = ["""
    use core::traits::BitXor;

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
    completion_label = "BoolImpl"
    completion_label_path = "(use core::boolean::BoolImpl)"
    text_edits = ["""
    use core::boolean::BoolImpl;

    """]

    [[completions]]
    completion_label = "BoolTrait"
    completion_label_path = "(use core::boolean::BoolTrait)"
    text_edits = ["""
    use core::boolean::BoolTrait;

    """]

    [[completions]]
    completion_label = "Bounded"
    completion_label_path = "(use core::num::traits::Bounded)"
    text_edits = ["""
    use core::num::traits::Bounded;

    """]

    [[completions]]
    completion_label = "BoundedInt"
    completion_label_path = "(use core::integer::BoundedInt)"
    text_edits = ["""
    use core::integer::BoundedInt;

    """]

    [[completions]]
    completion_label = "BoundedInt"
    completion_label_path = "(use core::internal::bounded_int::BoundedInt)"
    text_edits = ["""
    use core::internal::bounded_int::BoundedInt;

    """]

    [[completions]]
    completion_label = "BoxImpl"
    completion_label_path = "(use core::box::BoxImpl)"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "BuiltinCosts"
    completion_label_path = "(use core::gas::BuiltinCosts)"
    text_edits = ["""
    use core::gas::BuiltinCosts;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl"
    completion_label_path = "(use core::byte_array::ByteArrayImpl)"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

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
    completion_label = "ByteSpanImpl"
    completion_label_path = "(use core::byte_array::ByteSpanImpl)"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanIter"
    completion_label_path = "(use core::byte_array::ByteSpanIter)"
    text_edits = ["""
    use core::byte_array::ByteSpanIter;

    """]

    [[completions]]
    completion_label = "ByteSpanTrait"
    completion_label_path = "(use core::byte_array::ByteSpanTrait)"
    text_edits = ["""
    use core::byte_array::ByteSpanTrait;

    """]

    [[completions]]
    completion_label = "Bytes31Impl"
    completion_label_path = "(use core::bytes_31::Bytes31Impl)"
    text_edits = ["""
    use core::bytes_31::Bytes31Impl;

    """]

    [[completions]]
    completion_label = "Call"
    completion_label_path = "(use starknet::account::Call)"
    text_edits = ["""
    use starknet::account::Call;

    """]

    [[completions]]
    completion_label = "CheckedAdd"
    completion_label_path = "(use core::num::traits::CheckedAdd)"
    text_edits = ["""
    use core::num::traits::CheckedAdd;

    """]

    [[completions]]
    completion_label = "CheckedMul"
    completion_label_path = "(use core::num::traits::CheckedMul)"
    text_edits = ["""
    use core::num::traits::CheckedMul;

    """]

    [[completions]]
    completion_label = "CheckedSub"
    completion_label_path = "(use core::num::traits::CheckedSub)"
    text_edits = ["""
    use core::num::traits::CheckedSub;

    """]

    [[completions]]
    completion_label = "Circuit"
    completion_label_path = "(use core::circuit::Circuit)"
    text_edits = ["""
    use core::circuit::Circuit;

    """]

    [[completions]]
    completion_label = "CircuitDefinition"
    completion_label_path = "(use core::circuit::CircuitDefinition)"
    text_edits = ["""
    use core::circuit::CircuitDefinition;

    """]

    [[completions]]
    completion_label = "CircuitElement"
    completion_label_path = "(use core::circuit::CircuitElement)"
    text_edits = ["""
    use core::circuit::CircuitElement;

    """]

    [[completions]]
    completion_label = "CircuitElementCopy"
    completion_label_path = "(use core::circuit::CircuitElementCopy)"
    text_edits = ["""
    use core::circuit::CircuitElementCopy;

    """]

    [[completions]]
    completion_label = "CircuitElementDrop"
    completion_label_path = "(use core::circuit::CircuitElementDrop)"
    text_edits = ["""
    use core::circuit::CircuitElementDrop;

    """]

    [[completions]]
    completion_label = "CircuitElementTrait"
    completion_label_path = "(use core::circuit::CircuitElementTrait)"
    text_edits = ["""
    use core::circuit::CircuitElementTrait;

    """]

    [[completions]]
    completion_label = "CircuitInput"
    completion_label_path = "(use core::circuit::CircuitInput)"
    text_edits = ["""
    use core::circuit::CircuitInput;

    """]

    [[completions]]
    completion_label = "CircuitInputs"
    completion_label_path = "(use core::circuit::CircuitInputs)"
    text_edits = ["""
    use core::circuit::CircuitInputs;

    """]

    [[completions]]
    completion_label = "CircuitInputsImpl"
    completion_label_path = "(use core::circuit::CircuitInputsImpl)"
    text_edits = ["""
    use core::circuit::CircuitInputsImpl;

    """]

    [[completions]]
    completion_label = "CircuitModulus"
    completion_label_path = "(use core::circuit::CircuitModulus)"
    text_edits = ["""
    use core::circuit::CircuitModulus;

    """]

    [[completions]]
    completion_label = "CircuitOutputsTrait"
    completion_label_path = "(use core::circuit::CircuitOutputsTrait)"
    text_edits = ["""
    use core::circuit::CircuitOutputsTrait;

    """]

    [[completions]]
    completion_label = "ClassHash"
    completion_label_path = "(use starknet::ClassHash)"
    text_edits = ["""
    use starknet::ClassHash;

    """]

    [[completions]]
    completion_label = "ClassHashIntoFelt252"
    completion_label_path = "(use starknet::class_hash::ClassHashIntoFelt252)"
    text_edits = ["""
    use starknet::class_hash::ClassHashIntoFelt252;

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
    completion_label = "ConstrainHelper"
    completion_label_path = "(use core::internal::bounded_int::ConstrainHelper)"
    text_edits = ["""
    use core::internal::bounded_int::ConstrainHelper;

    """]

    [[completions]]
    completion_label = "ContractAddress"
    completion_label_path = "(use starknet::ContractAddress)"
    text_edits = ["""
    use starknet::ContractAddress;

    """]

    [[completions]]
    completion_label = "ContractAddressIntoFelt252"
    completion_label_path = "(use starknet::contract_address::ContractAddressIntoFelt252)"
    text_edits = ["""
    use starknet::contract_address::ContractAddressIntoFelt252;

    """]

    [[completions]]
    completion_label = "Debug"
    completion_label_path = "(use core::fmt::Debug)"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::DebugImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "DeploymentParams"
    completion_label_path = "(use starknet::deployment::DeploymentParams)"
    text_edits = ["""
    use starknet::deployment::DeploymentParams;

    """]

    [[completions]]
    completion_label = "DerefMut"
    completion_label_path = "(use core::ops::DerefMut)"
    text_edits = ["""
    use core::ops::DerefMut;

    """]

    [[completions]]
    completion_label = "DestructFailureGuarantee"
    completion_label_path = "(use core::circuit::DestructFailureGuarantee)"
    text_edits = ["""
    use core::circuit::DestructFailureGuarantee;

    """]

    [[completions]]
    completion_label = "DestructOption"
    completion_label_path = "(use core::option::DestructOption)"
    text_edits = ["""
    use core::option::DestructOption;

    """]

    [[completions]]
    completion_label = "DestructWith"
    completion_label_path = "(use core::internal::DestructWith)"
    text_edits = ["""
    use core::internal::DestructWith;

    """]

    [[completions]]
    completion_label = "Display"
    completion_label_path = "(use core::fmt::Display)"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "DivAssign"
    completion_label_path = "(use core::ops::DivAssign)"
    text_edits = ["""
    use core::ops::DivAssign;

    """]

    [[completions]]
    completion_label = "DivEq"
    completion_label_path = "(use core::traits::DivEq)"
    text_edits = ["""
    use core::traits::DivEq;

    """]

    [[completions]]
    completion_label = "DivRem"
    completion_label_path = "(use core::num::traits::DivRem)"
    text_edits = ["""
    use core::num::traits::DivRem;

    """]

    [[completions]]
    completion_label = "DivRemHelper"
    completion_label_path = "(use core::internal::bounded_int::DivRemHelper)"
    text_edits = ["""
    use core::internal::bounded_int::DivRemHelper;

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
    completion_label = "EcPointImpl"
    completion_label_path = "(use core::ec::EcPointImpl)"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcPointTrait"
    completion_label_path = "(use core::ec::EcPointTrait)"
    text_edits = ["""
    use core::ec::EcPointTrait;

    """]

    [[completions]]
    completion_label = "EcState"
    completion_label_path = "(use core::ec::EcState)"
    text_edits = ["""
    use core::ec::EcState;

    """]

    [[completions]]
    completion_label = "EcStateImpl"
    completion_label_path = "(use core::ec::EcStateImpl)"
    text_edits = ["""
    use core::ec::EcStateImpl;

    """]

    [[completions]]
    completion_label = "EcStateTrait"
    completion_label_path = "(use core::ec::EcStateTrait)"
    text_edits = ["""
    use core::ec::EcStateTrait;

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
    completion_label = "EthAddressIntoFelt252"
    completion_label_path = "(use starknet::eth_address::EthAddressIntoFelt252)"
    text_edits = ["""
    use starknet::eth_address::EthAddressIntoFelt252;

    """]

    [[completions]]
    completion_label = "EthAddressPrintImpl"
    completion_label_path = "(use starknet::eth_address::EthAddressPrintImpl)"
    text_edits = ["""
    use starknet::eth_address::EthAddressPrintImpl;

    """]

    [[completions]]
    completion_label = "EthAddressSerde"
    completion_label_path = "(use starknet::eth_address::EthAddressSerde)"
    text_edits = ["""
    use starknet::eth_address::EthAddressSerde;

    """]

    [[completions]]
    completion_label = "EvalCircuitImpl"
    completion_label_path = "(use core::circuit::EvalCircuitImpl)"
    text_edits = ["""
    use core::circuit::EvalCircuitImpl;

    """]

    [[completions]]
    completion_label = "EvalCircuitTrait"
    completion_label_path = "(use core::circuit::EvalCircuitTrait)"
    text_edits = ["""
    use core::circuit::EvalCircuitTrait;

    """]

    [[completions]]
    completion_label = "Event"
    completion_label_path = "(use starknet::Event)"
    text_edits = ["""
    use starknet::Event;

    """]

    [[completions]]
    completion_label = "EventEmitter"
    completion_label_path = "(use starknet::event::EventEmitter)"
    text_edits = ["""
    use starknet::event::EventEmitter;

    """]

    [[completions]]
    completion_label = "ExecutionInfo"
    completion_label_path = "(use starknet::ExecutionInfo)"
    text_edits = ["""
    use starknet::ExecutionInfo;

    """]

    [[completions]]
    completion_label = "Extend"
    completion_label_path = "(use core::iter::Extend)"
    text_edits = ["""
    use core::iter::Extend;

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
    completion_label = "Felt252DictEntryTrait"
    completion_label_path = "(use core::dict::Felt252DictEntryTrait)"
    text_edits = ["""
    use core::dict::Felt252DictEntryTrait;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoClassHash"
    completion_label_path = "(use starknet::class_hash::Felt252TryIntoClassHash)"
    text_edits = ["""
    use starknet::class_hash::Felt252TryIntoClassHash;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoEthAddress"
    completion_label_path = "(use starknet::eth_address::Felt252TryIntoEthAddress)"
    text_edits = ["""
    use starknet::eth_address::Felt252TryIntoEthAddress;

    """]

    [[completions]]
    completion_label = "FlattenedStorage"
    completion_label_path = "(use starknet::storage::FlattenedStorage)"
    text_edits = ["""
    use starknet::storage::FlattenedStorage;

    """]

    [[completions]]
    completion_label = "Fn"
    completion_label_path = "(use core::ops::Fn)"
    text_edits = ["""
    use core::ops::Fn;

    """]

    [[completions]]
    completion_label = "FnOnce"
    completion_label_path = "(use core::ops::FnOnce)"
    text_edits = ["""
    use core::ops::FnOnce;

    """]

    [[completions]]
    completion_label = "FormatAsByteArray"
    completion_label_path = "(use core::to_byte_array::FormatAsByteArray)"
    text_edits = ["""
    use core::to_byte_array::FormatAsByteArray;

    """]

    [[completions]]
    completion_label = "Formatter"
    completion_label_path = "(use core::fmt::Formatter)"
    text_edits = ["""
    use core::fmt::Formatter;

    """]

    [[completions]]
    completion_label = "ForwardingClassHash"
    completion_label_path = "(use starknet::ForwardingClassHash)"
    text_edits = ["""
    use starknet::ForwardingClassHash;

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
    completion_label = "Get"
    completion_label_path = "(use core::ops::Get)"
    text_edits = ["""
    use core::ops::Get;

    """]

    [[completions]]
    completion_label = "Hash"
    completion_label_path = "(use core::hash::Hash)"
    text_edits = ["""
    use core::hash::Hash;

    """]

    [[completions]]
    completion_label = "HashImpl"
    completion_label_path = "(use core::hash::into_felt252_based::HashImpl)"
    text_edits = ["""
    use core::hash::into_felt252_based::HashImpl;

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
    completion_label = "HashStateExTrait"
    completion_label_path = "(use core::hash::HashStateExTrait)"
    text_edits = ["""
    use core::hash::HashStateExTrait;

    """]

    [[completions]]
    completion_label = "HashStateTrait"
    completion_label_path = "(use core::hash::HashStateTrait)"
    text_edits = ["""
    use core::hash::HashStateTrait;

    """]

    [[completions]]
    completion_label = "Index"
    completion_label_path = "(use core::ops::Index)"
    text_edits = ["""
    use core::ops::Index;

    """]

    [[completions]]
    completion_label = "Index"
    completion_label_path = "(use core::traits::Index)"
    text_edits = ["""
    use core::traits::Index;

    """]

    [[completions]]
    completion_label = "IndexView"
    completion_label_path = "(use core::ops::IndexView)"
    text_edits = ["""
    use core::ops::IndexView;

    """]

    [[completions]]
    completion_label = "IndexView"
    completion_label_path = "(use core::traits::IndexView)"
    text_edits = ["""
    use core::traits::IndexView;

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
    completion_label = "IntoIterRange"
    completion_label_path = "(use starknet::storage::IntoIterRange)"
    text_edits = ["""
    use starknet::storage::IntoIterRange;

    """]

    [[completions]]
    completion_label = "LegacyHash"
    completion_label_path = "(use core::hash::LegacyHash)"
    text_edits = ["""
    use core::hash::LegacyHash;

    """]

    [[completions]]
    completion_label = "LoopResult"
    completion_label_path = "(use core::internal::LoopResult)"
    text_edits = ["""
    use core::internal::LoopResult;

    """]

    [[completions]]
    completion_label = "LowerHex"
    completion_label_path = "(use core::fmt::LowerHex)"
    text_edits = ["""
    use core::fmt::LowerHex;

    """]

    [[completions]]
    completion_label = "LowerHexImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::LowerHexImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::LowerHexImpl;

    """]

    [[completions]]
    completion_label = "Map"
    completion_label_path = "(use starknet::storage::Map)"
    text_edits = ["""
    use starknet::storage::Map;

    """]

    [[completions]]
    completion_label = "MulAssign"
    completion_label_path = "(use core::ops::MulAssign)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "MulEq"
    completion_label_path = "(use core::traits::MulEq)"
    text_edits = ["""
    use core::traits::MulEq;

    """]

    [[completions]]
    completion_label = "MulHelper"
    completion_label_path = "(use core::internal::bounded_int::MulHelper)"
    text_edits = ["""
    use core::internal::bounded_int::MulHelper;

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
    completion_label = "MutableVecTrait"
    completion_label_path = "(use starknet::storage::MutableVecTrait)"
    text_edits = ["""
    use starknet::storage::MutableVecTrait;

    """]

    [[completions]]
    completion_label = "NegateHelper"
    completion_label_path = "(use core::internal::bounded_int::NegateHelper)"
    text_edits = ["""
    use core::internal::bounded_int::NegateHelper;

    """]

    [[completions]]
    completion_label = "NonZeroEcPoint"
    completion_label_path = "(use core::ec::NonZeroEcPoint)"
    text_edits = ["""
    use core::ec::NonZeroEcPoint;

    """]

    [[completions]]
    completion_label = "NullableImpl"
    completion_label_path = "(use core::nullable::NullableImpl)"
    text_edits = ["""
    use core::nullable::NullableImpl;

    """]

    [[completions]]
    completion_label = "NumericLiteral"
    completion_label_path = "(use core::integer::NumericLiteral)"
    text_edits = ["""
    use core::integer::NumericLiteral;

    """]

    [[completions]]
    completion_label = "One"
    completion_label_path = "(use core::num::traits::One)"
    text_edits = ["""
    use core::num::traits::One;

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
    completion_label = "OptionTraitImpl"
    completion_label_path = "(use core::option::OptionTraitImpl)"
    text_edits = ["""
    use core::option::OptionTraitImpl;

    """]

    [[completions]]
    completion_label = "OverflowingAdd"
    completion_label_path = "(use core::num::traits::OverflowingAdd)"
    text_edits = ["""
    use core::num::traits::OverflowingAdd;

    """]

    [[completions]]
    completion_label = "OverflowingMul"
    completion_label_path = "(use core::num::traits::OverflowingMul)"
    text_edits = ["""
    use core::num::traits::OverflowingMul;

    """]

    [[completions]]
    completion_label = "OverflowingSub"
    completion_label_path = "(use core::num::traits::OverflowingSub)"
    text_edits = ["""
    use core::num::traits::OverflowingSub;

    """]

    [[completions]]
    completion_label = "Pedersen"
    completion_label_path = "(use core::pedersen::Pedersen)"
    text_edits = ["""
    use core::pedersen::Pedersen;

    """]

    [[completions]]
    completion_label = "PedersenImpl"
    completion_label_path = "(use core::pedersen::PedersenImpl)"
    text_edits = ["""
    use core::pedersen::PedersenImpl;

    """]

    [[completions]]
    completion_label = "PedersenTrait"
    completion_label_path = "(use core::pedersen::PedersenTrait)"
    text_edits = ["""
    use core::pedersen::PedersenTrait;

    """]

    [[completions]]
    completion_label = "PeekableTrait"
    completion_label_path = "(use core::iter::PeekableTrait)"
    text_edits = ["""
    use core::iter::PeekableTrait;

    """]

    [[completions]]
    completion_label = "PendingStoragePath"
    completion_label_path = "(use starknet::storage::PendingStoragePath)"
    text_edits = ["""
    use starknet::storage::PendingStoragePath;

    """]

    [[completions]]
    completion_label = "PendingStoragePathTrait"
    completion_label_path = "(use starknet::storage::PendingStoragePathTrait)"
    text_edits = ["""
    use starknet::storage::PendingStoragePathTrait;

    """]

    [[completions]]
    completion_label = "Poseidon"
    completion_label_path = "(use core::poseidon::Poseidon)"
    text_edits = ["""
    use core::poseidon::Poseidon;

    """]

    [[completions]]
    completion_label = "PoseidonImpl"
    completion_label_path = "(use core::poseidon::PoseidonImpl)"
    text_edits = ["""
    use core::poseidon::PoseidonImpl;

    """]

    [[completions]]
    completion_label = "PoseidonTrait"
    completion_label_path = "(use core::poseidon::PoseidonTrait)"
    text_edits = ["""
    use core::poseidon::PoseidonTrait;

    """]

    [[completions]]
    completion_label = "Pow"
    completion_label_path = "(use core::num::traits::Pow)"
    text_edits = ["""
    use core::num::traits::Pow;

    """]

    [[completions]]
    completion_label = "Product"
    completion_label_path = "(use core::iter::Product)"
    text_edits = ["""
    use core::iter::Product;

    """]

    [[completions]]
    completion_label = "QM31Trait"
    completion_label_path = "(use core::qm31::QM31Trait)"
    text_edits = ["""
    use core::qm31::QM31Trait;

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
    completion_label = "RangeInclusiveTrait"
    completion_label_path = "(use core::ops::RangeInclusiveTrait)"
    text_edits = ["""
    use core::ops::RangeInclusiveTrait;

    """]

    [[completions]]
    completion_label = "RangeIterator"
    completion_label_path = "(use core::ops::RangeIterator)"
    text_edits = ["""
    use core::ops::RangeIterator;

    """]

    [[completions]]
    completion_label = "RangeTrait"
    completion_label_path = "(use core::ops::RangeTrait)"
    text_edits = ["""
    use core::ops::RangeTrait;

    """]

    [[completions]]
    completion_label = "RemAssign"
    completion_label_path = "(use core::ops::RemAssign)"
    text_edits = ["""
    use core::ops::RemAssign;

    """]

    [[completions]]
    completion_label = "RemEq"
    completion_label_path = "(use core::traits::RemEq)"
    text_edits = ["""
    use core::traits::RemEq;

    """]

    [[completions]]
    completion_label = "ResourceBounds"
    completion_label_path = "(use starknet::ResourceBounds)"
    text_edits = ["""
    use starknet::ResourceBounds;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl"
    completion_label_path = "(use core::result::ResultTraitImpl)"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "SaturatingAdd"
    completion_label_path = "(use core::num::traits::SaturatingAdd)"
    text_edits = ["""
    use core::num::traits::SaturatingAdd;

    """]

    [[completions]]
    completion_label = "SaturatingMul"
    completion_label_path = "(use core::num::traits::SaturatingMul)"
    text_edits = ["""
    use core::num::traits::SaturatingMul;

    """]

    [[completions]]
    completion_label = "SaturatingSub"
    completion_label_path = "(use core::num::traits::SaturatingSub)"
    text_edits = ["""
    use core::num::traits::SaturatingSub;

    """]

    [[completions]]
    completion_label = "Secp256PointTrait"
    completion_label_path = "(use starknet::secp256_trait::Secp256PointTrait)"
    text_edits = ["""
    use starknet::secp256_trait::Secp256PointTrait;

    """]

    [[completions]]
    completion_label = "Secp256Trait"
    completion_label_path = "(use starknet::secp256_trait::Secp256Trait)"
    text_edits = ["""
    use starknet::secp256_trait::Secp256Trait;

    """]

    [[completions]]
    completion_label = "Secp256k1Impl"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Impl)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Impl;

    """]

    [[completions]]
    completion_label = "Secp256k1Point"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Point)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Point;

    """]

    [[completions]]
    completion_label = "Secp256k1PointImpl"
    completion_label_path = "(use starknet::secp256k1::Secp256k1PointImpl)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1PointImpl;

    """]

    [[completions]]
    completion_label = "Secp256r1Impl"
    completion_label_path = "(use starknet::secp256r1::Secp256r1Impl)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1Impl;

    """]

    [[completions]]
    completion_label = "Secp256r1Point"
    completion_label_path = "(use starknet::secp256r1::Secp256r1Point)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1Point;

    """]

    [[completions]]
    completion_label = "Secp256r1PointImpl"
    completion_label_path = "(use starknet::secp256r1::Secp256r1PointImpl)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1PointImpl;

    """]

    [[completions]]
    completion_label = "SegmentArena"
    completion_label_path = "(use core::SegmentArena)"
    text_edits = ["""
    use core::SegmentArena;

    """]

    [[completions]]
    completion_label = "SerdeImpl"
    completion_label_path = "(use core::serde::into_felt252_based::SerdeImpl)"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

    """]

    [[completions]]
    completion_label = "Signature"
    completion_label_path = "(use starknet::secp256_trait::Signature)"
    text_edits = ["""
    use starknet::secp256_trait::Signature;

    """]

    [[completions]]
    completion_label = "SpanImpl"
    completion_label_path = "(use core::array::SpanImpl)"
    text_edits = ["""
    use core::array::SpanImpl;

    """]

    [[completions]]
    completion_label = "SpanIndex"
    completion_label_path = "(use core::array::SpanIndex)"
    text_edits = ["""
    use core::array::SpanIndex;

    """]

    [[completions]]
    completion_label = "SpanIter"
    completion_label_path = "(use core::array::SpanIter)"
    text_edits = ["""
    use core::array::SpanIter;

    """]

    [[completions]]
    completion_label = "Sqrt"
    completion_label_path = "(use core::num::traits::Sqrt)"
    text_edits = ["""
    use core::num::traits::Sqrt;

    """]

    [[completions]]
    completion_label = "SquashedFelt252Dict"
    completion_label_path = "(use core::dict::SquashedFelt252Dict)"
    text_edits = ["""
    use core::dict::SquashedFelt252Dict;

    """]

    [[completions]]
    completion_label = "SquashedFelt252DictImpl"
    completion_label_path = "(use core::dict::SquashedFelt252DictImpl)"
    text_edits = ["""
    use core::dict::SquashedFelt252DictImpl;

    """]

    [[completions]]
    completion_label = "SquashedFelt252DictTrait"
    completion_label_path = "(use core::dict::SquashedFelt252DictTrait)"
    text_edits = ["""
    use core::dict::SquashedFelt252DictTrait;

    """]

    [[completions]]
    completion_label = "StorableStoragePointerReadAccess"
    completion_label_path = "(use starknet::storage::StorableStoragePointerReadAccess)"
    text_edits = ["""
    use starknet::storage::StorableStoragePointerReadAccess;

    """]

    [[completions]]
    completion_label = "StorageAddress"
    completion_label_path = "(use starknet::StorageAddress)"
    text_edits = ["""
    use starknet::StorageAddress;

    """]

    [[completions]]
    completion_label = "StorageAsPath"
    completion_label_path = "(use starknet::storage::StorageAsPath)"
    text_edits = ["""
    use starknet::storage::StorageAsPath;

    """]

    [[completions]]
    completion_label = "StorageAsPointer"
    completion_label_path = "(use starknet::storage::StorageAsPointer)"
    text_edits = ["""
    use starknet::storage::StorageAsPointer;

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
    completion_label = "StorageMapReadAccess"
    completion_label_path = "(use starknet::storage::StorageMapReadAccess)"
    text_edits = ["""
    use starknet::storage::StorageMapReadAccess;

    """]

    [[completions]]
    completion_label = "StorageMapWriteAccess"
    completion_label_path = "(use starknet::storage::StorageMapWriteAccess)"
    text_edits = ["""
    use starknet::storage::StorageMapWriteAccess;

    """]

    [[completions]]
    completion_label = "StorageNode"
    completion_label_path = "(use starknet::storage::StorageNode)"
    text_edits = ["""
    use starknet::storage::StorageNode;

    """]

    [[completions]]
    completion_label = "StorageNodeDeref"
    completion_label_path = "(use starknet::storage::StorageNodeDeref)"
    text_edits = ["""
    use starknet::storage::StorageNodeDeref;

    """]

    [[completions]]
    completion_label = "StorageNodeMut"
    completion_label_path = "(use starknet::storage::StorageNodeMut)"
    text_edits = ["""
    use starknet::storage::StorageNodeMut;

    """]

    [[completions]]
    completion_label = "StorageNodeMutDeref"
    completion_label_path = "(use starknet::storage::StorageNodeMutDeref)"
    text_edits = ["""
    use starknet::storage::StorageNodeMutDeref;

    """]

    [[completions]]
    completion_label = "StoragePath"
    completion_label_path = "(use starknet::storage::StoragePath)"
    text_edits = ["""
    use starknet::storage::StoragePath;

    """]

    [[completions]]
    completion_label = "StoragePathEntry"
    completion_label_path = "(use starknet::storage::StoragePathEntry)"
    text_edits = ["""
    use starknet::storage::StoragePathEntry;

    """]

    [[completions]]
    completion_label = "StoragePathMutableConversion"
    completion_label_path = "(use starknet::storage::StoragePathMutableConversion)"
    text_edits = ["""
    use starknet::storage::StoragePathMutableConversion;

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
    completion_label = "StoragePointerReadAccess"
    completion_label_path = "(use starknet::storage::StoragePointerReadAccess)"
    text_edits = ["""
    use starknet::storage::StoragePointerReadAccess;

    """]

    [[completions]]
    completion_label = "StoragePointerWriteAccess"
    completion_label_path = "(use starknet::storage::StoragePointerWriteAccess)"
    text_edits = ["""
    use starknet::storage::StoragePointerWriteAccess;

    """]

    [[completions]]
    completion_label = "StorageTrait"
    completion_label_path = "(use starknet::storage::StorageTrait)"
    text_edits = ["""
    use starknet::storage::StorageTrait;

    """]

    [[completions]]
    completion_label = "StorageTraitMut"
    completion_label_path = "(use starknet::storage::StorageTraitMut)"
    text_edits = ["""
    use starknet::storage::StorageTraitMut;

    """]

    [[completions]]
    completion_label = "Store"
    completion_label_path = "(use starknet::Store)"
    text_edits = ["""
    use starknet::Store;

    """]

    [[completions]]
    completion_label = "StorePacking"
    completion_label_path = "(use starknet::storage_access::StorePacking)"
    text_edits = ["""
    use starknet::storage_access::StorePacking;

    """]

    [[completions]]
    completion_label = "StringLiteral"
    completion_label_path = "(use core::string::StringLiteral)"
    text_edits = ["""
    use core::string::StringLiteral;

    """]

    [[completions]]
    completion_label = "SubAssign"
    completion_label_path = "(use core::ops::SubAssign)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]

    [[completions]]
    completion_label = "SubEq"
    completion_label_path = "(use core::traits::SubEq)"
    text_edits = ["""
    use core::traits::SubEq;

    """]

    [[completions]]
    completion_label = "SubHelper"
    completion_label_path = "(use core::internal::bounded_int::SubHelper)"
    text_edits = ["""
    use core::internal::bounded_int::SubHelper;

    """]

    [[completions]]
    completion_label = "SubPointers"
    completion_label_path = "(use starknet::storage::SubPointers)"
    text_edits = ["""
    use starknet::storage::SubPointers;

    """]

    [[completions]]
    completion_label = "SubPointersDeref"
    completion_label_path = "(use starknet::storage::SubPointersDeref)"
    text_edits = ["""
    use starknet::storage::SubPointersDeref;

    """]

    [[completions]]
    completion_label = "SubPointersForward"
    completion_label_path = "(use starknet::storage::SubPointersForward)"
    text_edits = ["""
    use starknet::storage::SubPointersForward;

    """]

    [[completions]]
    completion_label = "SubPointersMut"
    completion_label_path = "(use starknet::storage::SubPointersMut)"
    text_edits = ["""
    use starknet::storage::SubPointersMut;

    """]

    [[completions]]
    completion_label = "SubPointersMutDeref"
    completion_label_path = "(use starknet::storage::SubPointersMutDeref)"
    text_edits = ["""
    use starknet::storage::SubPointersMutDeref;

    """]

    [[completions]]
    completion_label = "SubPointersMutForward"
    completion_label_path = "(use starknet::storage::SubPointersMutForward)"
    text_edits = ["""
    use starknet::storage::SubPointersMutForward;

    """]

    [[completions]]
    completion_label = "Sum"
    completion_label_path = "(use core::iter::Sum)"
    text_edits = ["""
    use core::iter::Sum;

    """]

    [[completions]]
    completion_label = "SyscallResult"
    completion_label_path = "(use starknet::SyscallResult)"
    text_edits = ["""
    use starknet::SyscallResult;

    """]

    [[completions]]
    completion_label = "SyscallResultTrait"
    completion_label_path = "(use starknet::SyscallResultTrait)"
    text_edits = ["""
    use starknet::SyscallResultTrait;

    """]

    [[completions]]
    completion_label = "ToByteSpanTrait"
    completion_label_path = "(use core::byte_array::ToByteSpanTrait)"
    text_edits = ["""
    use core::byte_array::ToByteSpanTrait;

    """]

    [[completions]]
    completion_label = "TrimMaxHelper"
    completion_label_path = "(use core::internal::bounded_int::TrimMaxHelper)"
    text_edits = ["""
    use core::internal::bounded_int::TrimMaxHelper;

    """]

    [[completions]]
    completion_label = "TrimMinHelper"
    completion_label_path = "(use core::internal::bounded_int::TrimMinHelper)"
    text_edits = ["""
    use core::internal::bounded_int::TrimMinHelper;

    """]

    [[completions]]
    completion_label = "TxInfo"
    completion_label_path = "(use starknet::TxInfo)"
    text_edits = ["""
    use starknet::TxInfo;

    """]

    [[completions]]
    completion_label = "TypeEqual"
    completion_label_path = "(use core::metaprogramming::TypeEqual)"
    text_edits = ["""
    use core::metaprogramming::TypeEqual;

    """]

    [[completions]]
    completion_label = "U128MulGuarantee"
    completion_label_path = "(use core::integer::U128MulGuarantee)"
    text_edits = ["""
    use core::integer::U128MulGuarantee;

    """]

    [[completions]]
    completion_label = "U256IntoEthAddress"
    completion_label_path = "(use starknet::eth_address::U256IntoEthAddress)"
    text_edits = ["""
    use starknet::eth_address::U256IntoEthAddress;

    """]

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]

    [[completions]]
    completion_label = "ValidStorageTypeTrait"
    completion_label_path = "(use starknet::storage::ValidStorageTypeTrait)"
    text_edits = ["""
    use starknet::storage::ValidStorageTypeTrait;

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
    completion_label = "VecTrait"
    completion_label_path = "(use starknet::storage::VecTrait)"
    text_edits = ["""
    use starknet::storage::VecTrait;

    """]

    [[completions]]
    completion_label = "WideMul"
    completion_label_path = "(use core::num::traits::WideMul)"
    text_edits = ["""
    use core::num::traits::WideMul;

    """]

    [[completions]]
    completion_label = "WideSquare"
    completion_label_path = "(use core::num::traits::WideSquare)"
    text_edits = ["""
    use core::num::traits::WideSquare;

    """]

    [[completions]]
    completion_label = "WrappingAdd"
    completion_label_path = "(use core::num::traits::WrappingAdd)"
    text_edits = ["""
    use core::num::traits::WrappingAdd;

    """]

    [[completions]]
    completion_label = "WrappingMul"
    completion_label_path = "(use core::num::traits::WrappingMul)"
    text_edits = ["""
    use core::num::traits::WrappingMul;

    """]

    [[completions]]
    completion_label = "WrappingSub"
    completion_label_path = "(use core::num::traits::WrappingSub)"
    text_edits = ["""
    use core::num::traits::WrappingSub;

    """]

    [[completions]]
    completion_label = "Zero"
    completion_label_path = "(use core::num::traits::Zero)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "account"
    completion_label_path = "(use starknet::account)"
    text_edits = ["""
    use starknet::account;

    """]

    [[completions]]
    completion_label = "array"
    completion_label_path = "(use core::array)"
    text_edits = ["""
    use core::array;

    """]

    [[completions]]
    completion_label = "bit_size"
    completion_label_path = "(use core::num::traits::bit_size)"
    text_edits = ["""
    use core::num::traits::bit_size;

    """]

    [[completions]]
    completion_label = "blake"
    completion_label_path = "(use core::blake)"
    text_edits = ["""
    use core::blake;

    """]

    [[completions]]
    completion_label = "boolean"
    completion_label_path = "(use core::boolean)"
    text_edits = ["""
    use core::boolean;

    """]

    [[completions]]
    completion_label = "bounded_int"
    completion_label_path = "(use core::internal::bounded_int)"
    text_edits = ["""
    use core::internal::bounded_int;

    """]

    [[completions]]
    completion_label = "box"
    completion_label_path = "(use core::box)"
    text_edits = ["""
    use core::box;

    """]

    [[completions]]
    completion_label = "byte_array"
    completion_label_path = "(use core::byte_array)"
    text_edits = ["""
    use core::byte_array;

    """]

    [[completions]]
    completion_label = "bytes_31"
    completion_label_path = "(use core::bytes_31)"
    text_edits = ["""
    use core::bytes_31;

    """]

    [[completions]]
    completion_label = "checked"
    completion_label_path = "(use core::num::traits::ops::checked)"
    text_edits = ["""
    use core::num::traits::ops::checked;

    """]

    [[completions]]
    completion_label = "circuit"
    completion_label_path = "(use core::circuit)"
    text_edits = ["""
    use core::circuit;

    """]

    [[completions]]
    completion_label = "class_hash"
    completion_label_path = "(use starknet::class_hash)"
    text_edits = ["""
    use starknet::class_hash;

    """]

    [[completions]]
    completion_label = "clone"
    completion_label_path = "(use core::clone)"
    text_edits = ["""
    use core::clone;

    """]

    [[completions]]
    completion_label = "cmp"
    completion_label_path = "(use core::cmp)"
    text_edits = ["""
    use core::cmp;

    """]

    [[completions]]
    completion_label = "contract_address"
    completion_label_path = "(use starknet::contract_address)"
    text_edits = ["""
    use starknet::contract_address;

    """]

    [[completions]]
    completion_label = "debug"
    completion_label_path = "(use core::debug)"
    text_edits = ["""
    use core::debug;

    """]

    [[completions]]
    completion_label = "deployment"
    completion_label_path = "(use starknet::deployment)"
    text_edits = ["""
    use starknet::deployment;

    """]

    [[completions]]
    completion_label = "dict"
    completion_label_path = "(use core::dict)"
    text_edits = ["""
    use core::dict;

    """]

    [[completions]]
    completion_label = "divrem"
    completion_label_path = "(use core::num::traits::ops::divrem)"
    text_edits = ["""
    use core::num::traits::ops::divrem;

    """]

    [[completions]]
    completion_label = "ec"
    completion_label_path = "(use core::ec)"
    text_edits = ["""
    use core::ec;

    """]

    [[completions]]
    completion_label = "ecdsa"
    completion_label_path = "(use core::ecdsa)"
    text_edits = ["""
    use core::ecdsa;

    """]

    [[completions]]
    completion_label = "eth_address"
    completion_label_path = "(use starknet::eth_address)"
    text_edits = ["""
    use starknet::eth_address;

    """]

    [[completions]]
    completion_label = "eth_signature"
    completion_label_path = "(use starknet::eth_signature)"
    text_edits = ["""
    use starknet::eth_signature;

    """]

    [[completions]]
    completion_label = "event"
    completion_label_path = "(use starknet::event)"
    text_edits = ["""
    use starknet::event;

    """]

    [[completions]]
    completion_label = "fmt"
    completion_label_path = "(use core::fmt)"
    text_edits = ["""
    use core::fmt;

    """]

    [[completions]]
    completion_label = "gas"
    completion_label_path = "(use core::gas)"
    text_edits = ["""
    use core::gas;

    """]

    [[completions]]
    completion_label = "get"
    completion_label_path = "(use core::ops::get)"
    text_edits = ["""
    use core::ops::get;

    """]

    [[completions]]
    completion_label = "hash"
    completion_label_path = "(use core::hash)"
    text_edits = ["""
    use core::hash;

    """]

    [[completions]]
    completion_label = "index"
    completion_label_path = "(use core::ops::index)"
    text_edits = ["""
    use core::ops::index;

    """]

    [[completions]]
    completion_label = "integer"
    completion_label_path = "(use core::integer)"
    text_edits = ["""
    use core::integer;

    """]

    [[completions]]
    completion_label = "internal"
    completion_label_path = "(use core::internal)"
    text_edits = ["""
    use core::internal;

    """]

    [[completions]]
    completion_label = "into_felt252_based"
    completion_label_path = "(use core::fmt::into_felt252_based)"
    text_edits = ["""
    use core::fmt::into_felt252_based;

    """]

    [[completions]]
    completion_label = "into_felt252_based"
    completion_label_path = "(use core::hash::into_felt252_based)"
    text_edits = ["""
    use core::hash::into_felt252_based;

    """]

    [[completions]]
    completion_label = "into_felt252_based"
    completion_label_path = "(use core::serde::into_felt252_based)"
    text_edits = ["""
    use core::serde::into_felt252_based;

    """]

    [[completions]]
    completion_label = "iter"
    completion_label_path = "(use core::iter)"
    text_edits = ["""
    use core::iter;

    """]

    [[completions]]
    completion_label = "keccak"
    completion_label_path = "(use core::keccak)"
    text_edits = ["""
    use core::keccak;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "m31_ops"
    completion_label_path = "(use core::qm31::m31_ops)"
    text_edits = ["""
    use core::qm31::m31_ops;

    """]

    [[completions]]
    completion_label = "math"
    completion_label_path = "(use core::math)"
    text_edits = ["""
    use core::math;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
    text_edits = ["""
    use core::metaprogramming;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "nullable"
    completion_label_path = "(use core::nullable)"
    text_edits = ["""
    use core::nullable;

    """]

    [[completions]]
    completion_label = "num"
    completion_label_path = "(use core::num)"
    text_edits = ["""
    use core::num;

    """]

    [[completions]]
    completion_label = "one"
    completion_label_path = "(use core::num::traits::one)"
    text_edits = ["""
    use core::num::traits::one;

    """]

    [[completions]]
    completion_label = "ops"
    completion_label_path = "(use core::num::traits::ops)"
    text_edits = ["""
    use core::num::traits::ops;

    """]

    [[completions]]
    completion_label = "ops"
    completion_label_path = "(use core::ops)"
    text_edits = ["""
    use core::ops;

    """]

    [[completions]]
    completion_label = "option"
    completion_label_path = "(use core::option)"
    text_edits = ["""
    use core::option;

    """]

    [[completions]]
    completion_label = "overflowing"
    completion_label_path = "(use core::num::traits::ops::overflowing)"
    text_edits = ["""
    use core::num::traits::ops::overflowing;

    """]

    [[completions]]
    completion_label = "panics"
    completion_label_path = "(use core::panics)"
    text_edits = ["""
    use core::panics;

    """]

    [[completions]]
    completion_label = "pedersen"
    completion_label_path = "(use core::pedersen)"
    text_edits = ["""
    use core::pedersen;

    """]

    [[completions]]
    completion_label = "poseidon"
    completion_label_path = "(use core::poseidon)"
    text_edits = ["""
    use core::poseidon;

    """]

    [[completions]]
    completion_label = "pow"
    completion_label_path = "(use core::num::traits::ops::pow)"
    text_edits = ["""
    use core::num::traits::ops::pow;

    """]

    [[completions]]
    completion_label = "qm31"
    completion_label_path = "(use core::qm31)"
    text_edits = ["""
    use core::qm31;

    """]

    [[completions]]
    completion_label = "qm31"
    completion_label_path = "(use core::qm31::qm31)"
    text_edits = ["""
    use core::qm31::qm31;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]

    [[completions]]
    completion_label = "saturating"
    completion_label_path = "(use core::num::traits::ops::saturating)"
    text_edits = ["""
    use core::num::traits::ops::saturating;

    """]

    [[completions]]
    completion_label = "secp256_trait"
    completion_label_path = "(use starknet::secp256_trait)"
    text_edits = ["""
    use starknet::secp256_trait;

    """]

    [[completions]]
    completion_label = "secp256k1"
    completion_label_path = "(use starknet::secp256k1)"
    text_edits = ["""
    use starknet::secp256k1;

    """]

    [[completions]]
    completion_label = "secp256r1"
    completion_label_path = "(use starknet::secp256r1)"
    text_edits = ["""
    use starknet::secp256r1;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "sha256"
    completion_label_path = "(use core::sha256)"
    text_edits = ["""
    use core::sha256;

    """]

    [[completions]]
    completion_label = "sha384"
    completion_label_path = "(use core::sha384)"
    text_edits = ["""
    use core::sha384;

    """]

    [[completions]]
    completion_label = "sha512"
    completion_label_path = "(use core::sha512)"
    text_edits = ["""
    use core::sha512;

    """]

    [[completions]]
    completion_label = "stark_curve"
    completion_label_path = "(use core::ec::stark_curve)"
    text_edits = ["""
    use core::ec::stark_curve;

    """]

    [[completions]]
    completion_label = "storage"
    completion_label_path = "(use starknet::storage)"
    text_edits = ["""
    use starknet::storage;

    """]

    [[completions]]
    completion_label = "storage_access"
    completion_label_path = "(use starknet::storage_access)"
    text_edits = ["""
    use starknet::storage_access;

    """]

    [[completions]]
    completion_label = "string"
    completion_label_path = "(use core::string)"
    text_edits = ["""
    use core::string;

    """]

    [[completions]]
    completion_label = "syscalls"
    completion_label_path = "(use starknet::syscalls)"
    text_edits = ["""
    use starknet::syscalls;

    """]

    [[completions]]
    completion_label = "testing"
    completion_label_path = "(use core::testing)"
    text_edits = ["""
    use core::testing;

    """]

    [[completions]]
    completion_label = "testing"
    completion_label_path = "(use starknet::testing)"
    text_edits = ["""
    use starknet::testing;

    """]

    [[completions]]
    completion_label = "to_byte_array"
    completion_label_path = "(use core::to_byte_array)"
    text_edits = ["""
    use core::to_byte_array;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "(use core::num::traits)"
    text_edits = ["""
    use core::num::traits;

    """]

    [[completions]]
    completion_label = "traits"
    completion_label_path = "(use core::traits)"
    text_edits = ["""
    use core::traits;

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

    [[completions]]
    completion_label = "wrapping"
    completion_label_path = "(use core::num::traits::ops::wrapping)"
    text_edits = ["""
    use core::num::traits::ops::wrapping;

    """]

    [[completions]]
    completion_label = "zero"
    completion_label_path = "(use core::num::traits::zero)"
    text_edits = ["""
    use core::num::traits::zero;

    """]

    [[completions]]
    completion_label = "zeroable"
    completion_label_path = "(use core::zeroable)"
    text_edits = ["""
    use core::zeroable;

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
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

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

    [[completions]]
    completion_label = "FooTrait"
    completion_label_path = "(use module::FooTrait)"

    [[completions]]
    completion_label = "nested"
    completion_label_path = "(use module::nested)"
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
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

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
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

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
    completion_label = "FooTrait"

    [[completions]]
    completion_label = "bool"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

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

    [[completions]]
    completion_label = "OverflowingAdd"
    completion_label_path = "(use core::num::traits::OverflowingAdd)"
    text_edits = ["""
    use core::num::traits::OverflowingAdd;

    """]

    [[completions]]
    completion_label = "OverflowingMul"
    completion_label_path = "(use core::num::traits::OverflowingMul)"
    text_edits = ["""
    use core::num::traits::OverflowingMul;

    """]

    [[completions]]
    completion_label = "OverflowingSub"
    completion_label_path = "(use core::num::traits::OverflowingSub)"
    text_edits = ["""
    use core::num::traits::OverflowingSub;

    """]
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
    completion_label = "SpanImpl"
    completion_label_path = "(use core::array::SpanImpl)"
    text_edits = ["""
    use core::array::SpanImpl;

    """]

    [[completions]]
    completion_label = "SpanIter"
    completion_label_path = "(use core::array::SpanIter)"
    text_edits = ["""
    use core::array::SpanIter;

    """]

    [[completions]]
    completion_label = "panics"
    completion_label_path = "(use core::panics)"
    text_edits = ["""
    use core::panics;

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
fn impl_in_let_type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        type Bar;
    }

    impl MyImpl of MyTrait {
        type Bar = u32;
    }

    fn a() {
        let a: MyIm<caret> = 1;
    }
    ",@r#"
    caret = """
        let a: MyIm<caret> = 1;
    """

    [[completions]]
    completion_label = "MyImpl"
    "#);
}

#[test]
fn trait_in_let_type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        type Bar;
    }

    impl MyImpl of MyTrait {
        type Bar = u32;
    }

    fn a() {
        let a: MyTr<caret> = 1;
    }
    ",@r#"
    caret = """
        let a: MyTr<caret> = 1;
    """

    [[completions]]
    completion_label = "MyTrait"
    "#);
}
