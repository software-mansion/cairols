use lsp_types::request::Completion;

use crate::{completions::completion_fixture, support::insta::test_transform_plain};

#[test]
fn no_text_in_function_context() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        <caret>
    }
    ",@r#"
    caret = """
        <caret>
    """

    [[completions]]
    completion_label = "MyStruct"

    [[completions]]
    completion_label = "a(...)"
    completion_label_path = "(use a)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "a()"

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
    completion_label = "Err"

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
    completion_label = "None"

    [[completions]]
    completion_label = "Not"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "NullableTrait"

    [[completions]]
    completion_label = "Ok"

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
    completion_label = "Some"

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
    completion_label = "assert(...)"
    completion_label_path = "(use assert)"
    completion_label_type_info = "fn(cond: bool, err_code: felt252) -> ()"
    insert_text = "assert(${1:cond}, ${2:err_code})"

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
    completion_label = "panic(...)"
    completion_label_path = "(use panic)"
    completion_label_type_info = "fn(data: Array<felt252>) -> crate::never"
    insert_text = "panic(${1:data})"

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
    completion_label = "ALPHA"
    completion_label_path = "(use core::ec::stark_curve::ALPHA)"
    text_edits = ["""
    use core::ec::stark_curve::ALPHA;

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
    completion_label = "BETA"
    completion_label_path = "(use core::ec::stark_curve::BETA)"
    text_edits = ["""
    use core::ec::stark_curve::BETA;

    """]

    [[completions]]
    completion_label = "BYTE_ARRAY_MAGIC"
    completion_label_path = "(use core::byte_array::BYTE_ARRAY_MAGIC)"
    text_edits = ["""
    use core::byte_array::BYTE_ARRAY_MAGIC;

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
    completion_label = "ClassHashZeroable"
    completion_label_path = "(use starknet::class_hash::ClassHashZeroable)"
    text_edits = ["""
    use starknet::class_hash::ClassHashZeroable;

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
    completion_label = "ContractAddressZeroable"
    completion_label_path = "(use starknet::contract_address::ContractAddressZeroable)"
    text_edits = ["""
    use starknet::contract_address::ContractAddressZeroable;

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
    completion_label = "Done"
    completion_label_path = "(use core::circuit::AddInputResult::Done)"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "(use core::internal::DropWith)"
    text_edits = ["""
    use core::internal::DropWith;

    """]

    [[completions]]
    completion_label = "EarlyReturn"
    completion_label_path = "(use core::internal::LoopResult::EarlyReturn)"
    text_edits = ["""
    use core::internal::LoopResult::EarlyReturn;

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
    completion_label = "Err"
    completion_label_path = "(use PanicResult::Err)"
    text_edits = ["""
    use PanicResult::Err;

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
    completion_label = "EthAddressZeroable"
    completion_label_path = "(use starknet::eth_address::EthAddressZeroable)"
    text_edits = ["""
    use starknet::eth_address::EthAddressZeroable;

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
    completion_label = "False"
    completion_label_path = "(use bool::False)"
    text_edits = ["""
    use bool::False;

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
    completion_label = "FromNullableResult"
    completion_label_path = "(use core::nullable::FromNullableResult)"
    text_edits = ["""
    use core::nullable::FromNullableResult;

    """]

    [[completions]]
    completion_label = "GEN_X"
    completion_label_path = "(use core::ec::stark_curve::GEN_X)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_X;

    """]

    [[completions]]
    completion_label = "GEN_Y"
    completion_label_path = "(use core::ec::stark_curve::GEN_Y)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_Y;

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
    completion_label = "More"
    completion_label_path = "(use core::circuit::AddInputResult::More)"
    text_edits = ["""
    use core::circuit::AddInputResult::More;

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
    completion_label = "None"
    completion_label_path = "(use core::internal::OptionRev::None)"
    text_edits = ["""
    use core::internal::OptionRev::None;

    """]

    [[completions]]
    completion_label = "Normal"
    completion_label_path = "(use core::internal::LoopResult::Normal)"
    text_edits = ["""
    use core::internal::LoopResult::Normal;

    """]

    [[completions]]
    completion_label = "NotNull"
    completion_label_path = "(use core::nullable::FromNullableResult::NotNull)"
    text_edits = ["""
    use core::nullable::FromNullableResult::NotNull;

    """]

    [[completions]]
    completion_label = "Null"
    completion_label_path = "(use core::nullable::FromNullableResult::Null)"
    text_edits = ["""
    use core::nullable::FromNullableResult::Null;

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
    completion_label = "ORDER"
    completion_label_path = "(use core::ec::stark_curve::ORDER)"
    text_edits = ["""
    use core::ec::stark_curve::ORDER;

    """]

    [[completions]]
    completion_label = "Ok"
    completion_label_path = "(use PanicResult::Ok)"
    text_edits = ["""
    use PanicResult::Ok;

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
    completion_label = "Some"
    completion_label_path = "(use core::internal::OptionRev::Some)"
    text_edits = ["""
    use core::internal::OptionRev::Some;

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
    completion_label = "True"
    completion_label_path = "(use bool::True)"
    text_edits = ["""
    use bool::True;

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
    completion_label = "VALIDATED"
    completion_label_path = "(use starknet::VALIDATED)"
    text_edits = ["""
    use starknet::VALIDATED;

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
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

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
    completion_label = "bounded_int_add(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_add)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_add;

    """]

    [[completions]]
    completion_label = "bounded_int_constrain(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_constrain)"
    completion_label_type_info = "fn(value: T) -> Result<H::LowT, H::HighT> nopanic"
    insert_text = "bounded_int_constrain(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_constrain;

    """]

    [[completions]]
    completion_label = "bounded_int_div_rem(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_div_rem)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: NonZero<Rhs>) -> (H::DivT, H::RemT) nopanic"
    insert_text = "bounded_int_div_rem(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_div_rem;

    """]

    [[completions]]
    completion_label = "bounded_int_is_zero(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_is_zero)"
    completion_label_type_info = "fn(value: T) -> crate::zeroable::IsZeroResult<T> nopanic"
    insert_text = "bounded_int_is_zero(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_is_zero;

    """]

    [[completions]]
    completion_label = "bounded_int_mul(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_mul)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_mul;

    """]

    [[completions]]
    completion_label = "bounded_int_sub(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_sub)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_sub;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_max(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_max)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_max(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_max;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_min(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_min)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_min(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_min;

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
    completion_label = "cairo_keccak(...)"
    completion_label_path = "(use core::keccak::cairo_keccak)"
    completion_label_type_info = "fn(ref input: Array<u64>, last_input_word: u64, last_input_num_bytes: u32) -> u256"
    insert_text = "cairo_keccak(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::keccak::cairo_keccak;

    """]

    [[completions]]
    completion_label = "call_contract_syscall(...)"
    completion_label_path = "(use starknet::syscalls::call_contract_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "call_contract_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::call_contract_syscall;

    """]

    [[completions]]
    completion_label = "cheatcode(...)"
    completion_label_path = "(use starknet::testing::cheatcode)"
    completion_label_type_info = "fn(input: Span<felt252>) -> Span<felt252> nopanic"
    insert_text = "cheatcode(${1:input})"
    text_edits = ["""
    use starknet::testing::cheatcode;

    """]

    [[completions]]
    completion_label = "check_ecdsa_signature(...)"
    completion_label_path = "(use core::ecdsa::check_ecdsa_signature)"
    completion_label_type_info = "fn(message_hash: felt252, public_key: felt252, signature_r: felt252, signature_s: felt252) -> bool"
    insert_text = "check_ecdsa_signature(${1:message_hash}, ${2:public_key}, ${3:signature_r}, ${4:signature_s})"
    text_edits = ["""
    use core::ecdsa::check_ecdsa_signature;

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
    completion_label = "circuit_add(...)"
    completion_label_path = "(use core::circuit::circuit_add)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<AddModGate<Lhs, Rhs>>"
    insert_text = "circuit_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_add;

    """]

    [[completions]]
    completion_label = "circuit_inverse(...)"
    completion_label_path = "(use core::circuit::circuit_inverse)"
    completion_label_type_info = "fn(input: CircuitElement<Input>) -> CircuitElement<InverseGate<Input>>"
    insert_text = "circuit_inverse(${1:input})"
    text_edits = ["""
    use core::circuit::circuit_inverse;

    """]

    [[completions]]
    completion_label = "circuit_mul(...)"
    completion_label_path = "(use core::circuit::circuit_mul)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<MulModGate<Lhs, Rhs>>"
    insert_text = "circuit_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_mul;

    """]

    [[completions]]
    completion_label = "circuit_sub(...)"
    completion_label_path = "(use core::circuit::circuit_sub)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<SubModGate<Lhs, Rhs>>"
    insert_text = "circuit_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_sub;

    """]

    [[completions]]
    completion_label = "class_hash"
    completion_label_path = "(use starknet::class_hash)"
    text_edits = ["""
    use starknet::class_hash;

    """]

    [[completions]]
    completion_label = "class_hash_const(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_const)"
    completion_label_type_info = "fn() -> ClassHash nopanic"
    insert_text = "class_hash_const()"
    text_edits = ["""
    use starknet::class_hash::class_hash_const;

    """]

    [[completions]]
    completion_label = "class_hash_to_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_to_felt252)"
    completion_label_type_info = "fn(address: ClassHash) -> felt252 nopanic"
    insert_text = "class_hash_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_to_felt252;

    """]

    [[completions]]
    completion_label = "class_hash_try_from_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ClassHash> nopanic"
    insert_text = "class_hash_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_try_from_felt252;

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
    completion_label = "compute_keccak_byte_array(...)"
    completion_label_path = "(use core::keccak::compute_keccak_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> u256"
    insert_text = "compute_keccak_byte_array(${1:arr})"
    text_edits = ["""
    use core::keccak::compute_keccak_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_byte_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> [u32; 8]"
    insert_text = "compute_sha256_byte_array(${1:arr})"
    text_edits = ["""
    use core::sha256::compute_sha256_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: u32) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array_safe(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array_safe)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: BoundedInt<0, 3>) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array_safe(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array_safe;

    """]

    [[completions]]
    completion_label = "contract_address"
    completion_label_path = "(use starknet::contract_address)"
    text_edits = ["""
    use starknet::contract_address;

    """]

    [[completions]]
    completion_label = "contract_address_const(...)"
    completion_label_path = "(use starknet::contract_address_const)"
    completion_label_type_info = "fn() -> ContractAddress nopanic"
    insert_text = "contract_address_const()"
    text_edits = ["""
    use starknet::contract_address_const;

    """]

    [[completions]]
    completion_label = "contract_address_to_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_to_felt252)"
    completion_label_type_info = "fn(address: ContractAddress) -> felt252 nopanic"
    insert_text = "contract_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_to_felt252;

    """]

    [[completions]]
    completion_label = "contract_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ContractAddress> nopanic"
    insert_text = "contract_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "debug"
    completion_label_path = "(use core::debug)"
    text_edits = ["""
    use core::debug;

    """]

    [[completions]]
    completion_label = "deploy_syscall(...)"
    completion_label_path = "(use starknet::syscalls::deploy_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, contract_address_salt: felt252, calldata: Span<felt252>, deploy_from_zero: bool) -> Result<(ContractAddress, Span<felt252>), Array<felt252>> nopanic"
    insert_text = "deploy_syscall(${1:class_hash}, ${2:contract_address_salt}, ${3:calldata}, ${4:deploy_from_zero})"
    text_edits = ["""
    use starknet::syscalls::deploy_syscall;

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
    completion_label = "downcast(...)"
    completion_label_path = "(use core::internal::bounded_int::downcast)"
    completion_label_type_info = "fn(x: FromType) -> Option<ToType> nopanic"
    insert_text = "downcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::downcast;

    """]

    [[completions]]
    completion_label = "ec"
    completion_label_path = "(use core::ec)"
    text_edits = ["""
    use core::ec;

    """]

    [[completions]]
    completion_label = "ec_point_unwrap(...)"
    completion_label_path = "(use core::ec::ec_point_unwrap)"
    completion_label_type_info = "fn(p: NonZero<EcPoint>) -> (felt252, felt252) nopanic"
    insert_text = "ec_point_unwrap(${1:p})"
    text_edits = ["""
    use core::ec::ec_point_unwrap;

    """]

    [[completions]]
    completion_label = "ecdsa"
    completion_label_path = "(use core::ecdsa)"
    text_edits = ["""
    use core::ecdsa;

    """]

    [[completions]]
    completion_label = "egcd(...)"
    completion_label_path = "(use core::math::egcd)"
    completion_label_type_info = "fn(a: NonZero<T>, b: NonZero<T>) -> (T, T, T, bool)"
    insert_text = "egcd(${1:a}, ${2:b})"
    text_edits = ["""
    use core::math::egcd;

    """]

    [[completions]]
    completion_label = "emit_event_syscall(...)"
    completion_label_path = "(use starknet::syscalls::emit_event_syscall)"
    completion_label_type_info = "fn(keys: Span<felt252>, data: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "emit_event_syscall(${1:keys}, ${2:data})"
    text_edits = ["""
    use starknet::syscalls::emit_event_syscall;

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
    completion_label = "felt252_div(...)"
    completion_label_path = "(use core::felt252_div)"
    completion_label_type_info = "fn(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic"
    insert_text = "felt252_div(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::felt252_div;

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
    completion_label = "gas_reserve_create(...)"
    completion_label_path = "(use core::gas::gas_reserve_create)"
    completion_label_type_info = "fn(amount: u128) -> Option<GasReserve> nopanic"
    insert_text = "gas_reserve_create(${1:amount})"
    text_edits = ["""
    use core::gas::gas_reserve_create;

    """]

    [[completions]]
    completion_label = "gas_reserve_utilize(...)"
    completion_label_path = "(use core::gas::gas_reserve_utilize)"
    completion_label_type_info = "fn(reserve: GasReserve) -> () nopanic"
    insert_text = "gas_reserve_utilize(${1:reserve})"
    text_edits = ["""
    use core::gas::gas_reserve_utilize;

    """]

    [[completions]]
    completion_label = "get"
    completion_label_path = "(use core::ops::get)"
    text_edits = ["""
    use core::ops::get;

    """]

    [[completions]]
    completion_label = "get_available_gas(...)"
    completion_label_path = "(use core::testing::get_available_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_available_gas()"
    text_edits = ["""
    use core::testing::get_available_gas;

    """]

    [[completions]]
    completion_label = "get_block_hash_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_block_hash_syscall)"
    completion_label_type_info = "fn(block_number: u64) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "get_block_hash_syscall(${1:block_number})"
    text_edits = ["""
    use starknet::syscalls::get_block_hash_syscall;

    """]

    [[completions]]
    completion_label = "get_block_info(...)"
    completion_label_path = "(use starknet::get_block_info)"
    completion_label_type_info = "fn() -> Box<BlockInfo>"
    insert_text = "get_block_info()"
    text_edits = ["""
    use starknet::get_block_info;

    """]

    [[completions]]
    completion_label = "get_block_number(...)"
    completion_label_path = "(use starknet::get_block_number)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_number()"
    text_edits = ["""
    use starknet::get_block_number;

    """]

    [[completions]]
    completion_label = "get_block_timestamp(...)"
    completion_label_path = "(use starknet::get_block_timestamp)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_timestamp()"
    text_edits = ["""
    use starknet::get_block_timestamp;

    """]

    [[completions]]
    completion_label = "get_builtin_costs(...)"
    completion_label_path = "(use core::gas::get_builtin_costs)"
    completion_label_type_info = "fn() -> BuiltinCosts nopanic"
    insert_text = "get_builtin_costs()"
    text_edits = ["""
    use core::gas::get_builtin_costs;

    """]

    [[completions]]
    completion_label = "get_caller_address(...)"
    completion_label_path = "(use starknet::get_caller_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_caller_address()"
    text_edits = ["""
    use starknet::get_caller_address;

    """]

    [[completions]]
    completion_label = "get_class_hash_at_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_class_hash_at_syscall)"
    completion_label_type_info = "fn(contract_address: ContractAddress) -> Result<ClassHash, Array<felt252>> nopanic"
    insert_text = "get_class_hash_at_syscall(${1:contract_address})"
    text_edits = ["""
    use starknet::syscalls::get_class_hash_at_syscall;

    """]

    [[completions]]
    completion_label = "get_contract_address(...)"
    completion_label_path = "(use starknet::get_contract_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_contract_address()"
    text_edits = ["""
    use starknet::get_contract_address;

    """]

    [[completions]]
    completion_label = "get_execution_info(...)"
    completion_label_path = "(use starknet::get_execution_info)"
    completion_label_type_info = "fn() -> Box<starknet::ExecutionInfo>"
    insert_text = "get_execution_info()"
    text_edits = ["""
    use starknet::get_execution_info;

    """]

    [[completions]]
    completion_label = "get_execution_info_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v2_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v2_syscall)"
    completion_label_type_info = "fn() -> Result<Box<starknet::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v2_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v2_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v3_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v3_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::v3::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v3_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v3_syscall;

    """]

    [[completions]]
    completion_label = "get_tx_info(...)"
    completion_label_path = "(use starknet::get_tx_info)"
    completion_label_type_info = "fn() -> Box<starknet::TxInfo>"
    insert_text = "get_tx_info()"
    text_edits = ["""
    use starknet::get_tx_info;

    """]

    [[completions]]
    completion_label = "get_unspent_gas(...)"
    completion_label_path = "(use core::testing::get_unspent_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_unspent_gas()"
    text_edits = ["""
    use core::testing::get_unspent_gas;

    """]

    [[completions]]
    completion_label = "hades_permutation(...)"
    completion_label_path = "(use core::poseidon::hades_permutation)"
    completion_label_type_info = "fn(s0: felt252, s1: felt252, s2: felt252) -> (felt252, felt252, felt252) nopanic"
    insert_text = "hades_permutation(${1:s0}, ${2:s1}, ${3:s2})"
    text_edits = ["""
    use core::poseidon::hades_permutation;

    """]

    [[completions]]
    completion_label = "hash"
    completion_label_path = "(use core::hash)"
    text_edits = ["""
    use core::hash;

    """]

    [[completions]]
    completion_label = "i128_diff(...)"
    completion_label_path = "(use core::integer::i128_diff)"
    completion_label_type_info = "fn(lhs: i128, rhs: i128) -> Result<u128, u128> nopanic"
    insert_text = "i128_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i128_diff;

    """]

    [[completions]]
    completion_label = "i16_diff(...)"
    completion_label_path = "(use core::integer::i16_diff)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> Result<u16, u16> nopanic"
    insert_text = "i16_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_diff;

    """]

    [[completions]]
    completion_label = "i16_wide_mul(...)"
    completion_label_path = "(use core::integer::i16_wide_mul)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> i32 nopanic"
    insert_text = "i16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_wide_mul;

    """]

    [[completions]]
    completion_label = "i32_diff(...)"
    completion_label_path = "(use core::integer::i32_diff)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> Result<u32, u32> nopanic"
    insert_text = "i32_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_diff;

    """]

    [[completions]]
    completion_label = "i32_wide_mul(...)"
    completion_label_path = "(use core::integer::i32_wide_mul)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> i64 nopanic"
    insert_text = "i32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_wide_mul;

    """]

    [[completions]]
    completion_label = "i64_diff(...)"
    completion_label_path = "(use core::integer::i64_diff)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> Result<u64, u64> nopanic"
    insert_text = "i64_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_diff;

    """]

    [[completions]]
    completion_label = "i64_wide_mul(...)"
    completion_label_path = "(use core::integer::i64_wide_mul)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> i128 nopanic"
    insert_text = "i64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_wide_mul;

    """]

    [[completions]]
    completion_label = "i8_diff(...)"
    completion_label_path = "(use core::integer::i8_diff)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> Result<u8, u8> nopanic"
    insert_text = "i8_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_diff;

    """]

    [[completions]]
    completion_label = "i8_wide_mul(...)"
    completion_label_path = "(use core::integer::i8_wide_mul)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> i16 nopanic"
    insert_text = "i8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_wide_mul;

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
    completion_label = "inv_mod(...)"
    completion_label_path = "(use core::math::inv_mod)"
    completion_label_type_info = "fn(a: NonZero<T>, n: NonZero<T>) -> Option<T>"
    insert_text = "inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::inv_mod;

    """]

    [[completions]]
    completion_label = "is_eth_signature_valid(...)"
    completion_label_path = "(use starknet::eth_signature::is_eth_signature_valid)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> Result<(), felt252>"
    insert_text = "is_eth_signature_valid(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::is_eth_signature_valid;

    """]

    [[completions]]
    completion_label = "is_signature_entry_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_entry_valid)"
    completion_label_type_info = "fn(value: u256) -> bool"
    insert_text = "is_signature_entry_valid(${1:value})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_entry_valid;

    """]

    [[completions]]
    completion_label = "is_signature_s_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_s_valid)"
    completion_label_type_info = "fn(s: u256) -> bool"
    insert_text = "is_signature_s_valid(${1:s})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_s_valid;

    """]

    [[completions]]
    completion_label = "is_valid_signature(...)"
    completion_label_path = "(use starknet::secp256_trait::is_valid_signature)"
    completion_label_type_info = "fn(msg_hash: u256, r: u256, s: u256, public_key: Secp256Point) -> bool"
    insert_text = "is_valid_signature(${1:msg_hash}, ${2:r}, ${3:s}, ${4:public_key})"
    text_edits = ["""
    use starknet::secp256_trait::is_valid_signature;

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
    completion_label = "keccak_syscall(...)"
    completion_label_path = "(use starknet::syscalls::keccak_syscall)"
    completion_label_type_info = "fn(input: Span<u64>) -> Result<u256, Array<felt252>> nopanic"
    insert_text = "keccak_syscall(${1:input})"
    text_edits = ["""
    use starknet::syscalls::keccak_syscall;

    """]

    [[completions]]
    completion_label = "keccak_u256s_be_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_be_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_be_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_be_inputs;

    """]

    [[completions]]
    completion_label = "keccak_u256s_le_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_le_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_le_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_le_inputs;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "m31_add(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_add)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_add(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_add;

    """]

    [[completions]]
    completion_label = "m31_div(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_div)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: NonZero<crate::internal::bounded_int::BoundedInt<0, 2147483646>>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_div(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_div;

    """]

    [[completions]]
    completion_label = "m31_mul(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_mul)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_mul;

    """]

    [[completions]]
    completion_label = "m31_ops"
    completion_label_path = "(use core::qm31::m31_ops)"
    text_edits = ["""
    use core::qm31::m31_ops;

    """]

    [[completions]]
    completion_label = "m31_sub(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_sub)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_sub;

    """]

    [[completions]]
    completion_label = "match_nullable(...)"
    completion_label_path = "(use core::nullable::match_nullable)"
    completion_label_type_info = "fn(value: Nullable<T>) -> FromNullableResult<T> nopanic"
    insert_text = "match_nullable(${1:value})"
    text_edits = ["""
    use core::nullable::match_nullable;

    """]

    [[completions]]
    completion_label = "math"
    completion_label_path = "(use core::math)"
    text_edits = ["""
    use core::math;

    """]

    [[completions]]
    completion_label = "max(...)"
    completion_label_path = "(use core::cmp::max)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "max(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::max;

    """]

    [[completions]]
    completion_label = "meta_tx_v0_syscall(...)"
    completion_label_path = "(use starknet::syscalls::meta_tx_v0_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>, signature: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "meta_tx_v0_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata}, ${4:signature})"
    text_edits = ["""
    use starknet::syscalls::meta_tx_v0_syscall;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
    text_edits = ["""
    use core::metaprogramming;

    """]

    [[completions]]
    completion_label = "min(...)"
    completion_label_path = "(use core::cmp::min)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "min(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::min;

    """]

    [[completions]]
    completion_label = "minmax(...)"
    completion_label_path = "(use core::cmp::minmax)"
    completion_label_type_info = "fn(a: T, b: T) -> (T, T)"
    insert_text = "minmax(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::minmax;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "null(...)"
    completion_label_path = "(use core::nullable::null)"
    completion_label_type_info = "fn() -> Nullable<T> nopanic"
    insert_text = "null()"
    text_edits = ["""
    use core::nullable::null;

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
    completion_label = "panic_with_byte_array(...)"
    completion_label_path = "(use core::panics::panic_with_byte_array)"
    completion_label_type_info = "fn(err: @ByteArray) -> crate::never"
    insert_text = "panic_with_byte_array(${1:err})"
    text_edits = ["""
    use core::panics::panic_with_byte_array;

    """]

    [[completions]]
    completion_label = "panic_with_const_felt252(...)"
    completion_label_path = "(use core::panic_with_const_felt252)"
    completion_label_type_info = "fn() -> never"
    insert_text = "panic_with_const_felt252()"
    text_edits = ["""
    use core::panic_with_const_felt252;

    """]

    [[completions]]
    completion_label = "panic_with_felt252(...)"
    completion_label_path = "(use core::panic_with_felt252)"
    completion_label_type_info = "fn(err_code: felt252) -> never"
    insert_text = "panic_with_felt252(${1:err_code})"
    text_edits = ["""
    use core::panic_with_felt252;

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
    completion_label = "pedersen(...)"
    completion_label_path = "(use core::pedersen::pedersen)"
    completion_label_type_info = "fn(a: felt252, b: felt252) -> felt252 nopanic"
    insert_text = "pedersen(${1:a}, ${2:b})"
    text_edits = ["""
    use core::pedersen::pedersen;

    """]

    [[completions]]
    completion_label = "pop_l2_to_l1_message(...)"
    completion_label_path = "(use starknet::testing::pop_l2_to_l1_message)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(felt252, Span<felt252>)>"
    insert_text = "pop_l2_to_l1_message(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_l2_to_l1_message;

    """]

    [[completions]]
    completion_label = "pop_log(...)"
    completion_label_path = "(use starknet::testing::pop_log)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<T>"
    insert_text = "pop_log(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log;

    """]

    [[completions]]
    completion_label = "pop_log_raw(...)"
    completion_label_path = "(use starknet::testing::pop_log_raw)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(Span<felt252>, Span<felt252>)>"
    insert_text = "pop_log_raw(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log_raw;

    """]

    [[completions]]
    completion_label = "poseidon"
    completion_label_path = "(use core::poseidon)"
    text_edits = ["""
    use core::poseidon;

    """]

    [[completions]]
    completion_label = "poseidon_hash_span(...)"
    completion_label_path = "(use core::poseidon::poseidon_hash_span)"
    completion_label_type_info = "fn(span: Span<felt252>) -> felt252"
    insert_text = "poseidon_hash_span(${1:span})"
    text_edits = ["""
    use core::poseidon::poseidon_hash_span;

    """]

    [[completions]]
    completion_label = "pow"
    completion_label_path = "(use core::num::traits::ops::pow)"
    text_edits = ["""
    use core::num::traits::ops::pow;

    """]

    [[completions]]
    completion_label = "print_byte_array_as_string(...)"
    completion_label_path = "(use core::debug::print_byte_array_as_string)"
    completion_label_type_info = "fn(self: @ByteArray) -> ()"
    insert_text = "print_byte_array_as_string()"
    text_edits = ["""
    use core::debug::print_byte_array_as_string;

    """]

    [[completions]]
    completion_label = "public_key_point_to_eth_address(...)"
    completion_label_path = "(use starknet::eth_signature::public_key_point_to_eth_address)"
    completion_label_type_info = "fn(public_key_point: Secp256Point) -> EthAddress"
    insert_text = "public_key_point_to_eth_address(${1:public_key_point})"
    text_edits = ["""
    use starknet::eth_signature::public_key_point_to_eth_address;

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
    completion_label = "qm31_const(...)"
    completion_label_path = "(use core::qm31::qm31_const)"
    completion_label_type_info = "fn() -> qm31 nopanic"
    insert_text = "qm31_const()"
    text_edits = ["""
    use core::qm31::qm31_const;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use core::ecdsa::recover_public_key)"
    completion_label_type_info = "fn(message_hash: felt252, signature_r: felt252, signature_s: felt252, y_parity: bool) -> Option<felt252>"
    insert_text = "recover_public_key(${1:message_hash}, ${2:signature_r}, ${3:signature_s}, ${4:y_parity})"
    text_edits = ["""
    use core::ecdsa::recover_public_key;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use starknet::secp256_trait::recover_public_key)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature) -> Option<Secp256Point>"
    insert_text = "recover_public_key(${1:msg_hash}, ${2:signature})"
    text_edits = ["""
    use starknet::secp256_trait::recover_public_key;

    """]

    [[completions]]
    completion_label = "redeposit_gas(...)"
    completion_label_path = "(use core::gas::redeposit_gas)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "redeposit_gas()"
    text_edits = ["""
    use core::gas::redeposit_gas;

    """]

    [[completions]]
    completion_label = "replace_class_syscall(...)"
    completion_label_path = "(use starknet::syscalls::replace_class_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash) -> Result<(), Array<felt252>> nopanic"
    insert_text = "replace_class_syscall(${1:class_hash})"
    text_edits = ["""
    use starknet::syscalls::replace_class_syscall;

    """]

    [[completions]]
    completion_label = "require_implicit(...)"
    completion_label_path = "(use core::internal::require_implicit)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "require_implicit()"
    text_edits = ["""
    use core::internal::require_implicit;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]

    [[completions]]
    completion_label = "revoke_ap_tracking(...)"
    completion_label_path = "(use core::internal::revoke_ap_tracking)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "revoke_ap_tracking()"
    text_edits = ["""
    use core::internal::revoke_ap_tracking;

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
    completion_label = "send_message_to_l1_syscall(...)"
    completion_label_path = "(use starknet::syscalls::send_message_to_l1_syscall)"
    completion_label_type_info = "fn(to_address: felt252, payload: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "send_message_to_l1_syscall(${1:to_address}, ${2:payload})"
    text_edits = ["""
    use starknet::syscalls::send_message_to_l1_syscall;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "set_account_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_account_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_account_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_account_contract_address;

    """]

    [[completions]]
    completion_label = "set_block_hash(...)"
    completion_label_path = "(use starknet::testing::set_block_hash)"
    completion_label_type_info = "fn(block_number: u64, value: felt252) -> ()"
    insert_text = "set_block_hash(${1:block_number}, ${2:value})"
    text_edits = ["""
    use starknet::testing::set_block_hash;

    """]

    [[completions]]
    completion_label = "set_block_number(...)"
    completion_label_path = "(use starknet::testing::set_block_number)"
    completion_label_type_info = "fn(block_number: u64) -> ()"
    insert_text = "set_block_number(${1:block_number})"
    text_edits = ["""
    use starknet::testing::set_block_number;

    """]

    [[completions]]
    completion_label = "set_block_timestamp(...)"
    completion_label_path = "(use starknet::testing::set_block_timestamp)"
    completion_label_type_info = "fn(block_timestamp: u64) -> ()"
    insert_text = "set_block_timestamp(${1:block_timestamp})"
    text_edits = ["""
    use starknet::testing::set_block_timestamp;

    """]

    [[completions]]
    completion_label = "set_caller_address(...)"
    completion_label_path = "(use starknet::testing::set_caller_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_caller_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_caller_address;

    """]

    [[completions]]
    completion_label = "set_chain_id(...)"
    completion_label_path = "(use starknet::testing::set_chain_id)"
    completion_label_type_info = "fn(chain_id: felt252) -> ()"
    insert_text = "set_chain_id(${1:chain_id})"
    text_edits = ["""
    use starknet::testing::set_chain_id;

    """]

    [[completions]]
    completion_label = "set_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_contract_address;

    """]

    [[completions]]
    completion_label = "set_max_fee(...)"
    completion_label_path = "(use starknet::testing::set_max_fee)"
    completion_label_type_info = "fn(fee: u128) -> ()"
    insert_text = "set_max_fee(${1:fee})"
    text_edits = ["""
    use starknet::testing::set_max_fee;

    """]

    [[completions]]
    completion_label = "set_nonce(...)"
    completion_label_path = "(use starknet::testing::set_nonce)"
    completion_label_type_info = "fn(nonce: felt252) -> ()"
    insert_text = "set_nonce(${1:nonce})"
    text_edits = ["""
    use starknet::testing::set_nonce;

    """]

    [[completions]]
    completion_label = "set_sequencer_address(...)"
    completion_label_path = "(use starknet::testing::set_sequencer_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_sequencer_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_sequencer_address;

    """]

    [[completions]]
    completion_label = "set_signature(...)"
    completion_label_path = "(use starknet::testing::set_signature)"
    completion_label_type_info = "fn(signature: Span<felt252>) -> ()"
    insert_text = "set_signature(${1:signature})"
    text_edits = ["""
    use starknet::testing::set_signature;

    """]

    [[completions]]
    completion_label = "set_transaction_hash(...)"
    completion_label_path = "(use starknet::testing::set_transaction_hash)"
    completion_label_type_info = "fn(hash: felt252) -> ()"
    insert_text = "set_transaction_hash(${1:hash})"
    text_edits = ["""
    use starknet::testing::set_transaction_hash;

    """]

    [[completions]]
    completion_label = "set_version(...)"
    completion_label_path = "(use starknet::testing::set_version)"
    completion_label_type_info = "fn(version: felt252) -> ()"
    insert_text = "set_version(${1:version})"
    text_edits = ["""
    use starknet::testing::set_version;

    """]

    [[completions]]
    completion_label = "sha256"
    completion_label_path = "(use core::sha256)"
    text_edits = ["""
    use core::sha256;

    """]

    [[completions]]
    completion_label = "sha256_process_block_syscall(...)"
    completion_label_path = "(use starknet::syscalls::sha256_process_block_syscall)"
    completion_label_type_info = "fn(state: crate::sha256::Sha256StateHandle, input: Box<[u32; 16]>) -> Result<crate::sha256::Sha256StateHandle, Array<felt252>> nopanic"
    insert_text = "sha256_process_block_syscall(${1:state}, ${2:input})"
    text_edits = ["""
    use starknet::syscalls::sha256_process_block_syscall;

    """]

    [[completions]]
    completion_label = "signature_from_vrs(...)"
    completion_label_path = "(use starknet::secp256_trait::signature_from_vrs)"
    completion_label_type_info = "fn(v: u32, r: u256, s: u256) -> Signature"
    insert_text = "signature_from_vrs(${1:v}, ${2:r}, ${3:s})"
    text_edits = ["""
    use starknet::secp256_trait::signature_from_vrs;

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
    completion_label = "storage_address_from_base(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base)"
    completion_label_type_info = "fn(base: StorageBaseAddress) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base(${1:base})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base;

    """]

    [[completions]]
    completion_label = "storage_address_from_base_and_offset(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base_and_offset)"
    completion_label_type_info = "fn(base: StorageBaseAddress, offset: u8) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base_and_offset(${1:base}, ${2:offset})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base_and_offset;

    """]

    [[completions]]
    completion_label = "storage_address_to_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_to_felt252)"
    completion_label_type_info = "fn(address: StorageAddress) -> felt252 nopanic"
    insert_text = "storage_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_to_felt252;

    """]

    [[completions]]
    completion_label = "storage_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<StorageAddress> nopanic"
    insert_text = "storage_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_base_address_const(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_const)"
    completion_label_type_info = "fn() -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_const()"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_const;

    """]

    [[completions]]
    completion_label = "storage_base_address_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_from_felt252)"
    completion_label_type_info = "fn(addr: felt252) -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_from_felt252(${1:addr})"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_read_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_read_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "storage_read_syscall(${1:address_domain}, ${2:address})"
    text_edits = ["""
    use starknet::syscalls::storage_read_syscall;

    """]

    [[completions]]
    completion_label = "storage_write_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_write_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress, value: felt252) -> Result<(), Array<felt252>> nopanic"
    insert_text = "storage_write_syscall(${1:address_domain}, ${2:address}, ${3:value})"
    text_edits = ["""
    use starknet::syscalls::storage_write_syscall;

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
    completion_label = "u128_byte_reverse(...)"
    completion_label_path = "(use core::integer::u128_byte_reverse)"
    completion_label_type_info = "fn(input: u128) -> u128 nopanic"
    insert_text = "u128_byte_reverse(${1:input})"
    text_edits = ["""
    use core::integer::u128_byte_reverse;

    """]

    [[completions]]
    completion_label = "u128_overflowing_add(...)"
    completion_label_path = "(use core::integer::u128_overflowing_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_add;

    """]

    [[completions]]
    completion_label = "u128_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u128_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> (u128, bool) nopanic"
    insert_text = "u128_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u128_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u128_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u128_safe_divmod(...)"
    completion_label_path = "(use core::integer::u128_safe_divmod)"
    completion_label_type_info = "fn(lhs: u128, rhs: NonZero<u128>) -> (u128, u128) nopanic"
    insert_text = "u128_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_safe_divmod;

    """]

    [[completions]]
    completion_label = "u128_sqrt(...)"
    completion_label_path = "(use core::integer::u128_sqrt)"
    completion_label_type_info = "fn(value: u128) -> u64 nopanic"
    insert_text = "u128_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u128_sqrt;

    """]

    [[completions]]
    completion_label = "u128_wide_mul(...)"
    completion_label_path = "(use core::integer::u128_wide_mul)"
    completion_label_type_info = "fn(a: u128, b: u128) -> (u128, u128) nopanic"
    insert_text = "u128_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wide_mul;

    """]

    [[completions]]
    completion_label = "u128_wrapping_add(...)"
    completion_label_path = "(use core::integer::u128_wrapping_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_wrapping_add;

    """]

    [[completions]]
    completion_label = "u128_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u128_wrapping_sub)"
    completion_label_type_info = "fn(a: u128, b: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u16_overflowing_add(...)"
    completion_label_path = "(use core::integer::u16_overflowing_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_add;

    """]

    [[completions]]
    completion_label = "u16_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u16_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u16_safe_divmod(...)"
    completion_label_path = "(use core::integer::u16_safe_divmod)"
    completion_label_type_info = "fn(lhs: u16, rhs: NonZero<u16>) -> (u16, u16) nopanic"
    insert_text = "u16_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_safe_divmod;

    """]

    [[completions]]
    completion_label = "u16_sqrt(...)"
    completion_label_path = "(use core::integer::u16_sqrt)"
    completion_label_type_info = "fn(value: u16) -> u8 nopanic"
    insert_text = "u16_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u16_sqrt;

    """]

    [[completions]]
    completion_label = "u16_wide_mul(...)"
    completion_label_path = "(use core::integer::u16_wide_mul)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u32 nopanic"
    insert_text = "u16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wide_mul;

    """]

    [[completions]]
    completion_label = "u16_wrapping_add(...)"
    completion_label_path = "(use core::integer::u16_wrapping_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_add;

    """]

    [[completions]]
    completion_label = "u16_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u16_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u2"
    completion_label_path = "(use core::sha256::u2)"
    text_edits = ["""
    use core::sha256::u2;

    """]

    [[completions]]
    completion_label = "u256_div_mod_n(...)"
    completion_label_path = "(use core::math::u256_div_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> Option<u256>"
    insert_text = "u256_div_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_div_mod_n;

    """]

    [[completions]]
    completion_label = "u256_inv_mod(...)"
    completion_label_path = "(use core::math::u256_inv_mod)"
    completion_label_type_info = "fn(a: u256, n: NonZero<u256>) -> Option<NonZero<u256>>"
    insert_text = "u256_inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::u256_inv_mod;

    """]

    [[completions]]
    completion_label = "u256_mul_mod_n(...)"
    completion_label_path = "(use core::math::u256_mul_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> u256"
    insert_text = "u256_mul_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_mul_mod_n;

    """]

    [[completions]]
    completion_label = "u256_overflow_mul(...)"
    completion_label_path = "(use core::integer::u256_overflow_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflow_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_mul;

    """]

    [[completions]]
    completion_label = "u256_overflow_sub(...)"
    completion_label_path = "(use core::integer::u256_overflow_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflow_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_sub;

    """]

    [[completions]]
    completion_label = "u256_overflowing_add(...)"
    completion_label_path = "(use core::integer::u256_overflowing_add)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_add;

    """]

    [[completions]]
    completion_label = "u256_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u256_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u256_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u256_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u256_sqrt(...)"
    completion_label_path = "(use core::integer::u256_sqrt)"
    completion_label_type_info = "fn(a: u256) -> u128 nopanic"
    insert_text = "u256_sqrt(${1:a})"
    text_edits = ["""
    use core::integer::u256_sqrt;

    """]

    [[completions]]
    completion_label = "u256_wide_mul(...)"
    completion_label_path = "(use core::integer::u256_wide_mul)"
    completion_label_type_info = "fn(a: u256, b: u256) -> u512 nopanic"
    insert_text = "u256_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u256_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_overflowing_add(...)"
    completion_label_path = "(use core::integer::u32_overflowing_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_add;

    """]

    [[completions]]
    completion_label = "u32_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u32_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u32_safe_divmod(...)"
    completion_label_path = "(use core::integer::u32_safe_divmod)"
    completion_label_type_info = "fn(lhs: u32, rhs: NonZero<u32>) -> (u32, u32) nopanic"
    insert_text = "u32_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_safe_divmod;

    """]

    [[completions]]
    completion_label = "u32_sqrt(...)"
    completion_label_path = "(use core::integer::u32_sqrt)"
    completion_label_type_info = "fn(value: u32) -> u16 nopanic"
    insert_text = "u32_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u32_sqrt;

    """]

    [[completions]]
    completion_label = "u32_wide_mul(...)"
    completion_label_path = "(use core::integer::u32_wide_mul)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u64 nopanic"
    insert_text = "u32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_wrapping_add(...)"
    completion_label_path = "(use core::integer::u32_wrapping_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_add;

    """]

    [[completions]]
    completion_label = "u32_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u32_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_sub;

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
    completion_label = "u512_safe_div_rem_by_u256(...)"
    completion_label_path = "(use core::integer::u512_safe_div_rem_by_u256)"
    completion_label_type_info = "fn(lhs: u512, rhs: NonZero<u256>) -> (u512, u256) nopanic"
    insert_text = "u512_safe_div_rem_by_u256(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u512_safe_div_rem_by_u256;

    """]

    [[completions]]
    completion_label = "u64_overflowing_add(...)"
    completion_label_path = "(use core::integer::u64_overflowing_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_add;

    """]

    [[completions]]
    completion_label = "u64_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u64_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u64_safe_divmod(...)"
    completion_label_path = "(use core::integer::u64_safe_divmod)"
    completion_label_type_info = "fn(lhs: u64, rhs: NonZero<u64>) -> (u64, u64) nopanic"
    insert_text = "u64_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_safe_divmod;

    """]

    [[completions]]
    completion_label = "u64_sqrt(...)"
    completion_label_path = "(use core::integer::u64_sqrt)"
    completion_label_type_info = "fn(value: u64) -> u32 nopanic"
    insert_text = "u64_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u64_sqrt;

    """]

    [[completions]]
    completion_label = "u64_wide_mul(...)"
    completion_label_path = "(use core::integer::u64_wide_mul)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u128 nopanic"
    insert_text = "u64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wide_mul;

    """]

    [[completions]]
    completion_label = "u64_wrapping_add(...)"
    completion_label_path = "(use core::integer::u64_wrapping_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_add;

    """]

    [[completions]]
    completion_label = "u64_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u64_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u8_overflowing_add(...)"
    completion_label_path = "(use core::integer::u8_overflowing_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_add;

    """]

    [[completions]]
    completion_label = "u8_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u8_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u8_safe_divmod(...)"
    completion_label_path = "(use core::integer::u8_safe_divmod)"
    completion_label_type_info = "fn(lhs: u8, rhs: NonZero<u8>) -> (u8, u8) nopanic"
    insert_text = "u8_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_safe_divmod;

    """]

    [[completions]]
    completion_label = "u8_sqrt(...)"
    completion_label_path = "(use core::integer::u8_sqrt)"
    completion_label_type_info = "fn(value: u8) -> u8 nopanic"
    insert_text = "u8_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u8_sqrt;

    """]

    [[completions]]
    completion_label = "u8_wide_mul(...)"
    completion_label_path = "(use core::integer::u8_wide_mul)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u16 nopanic"
    insert_text = "u8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wide_mul;

    """]

    [[completions]]
    completion_label = "u8_wrapping_add(...)"
    completion_label_path = "(use core::integer::u8_wrapping_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_add;

    """]

    [[completions]]
    completion_label = "u8_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u8_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u96"
    completion_label_path = "(use core::circuit::u96)"
    text_edits = ["""
    use core::circuit::u96;

    """]

    [[completions]]
    completion_label = "upcast(...)"
    completion_label_path = "(use core::internal::bounded_int::upcast)"
    completion_label_type_info = "fn(x: FromType) -> ToType nopanic"
    insert_text = "upcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::upcast;

    """]

    [[completions]]
    completion_label = "verify_eth_signature(...)"
    completion_label_path = "(use starknet::eth_signature::verify_eth_signature)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> ()"
    insert_text = "verify_eth_signature(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::verify_eth_signature;

    """]

    [[completions]]
    completion_label = "withdraw_gas(...)"
    completion_label_path = "(use core::gas::withdraw_gas)"
    completion_label_type_info = "fn() -> Option<()> nopanic"
    insert_text = "withdraw_gas()"
    text_edits = ["""
    use core::gas::withdraw_gas;

    """]

    [[completions]]
    completion_label = "withdraw_gas_all(...)"
    completion_label_path = "(use core::gas::withdraw_gas_all)"
    completion_label_type_info = "fn(costs: BuiltinCosts) -> Option<()> nopanic"
    insert_text = "withdraw_gas_all(${1:costs})"
    text_edits = ["""
    use core::gas::withdraw_gas_all;

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

    [[completions]]
    completion_label = "zip(...)"
    completion_label_path = "(use core::iter::zip)"
    completion_label_type_info = "fn(a: A, b: B) -> Zip<AIntoIter::IntoIter, BIntoIter::IntoIter>"
    insert_text = "zip(${1:a}, ${2:b})"
    text_edits = ["""
    use core::iter::zip;

    """]
    "#);
}

#[test]
fn no_text_after_semicolon() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        let _x = 1;<caret>
    }
    ",@r#"
    caret = """
        let _x = 1;<caret>
    """

    [[completions]]
    completion_label = "MyStruct"

    [[completions]]
    completion_label = "a(...)"
    completion_label_path = "(use a)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "a()"

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
    completion_label = "Err"

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
    completion_label = "None"

    [[completions]]
    completion_label = "Not"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "NullableTrait"

    [[completions]]
    completion_label = "Ok"

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
    completion_label = "Some"

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
    completion_label = "assert(...)"
    completion_label_path = "(use assert)"
    completion_label_type_info = "fn(cond: bool, err_code: felt252) -> ()"
    insert_text = "assert(${1:cond}, ${2:err_code})"

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
    completion_label = "panic(...)"
    completion_label_path = "(use panic)"
    completion_label_type_info = "fn(data: Array<felt252>) -> crate::never"
    insert_text = "panic(${1:data})"

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
    completion_label = "ALPHA"
    completion_label_path = "(use core::ec::stark_curve::ALPHA)"
    text_edits = ["""
    use core::ec::stark_curve::ALPHA;

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
    completion_label = "BETA"
    completion_label_path = "(use core::ec::stark_curve::BETA)"
    text_edits = ["""
    use core::ec::stark_curve::BETA;

    """]

    [[completions]]
    completion_label = "BYTE_ARRAY_MAGIC"
    completion_label_path = "(use core::byte_array::BYTE_ARRAY_MAGIC)"
    text_edits = ["""
    use core::byte_array::BYTE_ARRAY_MAGIC;

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
    completion_label = "ClassHashZeroable"
    completion_label_path = "(use starknet::class_hash::ClassHashZeroable)"
    text_edits = ["""
    use starknet::class_hash::ClassHashZeroable;

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
    completion_label = "ContractAddressZeroable"
    completion_label_path = "(use starknet::contract_address::ContractAddressZeroable)"
    text_edits = ["""
    use starknet::contract_address::ContractAddressZeroable;

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
    completion_label = "Done"
    completion_label_path = "(use core::circuit::AddInputResult::Done)"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "(use core::internal::DropWith)"
    text_edits = ["""
    use core::internal::DropWith;

    """]

    [[completions]]
    completion_label = "EarlyReturn"
    completion_label_path = "(use core::internal::LoopResult::EarlyReturn)"
    text_edits = ["""
    use core::internal::LoopResult::EarlyReturn;

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
    completion_label = "Err"
    completion_label_path = "(use PanicResult::Err)"
    text_edits = ["""
    use PanicResult::Err;

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
    completion_label = "EthAddressZeroable"
    completion_label_path = "(use starknet::eth_address::EthAddressZeroable)"
    text_edits = ["""
    use starknet::eth_address::EthAddressZeroable;

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
    completion_label = "False"
    completion_label_path = "(use bool::False)"
    text_edits = ["""
    use bool::False;

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
    completion_label = "FromNullableResult"
    completion_label_path = "(use core::nullable::FromNullableResult)"
    text_edits = ["""
    use core::nullable::FromNullableResult;

    """]

    [[completions]]
    completion_label = "GEN_X"
    completion_label_path = "(use core::ec::stark_curve::GEN_X)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_X;

    """]

    [[completions]]
    completion_label = "GEN_Y"
    completion_label_path = "(use core::ec::stark_curve::GEN_Y)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_Y;

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
    completion_label = "More"
    completion_label_path = "(use core::circuit::AddInputResult::More)"
    text_edits = ["""
    use core::circuit::AddInputResult::More;

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
    completion_label = "None"
    completion_label_path = "(use core::internal::OptionRev::None)"
    text_edits = ["""
    use core::internal::OptionRev::None;

    """]

    [[completions]]
    completion_label = "Normal"
    completion_label_path = "(use core::internal::LoopResult::Normal)"
    text_edits = ["""
    use core::internal::LoopResult::Normal;

    """]

    [[completions]]
    completion_label = "NotNull"
    completion_label_path = "(use core::nullable::FromNullableResult::NotNull)"
    text_edits = ["""
    use core::nullable::FromNullableResult::NotNull;

    """]

    [[completions]]
    completion_label = "Null"
    completion_label_path = "(use core::nullable::FromNullableResult::Null)"
    text_edits = ["""
    use core::nullable::FromNullableResult::Null;

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
    completion_label = "ORDER"
    completion_label_path = "(use core::ec::stark_curve::ORDER)"
    text_edits = ["""
    use core::ec::stark_curve::ORDER;

    """]

    [[completions]]
    completion_label = "Ok"
    completion_label_path = "(use PanicResult::Ok)"
    text_edits = ["""
    use PanicResult::Ok;

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
    completion_label = "Some"
    completion_label_path = "(use core::internal::OptionRev::Some)"
    text_edits = ["""
    use core::internal::OptionRev::Some;

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
    completion_label = "True"
    completion_label_path = "(use bool::True)"
    text_edits = ["""
    use bool::True;

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
    completion_label = "VALIDATED"
    completion_label_path = "(use starknet::VALIDATED)"
    text_edits = ["""
    use starknet::VALIDATED;

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
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

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
    completion_label = "bounded_int_add(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_add)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_add;

    """]

    [[completions]]
    completion_label = "bounded_int_constrain(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_constrain)"
    completion_label_type_info = "fn(value: T) -> Result<H::LowT, H::HighT> nopanic"
    insert_text = "bounded_int_constrain(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_constrain;

    """]

    [[completions]]
    completion_label = "bounded_int_div_rem(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_div_rem)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: NonZero<Rhs>) -> (H::DivT, H::RemT) nopanic"
    insert_text = "bounded_int_div_rem(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_div_rem;

    """]

    [[completions]]
    completion_label = "bounded_int_is_zero(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_is_zero)"
    completion_label_type_info = "fn(value: T) -> crate::zeroable::IsZeroResult<T> nopanic"
    insert_text = "bounded_int_is_zero(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_is_zero;

    """]

    [[completions]]
    completion_label = "bounded_int_mul(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_mul)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_mul;

    """]

    [[completions]]
    completion_label = "bounded_int_sub(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_sub)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_sub;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_max(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_max)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_max(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_max;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_min(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_min)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_min(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_min;

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
    completion_label = "cairo_keccak(...)"
    completion_label_path = "(use core::keccak::cairo_keccak)"
    completion_label_type_info = "fn(ref input: Array<u64>, last_input_word: u64, last_input_num_bytes: u32) -> u256"
    insert_text = "cairo_keccak(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::keccak::cairo_keccak;

    """]

    [[completions]]
    completion_label = "call_contract_syscall(...)"
    completion_label_path = "(use starknet::syscalls::call_contract_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "call_contract_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::call_contract_syscall;

    """]

    [[completions]]
    completion_label = "cheatcode(...)"
    completion_label_path = "(use starknet::testing::cheatcode)"
    completion_label_type_info = "fn(input: Span<felt252>) -> Span<felt252> nopanic"
    insert_text = "cheatcode(${1:input})"
    text_edits = ["""
    use starknet::testing::cheatcode;

    """]

    [[completions]]
    completion_label = "check_ecdsa_signature(...)"
    completion_label_path = "(use core::ecdsa::check_ecdsa_signature)"
    completion_label_type_info = "fn(message_hash: felt252, public_key: felt252, signature_r: felt252, signature_s: felt252) -> bool"
    insert_text = "check_ecdsa_signature(${1:message_hash}, ${2:public_key}, ${3:signature_r}, ${4:signature_s})"
    text_edits = ["""
    use core::ecdsa::check_ecdsa_signature;

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
    completion_label = "circuit_add(...)"
    completion_label_path = "(use core::circuit::circuit_add)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<AddModGate<Lhs, Rhs>>"
    insert_text = "circuit_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_add;

    """]

    [[completions]]
    completion_label = "circuit_inverse(...)"
    completion_label_path = "(use core::circuit::circuit_inverse)"
    completion_label_type_info = "fn(input: CircuitElement<Input>) -> CircuitElement<InverseGate<Input>>"
    insert_text = "circuit_inverse(${1:input})"
    text_edits = ["""
    use core::circuit::circuit_inverse;

    """]

    [[completions]]
    completion_label = "circuit_mul(...)"
    completion_label_path = "(use core::circuit::circuit_mul)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<MulModGate<Lhs, Rhs>>"
    insert_text = "circuit_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_mul;

    """]

    [[completions]]
    completion_label = "circuit_sub(...)"
    completion_label_path = "(use core::circuit::circuit_sub)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<SubModGate<Lhs, Rhs>>"
    insert_text = "circuit_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_sub;

    """]

    [[completions]]
    completion_label = "class_hash"
    completion_label_path = "(use starknet::class_hash)"
    text_edits = ["""
    use starknet::class_hash;

    """]

    [[completions]]
    completion_label = "class_hash_const(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_const)"
    completion_label_type_info = "fn() -> ClassHash nopanic"
    insert_text = "class_hash_const()"
    text_edits = ["""
    use starknet::class_hash::class_hash_const;

    """]

    [[completions]]
    completion_label = "class_hash_to_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_to_felt252)"
    completion_label_type_info = "fn(address: ClassHash) -> felt252 nopanic"
    insert_text = "class_hash_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_to_felt252;

    """]

    [[completions]]
    completion_label = "class_hash_try_from_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ClassHash> nopanic"
    insert_text = "class_hash_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_try_from_felt252;

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
    completion_label = "compute_keccak_byte_array(...)"
    completion_label_path = "(use core::keccak::compute_keccak_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> u256"
    insert_text = "compute_keccak_byte_array(${1:arr})"
    text_edits = ["""
    use core::keccak::compute_keccak_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_byte_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> [u32; 8]"
    insert_text = "compute_sha256_byte_array(${1:arr})"
    text_edits = ["""
    use core::sha256::compute_sha256_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: u32) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array_safe(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array_safe)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: BoundedInt<0, 3>) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array_safe(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array_safe;

    """]

    [[completions]]
    completion_label = "contract_address"
    completion_label_path = "(use starknet::contract_address)"
    text_edits = ["""
    use starknet::contract_address;

    """]

    [[completions]]
    completion_label = "contract_address_const(...)"
    completion_label_path = "(use starknet::contract_address_const)"
    completion_label_type_info = "fn() -> ContractAddress nopanic"
    insert_text = "contract_address_const()"
    text_edits = ["""
    use starknet::contract_address_const;

    """]

    [[completions]]
    completion_label = "contract_address_to_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_to_felt252)"
    completion_label_type_info = "fn(address: ContractAddress) -> felt252 nopanic"
    insert_text = "contract_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_to_felt252;

    """]

    [[completions]]
    completion_label = "contract_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ContractAddress> nopanic"
    insert_text = "contract_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "debug"
    completion_label_path = "(use core::debug)"
    text_edits = ["""
    use core::debug;

    """]

    [[completions]]
    completion_label = "deploy_syscall(...)"
    completion_label_path = "(use starknet::syscalls::deploy_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, contract_address_salt: felt252, calldata: Span<felt252>, deploy_from_zero: bool) -> Result<(ContractAddress, Span<felt252>), Array<felt252>> nopanic"
    insert_text = "deploy_syscall(${1:class_hash}, ${2:contract_address_salt}, ${3:calldata}, ${4:deploy_from_zero})"
    text_edits = ["""
    use starknet::syscalls::deploy_syscall;

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
    completion_label = "downcast(...)"
    completion_label_path = "(use core::internal::bounded_int::downcast)"
    completion_label_type_info = "fn(x: FromType) -> Option<ToType> nopanic"
    insert_text = "downcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::downcast;

    """]

    [[completions]]
    completion_label = "ec"
    completion_label_path = "(use core::ec)"
    text_edits = ["""
    use core::ec;

    """]

    [[completions]]
    completion_label = "ec_point_unwrap(...)"
    completion_label_path = "(use core::ec::ec_point_unwrap)"
    completion_label_type_info = "fn(p: NonZero<EcPoint>) -> (felt252, felt252) nopanic"
    insert_text = "ec_point_unwrap(${1:p})"
    text_edits = ["""
    use core::ec::ec_point_unwrap;

    """]

    [[completions]]
    completion_label = "ecdsa"
    completion_label_path = "(use core::ecdsa)"
    text_edits = ["""
    use core::ecdsa;

    """]

    [[completions]]
    completion_label = "egcd(...)"
    completion_label_path = "(use core::math::egcd)"
    completion_label_type_info = "fn(a: NonZero<T>, b: NonZero<T>) -> (T, T, T, bool)"
    insert_text = "egcd(${1:a}, ${2:b})"
    text_edits = ["""
    use core::math::egcd;

    """]

    [[completions]]
    completion_label = "emit_event_syscall(...)"
    completion_label_path = "(use starknet::syscalls::emit_event_syscall)"
    completion_label_type_info = "fn(keys: Span<felt252>, data: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "emit_event_syscall(${1:keys}, ${2:data})"
    text_edits = ["""
    use starknet::syscalls::emit_event_syscall;

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
    completion_label = "felt252_div(...)"
    completion_label_path = "(use core::felt252_div)"
    completion_label_type_info = "fn(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic"
    insert_text = "felt252_div(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::felt252_div;

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
    completion_label = "gas_reserve_create(...)"
    completion_label_path = "(use core::gas::gas_reserve_create)"
    completion_label_type_info = "fn(amount: u128) -> Option<GasReserve> nopanic"
    insert_text = "gas_reserve_create(${1:amount})"
    text_edits = ["""
    use core::gas::gas_reserve_create;

    """]

    [[completions]]
    completion_label = "gas_reserve_utilize(...)"
    completion_label_path = "(use core::gas::gas_reserve_utilize)"
    completion_label_type_info = "fn(reserve: GasReserve) -> () nopanic"
    insert_text = "gas_reserve_utilize(${1:reserve})"
    text_edits = ["""
    use core::gas::gas_reserve_utilize;

    """]

    [[completions]]
    completion_label = "get"
    completion_label_path = "(use core::ops::get)"
    text_edits = ["""
    use core::ops::get;

    """]

    [[completions]]
    completion_label = "get_available_gas(...)"
    completion_label_path = "(use core::testing::get_available_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_available_gas()"
    text_edits = ["""
    use core::testing::get_available_gas;

    """]

    [[completions]]
    completion_label = "get_block_hash_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_block_hash_syscall)"
    completion_label_type_info = "fn(block_number: u64) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "get_block_hash_syscall(${1:block_number})"
    text_edits = ["""
    use starknet::syscalls::get_block_hash_syscall;

    """]

    [[completions]]
    completion_label = "get_block_info(...)"
    completion_label_path = "(use starknet::get_block_info)"
    completion_label_type_info = "fn() -> Box<BlockInfo>"
    insert_text = "get_block_info()"
    text_edits = ["""
    use starknet::get_block_info;

    """]

    [[completions]]
    completion_label = "get_block_number(...)"
    completion_label_path = "(use starknet::get_block_number)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_number()"
    text_edits = ["""
    use starknet::get_block_number;

    """]

    [[completions]]
    completion_label = "get_block_timestamp(...)"
    completion_label_path = "(use starknet::get_block_timestamp)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_timestamp()"
    text_edits = ["""
    use starknet::get_block_timestamp;

    """]

    [[completions]]
    completion_label = "get_builtin_costs(...)"
    completion_label_path = "(use core::gas::get_builtin_costs)"
    completion_label_type_info = "fn() -> BuiltinCosts nopanic"
    insert_text = "get_builtin_costs()"
    text_edits = ["""
    use core::gas::get_builtin_costs;

    """]

    [[completions]]
    completion_label = "get_caller_address(...)"
    completion_label_path = "(use starknet::get_caller_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_caller_address()"
    text_edits = ["""
    use starknet::get_caller_address;

    """]

    [[completions]]
    completion_label = "get_class_hash_at_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_class_hash_at_syscall)"
    completion_label_type_info = "fn(contract_address: ContractAddress) -> Result<ClassHash, Array<felt252>> nopanic"
    insert_text = "get_class_hash_at_syscall(${1:contract_address})"
    text_edits = ["""
    use starknet::syscalls::get_class_hash_at_syscall;

    """]

    [[completions]]
    completion_label = "get_contract_address(...)"
    completion_label_path = "(use starknet::get_contract_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_contract_address()"
    text_edits = ["""
    use starknet::get_contract_address;

    """]

    [[completions]]
    completion_label = "get_execution_info(...)"
    completion_label_path = "(use starknet::get_execution_info)"
    completion_label_type_info = "fn() -> Box<starknet::ExecutionInfo>"
    insert_text = "get_execution_info()"
    text_edits = ["""
    use starknet::get_execution_info;

    """]

    [[completions]]
    completion_label = "get_execution_info_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v2_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v2_syscall)"
    completion_label_type_info = "fn() -> Result<Box<starknet::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v2_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v2_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v3_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v3_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::v3::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v3_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v3_syscall;

    """]

    [[completions]]
    completion_label = "get_tx_info(...)"
    completion_label_path = "(use starknet::get_tx_info)"
    completion_label_type_info = "fn() -> Box<starknet::TxInfo>"
    insert_text = "get_tx_info()"
    text_edits = ["""
    use starknet::get_tx_info;

    """]

    [[completions]]
    completion_label = "get_unspent_gas(...)"
    completion_label_path = "(use core::testing::get_unspent_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_unspent_gas()"
    text_edits = ["""
    use core::testing::get_unspent_gas;

    """]

    [[completions]]
    completion_label = "hades_permutation(...)"
    completion_label_path = "(use core::poseidon::hades_permutation)"
    completion_label_type_info = "fn(s0: felt252, s1: felt252, s2: felt252) -> (felt252, felt252, felt252) nopanic"
    insert_text = "hades_permutation(${1:s0}, ${2:s1}, ${3:s2})"
    text_edits = ["""
    use core::poseidon::hades_permutation;

    """]

    [[completions]]
    completion_label = "hash"
    completion_label_path = "(use core::hash)"
    text_edits = ["""
    use core::hash;

    """]

    [[completions]]
    completion_label = "i128_diff(...)"
    completion_label_path = "(use core::integer::i128_diff)"
    completion_label_type_info = "fn(lhs: i128, rhs: i128) -> Result<u128, u128> nopanic"
    insert_text = "i128_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i128_diff;

    """]

    [[completions]]
    completion_label = "i16_diff(...)"
    completion_label_path = "(use core::integer::i16_diff)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> Result<u16, u16> nopanic"
    insert_text = "i16_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_diff;

    """]

    [[completions]]
    completion_label = "i16_wide_mul(...)"
    completion_label_path = "(use core::integer::i16_wide_mul)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> i32 nopanic"
    insert_text = "i16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_wide_mul;

    """]

    [[completions]]
    completion_label = "i32_diff(...)"
    completion_label_path = "(use core::integer::i32_diff)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> Result<u32, u32> nopanic"
    insert_text = "i32_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_diff;

    """]

    [[completions]]
    completion_label = "i32_wide_mul(...)"
    completion_label_path = "(use core::integer::i32_wide_mul)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> i64 nopanic"
    insert_text = "i32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_wide_mul;

    """]

    [[completions]]
    completion_label = "i64_diff(...)"
    completion_label_path = "(use core::integer::i64_diff)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> Result<u64, u64> nopanic"
    insert_text = "i64_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_diff;

    """]

    [[completions]]
    completion_label = "i64_wide_mul(...)"
    completion_label_path = "(use core::integer::i64_wide_mul)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> i128 nopanic"
    insert_text = "i64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_wide_mul;

    """]

    [[completions]]
    completion_label = "i8_diff(...)"
    completion_label_path = "(use core::integer::i8_diff)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> Result<u8, u8> nopanic"
    insert_text = "i8_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_diff;

    """]

    [[completions]]
    completion_label = "i8_wide_mul(...)"
    completion_label_path = "(use core::integer::i8_wide_mul)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> i16 nopanic"
    insert_text = "i8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_wide_mul;

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
    completion_label = "inv_mod(...)"
    completion_label_path = "(use core::math::inv_mod)"
    completion_label_type_info = "fn(a: NonZero<T>, n: NonZero<T>) -> Option<T>"
    insert_text = "inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::inv_mod;

    """]

    [[completions]]
    completion_label = "is_eth_signature_valid(...)"
    completion_label_path = "(use starknet::eth_signature::is_eth_signature_valid)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> Result<(), felt252>"
    insert_text = "is_eth_signature_valid(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::is_eth_signature_valid;

    """]

    [[completions]]
    completion_label = "is_signature_entry_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_entry_valid)"
    completion_label_type_info = "fn(value: u256) -> bool"
    insert_text = "is_signature_entry_valid(${1:value})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_entry_valid;

    """]

    [[completions]]
    completion_label = "is_signature_s_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_s_valid)"
    completion_label_type_info = "fn(s: u256) -> bool"
    insert_text = "is_signature_s_valid(${1:s})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_s_valid;

    """]

    [[completions]]
    completion_label = "is_valid_signature(...)"
    completion_label_path = "(use starknet::secp256_trait::is_valid_signature)"
    completion_label_type_info = "fn(msg_hash: u256, r: u256, s: u256, public_key: Secp256Point) -> bool"
    insert_text = "is_valid_signature(${1:msg_hash}, ${2:r}, ${3:s}, ${4:public_key})"
    text_edits = ["""
    use starknet::secp256_trait::is_valid_signature;

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
    completion_label = "keccak_syscall(...)"
    completion_label_path = "(use starknet::syscalls::keccak_syscall)"
    completion_label_type_info = "fn(input: Span<u64>) -> Result<u256, Array<felt252>> nopanic"
    insert_text = "keccak_syscall(${1:input})"
    text_edits = ["""
    use starknet::syscalls::keccak_syscall;

    """]

    [[completions]]
    completion_label = "keccak_u256s_be_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_be_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_be_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_be_inputs;

    """]

    [[completions]]
    completion_label = "keccak_u256s_le_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_le_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_le_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_le_inputs;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "m31_add(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_add)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_add(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_add;

    """]

    [[completions]]
    completion_label = "m31_div(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_div)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: NonZero<crate::internal::bounded_int::BoundedInt<0, 2147483646>>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_div(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_div;

    """]

    [[completions]]
    completion_label = "m31_mul(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_mul)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_mul;

    """]

    [[completions]]
    completion_label = "m31_ops"
    completion_label_path = "(use core::qm31::m31_ops)"
    text_edits = ["""
    use core::qm31::m31_ops;

    """]

    [[completions]]
    completion_label = "m31_sub(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_sub)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_sub;

    """]

    [[completions]]
    completion_label = "match_nullable(...)"
    completion_label_path = "(use core::nullable::match_nullable)"
    completion_label_type_info = "fn(value: Nullable<T>) -> FromNullableResult<T> nopanic"
    insert_text = "match_nullable(${1:value})"
    text_edits = ["""
    use core::nullable::match_nullable;

    """]

    [[completions]]
    completion_label = "math"
    completion_label_path = "(use core::math)"
    text_edits = ["""
    use core::math;

    """]

    [[completions]]
    completion_label = "max(...)"
    completion_label_path = "(use core::cmp::max)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "max(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::max;

    """]

    [[completions]]
    completion_label = "meta_tx_v0_syscall(...)"
    completion_label_path = "(use starknet::syscalls::meta_tx_v0_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>, signature: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "meta_tx_v0_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata}, ${4:signature})"
    text_edits = ["""
    use starknet::syscalls::meta_tx_v0_syscall;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
    text_edits = ["""
    use core::metaprogramming;

    """]

    [[completions]]
    completion_label = "min(...)"
    completion_label_path = "(use core::cmp::min)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "min(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::min;

    """]

    [[completions]]
    completion_label = "minmax(...)"
    completion_label_path = "(use core::cmp::minmax)"
    completion_label_type_info = "fn(a: T, b: T) -> (T, T)"
    insert_text = "minmax(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::minmax;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "null(...)"
    completion_label_path = "(use core::nullable::null)"
    completion_label_type_info = "fn() -> Nullable<T> nopanic"
    insert_text = "null()"
    text_edits = ["""
    use core::nullable::null;

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
    completion_label = "panic_with_byte_array(...)"
    completion_label_path = "(use core::panics::panic_with_byte_array)"
    completion_label_type_info = "fn(err: @ByteArray) -> crate::never"
    insert_text = "panic_with_byte_array(${1:err})"
    text_edits = ["""
    use core::panics::panic_with_byte_array;

    """]

    [[completions]]
    completion_label = "panic_with_const_felt252(...)"
    completion_label_path = "(use core::panic_with_const_felt252)"
    completion_label_type_info = "fn() -> never"
    insert_text = "panic_with_const_felt252()"
    text_edits = ["""
    use core::panic_with_const_felt252;

    """]

    [[completions]]
    completion_label = "panic_with_felt252(...)"
    completion_label_path = "(use core::panic_with_felt252)"
    completion_label_type_info = "fn(err_code: felt252) -> never"
    insert_text = "panic_with_felt252(${1:err_code})"
    text_edits = ["""
    use core::panic_with_felt252;

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
    completion_label = "pedersen(...)"
    completion_label_path = "(use core::pedersen::pedersen)"
    completion_label_type_info = "fn(a: felt252, b: felt252) -> felt252 nopanic"
    insert_text = "pedersen(${1:a}, ${2:b})"
    text_edits = ["""
    use core::pedersen::pedersen;

    """]

    [[completions]]
    completion_label = "pop_l2_to_l1_message(...)"
    completion_label_path = "(use starknet::testing::pop_l2_to_l1_message)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(felt252, Span<felt252>)>"
    insert_text = "pop_l2_to_l1_message(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_l2_to_l1_message;

    """]

    [[completions]]
    completion_label = "pop_log(...)"
    completion_label_path = "(use starknet::testing::pop_log)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<T>"
    insert_text = "pop_log(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log;

    """]

    [[completions]]
    completion_label = "pop_log_raw(...)"
    completion_label_path = "(use starknet::testing::pop_log_raw)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(Span<felt252>, Span<felt252>)>"
    insert_text = "pop_log_raw(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log_raw;

    """]

    [[completions]]
    completion_label = "poseidon"
    completion_label_path = "(use core::poseidon)"
    text_edits = ["""
    use core::poseidon;

    """]

    [[completions]]
    completion_label = "poseidon_hash_span(...)"
    completion_label_path = "(use core::poseidon::poseidon_hash_span)"
    completion_label_type_info = "fn(span: Span<felt252>) -> felt252"
    insert_text = "poseidon_hash_span(${1:span})"
    text_edits = ["""
    use core::poseidon::poseidon_hash_span;

    """]

    [[completions]]
    completion_label = "pow"
    completion_label_path = "(use core::num::traits::ops::pow)"
    text_edits = ["""
    use core::num::traits::ops::pow;

    """]

    [[completions]]
    completion_label = "print_byte_array_as_string(...)"
    completion_label_path = "(use core::debug::print_byte_array_as_string)"
    completion_label_type_info = "fn(self: @ByteArray) -> ()"
    insert_text = "print_byte_array_as_string()"
    text_edits = ["""
    use core::debug::print_byte_array_as_string;

    """]

    [[completions]]
    completion_label = "public_key_point_to_eth_address(...)"
    completion_label_path = "(use starknet::eth_signature::public_key_point_to_eth_address)"
    completion_label_type_info = "fn(public_key_point: Secp256Point) -> EthAddress"
    insert_text = "public_key_point_to_eth_address(${1:public_key_point})"
    text_edits = ["""
    use starknet::eth_signature::public_key_point_to_eth_address;

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
    completion_label = "qm31_const(...)"
    completion_label_path = "(use core::qm31::qm31_const)"
    completion_label_type_info = "fn() -> qm31 nopanic"
    insert_text = "qm31_const()"
    text_edits = ["""
    use core::qm31::qm31_const;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use core::ecdsa::recover_public_key)"
    completion_label_type_info = "fn(message_hash: felt252, signature_r: felt252, signature_s: felt252, y_parity: bool) -> Option<felt252>"
    insert_text = "recover_public_key(${1:message_hash}, ${2:signature_r}, ${3:signature_s}, ${4:y_parity})"
    text_edits = ["""
    use core::ecdsa::recover_public_key;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use starknet::secp256_trait::recover_public_key)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature) -> Option<Secp256Point>"
    insert_text = "recover_public_key(${1:msg_hash}, ${2:signature})"
    text_edits = ["""
    use starknet::secp256_trait::recover_public_key;

    """]

    [[completions]]
    completion_label = "redeposit_gas(...)"
    completion_label_path = "(use core::gas::redeposit_gas)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "redeposit_gas()"
    text_edits = ["""
    use core::gas::redeposit_gas;

    """]

    [[completions]]
    completion_label = "replace_class_syscall(...)"
    completion_label_path = "(use starknet::syscalls::replace_class_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash) -> Result<(), Array<felt252>> nopanic"
    insert_text = "replace_class_syscall(${1:class_hash})"
    text_edits = ["""
    use starknet::syscalls::replace_class_syscall;

    """]

    [[completions]]
    completion_label = "require_implicit(...)"
    completion_label_path = "(use core::internal::require_implicit)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "require_implicit()"
    text_edits = ["""
    use core::internal::require_implicit;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]

    [[completions]]
    completion_label = "revoke_ap_tracking(...)"
    completion_label_path = "(use core::internal::revoke_ap_tracking)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "revoke_ap_tracking()"
    text_edits = ["""
    use core::internal::revoke_ap_tracking;

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
    completion_label = "send_message_to_l1_syscall(...)"
    completion_label_path = "(use starknet::syscalls::send_message_to_l1_syscall)"
    completion_label_type_info = "fn(to_address: felt252, payload: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "send_message_to_l1_syscall(${1:to_address}, ${2:payload})"
    text_edits = ["""
    use starknet::syscalls::send_message_to_l1_syscall;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "set_account_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_account_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_account_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_account_contract_address;

    """]

    [[completions]]
    completion_label = "set_block_hash(...)"
    completion_label_path = "(use starknet::testing::set_block_hash)"
    completion_label_type_info = "fn(block_number: u64, value: felt252) -> ()"
    insert_text = "set_block_hash(${1:block_number}, ${2:value})"
    text_edits = ["""
    use starknet::testing::set_block_hash;

    """]

    [[completions]]
    completion_label = "set_block_number(...)"
    completion_label_path = "(use starknet::testing::set_block_number)"
    completion_label_type_info = "fn(block_number: u64) -> ()"
    insert_text = "set_block_number(${1:block_number})"
    text_edits = ["""
    use starknet::testing::set_block_number;

    """]

    [[completions]]
    completion_label = "set_block_timestamp(...)"
    completion_label_path = "(use starknet::testing::set_block_timestamp)"
    completion_label_type_info = "fn(block_timestamp: u64) -> ()"
    insert_text = "set_block_timestamp(${1:block_timestamp})"
    text_edits = ["""
    use starknet::testing::set_block_timestamp;

    """]

    [[completions]]
    completion_label = "set_caller_address(...)"
    completion_label_path = "(use starknet::testing::set_caller_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_caller_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_caller_address;

    """]

    [[completions]]
    completion_label = "set_chain_id(...)"
    completion_label_path = "(use starknet::testing::set_chain_id)"
    completion_label_type_info = "fn(chain_id: felt252) -> ()"
    insert_text = "set_chain_id(${1:chain_id})"
    text_edits = ["""
    use starknet::testing::set_chain_id;

    """]

    [[completions]]
    completion_label = "set_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_contract_address;

    """]

    [[completions]]
    completion_label = "set_max_fee(...)"
    completion_label_path = "(use starknet::testing::set_max_fee)"
    completion_label_type_info = "fn(fee: u128) -> ()"
    insert_text = "set_max_fee(${1:fee})"
    text_edits = ["""
    use starknet::testing::set_max_fee;

    """]

    [[completions]]
    completion_label = "set_nonce(...)"
    completion_label_path = "(use starknet::testing::set_nonce)"
    completion_label_type_info = "fn(nonce: felt252) -> ()"
    insert_text = "set_nonce(${1:nonce})"
    text_edits = ["""
    use starknet::testing::set_nonce;

    """]

    [[completions]]
    completion_label = "set_sequencer_address(...)"
    completion_label_path = "(use starknet::testing::set_sequencer_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_sequencer_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_sequencer_address;

    """]

    [[completions]]
    completion_label = "set_signature(...)"
    completion_label_path = "(use starknet::testing::set_signature)"
    completion_label_type_info = "fn(signature: Span<felt252>) -> ()"
    insert_text = "set_signature(${1:signature})"
    text_edits = ["""
    use starknet::testing::set_signature;

    """]

    [[completions]]
    completion_label = "set_transaction_hash(...)"
    completion_label_path = "(use starknet::testing::set_transaction_hash)"
    completion_label_type_info = "fn(hash: felt252) -> ()"
    insert_text = "set_transaction_hash(${1:hash})"
    text_edits = ["""
    use starknet::testing::set_transaction_hash;

    """]

    [[completions]]
    completion_label = "set_version(...)"
    completion_label_path = "(use starknet::testing::set_version)"
    completion_label_type_info = "fn(version: felt252) -> ()"
    insert_text = "set_version(${1:version})"
    text_edits = ["""
    use starknet::testing::set_version;

    """]

    [[completions]]
    completion_label = "sha256"
    completion_label_path = "(use core::sha256)"
    text_edits = ["""
    use core::sha256;

    """]

    [[completions]]
    completion_label = "sha256_process_block_syscall(...)"
    completion_label_path = "(use starknet::syscalls::sha256_process_block_syscall)"
    completion_label_type_info = "fn(state: crate::sha256::Sha256StateHandle, input: Box<[u32; 16]>) -> Result<crate::sha256::Sha256StateHandle, Array<felt252>> nopanic"
    insert_text = "sha256_process_block_syscall(${1:state}, ${2:input})"
    text_edits = ["""
    use starknet::syscalls::sha256_process_block_syscall;

    """]

    [[completions]]
    completion_label = "signature_from_vrs(...)"
    completion_label_path = "(use starknet::secp256_trait::signature_from_vrs)"
    completion_label_type_info = "fn(v: u32, r: u256, s: u256) -> Signature"
    insert_text = "signature_from_vrs(${1:v}, ${2:r}, ${3:s})"
    text_edits = ["""
    use starknet::secp256_trait::signature_from_vrs;

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
    completion_label = "storage_address_from_base(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base)"
    completion_label_type_info = "fn(base: StorageBaseAddress) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base(${1:base})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base;

    """]

    [[completions]]
    completion_label = "storage_address_from_base_and_offset(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base_and_offset)"
    completion_label_type_info = "fn(base: StorageBaseAddress, offset: u8) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base_and_offset(${1:base}, ${2:offset})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base_and_offset;

    """]

    [[completions]]
    completion_label = "storage_address_to_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_to_felt252)"
    completion_label_type_info = "fn(address: StorageAddress) -> felt252 nopanic"
    insert_text = "storage_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_to_felt252;

    """]

    [[completions]]
    completion_label = "storage_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<StorageAddress> nopanic"
    insert_text = "storage_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_base_address_const(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_const)"
    completion_label_type_info = "fn() -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_const()"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_const;

    """]

    [[completions]]
    completion_label = "storage_base_address_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_from_felt252)"
    completion_label_type_info = "fn(addr: felt252) -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_from_felt252(${1:addr})"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_read_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_read_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "storage_read_syscall(${1:address_domain}, ${2:address})"
    text_edits = ["""
    use starknet::syscalls::storage_read_syscall;

    """]

    [[completions]]
    completion_label = "storage_write_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_write_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress, value: felt252) -> Result<(), Array<felt252>> nopanic"
    insert_text = "storage_write_syscall(${1:address_domain}, ${2:address}, ${3:value})"
    text_edits = ["""
    use starknet::syscalls::storage_write_syscall;

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
    completion_label = "u128_byte_reverse(...)"
    completion_label_path = "(use core::integer::u128_byte_reverse)"
    completion_label_type_info = "fn(input: u128) -> u128 nopanic"
    insert_text = "u128_byte_reverse(${1:input})"
    text_edits = ["""
    use core::integer::u128_byte_reverse;

    """]

    [[completions]]
    completion_label = "u128_overflowing_add(...)"
    completion_label_path = "(use core::integer::u128_overflowing_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_add;

    """]

    [[completions]]
    completion_label = "u128_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u128_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> (u128, bool) nopanic"
    insert_text = "u128_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u128_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u128_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u128_safe_divmod(...)"
    completion_label_path = "(use core::integer::u128_safe_divmod)"
    completion_label_type_info = "fn(lhs: u128, rhs: NonZero<u128>) -> (u128, u128) nopanic"
    insert_text = "u128_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_safe_divmod;

    """]

    [[completions]]
    completion_label = "u128_sqrt(...)"
    completion_label_path = "(use core::integer::u128_sqrt)"
    completion_label_type_info = "fn(value: u128) -> u64 nopanic"
    insert_text = "u128_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u128_sqrt;

    """]

    [[completions]]
    completion_label = "u128_wide_mul(...)"
    completion_label_path = "(use core::integer::u128_wide_mul)"
    completion_label_type_info = "fn(a: u128, b: u128) -> (u128, u128) nopanic"
    insert_text = "u128_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wide_mul;

    """]

    [[completions]]
    completion_label = "u128_wrapping_add(...)"
    completion_label_path = "(use core::integer::u128_wrapping_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_wrapping_add;

    """]

    [[completions]]
    completion_label = "u128_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u128_wrapping_sub)"
    completion_label_type_info = "fn(a: u128, b: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u16_overflowing_add(...)"
    completion_label_path = "(use core::integer::u16_overflowing_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_add;

    """]

    [[completions]]
    completion_label = "u16_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u16_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u16_safe_divmod(...)"
    completion_label_path = "(use core::integer::u16_safe_divmod)"
    completion_label_type_info = "fn(lhs: u16, rhs: NonZero<u16>) -> (u16, u16) nopanic"
    insert_text = "u16_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_safe_divmod;

    """]

    [[completions]]
    completion_label = "u16_sqrt(...)"
    completion_label_path = "(use core::integer::u16_sqrt)"
    completion_label_type_info = "fn(value: u16) -> u8 nopanic"
    insert_text = "u16_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u16_sqrt;

    """]

    [[completions]]
    completion_label = "u16_wide_mul(...)"
    completion_label_path = "(use core::integer::u16_wide_mul)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u32 nopanic"
    insert_text = "u16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wide_mul;

    """]

    [[completions]]
    completion_label = "u16_wrapping_add(...)"
    completion_label_path = "(use core::integer::u16_wrapping_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_add;

    """]

    [[completions]]
    completion_label = "u16_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u16_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u2"
    completion_label_path = "(use core::sha256::u2)"
    text_edits = ["""
    use core::sha256::u2;

    """]

    [[completions]]
    completion_label = "u256_div_mod_n(...)"
    completion_label_path = "(use core::math::u256_div_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> Option<u256>"
    insert_text = "u256_div_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_div_mod_n;

    """]

    [[completions]]
    completion_label = "u256_inv_mod(...)"
    completion_label_path = "(use core::math::u256_inv_mod)"
    completion_label_type_info = "fn(a: u256, n: NonZero<u256>) -> Option<NonZero<u256>>"
    insert_text = "u256_inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::u256_inv_mod;

    """]

    [[completions]]
    completion_label = "u256_mul_mod_n(...)"
    completion_label_path = "(use core::math::u256_mul_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> u256"
    insert_text = "u256_mul_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_mul_mod_n;

    """]

    [[completions]]
    completion_label = "u256_overflow_mul(...)"
    completion_label_path = "(use core::integer::u256_overflow_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflow_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_mul;

    """]

    [[completions]]
    completion_label = "u256_overflow_sub(...)"
    completion_label_path = "(use core::integer::u256_overflow_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflow_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_sub;

    """]

    [[completions]]
    completion_label = "u256_overflowing_add(...)"
    completion_label_path = "(use core::integer::u256_overflowing_add)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_add;

    """]

    [[completions]]
    completion_label = "u256_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u256_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u256_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u256_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u256_sqrt(...)"
    completion_label_path = "(use core::integer::u256_sqrt)"
    completion_label_type_info = "fn(a: u256) -> u128 nopanic"
    insert_text = "u256_sqrt(${1:a})"
    text_edits = ["""
    use core::integer::u256_sqrt;

    """]

    [[completions]]
    completion_label = "u256_wide_mul(...)"
    completion_label_path = "(use core::integer::u256_wide_mul)"
    completion_label_type_info = "fn(a: u256, b: u256) -> u512 nopanic"
    insert_text = "u256_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u256_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_overflowing_add(...)"
    completion_label_path = "(use core::integer::u32_overflowing_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_add;

    """]

    [[completions]]
    completion_label = "u32_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u32_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u32_safe_divmod(...)"
    completion_label_path = "(use core::integer::u32_safe_divmod)"
    completion_label_type_info = "fn(lhs: u32, rhs: NonZero<u32>) -> (u32, u32) nopanic"
    insert_text = "u32_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_safe_divmod;

    """]

    [[completions]]
    completion_label = "u32_sqrt(...)"
    completion_label_path = "(use core::integer::u32_sqrt)"
    completion_label_type_info = "fn(value: u32) -> u16 nopanic"
    insert_text = "u32_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u32_sqrt;

    """]

    [[completions]]
    completion_label = "u32_wide_mul(...)"
    completion_label_path = "(use core::integer::u32_wide_mul)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u64 nopanic"
    insert_text = "u32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_wrapping_add(...)"
    completion_label_path = "(use core::integer::u32_wrapping_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_add;

    """]

    [[completions]]
    completion_label = "u32_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u32_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_sub;

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
    completion_label = "u512_safe_div_rem_by_u256(...)"
    completion_label_path = "(use core::integer::u512_safe_div_rem_by_u256)"
    completion_label_type_info = "fn(lhs: u512, rhs: NonZero<u256>) -> (u512, u256) nopanic"
    insert_text = "u512_safe_div_rem_by_u256(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u512_safe_div_rem_by_u256;

    """]

    [[completions]]
    completion_label = "u64_overflowing_add(...)"
    completion_label_path = "(use core::integer::u64_overflowing_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_add;

    """]

    [[completions]]
    completion_label = "u64_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u64_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u64_safe_divmod(...)"
    completion_label_path = "(use core::integer::u64_safe_divmod)"
    completion_label_type_info = "fn(lhs: u64, rhs: NonZero<u64>) -> (u64, u64) nopanic"
    insert_text = "u64_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_safe_divmod;

    """]

    [[completions]]
    completion_label = "u64_sqrt(...)"
    completion_label_path = "(use core::integer::u64_sqrt)"
    completion_label_type_info = "fn(value: u64) -> u32 nopanic"
    insert_text = "u64_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u64_sqrt;

    """]

    [[completions]]
    completion_label = "u64_wide_mul(...)"
    completion_label_path = "(use core::integer::u64_wide_mul)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u128 nopanic"
    insert_text = "u64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wide_mul;

    """]

    [[completions]]
    completion_label = "u64_wrapping_add(...)"
    completion_label_path = "(use core::integer::u64_wrapping_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_add;

    """]

    [[completions]]
    completion_label = "u64_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u64_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u8_overflowing_add(...)"
    completion_label_path = "(use core::integer::u8_overflowing_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_add;

    """]

    [[completions]]
    completion_label = "u8_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u8_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u8_safe_divmod(...)"
    completion_label_path = "(use core::integer::u8_safe_divmod)"
    completion_label_type_info = "fn(lhs: u8, rhs: NonZero<u8>) -> (u8, u8) nopanic"
    insert_text = "u8_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_safe_divmod;

    """]

    [[completions]]
    completion_label = "u8_sqrt(...)"
    completion_label_path = "(use core::integer::u8_sqrt)"
    completion_label_type_info = "fn(value: u8) -> u8 nopanic"
    insert_text = "u8_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u8_sqrt;

    """]

    [[completions]]
    completion_label = "u8_wide_mul(...)"
    completion_label_path = "(use core::integer::u8_wide_mul)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u16 nopanic"
    insert_text = "u8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wide_mul;

    """]

    [[completions]]
    completion_label = "u8_wrapping_add(...)"
    completion_label_path = "(use core::integer::u8_wrapping_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_add;

    """]

    [[completions]]
    completion_label = "u8_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u8_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u96"
    completion_label_path = "(use core::circuit::u96)"
    text_edits = ["""
    use core::circuit::u96;

    """]

    [[completions]]
    completion_label = "upcast(...)"
    completion_label_path = "(use core::internal::bounded_int::upcast)"
    completion_label_type_info = "fn(x: FromType) -> ToType nopanic"
    insert_text = "upcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::upcast;

    """]

    [[completions]]
    completion_label = "verify_eth_signature(...)"
    completion_label_path = "(use starknet::eth_signature::verify_eth_signature)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> ()"
    insert_text = "verify_eth_signature(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::verify_eth_signature;

    """]

    [[completions]]
    completion_label = "withdraw_gas(...)"
    completion_label_path = "(use core::gas::withdraw_gas)"
    completion_label_type_info = "fn() -> Option<()> nopanic"
    insert_text = "withdraw_gas()"
    text_edits = ["""
    use core::gas::withdraw_gas;

    """]

    [[completions]]
    completion_label = "withdraw_gas_all(...)"
    completion_label_path = "(use core::gas::withdraw_gas_all)"
    completion_label_type_info = "fn(costs: BuiltinCosts) -> Option<()> nopanic"
    insert_text = "withdraw_gas_all(${1:costs})"
    text_edits = ["""
    use core::gas::withdraw_gas_all;

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

    [[completions]]
    completion_label = "zip(...)"
    completion_label_path = "(use core::iter::zip)"
    completion_label_type_info = "fn(a: A, b: B) -> Zip<AIntoIter::IntoIter, BIntoIter::IntoIter>"
    insert_text = "zip(${1:a}, ${2:b})"
    text_edits = ["""
    use core::iter::zip;

    """]
    "#);
}

#[test]
fn no_text_before_statement() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        <caret>let _x = 1;
    }
    ",@r#"
    caret = """
        <caret>let _x = 1;
    """

    [[completions]]
    completion_label = "MyStruct"

    [[completions]]
    completion_label = "a(...)"
    completion_label_path = "(use a)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "a()"

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
    completion_label = "Err"

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
    completion_label = "None"

    [[completions]]
    completion_label = "Not"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "NullableTrait"

    [[completions]]
    completion_label = "Ok"

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
    completion_label = "Some"

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
    completion_label = "assert(...)"
    completion_label_path = "(use assert)"
    completion_label_type_info = "fn(cond: bool, err_code: felt252) -> ()"
    insert_text = "assert(${1:cond}, ${2:err_code})"

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
    completion_label = "panic(...)"
    completion_label_path = "(use panic)"
    completion_label_type_info = "fn(data: Array<felt252>) -> crate::never"
    insert_text = "panic(${1:data})"

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
    completion_label = "ALPHA"
    completion_label_path = "(use core::ec::stark_curve::ALPHA)"
    text_edits = ["""
    use core::ec::stark_curve::ALPHA;

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
    completion_label = "BETA"
    completion_label_path = "(use core::ec::stark_curve::BETA)"
    text_edits = ["""
    use core::ec::stark_curve::BETA;

    """]

    [[completions]]
    completion_label = "BYTE_ARRAY_MAGIC"
    completion_label_path = "(use core::byte_array::BYTE_ARRAY_MAGIC)"
    text_edits = ["""
    use core::byte_array::BYTE_ARRAY_MAGIC;

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
    completion_label = "ClassHashZeroable"
    completion_label_path = "(use starknet::class_hash::ClassHashZeroable)"
    text_edits = ["""
    use starknet::class_hash::ClassHashZeroable;

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
    completion_label = "ContractAddressZeroable"
    completion_label_path = "(use starknet::contract_address::ContractAddressZeroable)"
    text_edits = ["""
    use starknet::contract_address::ContractAddressZeroable;

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
    completion_label = "Done"
    completion_label_path = "(use core::circuit::AddInputResult::Done)"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "(use core::internal::DropWith)"
    text_edits = ["""
    use core::internal::DropWith;

    """]

    [[completions]]
    completion_label = "EarlyReturn"
    completion_label_path = "(use core::internal::LoopResult::EarlyReturn)"
    text_edits = ["""
    use core::internal::LoopResult::EarlyReturn;

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
    completion_label = "Err"
    completion_label_path = "(use PanicResult::Err)"
    text_edits = ["""
    use PanicResult::Err;

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
    completion_label = "EthAddressZeroable"
    completion_label_path = "(use starknet::eth_address::EthAddressZeroable)"
    text_edits = ["""
    use starknet::eth_address::EthAddressZeroable;

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
    completion_label = "False"
    completion_label_path = "(use bool::False)"
    text_edits = ["""
    use bool::False;

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
    completion_label = "FromNullableResult"
    completion_label_path = "(use core::nullable::FromNullableResult)"
    text_edits = ["""
    use core::nullable::FromNullableResult;

    """]

    [[completions]]
    completion_label = "GEN_X"
    completion_label_path = "(use core::ec::stark_curve::GEN_X)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_X;

    """]

    [[completions]]
    completion_label = "GEN_Y"
    completion_label_path = "(use core::ec::stark_curve::GEN_Y)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_Y;

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
    completion_label = "More"
    completion_label_path = "(use core::circuit::AddInputResult::More)"
    text_edits = ["""
    use core::circuit::AddInputResult::More;

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
    completion_label = "None"
    completion_label_path = "(use core::internal::OptionRev::None)"
    text_edits = ["""
    use core::internal::OptionRev::None;

    """]

    [[completions]]
    completion_label = "Normal"
    completion_label_path = "(use core::internal::LoopResult::Normal)"
    text_edits = ["""
    use core::internal::LoopResult::Normal;

    """]

    [[completions]]
    completion_label = "NotNull"
    completion_label_path = "(use core::nullable::FromNullableResult::NotNull)"
    text_edits = ["""
    use core::nullable::FromNullableResult::NotNull;

    """]

    [[completions]]
    completion_label = "Null"
    completion_label_path = "(use core::nullable::FromNullableResult::Null)"
    text_edits = ["""
    use core::nullable::FromNullableResult::Null;

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
    completion_label = "ORDER"
    completion_label_path = "(use core::ec::stark_curve::ORDER)"
    text_edits = ["""
    use core::ec::stark_curve::ORDER;

    """]

    [[completions]]
    completion_label = "Ok"
    completion_label_path = "(use PanicResult::Ok)"
    text_edits = ["""
    use PanicResult::Ok;

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
    completion_label = "Some"
    completion_label_path = "(use core::internal::OptionRev::Some)"
    text_edits = ["""
    use core::internal::OptionRev::Some;

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
    completion_label = "True"
    completion_label_path = "(use bool::True)"
    text_edits = ["""
    use bool::True;

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
    completion_label = "VALIDATED"
    completion_label_path = "(use starknet::VALIDATED)"
    text_edits = ["""
    use starknet::VALIDATED;

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
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

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
    completion_label = "bounded_int_add(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_add)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_add;

    """]

    [[completions]]
    completion_label = "bounded_int_constrain(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_constrain)"
    completion_label_type_info = "fn(value: T) -> Result<H::LowT, H::HighT> nopanic"
    insert_text = "bounded_int_constrain(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_constrain;

    """]

    [[completions]]
    completion_label = "bounded_int_div_rem(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_div_rem)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: NonZero<Rhs>) -> (H::DivT, H::RemT) nopanic"
    insert_text = "bounded_int_div_rem(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_div_rem;

    """]

    [[completions]]
    completion_label = "bounded_int_is_zero(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_is_zero)"
    completion_label_type_info = "fn(value: T) -> crate::zeroable::IsZeroResult<T> nopanic"
    insert_text = "bounded_int_is_zero(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_is_zero;

    """]

    [[completions]]
    completion_label = "bounded_int_mul(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_mul)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_mul;

    """]

    [[completions]]
    completion_label = "bounded_int_sub(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_sub)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_sub;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_max(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_max)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_max(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_max;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_min(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_min)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_min(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_min;

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
    completion_label = "cairo_keccak(...)"
    completion_label_path = "(use core::keccak::cairo_keccak)"
    completion_label_type_info = "fn(ref input: Array<u64>, last_input_word: u64, last_input_num_bytes: u32) -> u256"
    insert_text = "cairo_keccak(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::keccak::cairo_keccak;

    """]

    [[completions]]
    completion_label = "call_contract_syscall(...)"
    completion_label_path = "(use starknet::syscalls::call_contract_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "call_contract_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::call_contract_syscall;

    """]

    [[completions]]
    completion_label = "cheatcode(...)"
    completion_label_path = "(use starknet::testing::cheatcode)"
    completion_label_type_info = "fn(input: Span<felt252>) -> Span<felt252> nopanic"
    insert_text = "cheatcode(${1:input})"
    text_edits = ["""
    use starknet::testing::cheatcode;

    """]

    [[completions]]
    completion_label = "check_ecdsa_signature(...)"
    completion_label_path = "(use core::ecdsa::check_ecdsa_signature)"
    completion_label_type_info = "fn(message_hash: felt252, public_key: felt252, signature_r: felt252, signature_s: felt252) -> bool"
    insert_text = "check_ecdsa_signature(${1:message_hash}, ${2:public_key}, ${3:signature_r}, ${4:signature_s})"
    text_edits = ["""
    use core::ecdsa::check_ecdsa_signature;

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
    completion_label = "circuit_add(...)"
    completion_label_path = "(use core::circuit::circuit_add)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<AddModGate<Lhs, Rhs>>"
    insert_text = "circuit_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_add;

    """]

    [[completions]]
    completion_label = "circuit_inverse(...)"
    completion_label_path = "(use core::circuit::circuit_inverse)"
    completion_label_type_info = "fn(input: CircuitElement<Input>) -> CircuitElement<InverseGate<Input>>"
    insert_text = "circuit_inverse(${1:input})"
    text_edits = ["""
    use core::circuit::circuit_inverse;

    """]

    [[completions]]
    completion_label = "circuit_mul(...)"
    completion_label_path = "(use core::circuit::circuit_mul)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<MulModGate<Lhs, Rhs>>"
    insert_text = "circuit_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_mul;

    """]

    [[completions]]
    completion_label = "circuit_sub(...)"
    completion_label_path = "(use core::circuit::circuit_sub)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<SubModGate<Lhs, Rhs>>"
    insert_text = "circuit_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_sub;

    """]

    [[completions]]
    completion_label = "class_hash"
    completion_label_path = "(use starknet::class_hash)"
    text_edits = ["""
    use starknet::class_hash;

    """]

    [[completions]]
    completion_label = "class_hash_const(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_const)"
    completion_label_type_info = "fn() -> ClassHash nopanic"
    insert_text = "class_hash_const()"
    text_edits = ["""
    use starknet::class_hash::class_hash_const;

    """]

    [[completions]]
    completion_label = "class_hash_to_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_to_felt252)"
    completion_label_type_info = "fn(address: ClassHash) -> felt252 nopanic"
    insert_text = "class_hash_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_to_felt252;

    """]

    [[completions]]
    completion_label = "class_hash_try_from_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ClassHash> nopanic"
    insert_text = "class_hash_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_try_from_felt252;

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
    completion_label = "compute_keccak_byte_array(...)"
    completion_label_path = "(use core::keccak::compute_keccak_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> u256"
    insert_text = "compute_keccak_byte_array(${1:arr})"
    text_edits = ["""
    use core::keccak::compute_keccak_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_byte_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> [u32; 8]"
    insert_text = "compute_sha256_byte_array(${1:arr})"
    text_edits = ["""
    use core::sha256::compute_sha256_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: u32) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array_safe(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array_safe)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: BoundedInt<0, 3>) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array_safe(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array_safe;

    """]

    [[completions]]
    completion_label = "contract_address"
    completion_label_path = "(use starknet::contract_address)"
    text_edits = ["""
    use starknet::contract_address;

    """]

    [[completions]]
    completion_label = "contract_address_const(...)"
    completion_label_path = "(use starknet::contract_address_const)"
    completion_label_type_info = "fn() -> ContractAddress nopanic"
    insert_text = "contract_address_const()"
    text_edits = ["""
    use starknet::contract_address_const;

    """]

    [[completions]]
    completion_label = "contract_address_to_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_to_felt252)"
    completion_label_type_info = "fn(address: ContractAddress) -> felt252 nopanic"
    insert_text = "contract_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_to_felt252;

    """]

    [[completions]]
    completion_label = "contract_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ContractAddress> nopanic"
    insert_text = "contract_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "debug"
    completion_label_path = "(use core::debug)"
    text_edits = ["""
    use core::debug;

    """]

    [[completions]]
    completion_label = "deploy_syscall(...)"
    completion_label_path = "(use starknet::syscalls::deploy_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, contract_address_salt: felt252, calldata: Span<felt252>, deploy_from_zero: bool) -> Result<(ContractAddress, Span<felt252>), Array<felt252>> nopanic"
    insert_text = "deploy_syscall(${1:class_hash}, ${2:contract_address_salt}, ${3:calldata}, ${4:deploy_from_zero})"
    text_edits = ["""
    use starknet::syscalls::deploy_syscall;

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
    completion_label = "downcast(...)"
    completion_label_path = "(use core::internal::bounded_int::downcast)"
    completion_label_type_info = "fn(x: FromType) -> Option<ToType> nopanic"
    insert_text = "downcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::downcast;

    """]

    [[completions]]
    completion_label = "ec"
    completion_label_path = "(use core::ec)"
    text_edits = ["""
    use core::ec;

    """]

    [[completions]]
    completion_label = "ec_point_unwrap(...)"
    completion_label_path = "(use core::ec::ec_point_unwrap)"
    completion_label_type_info = "fn(p: NonZero<EcPoint>) -> (felt252, felt252) nopanic"
    insert_text = "ec_point_unwrap(${1:p})"
    text_edits = ["""
    use core::ec::ec_point_unwrap;

    """]

    [[completions]]
    completion_label = "ecdsa"
    completion_label_path = "(use core::ecdsa)"
    text_edits = ["""
    use core::ecdsa;

    """]

    [[completions]]
    completion_label = "egcd(...)"
    completion_label_path = "(use core::math::egcd)"
    completion_label_type_info = "fn(a: NonZero<T>, b: NonZero<T>) -> (T, T, T, bool)"
    insert_text = "egcd(${1:a}, ${2:b})"
    text_edits = ["""
    use core::math::egcd;

    """]

    [[completions]]
    completion_label = "emit_event_syscall(...)"
    completion_label_path = "(use starknet::syscalls::emit_event_syscall)"
    completion_label_type_info = "fn(keys: Span<felt252>, data: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "emit_event_syscall(${1:keys}, ${2:data})"
    text_edits = ["""
    use starknet::syscalls::emit_event_syscall;

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
    completion_label = "felt252_div(...)"
    completion_label_path = "(use core::felt252_div)"
    completion_label_type_info = "fn(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic"
    insert_text = "felt252_div(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::felt252_div;

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
    completion_label = "gas_reserve_create(...)"
    completion_label_path = "(use core::gas::gas_reserve_create)"
    completion_label_type_info = "fn(amount: u128) -> Option<GasReserve> nopanic"
    insert_text = "gas_reserve_create(${1:amount})"
    text_edits = ["""
    use core::gas::gas_reserve_create;

    """]

    [[completions]]
    completion_label = "gas_reserve_utilize(...)"
    completion_label_path = "(use core::gas::gas_reserve_utilize)"
    completion_label_type_info = "fn(reserve: GasReserve) -> () nopanic"
    insert_text = "gas_reserve_utilize(${1:reserve})"
    text_edits = ["""
    use core::gas::gas_reserve_utilize;

    """]

    [[completions]]
    completion_label = "get"
    completion_label_path = "(use core::ops::get)"
    text_edits = ["""
    use core::ops::get;

    """]

    [[completions]]
    completion_label = "get_available_gas(...)"
    completion_label_path = "(use core::testing::get_available_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_available_gas()"
    text_edits = ["""
    use core::testing::get_available_gas;

    """]

    [[completions]]
    completion_label = "get_block_hash_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_block_hash_syscall)"
    completion_label_type_info = "fn(block_number: u64) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "get_block_hash_syscall(${1:block_number})"
    text_edits = ["""
    use starknet::syscalls::get_block_hash_syscall;

    """]

    [[completions]]
    completion_label = "get_block_info(...)"
    completion_label_path = "(use starknet::get_block_info)"
    completion_label_type_info = "fn() -> Box<BlockInfo>"
    insert_text = "get_block_info()"
    text_edits = ["""
    use starknet::get_block_info;

    """]

    [[completions]]
    completion_label = "get_block_number(...)"
    completion_label_path = "(use starknet::get_block_number)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_number()"
    text_edits = ["""
    use starknet::get_block_number;

    """]

    [[completions]]
    completion_label = "get_block_timestamp(...)"
    completion_label_path = "(use starknet::get_block_timestamp)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_timestamp()"
    text_edits = ["""
    use starknet::get_block_timestamp;

    """]

    [[completions]]
    completion_label = "get_builtin_costs(...)"
    completion_label_path = "(use core::gas::get_builtin_costs)"
    completion_label_type_info = "fn() -> BuiltinCosts nopanic"
    insert_text = "get_builtin_costs()"
    text_edits = ["""
    use core::gas::get_builtin_costs;

    """]

    [[completions]]
    completion_label = "get_caller_address(...)"
    completion_label_path = "(use starknet::get_caller_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_caller_address()"
    text_edits = ["""
    use starknet::get_caller_address;

    """]

    [[completions]]
    completion_label = "get_class_hash_at_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_class_hash_at_syscall)"
    completion_label_type_info = "fn(contract_address: ContractAddress) -> Result<ClassHash, Array<felt252>> nopanic"
    insert_text = "get_class_hash_at_syscall(${1:contract_address})"
    text_edits = ["""
    use starknet::syscalls::get_class_hash_at_syscall;

    """]

    [[completions]]
    completion_label = "get_contract_address(...)"
    completion_label_path = "(use starknet::get_contract_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_contract_address()"
    text_edits = ["""
    use starknet::get_contract_address;

    """]

    [[completions]]
    completion_label = "get_execution_info(...)"
    completion_label_path = "(use starknet::get_execution_info)"
    completion_label_type_info = "fn() -> Box<starknet::ExecutionInfo>"
    insert_text = "get_execution_info()"
    text_edits = ["""
    use starknet::get_execution_info;

    """]

    [[completions]]
    completion_label = "get_execution_info_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v2_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v2_syscall)"
    completion_label_type_info = "fn() -> Result<Box<starknet::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v2_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v2_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v3_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v3_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::v3::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v3_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v3_syscall;

    """]

    [[completions]]
    completion_label = "get_tx_info(...)"
    completion_label_path = "(use starknet::get_tx_info)"
    completion_label_type_info = "fn() -> Box<starknet::TxInfo>"
    insert_text = "get_tx_info()"
    text_edits = ["""
    use starknet::get_tx_info;

    """]

    [[completions]]
    completion_label = "get_unspent_gas(...)"
    completion_label_path = "(use core::testing::get_unspent_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_unspent_gas()"
    text_edits = ["""
    use core::testing::get_unspent_gas;

    """]

    [[completions]]
    completion_label = "hades_permutation(...)"
    completion_label_path = "(use core::poseidon::hades_permutation)"
    completion_label_type_info = "fn(s0: felt252, s1: felt252, s2: felt252) -> (felt252, felt252, felt252) nopanic"
    insert_text = "hades_permutation(${1:s0}, ${2:s1}, ${3:s2})"
    text_edits = ["""
    use core::poseidon::hades_permutation;

    """]

    [[completions]]
    completion_label = "hash"
    completion_label_path = "(use core::hash)"
    text_edits = ["""
    use core::hash;

    """]

    [[completions]]
    completion_label = "i128_diff(...)"
    completion_label_path = "(use core::integer::i128_diff)"
    completion_label_type_info = "fn(lhs: i128, rhs: i128) -> Result<u128, u128> nopanic"
    insert_text = "i128_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i128_diff;

    """]

    [[completions]]
    completion_label = "i16_diff(...)"
    completion_label_path = "(use core::integer::i16_diff)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> Result<u16, u16> nopanic"
    insert_text = "i16_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_diff;

    """]

    [[completions]]
    completion_label = "i16_wide_mul(...)"
    completion_label_path = "(use core::integer::i16_wide_mul)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> i32 nopanic"
    insert_text = "i16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_wide_mul;

    """]

    [[completions]]
    completion_label = "i32_diff(...)"
    completion_label_path = "(use core::integer::i32_diff)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> Result<u32, u32> nopanic"
    insert_text = "i32_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_diff;

    """]

    [[completions]]
    completion_label = "i32_wide_mul(...)"
    completion_label_path = "(use core::integer::i32_wide_mul)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> i64 nopanic"
    insert_text = "i32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_wide_mul;

    """]

    [[completions]]
    completion_label = "i64_diff(...)"
    completion_label_path = "(use core::integer::i64_diff)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> Result<u64, u64> nopanic"
    insert_text = "i64_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_diff;

    """]

    [[completions]]
    completion_label = "i64_wide_mul(...)"
    completion_label_path = "(use core::integer::i64_wide_mul)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> i128 nopanic"
    insert_text = "i64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_wide_mul;

    """]

    [[completions]]
    completion_label = "i8_diff(...)"
    completion_label_path = "(use core::integer::i8_diff)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> Result<u8, u8> nopanic"
    insert_text = "i8_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_diff;

    """]

    [[completions]]
    completion_label = "i8_wide_mul(...)"
    completion_label_path = "(use core::integer::i8_wide_mul)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> i16 nopanic"
    insert_text = "i8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_wide_mul;

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
    completion_label = "inv_mod(...)"
    completion_label_path = "(use core::math::inv_mod)"
    completion_label_type_info = "fn(a: NonZero<T>, n: NonZero<T>) -> Option<T>"
    insert_text = "inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::inv_mod;

    """]

    [[completions]]
    completion_label = "is_eth_signature_valid(...)"
    completion_label_path = "(use starknet::eth_signature::is_eth_signature_valid)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> Result<(), felt252>"
    insert_text = "is_eth_signature_valid(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::is_eth_signature_valid;

    """]

    [[completions]]
    completion_label = "is_signature_entry_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_entry_valid)"
    completion_label_type_info = "fn(value: u256) -> bool"
    insert_text = "is_signature_entry_valid(${1:value})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_entry_valid;

    """]

    [[completions]]
    completion_label = "is_signature_s_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_s_valid)"
    completion_label_type_info = "fn(s: u256) -> bool"
    insert_text = "is_signature_s_valid(${1:s})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_s_valid;

    """]

    [[completions]]
    completion_label = "is_valid_signature(...)"
    completion_label_path = "(use starknet::secp256_trait::is_valid_signature)"
    completion_label_type_info = "fn(msg_hash: u256, r: u256, s: u256, public_key: Secp256Point) -> bool"
    insert_text = "is_valid_signature(${1:msg_hash}, ${2:r}, ${3:s}, ${4:public_key})"
    text_edits = ["""
    use starknet::secp256_trait::is_valid_signature;

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
    completion_label = "keccak_syscall(...)"
    completion_label_path = "(use starknet::syscalls::keccak_syscall)"
    completion_label_type_info = "fn(input: Span<u64>) -> Result<u256, Array<felt252>> nopanic"
    insert_text = "keccak_syscall(${1:input})"
    text_edits = ["""
    use starknet::syscalls::keccak_syscall;

    """]

    [[completions]]
    completion_label = "keccak_u256s_be_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_be_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_be_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_be_inputs;

    """]

    [[completions]]
    completion_label = "keccak_u256s_le_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_le_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_le_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_le_inputs;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "m31_add(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_add)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_add(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_add;

    """]

    [[completions]]
    completion_label = "m31_div(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_div)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: NonZero<crate::internal::bounded_int::BoundedInt<0, 2147483646>>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_div(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_div;

    """]

    [[completions]]
    completion_label = "m31_mul(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_mul)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_mul;

    """]

    [[completions]]
    completion_label = "m31_ops"
    completion_label_path = "(use core::qm31::m31_ops)"
    text_edits = ["""
    use core::qm31::m31_ops;

    """]

    [[completions]]
    completion_label = "m31_sub(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_sub)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_sub;

    """]

    [[completions]]
    completion_label = "match_nullable(...)"
    completion_label_path = "(use core::nullable::match_nullable)"
    completion_label_type_info = "fn(value: Nullable<T>) -> FromNullableResult<T> nopanic"
    insert_text = "match_nullable(${1:value})"
    text_edits = ["""
    use core::nullable::match_nullable;

    """]

    [[completions]]
    completion_label = "math"
    completion_label_path = "(use core::math)"
    text_edits = ["""
    use core::math;

    """]

    [[completions]]
    completion_label = "max(...)"
    completion_label_path = "(use core::cmp::max)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "max(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::max;

    """]

    [[completions]]
    completion_label = "meta_tx_v0_syscall(...)"
    completion_label_path = "(use starknet::syscalls::meta_tx_v0_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>, signature: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "meta_tx_v0_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata}, ${4:signature})"
    text_edits = ["""
    use starknet::syscalls::meta_tx_v0_syscall;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
    text_edits = ["""
    use core::metaprogramming;

    """]

    [[completions]]
    completion_label = "min(...)"
    completion_label_path = "(use core::cmp::min)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "min(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::min;

    """]

    [[completions]]
    completion_label = "minmax(...)"
    completion_label_path = "(use core::cmp::minmax)"
    completion_label_type_info = "fn(a: T, b: T) -> (T, T)"
    insert_text = "minmax(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::minmax;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "null(...)"
    completion_label_path = "(use core::nullable::null)"
    completion_label_type_info = "fn() -> Nullable<T> nopanic"
    insert_text = "null()"
    text_edits = ["""
    use core::nullable::null;

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
    completion_label = "panic_with_byte_array(...)"
    completion_label_path = "(use core::panics::panic_with_byte_array)"
    completion_label_type_info = "fn(err: @ByteArray) -> crate::never"
    insert_text = "panic_with_byte_array(${1:err})"
    text_edits = ["""
    use core::panics::panic_with_byte_array;

    """]

    [[completions]]
    completion_label = "panic_with_const_felt252(...)"
    completion_label_path = "(use core::panic_with_const_felt252)"
    completion_label_type_info = "fn() -> never"
    insert_text = "panic_with_const_felt252()"
    text_edits = ["""
    use core::panic_with_const_felt252;

    """]

    [[completions]]
    completion_label = "panic_with_felt252(...)"
    completion_label_path = "(use core::panic_with_felt252)"
    completion_label_type_info = "fn(err_code: felt252) -> never"
    insert_text = "panic_with_felt252(${1:err_code})"
    text_edits = ["""
    use core::panic_with_felt252;

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
    completion_label = "pedersen(...)"
    completion_label_path = "(use core::pedersen::pedersen)"
    completion_label_type_info = "fn(a: felt252, b: felt252) -> felt252 nopanic"
    insert_text = "pedersen(${1:a}, ${2:b})"
    text_edits = ["""
    use core::pedersen::pedersen;

    """]

    [[completions]]
    completion_label = "pop_l2_to_l1_message(...)"
    completion_label_path = "(use starknet::testing::pop_l2_to_l1_message)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(felt252, Span<felt252>)>"
    insert_text = "pop_l2_to_l1_message(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_l2_to_l1_message;

    """]

    [[completions]]
    completion_label = "pop_log(...)"
    completion_label_path = "(use starknet::testing::pop_log)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<T>"
    insert_text = "pop_log(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log;

    """]

    [[completions]]
    completion_label = "pop_log_raw(...)"
    completion_label_path = "(use starknet::testing::pop_log_raw)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(Span<felt252>, Span<felt252>)>"
    insert_text = "pop_log_raw(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log_raw;

    """]

    [[completions]]
    completion_label = "poseidon"
    completion_label_path = "(use core::poseidon)"
    text_edits = ["""
    use core::poseidon;

    """]

    [[completions]]
    completion_label = "poseidon_hash_span(...)"
    completion_label_path = "(use core::poseidon::poseidon_hash_span)"
    completion_label_type_info = "fn(span: Span<felt252>) -> felt252"
    insert_text = "poseidon_hash_span(${1:span})"
    text_edits = ["""
    use core::poseidon::poseidon_hash_span;

    """]

    [[completions]]
    completion_label = "pow"
    completion_label_path = "(use core::num::traits::ops::pow)"
    text_edits = ["""
    use core::num::traits::ops::pow;

    """]

    [[completions]]
    completion_label = "print_byte_array_as_string(...)"
    completion_label_path = "(use core::debug::print_byte_array_as_string)"
    completion_label_type_info = "fn(self: @ByteArray) -> ()"
    insert_text = "print_byte_array_as_string()"
    text_edits = ["""
    use core::debug::print_byte_array_as_string;

    """]

    [[completions]]
    completion_label = "public_key_point_to_eth_address(...)"
    completion_label_path = "(use starknet::eth_signature::public_key_point_to_eth_address)"
    completion_label_type_info = "fn(public_key_point: Secp256Point) -> EthAddress"
    insert_text = "public_key_point_to_eth_address(${1:public_key_point})"
    text_edits = ["""
    use starknet::eth_signature::public_key_point_to_eth_address;

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
    completion_label = "qm31_const(...)"
    completion_label_path = "(use core::qm31::qm31_const)"
    completion_label_type_info = "fn() -> qm31 nopanic"
    insert_text = "qm31_const()"
    text_edits = ["""
    use core::qm31::qm31_const;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use core::ecdsa::recover_public_key)"
    completion_label_type_info = "fn(message_hash: felt252, signature_r: felt252, signature_s: felt252, y_parity: bool) -> Option<felt252>"
    insert_text = "recover_public_key(${1:message_hash}, ${2:signature_r}, ${3:signature_s}, ${4:y_parity})"
    text_edits = ["""
    use core::ecdsa::recover_public_key;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use starknet::secp256_trait::recover_public_key)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature) -> Option<Secp256Point>"
    insert_text = "recover_public_key(${1:msg_hash}, ${2:signature})"
    text_edits = ["""
    use starknet::secp256_trait::recover_public_key;

    """]

    [[completions]]
    completion_label = "redeposit_gas(...)"
    completion_label_path = "(use core::gas::redeposit_gas)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "redeposit_gas()"
    text_edits = ["""
    use core::gas::redeposit_gas;

    """]

    [[completions]]
    completion_label = "replace_class_syscall(...)"
    completion_label_path = "(use starknet::syscalls::replace_class_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash) -> Result<(), Array<felt252>> nopanic"
    insert_text = "replace_class_syscall(${1:class_hash})"
    text_edits = ["""
    use starknet::syscalls::replace_class_syscall;

    """]

    [[completions]]
    completion_label = "require_implicit(...)"
    completion_label_path = "(use core::internal::require_implicit)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "require_implicit()"
    text_edits = ["""
    use core::internal::require_implicit;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]

    [[completions]]
    completion_label = "revoke_ap_tracking(...)"
    completion_label_path = "(use core::internal::revoke_ap_tracking)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "revoke_ap_tracking()"
    text_edits = ["""
    use core::internal::revoke_ap_tracking;

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
    completion_label = "send_message_to_l1_syscall(...)"
    completion_label_path = "(use starknet::syscalls::send_message_to_l1_syscall)"
    completion_label_type_info = "fn(to_address: felt252, payload: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "send_message_to_l1_syscall(${1:to_address}, ${2:payload})"
    text_edits = ["""
    use starknet::syscalls::send_message_to_l1_syscall;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "set_account_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_account_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_account_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_account_contract_address;

    """]

    [[completions]]
    completion_label = "set_block_hash(...)"
    completion_label_path = "(use starknet::testing::set_block_hash)"
    completion_label_type_info = "fn(block_number: u64, value: felt252) -> ()"
    insert_text = "set_block_hash(${1:block_number}, ${2:value})"
    text_edits = ["""
    use starknet::testing::set_block_hash;

    """]

    [[completions]]
    completion_label = "set_block_number(...)"
    completion_label_path = "(use starknet::testing::set_block_number)"
    completion_label_type_info = "fn(block_number: u64) -> ()"
    insert_text = "set_block_number(${1:block_number})"
    text_edits = ["""
    use starknet::testing::set_block_number;

    """]

    [[completions]]
    completion_label = "set_block_timestamp(...)"
    completion_label_path = "(use starknet::testing::set_block_timestamp)"
    completion_label_type_info = "fn(block_timestamp: u64) -> ()"
    insert_text = "set_block_timestamp(${1:block_timestamp})"
    text_edits = ["""
    use starknet::testing::set_block_timestamp;

    """]

    [[completions]]
    completion_label = "set_caller_address(...)"
    completion_label_path = "(use starknet::testing::set_caller_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_caller_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_caller_address;

    """]

    [[completions]]
    completion_label = "set_chain_id(...)"
    completion_label_path = "(use starknet::testing::set_chain_id)"
    completion_label_type_info = "fn(chain_id: felt252) -> ()"
    insert_text = "set_chain_id(${1:chain_id})"
    text_edits = ["""
    use starknet::testing::set_chain_id;

    """]

    [[completions]]
    completion_label = "set_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_contract_address;

    """]

    [[completions]]
    completion_label = "set_max_fee(...)"
    completion_label_path = "(use starknet::testing::set_max_fee)"
    completion_label_type_info = "fn(fee: u128) -> ()"
    insert_text = "set_max_fee(${1:fee})"
    text_edits = ["""
    use starknet::testing::set_max_fee;

    """]

    [[completions]]
    completion_label = "set_nonce(...)"
    completion_label_path = "(use starknet::testing::set_nonce)"
    completion_label_type_info = "fn(nonce: felt252) -> ()"
    insert_text = "set_nonce(${1:nonce})"
    text_edits = ["""
    use starknet::testing::set_nonce;

    """]

    [[completions]]
    completion_label = "set_sequencer_address(...)"
    completion_label_path = "(use starknet::testing::set_sequencer_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_sequencer_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_sequencer_address;

    """]

    [[completions]]
    completion_label = "set_signature(...)"
    completion_label_path = "(use starknet::testing::set_signature)"
    completion_label_type_info = "fn(signature: Span<felt252>) -> ()"
    insert_text = "set_signature(${1:signature})"
    text_edits = ["""
    use starknet::testing::set_signature;

    """]

    [[completions]]
    completion_label = "set_transaction_hash(...)"
    completion_label_path = "(use starknet::testing::set_transaction_hash)"
    completion_label_type_info = "fn(hash: felt252) -> ()"
    insert_text = "set_transaction_hash(${1:hash})"
    text_edits = ["""
    use starknet::testing::set_transaction_hash;

    """]

    [[completions]]
    completion_label = "set_version(...)"
    completion_label_path = "(use starknet::testing::set_version)"
    completion_label_type_info = "fn(version: felt252) -> ()"
    insert_text = "set_version(${1:version})"
    text_edits = ["""
    use starknet::testing::set_version;

    """]

    [[completions]]
    completion_label = "sha256"
    completion_label_path = "(use core::sha256)"
    text_edits = ["""
    use core::sha256;

    """]

    [[completions]]
    completion_label = "sha256_process_block_syscall(...)"
    completion_label_path = "(use starknet::syscalls::sha256_process_block_syscall)"
    completion_label_type_info = "fn(state: crate::sha256::Sha256StateHandle, input: Box<[u32; 16]>) -> Result<crate::sha256::Sha256StateHandle, Array<felt252>> nopanic"
    insert_text = "sha256_process_block_syscall(${1:state}, ${2:input})"
    text_edits = ["""
    use starknet::syscalls::sha256_process_block_syscall;

    """]

    [[completions]]
    completion_label = "signature_from_vrs(...)"
    completion_label_path = "(use starknet::secp256_trait::signature_from_vrs)"
    completion_label_type_info = "fn(v: u32, r: u256, s: u256) -> Signature"
    insert_text = "signature_from_vrs(${1:v}, ${2:r}, ${3:s})"
    text_edits = ["""
    use starknet::secp256_trait::signature_from_vrs;

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
    completion_label = "storage_address_from_base(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base)"
    completion_label_type_info = "fn(base: StorageBaseAddress) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base(${1:base})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base;

    """]

    [[completions]]
    completion_label = "storage_address_from_base_and_offset(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base_and_offset)"
    completion_label_type_info = "fn(base: StorageBaseAddress, offset: u8) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base_and_offset(${1:base}, ${2:offset})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base_and_offset;

    """]

    [[completions]]
    completion_label = "storage_address_to_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_to_felt252)"
    completion_label_type_info = "fn(address: StorageAddress) -> felt252 nopanic"
    insert_text = "storage_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_to_felt252;

    """]

    [[completions]]
    completion_label = "storage_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<StorageAddress> nopanic"
    insert_text = "storage_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_base_address_const(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_const)"
    completion_label_type_info = "fn() -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_const()"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_const;

    """]

    [[completions]]
    completion_label = "storage_base_address_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_from_felt252)"
    completion_label_type_info = "fn(addr: felt252) -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_from_felt252(${1:addr})"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_read_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_read_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "storage_read_syscall(${1:address_domain}, ${2:address})"
    text_edits = ["""
    use starknet::syscalls::storage_read_syscall;

    """]

    [[completions]]
    completion_label = "storage_write_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_write_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress, value: felt252) -> Result<(), Array<felt252>> nopanic"
    insert_text = "storage_write_syscall(${1:address_domain}, ${2:address}, ${3:value})"
    text_edits = ["""
    use starknet::syscalls::storage_write_syscall;

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
    completion_label = "u128_byte_reverse(...)"
    completion_label_path = "(use core::integer::u128_byte_reverse)"
    completion_label_type_info = "fn(input: u128) -> u128 nopanic"
    insert_text = "u128_byte_reverse(${1:input})"
    text_edits = ["""
    use core::integer::u128_byte_reverse;

    """]

    [[completions]]
    completion_label = "u128_overflowing_add(...)"
    completion_label_path = "(use core::integer::u128_overflowing_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_add;

    """]

    [[completions]]
    completion_label = "u128_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u128_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> (u128, bool) nopanic"
    insert_text = "u128_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u128_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u128_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u128_safe_divmod(...)"
    completion_label_path = "(use core::integer::u128_safe_divmod)"
    completion_label_type_info = "fn(lhs: u128, rhs: NonZero<u128>) -> (u128, u128) nopanic"
    insert_text = "u128_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_safe_divmod;

    """]

    [[completions]]
    completion_label = "u128_sqrt(...)"
    completion_label_path = "(use core::integer::u128_sqrt)"
    completion_label_type_info = "fn(value: u128) -> u64 nopanic"
    insert_text = "u128_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u128_sqrt;

    """]

    [[completions]]
    completion_label = "u128_wide_mul(...)"
    completion_label_path = "(use core::integer::u128_wide_mul)"
    completion_label_type_info = "fn(a: u128, b: u128) -> (u128, u128) nopanic"
    insert_text = "u128_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wide_mul;

    """]

    [[completions]]
    completion_label = "u128_wrapping_add(...)"
    completion_label_path = "(use core::integer::u128_wrapping_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_wrapping_add;

    """]

    [[completions]]
    completion_label = "u128_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u128_wrapping_sub)"
    completion_label_type_info = "fn(a: u128, b: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u16_overflowing_add(...)"
    completion_label_path = "(use core::integer::u16_overflowing_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_add;

    """]

    [[completions]]
    completion_label = "u16_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u16_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u16_safe_divmod(...)"
    completion_label_path = "(use core::integer::u16_safe_divmod)"
    completion_label_type_info = "fn(lhs: u16, rhs: NonZero<u16>) -> (u16, u16) nopanic"
    insert_text = "u16_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_safe_divmod;

    """]

    [[completions]]
    completion_label = "u16_sqrt(...)"
    completion_label_path = "(use core::integer::u16_sqrt)"
    completion_label_type_info = "fn(value: u16) -> u8 nopanic"
    insert_text = "u16_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u16_sqrt;

    """]

    [[completions]]
    completion_label = "u16_wide_mul(...)"
    completion_label_path = "(use core::integer::u16_wide_mul)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u32 nopanic"
    insert_text = "u16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wide_mul;

    """]

    [[completions]]
    completion_label = "u16_wrapping_add(...)"
    completion_label_path = "(use core::integer::u16_wrapping_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_add;

    """]

    [[completions]]
    completion_label = "u16_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u16_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u2"
    completion_label_path = "(use core::sha256::u2)"
    text_edits = ["""
    use core::sha256::u2;

    """]

    [[completions]]
    completion_label = "u256_div_mod_n(...)"
    completion_label_path = "(use core::math::u256_div_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> Option<u256>"
    insert_text = "u256_div_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_div_mod_n;

    """]

    [[completions]]
    completion_label = "u256_inv_mod(...)"
    completion_label_path = "(use core::math::u256_inv_mod)"
    completion_label_type_info = "fn(a: u256, n: NonZero<u256>) -> Option<NonZero<u256>>"
    insert_text = "u256_inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::u256_inv_mod;

    """]

    [[completions]]
    completion_label = "u256_mul_mod_n(...)"
    completion_label_path = "(use core::math::u256_mul_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> u256"
    insert_text = "u256_mul_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_mul_mod_n;

    """]

    [[completions]]
    completion_label = "u256_overflow_mul(...)"
    completion_label_path = "(use core::integer::u256_overflow_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflow_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_mul;

    """]

    [[completions]]
    completion_label = "u256_overflow_sub(...)"
    completion_label_path = "(use core::integer::u256_overflow_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflow_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_sub;

    """]

    [[completions]]
    completion_label = "u256_overflowing_add(...)"
    completion_label_path = "(use core::integer::u256_overflowing_add)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_add;

    """]

    [[completions]]
    completion_label = "u256_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u256_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u256_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u256_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u256_sqrt(...)"
    completion_label_path = "(use core::integer::u256_sqrt)"
    completion_label_type_info = "fn(a: u256) -> u128 nopanic"
    insert_text = "u256_sqrt(${1:a})"
    text_edits = ["""
    use core::integer::u256_sqrt;

    """]

    [[completions]]
    completion_label = "u256_wide_mul(...)"
    completion_label_path = "(use core::integer::u256_wide_mul)"
    completion_label_type_info = "fn(a: u256, b: u256) -> u512 nopanic"
    insert_text = "u256_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u256_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_overflowing_add(...)"
    completion_label_path = "(use core::integer::u32_overflowing_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_add;

    """]

    [[completions]]
    completion_label = "u32_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u32_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u32_safe_divmod(...)"
    completion_label_path = "(use core::integer::u32_safe_divmod)"
    completion_label_type_info = "fn(lhs: u32, rhs: NonZero<u32>) -> (u32, u32) nopanic"
    insert_text = "u32_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_safe_divmod;

    """]

    [[completions]]
    completion_label = "u32_sqrt(...)"
    completion_label_path = "(use core::integer::u32_sqrt)"
    completion_label_type_info = "fn(value: u32) -> u16 nopanic"
    insert_text = "u32_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u32_sqrt;

    """]

    [[completions]]
    completion_label = "u32_wide_mul(...)"
    completion_label_path = "(use core::integer::u32_wide_mul)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u64 nopanic"
    insert_text = "u32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_wrapping_add(...)"
    completion_label_path = "(use core::integer::u32_wrapping_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_add;

    """]

    [[completions]]
    completion_label = "u32_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u32_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_sub;

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
    completion_label = "u512_safe_div_rem_by_u256(...)"
    completion_label_path = "(use core::integer::u512_safe_div_rem_by_u256)"
    completion_label_type_info = "fn(lhs: u512, rhs: NonZero<u256>) -> (u512, u256) nopanic"
    insert_text = "u512_safe_div_rem_by_u256(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u512_safe_div_rem_by_u256;

    """]

    [[completions]]
    completion_label = "u64_overflowing_add(...)"
    completion_label_path = "(use core::integer::u64_overflowing_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_add;

    """]

    [[completions]]
    completion_label = "u64_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u64_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u64_safe_divmod(...)"
    completion_label_path = "(use core::integer::u64_safe_divmod)"
    completion_label_type_info = "fn(lhs: u64, rhs: NonZero<u64>) -> (u64, u64) nopanic"
    insert_text = "u64_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_safe_divmod;

    """]

    [[completions]]
    completion_label = "u64_sqrt(...)"
    completion_label_path = "(use core::integer::u64_sqrt)"
    completion_label_type_info = "fn(value: u64) -> u32 nopanic"
    insert_text = "u64_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u64_sqrt;

    """]

    [[completions]]
    completion_label = "u64_wide_mul(...)"
    completion_label_path = "(use core::integer::u64_wide_mul)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u128 nopanic"
    insert_text = "u64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wide_mul;

    """]

    [[completions]]
    completion_label = "u64_wrapping_add(...)"
    completion_label_path = "(use core::integer::u64_wrapping_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_add;

    """]

    [[completions]]
    completion_label = "u64_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u64_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u8_overflowing_add(...)"
    completion_label_path = "(use core::integer::u8_overflowing_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_add;

    """]

    [[completions]]
    completion_label = "u8_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u8_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u8_safe_divmod(...)"
    completion_label_path = "(use core::integer::u8_safe_divmod)"
    completion_label_type_info = "fn(lhs: u8, rhs: NonZero<u8>) -> (u8, u8) nopanic"
    insert_text = "u8_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_safe_divmod;

    """]

    [[completions]]
    completion_label = "u8_sqrt(...)"
    completion_label_path = "(use core::integer::u8_sqrt)"
    completion_label_type_info = "fn(value: u8) -> u8 nopanic"
    insert_text = "u8_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u8_sqrt;

    """]

    [[completions]]
    completion_label = "u8_wide_mul(...)"
    completion_label_path = "(use core::integer::u8_wide_mul)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u16 nopanic"
    insert_text = "u8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wide_mul;

    """]

    [[completions]]
    completion_label = "u8_wrapping_add(...)"
    completion_label_path = "(use core::integer::u8_wrapping_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_add;

    """]

    [[completions]]
    completion_label = "u8_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u8_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u96"
    completion_label_path = "(use core::circuit::u96)"
    text_edits = ["""
    use core::circuit::u96;

    """]

    [[completions]]
    completion_label = "upcast(...)"
    completion_label_path = "(use core::internal::bounded_int::upcast)"
    completion_label_type_info = "fn(x: FromType) -> ToType nopanic"
    insert_text = "upcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::upcast;

    """]

    [[completions]]
    completion_label = "verify_eth_signature(...)"
    completion_label_path = "(use starknet::eth_signature::verify_eth_signature)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> ()"
    insert_text = "verify_eth_signature(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::verify_eth_signature;

    """]

    [[completions]]
    completion_label = "withdraw_gas(...)"
    completion_label_path = "(use core::gas::withdraw_gas)"
    completion_label_type_info = "fn() -> Option<()> nopanic"
    insert_text = "withdraw_gas()"
    text_edits = ["""
    use core::gas::withdraw_gas;

    """]

    [[completions]]
    completion_label = "withdraw_gas_all(...)"
    completion_label_path = "(use core::gas::withdraw_gas_all)"
    completion_label_type_info = "fn(costs: BuiltinCosts) -> Option<()> nopanic"
    insert_text = "withdraw_gas_all(${1:costs})"
    text_edits = ["""
    use core::gas::withdraw_gas_all;

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

    [[completions]]
    completion_label = "zip(...)"
    completion_label_path = "(use core::iter::zip)"
    completion_label_type_info = "fn(a: A, b: B) -> Zip<AIntoIter::IntoIter, BIntoIter::IntoIter>"
    insert_text = "zip(${1:a}, ${2:b})"
    text_edits = ["""
    use core::iter::zip;

    """]
    "#);
}

#[test]
fn no_text_after_statement() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct MyStruct {}

    fn a() {
        let _x = 1;
        <caret>
        let _y = 2;
    }
    ",@r#"
    caret = """
        <caret>
    """

    [[completions]]
    completion_label = "_x"
    completion_label_type_info = "felt252"

    [[completions]]
    completion_label = "MyStruct"

    [[completions]]
    completion_label = "a(...)"
    completion_label_path = "(use a)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "a()"

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
    completion_label = "Err"

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
    completion_label = "None"

    [[completions]]
    completion_label = "Not"

    [[completions]]
    completion_label = "Nullable"

    [[completions]]
    completion_label = "NullableTrait"

    [[completions]]
    completion_label = "Ok"

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
    completion_label = "Some"

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
    completion_label = "assert(...)"
    completion_label_path = "(use assert)"
    completion_label_type_info = "fn(cond: bool, err_code: felt252) -> ()"
    insert_text = "assert(${1:cond}, ${2:err_code})"

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
    completion_label = "panic(...)"
    completion_label_path = "(use panic)"
    completion_label_type_info = "fn(data: Array<felt252>) -> crate::never"
    insert_text = "panic(${1:data})"

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
    completion_label = "ALPHA"
    completion_label_path = "(use core::ec::stark_curve::ALPHA)"
    text_edits = ["""
    use core::ec::stark_curve::ALPHA;

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
    completion_label = "BETA"
    completion_label_path = "(use core::ec::stark_curve::BETA)"
    text_edits = ["""
    use core::ec::stark_curve::BETA;

    """]

    [[completions]]
    completion_label = "BYTE_ARRAY_MAGIC"
    completion_label_path = "(use core::byte_array::BYTE_ARRAY_MAGIC)"
    text_edits = ["""
    use core::byte_array::BYTE_ARRAY_MAGIC;

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
    completion_label = "ClassHashZeroable"
    completion_label_path = "(use starknet::class_hash::ClassHashZeroable)"
    text_edits = ["""
    use starknet::class_hash::ClassHashZeroable;

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
    completion_label = "ContractAddressZeroable"
    completion_label_path = "(use starknet::contract_address::ContractAddressZeroable)"
    text_edits = ["""
    use starknet::contract_address::ContractAddressZeroable;

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
    completion_label = "Done"
    completion_label_path = "(use core::circuit::AddInputResult::Done)"
    text_edits = ["""
    use core::circuit::AddInputResult::Done;

    """]

    [[completions]]
    completion_label = "DropWith"
    completion_label_path = "(use core::internal::DropWith)"
    text_edits = ["""
    use core::internal::DropWith;

    """]

    [[completions]]
    completion_label = "EarlyReturn"
    completion_label_path = "(use core::internal::LoopResult::EarlyReturn)"
    text_edits = ["""
    use core::internal::LoopResult::EarlyReturn;

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
    completion_label = "Err"
    completion_label_path = "(use PanicResult::Err)"
    text_edits = ["""
    use PanicResult::Err;

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
    completion_label = "EthAddressZeroable"
    completion_label_path = "(use starknet::eth_address::EthAddressZeroable)"
    text_edits = ["""
    use starknet::eth_address::EthAddressZeroable;

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
    completion_label = "False"
    completion_label_path = "(use bool::False)"
    text_edits = ["""
    use bool::False;

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
    completion_label = "FromNullableResult"
    completion_label_path = "(use core::nullable::FromNullableResult)"
    text_edits = ["""
    use core::nullable::FromNullableResult;

    """]

    [[completions]]
    completion_label = "GEN_X"
    completion_label_path = "(use core::ec::stark_curve::GEN_X)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_X;

    """]

    [[completions]]
    completion_label = "GEN_Y"
    completion_label_path = "(use core::ec::stark_curve::GEN_Y)"
    text_edits = ["""
    use core::ec::stark_curve::GEN_Y;

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
    completion_label = "More"
    completion_label_path = "(use core::circuit::AddInputResult::More)"
    text_edits = ["""
    use core::circuit::AddInputResult::More;

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
    completion_label = "None"
    completion_label_path = "(use core::internal::OptionRev::None)"
    text_edits = ["""
    use core::internal::OptionRev::None;

    """]

    [[completions]]
    completion_label = "Normal"
    completion_label_path = "(use core::internal::LoopResult::Normal)"
    text_edits = ["""
    use core::internal::LoopResult::Normal;

    """]

    [[completions]]
    completion_label = "NotNull"
    completion_label_path = "(use core::nullable::FromNullableResult::NotNull)"
    text_edits = ["""
    use core::nullable::FromNullableResult::NotNull;

    """]

    [[completions]]
    completion_label = "Null"
    completion_label_path = "(use core::nullable::FromNullableResult::Null)"
    text_edits = ["""
    use core::nullable::FromNullableResult::Null;

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
    completion_label = "ORDER"
    completion_label_path = "(use core::ec::stark_curve::ORDER)"
    text_edits = ["""
    use core::ec::stark_curve::ORDER;

    """]

    [[completions]]
    completion_label = "Ok"
    completion_label_path = "(use PanicResult::Ok)"
    text_edits = ["""
    use PanicResult::Ok;

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
    completion_label = "Some"
    completion_label_path = "(use core::internal::OptionRev::Some)"
    text_edits = ["""
    use core::internal::OptionRev::Some;

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
    completion_label = "True"
    completion_label_path = "(use bool::True)"
    text_edits = ["""
    use bool::True;

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
    completion_label = "VALIDATED"
    completion_label_path = "(use starknet::VALIDATED)"
    text_edits = ["""
    use starknet::VALIDATED;

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
    completion_label = "blake2s_compress(...)"
    completion_label_path = "(use core::blake::blake2s_compress)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_compress(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_compress;

    """]

    [[completions]]
    completion_label = "blake2s_finalize(...)"
    completion_label_path = "(use core::blake::blake2s_finalize)"
    completion_label_type_info = "fn(state: Box<[u32; 8]>, byte_count: u32, msg: Box<[u32; 16]>) -> Box<[u32; 8]> nopanic"
    insert_text = "blake2s_finalize(${1:state}, ${2:byte_count}, ${3:msg})"
    text_edits = ["""
    use core::blake::blake2s_finalize;

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
    completion_label = "bounded_int_add(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_add)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_add;

    """]

    [[completions]]
    completion_label = "bounded_int_constrain(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_constrain)"
    completion_label_type_info = "fn(value: T) -> Result<H::LowT, H::HighT> nopanic"
    insert_text = "bounded_int_constrain(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_constrain;

    """]

    [[completions]]
    completion_label = "bounded_int_div_rem(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_div_rem)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: NonZero<Rhs>) -> (H::DivT, H::RemT) nopanic"
    insert_text = "bounded_int_div_rem(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_div_rem;

    """]

    [[completions]]
    completion_label = "bounded_int_is_zero(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_is_zero)"
    completion_label_type_info = "fn(value: T) -> crate::zeroable::IsZeroResult<T> nopanic"
    insert_text = "bounded_int_is_zero(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_is_zero;

    """]

    [[completions]]
    completion_label = "bounded_int_mul(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_mul)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_mul;

    """]

    [[completions]]
    completion_label = "bounded_int_sub(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_sub)"
    completion_label_type_info = "fn(lhs: Lhs, rhs: Rhs) -> H::Result nopanic"
    insert_text = "bounded_int_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_sub;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_max(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_max)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_max(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_max;

    """]

    [[completions]]
    completion_label = "bounded_int_trim_min(...)"
    completion_label_path = "(use core::internal::bounded_int::bounded_int_trim_min)"
    completion_label_type_info = "fn(value: T) -> super::OptionRev<H::Target> nopanic"
    insert_text = "bounded_int_trim_min(${1:value})"
    text_edits = ["""
    use core::internal::bounded_int::bounded_int_trim_min;

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
    completion_label = "cairo_keccak(...)"
    completion_label_path = "(use core::keccak::cairo_keccak)"
    completion_label_type_info = "fn(ref input: Array<u64>, last_input_word: u64, last_input_num_bytes: u32) -> u256"
    insert_text = "cairo_keccak(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::keccak::cairo_keccak;

    """]

    [[completions]]
    completion_label = "call_contract_syscall(...)"
    completion_label_path = "(use starknet::syscalls::call_contract_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "call_contract_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::call_contract_syscall;

    """]

    [[completions]]
    completion_label = "cheatcode(...)"
    completion_label_path = "(use starknet::testing::cheatcode)"
    completion_label_type_info = "fn(input: Span<felt252>) -> Span<felt252> nopanic"
    insert_text = "cheatcode(${1:input})"
    text_edits = ["""
    use starknet::testing::cheatcode;

    """]

    [[completions]]
    completion_label = "check_ecdsa_signature(...)"
    completion_label_path = "(use core::ecdsa::check_ecdsa_signature)"
    completion_label_type_info = "fn(message_hash: felt252, public_key: felt252, signature_r: felt252, signature_s: felt252) -> bool"
    insert_text = "check_ecdsa_signature(${1:message_hash}, ${2:public_key}, ${3:signature_r}, ${4:signature_s})"
    text_edits = ["""
    use core::ecdsa::check_ecdsa_signature;

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
    completion_label = "circuit_add(...)"
    completion_label_path = "(use core::circuit::circuit_add)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<AddModGate<Lhs, Rhs>>"
    insert_text = "circuit_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_add;

    """]

    [[completions]]
    completion_label = "circuit_inverse(...)"
    completion_label_path = "(use core::circuit::circuit_inverse)"
    completion_label_type_info = "fn(input: CircuitElement<Input>) -> CircuitElement<InverseGate<Input>>"
    insert_text = "circuit_inverse(${1:input})"
    text_edits = ["""
    use core::circuit::circuit_inverse;

    """]

    [[completions]]
    completion_label = "circuit_mul(...)"
    completion_label_path = "(use core::circuit::circuit_mul)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<MulModGate<Lhs, Rhs>>"
    insert_text = "circuit_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_mul;

    """]

    [[completions]]
    completion_label = "circuit_sub(...)"
    completion_label_path = "(use core::circuit::circuit_sub)"
    completion_label_type_info = "fn(lhs: CircuitElement<Lhs>, rhs: CircuitElement<Rhs>) -> CircuitElement<SubModGate<Lhs, Rhs>>"
    insert_text = "circuit_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::circuit::circuit_sub;

    """]

    [[completions]]
    completion_label = "class_hash"
    completion_label_path = "(use starknet::class_hash)"
    text_edits = ["""
    use starknet::class_hash;

    """]

    [[completions]]
    completion_label = "class_hash_const(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_const)"
    completion_label_type_info = "fn() -> ClassHash nopanic"
    insert_text = "class_hash_const()"
    text_edits = ["""
    use starknet::class_hash::class_hash_const;

    """]

    [[completions]]
    completion_label = "class_hash_to_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_to_felt252)"
    completion_label_type_info = "fn(address: ClassHash) -> felt252 nopanic"
    insert_text = "class_hash_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_to_felt252;

    """]

    [[completions]]
    completion_label = "class_hash_try_from_felt252(...)"
    completion_label_path = "(use starknet::class_hash::class_hash_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ClassHash> nopanic"
    insert_text = "class_hash_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::class_hash::class_hash_try_from_felt252;

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
    completion_label = "compute_keccak_byte_array(...)"
    completion_label_path = "(use core::keccak::compute_keccak_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> u256"
    insert_text = "compute_keccak_byte_array(${1:arr})"
    text_edits = ["""
    use core::keccak::compute_keccak_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_byte_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_byte_array)"
    completion_label_type_info = "fn(arr: @ByteArray) -> [u32; 8]"
    insert_text = "compute_sha256_byte_array(${1:arr})"
    text_edits = ["""
    use core::sha256::compute_sha256_byte_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: u32) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array;

    """]

    [[completions]]
    completion_label = "compute_sha256_u32_array_safe(...)"
    completion_label_path = "(use core::sha256::compute_sha256_u32_array_safe)"
    completion_label_type_info = "fn(input: Array<u32>, last_input_word: u32, last_input_num_bytes: BoundedInt<0, 3>) -> [u32; 8]"
    insert_text = "compute_sha256_u32_array_safe(${1:input}, ${2:last_input_word}, ${3:last_input_num_bytes})"
    text_edits = ["""
    use core::sha256::compute_sha256_u32_array_safe;

    """]

    [[completions]]
    completion_label = "contract_address"
    completion_label_path = "(use starknet::contract_address)"
    text_edits = ["""
    use starknet::contract_address;

    """]

    [[completions]]
    completion_label = "contract_address_const(...)"
    completion_label_path = "(use starknet::contract_address_const)"
    completion_label_type_info = "fn() -> ContractAddress nopanic"
    insert_text = "contract_address_const()"
    text_edits = ["""
    use starknet::contract_address_const;

    """]

    [[completions]]
    completion_label = "contract_address_to_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_to_felt252)"
    completion_label_type_info = "fn(address: ContractAddress) -> felt252 nopanic"
    insert_text = "contract_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_to_felt252;

    """]

    [[completions]]
    completion_label = "contract_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::contract_address::contract_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<ContractAddress> nopanic"
    insert_text = "contract_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::contract_address::contract_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "debug"
    completion_label_path = "(use core::debug)"
    text_edits = ["""
    use core::debug;

    """]

    [[completions]]
    completion_label = "deploy_syscall(...)"
    completion_label_path = "(use starknet::syscalls::deploy_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, contract_address_salt: felt252, calldata: Span<felt252>, deploy_from_zero: bool) -> Result<(ContractAddress, Span<felt252>), Array<felt252>> nopanic"
    insert_text = "deploy_syscall(${1:class_hash}, ${2:contract_address_salt}, ${3:calldata}, ${4:deploy_from_zero})"
    text_edits = ["""
    use starknet::syscalls::deploy_syscall;

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
    completion_label = "downcast(...)"
    completion_label_path = "(use core::internal::bounded_int::downcast)"
    completion_label_type_info = "fn(x: FromType) -> Option<ToType> nopanic"
    insert_text = "downcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::downcast;

    """]

    [[completions]]
    completion_label = "ec"
    completion_label_path = "(use core::ec)"
    text_edits = ["""
    use core::ec;

    """]

    [[completions]]
    completion_label = "ec_point_unwrap(...)"
    completion_label_path = "(use core::ec::ec_point_unwrap)"
    completion_label_type_info = "fn(p: NonZero<EcPoint>) -> (felt252, felt252) nopanic"
    insert_text = "ec_point_unwrap(${1:p})"
    text_edits = ["""
    use core::ec::ec_point_unwrap;

    """]

    [[completions]]
    completion_label = "ecdsa"
    completion_label_path = "(use core::ecdsa)"
    text_edits = ["""
    use core::ecdsa;

    """]

    [[completions]]
    completion_label = "egcd(...)"
    completion_label_path = "(use core::math::egcd)"
    completion_label_type_info = "fn(a: NonZero<T>, b: NonZero<T>) -> (T, T, T, bool)"
    insert_text = "egcd(${1:a}, ${2:b})"
    text_edits = ["""
    use core::math::egcd;

    """]

    [[completions]]
    completion_label = "emit_event_syscall(...)"
    completion_label_path = "(use starknet::syscalls::emit_event_syscall)"
    completion_label_type_info = "fn(keys: Span<felt252>, data: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "emit_event_syscall(${1:keys}, ${2:data})"
    text_edits = ["""
    use starknet::syscalls::emit_event_syscall;

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
    completion_label = "felt252_div(...)"
    completion_label_path = "(use core::felt252_div)"
    completion_label_type_info = "fn(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic"
    insert_text = "felt252_div(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::felt252_div;

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
    completion_label = "gas_reserve_create(...)"
    completion_label_path = "(use core::gas::gas_reserve_create)"
    completion_label_type_info = "fn(amount: u128) -> Option<GasReserve> nopanic"
    insert_text = "gas_reserve_create(${1:amount})"
    text_edits = ["""
    use core::gas::gas_reserve_create;

    """]

    [[completions]]
    completion_label = "gas_reserve_utilize(...)"
    completion_label_path = "(use core::gas::gas_reserve_utilize)"
    completion_label_type_info = "fn(reserve: GasReserve) -> () nopanic"
    insert_text = "gas_reserve_utilize(${1:reserve})"
    text_edits = ["""
    use core::gas::gas_reserve_utilize;

    """]

    [[completions]]
    completion_label = "get"
    completion_label_path = "(use core::ops::get)"
    text_edits = ["""
    use core::ops::get;

    """]

    [[completions]]
    completion_label = "get_available_gas(...)"
    completion_label_path = "(use core::testing::get_available_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_available_gas()"
    text_edits = ["""
    use core::testing::get_available_gas;

    """]

    [[completions]]
    completion_label = "get_block_hash_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_block_hash_syscall)"
    completion_label_type_info = "fn(block_number: u64) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "get_block_hash_syscall(${1:block_number})"
    text_edits = ["""
    use starknet::syscalls::get_block_hash_syscall;

    """]

    [[completions]]
    completion_label = "get_block_info(...)"
    completion_label_path = "(use starknet::get_block_info)"
    completion_label_type_info = "fn() -> Box<BlockInfo>"
    insert_text = "get_block_info()"
    text_edits = ["""
    use starknet::get_block_info;

    """]

    [[completions]]
    completion_label = "get_block_number(...)"
    completion_label_path = "(use starknet::get_block_number)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_number()"
    text_edits = ["""
    use starknet::get_block_number;

    """]

    [[completions]]
    completion_label = "get_block_timestamp(...)"
    completion_label_path = "(use starknet::get_block_timestamp)"
    completion_label_type_info = "fn() -> u64"
    insert_text = "get_block_timestamp()"
    text_edits = ["""
    use starknet::get_block_timestamp;

    """]

    [[completions]]
    completion_label = "get_builtin_costs(...)"
    completion_label_path = "(use core::gas::get_builtin_costs)"
    completion_label_type_info = "fn() -> BuiltinCosts nopanic"
    insert_text = "get_builtin_costs()"
    text_edits = ["""
    use core::gas::get_builtin_costs;

    """]

    [[completions]]
    completion_label = "get_caller_address(...)"
    completion_label_path = "(use starknet::get_caller_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_caller_address()"
    text_edits = ["""
    use starknet::get_caller_address;

    """]

    [[completions]]
    completion_label = "get_class_hash_at_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_class_hash_at_syscall)"
    completion_label_type_info = "fn(contract_address: ContractAddress) -> Result<ClassHash, Array<felt252>> nopanic"
    insert_text = "get_class_hash_at_syscall(${1:contract_address})"
    text_edits = ["""
    use starknet::syscalls::get_class_hash_at_syscall;

    """]

    [[completions]]
    completion_label = "get_contract_address(...)"
    completion_label_path = "(use starknet::get_contract_address)"
    completion_label_type_info = "fn() -> ContractAddress"
    insert_text = "get_contract_address()"
    text_edits = ["""
    use starknet::get_contract_address;

    """]

    [[completions]]
    completion_label = "get_execution_info(...)"
    completion_label_path = "(use starknet::get_execution_info)"
    completion_label_type_info = "fn() -> Box<starknet::ExecutionInfo>"
    insert_text = "get_execution_info()"
    text_edits = ["""
    use starknet::get_execution_info;

    """]

    [[completions]]
    completion_label = "get_execution_info_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v2_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v2_syscall)"
    completion_label_type_info = "fn() -> Result<Box<starknet::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v2_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v2_syscall;

    """]

    [[completions]]
    completion_label = "get_execution_info_v3_syscall(...)"
    completion_label_path = "(use starknet::syscalls::get_execution_info_v3_syscall)"
    completion_label_type_info = "fn() -> Result<Box<super::info::v3::ExecutionInfo>, Array<felt252>> nopanic"
    insert_text = "get_execution_info_v3_syscall()"
    text_edits = ["""
    use starknet::syscalls::get_execution_info_v3_syscall;

    """]

    [[completions]]
    completion_label = "get_tx_info(...)"
    completion_label_path = "(use starknet::get_tx_info)"
    completion_label_type_info = "fn() -> Box<starknet::TxInfo>"
    insert_text = "get_tx_info()"
    text_edits = ["""
    use starknet::get_tx_info;

    """]

    [[completions]]
    completion_label = "get_unspent_gas(...)"
    completion_label_path = "(use core::testing::get_unspent_gas)"
    completion_label_type_info = "fn() -> u128 nopanic"
    insert_text = "get_unspent_gas()"
    text_edits = ["""
    use core::testing::get_unspent_gas;

    """]

    [[completions]]
    completion_label = "hades_permutation(...)"
    completion_label_path = "(use core::poseidon::hades_permutation)"
    completion_label_type_info = "fn(s0: felt252, s1: felt252, s2: felt252) -> (felt252, felt252, felt252) nopanic"
    insert_text = "hades_permutation(${1:s0}, ${2:s1}, ${3:s2})"
    text_edits = ["""
    use core::poseidon::hades_permutation;

    """]

    [[completions]]
    completion_label = "hash"
    completion_label_path = "(use core::hash)"
    text_edits = ["""
    use core::hash;

    """]

    [[completions]]
    completion_label = "i128_diff(...)"
    completion_label_path = "(use core::integer::i128_diff)"
    completion_label_type_info = "fn(lhs: i128, rhs: i128) -> Result<u128, u128> nopanic"
    insert_text = "i128_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i128_diff;

    """]

    [[completions]]
    completion_label = "i16_diff(...)"
    completion_label_path = "(use core::integer::i16_diff)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> Result<u16, u16> nopanic"
    insert_text = "i16_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_diff;

    """]

    [[completions]]
    completion_label = "i16_wide_mul(...)"
    completion_label_path = "(use core::integer::i16_wide_mul)"
    completion_label_type_info = "fn(lhs: i16, rhs: i16) -> i32 nopanic"
    insert_text = "i16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i16_wide_mul;

    """]

    [[completions]]
    completion_label = "i32_diff(...)"
    completion_label_path = "(use core::integer::i32_diff)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> Result<u32, u32> nopanic"
    insert_text = "i32_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_diff;

    """]

    [[completions]]
    completion_label = "i32_wide_mul(...)"
    completion_label_path = "(use core::integer::i32_wide_mul)"
    completion_label_type_info = "fn(lhs: i32, rhs: i32) -> i64 nopanic"
    insert_text = "i32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i32_wide_mul;

    """]

    [[completions]]
    completion_label = "i64_diff(...)"
    completion_label_path = "(use core::integer::i64_diff)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> Result<u64, u64> nopanic"
    insert_text = "i64_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_diff;

    """]

    [[completions]]
    completion_label = "i64_wide_mul(...)"
    completion_label_path = "(use core::integer::i64_wide_mul)"
    completion_label_type_info = "fn(lhs: i64, rhs: i64) -> i128 nopanic"
    insert_text = "i64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i64_wide_mul;

    """]

    [[completions]]
    completion_label = "i8_diff(...)"
    completion_label_path = "(use core::integer::i8_diff)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> Result<u8, u8> nopanic"
    insert_text = "i8_diff(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_diff;

    """]

    [[completions]]
    completion_label = "i8_wide_mul(...)"
    completion_label_path = "(use core::integer::i8_wide_mul)"
    completion_label_type_info = "fn(lhs: i8, rhs: i8) -> i16 nopanic"
    insert_text = "i8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::i8_wide_mul;

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
    completion_label = "inv_mod(...)"
    completion_label_path = "(use core::math::inv_mod)"
    completion_label_type_info = "fn(a: NonZero<T>, n: NonZero<T>) -> Option<T>"
    insert_text = "inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::inv_mod;

    """]

    [[completions]]
    completion_label = "is_eth_signature_valid(...)"
    completion_label_path = "(use starknet::eth_signature::is_eth_signature_valid)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> Result<(), felt252>"
    insert_text = "is_eth_signature_valid(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::is_eth_signature_valid;

    """]

    [[completions]]
    completion_label = "is_signature_entry_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_entry_valid)"
    completion_label_type_info = "fn(value: u256) -> bool"
    insert_text = "is_signature_entry_valid(${1:value})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_entry_valid;

    """]

    [[completions]]
    completion_label = "is_signature_s_valid(...)"
    completion_label_path = "(use starknet::secp256_trait::is_signature_s_valid)"
    completion_label_type_info = "fn(s: u256) -> bool"
    insert_text = "is_signature_s_valid(${1:s})"
    text_edits = ["""
    use starknet::secp256_trait::is_signature_s_valid;

    """]

    [[completions]]
    completion_label = "is_valid_signature(...)"
    completion_label_path = "(use starknet::secp256_trait::is_valid_signature)"
    completion_label_type_info = "fn(msg_hash: u256, r: u256, s: u256, public_key: Secp256Point) -> bool"
    insert_text = "is_valid_signature(${1:msg_hash}, ${2:r}, ${3:s}, ${4:public_key})"
    text_edits = ["""
    use starknet::secp256_trait::is_valid_signature;

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
    completion_label = "keccak_syscall(...)"
    completion_label_path = "(use starknet::syscalls::keccak_syscall)"
    completion_label_type_info = "fn(input: Span<u64>) -> Result<u256, Array<felt252>> nopanic"
    insert_text = "keccak_syscall(${1:input})"
    text_edits = ["""
    use starknet::syscalls::keccak_syscall;

    """]

    [[completions]]
    completion_label = "keccak_u256s_be_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_be_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_be_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_be_inputs;

    """]

    [[completions]]
    completion_label = "keccak_u256s_le_inputs(...)"
    completion_label_path = "(use core::keccak::keccak_u256s_le_inputs)"
    completion_label_type_info = "fn(input: Span<u256>) -> u256"
    insert_text = "keccak_u256s_le_inputs(${1:input})"
    text_edits = ["""
    use core::keccak::keccak_u256s_le_inputs;

    """]

    [[completions]]
    completion_label = "library_call_syscall(...)"
    completion_label_path = "(use starknet::syscalls::library_call_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash, function_selector: felt252, calldata: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "library_call_syscall(${1:class_hash}, ${2:function_selector}, ${3:calldata})"
    text_edits = ["""
    use starknet::syscalls::library_call_syscall;

    """]

    [[completions]]
    completion_label = "m31"
    completion_label_path = "(use core::qm31::m31)"
    text_edits = ["""
    use core::qm31::m31;

    """]

    [[completions]]
    completion_label = "m31_add(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_add)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_add(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_add;

    """]

    [[completions]]
    completion_label = "m31_div(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_div)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: NonZero<crate::internal::bounded_int::BoundedInt<0, 2147483646>>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_div(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_div;

    """]

    [[completions]]
    completion_label = "m31_mul(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_mul)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_mul;

    """]

    [[completions]]
    completion_label = "m31_ops"
    completion_label_path = "(use core::qm31::m31_ops)"
    text_edits = ["""
    use core::qm31::m31_ops;

    """]

    [[completions]]
    completion_label = "m31_sub(...)"
    completion_label_path = "(use core::qm31::m31_ops::m31_sub)"
    completion_label_type_info = "fn(a: crate::internal::bounded_int::BoundedInt<0, 2147483646>, b: crate::internal::bounded_int::BoundedInt<0, 2147483646>) -> crate::internal::bounded_int::BoundedInt<0, 2147483646> nopanic"
    insert_text = "m31_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::qm31::m31_ops::m31_sub;

    """]

    [[completions]]
    completion_label = "match_nullable(...)"
    completion_label_path = "(use core::nullable::match_nullable)"
    completion_label_type_info = "fn(value: Nullable<T>) -> FromNullableResult<T> nopanic"
    insert_text = "match_nullable(${1:value})"
    text_edits = ["""
    use core::nullable::match_nullable;

    """]

    [[completions]]
    completion_label = "math"
    completion_label_path = "(use core::math)"
    text_edits = ["""
    use core::math;

    """]

    [[completions]]
    completion_label = "max(...)"
    completion_label_path = "(use core::cmp::max)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "max(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::max;

    """]

    [[completions]]
    completion_label = "meta_tx_v0_syscall(...)"
    completion_label_path = "(use starknet::syscalls::meta_tx_v0_syscall)"
    completion_label_type_info = "fn(address: ContractAddress, entry_point_selector: felt252, calldata: Span<felt252>, signature: Span<felt252>) -> Result<Span<felt252>, Array<felt252>> nopanic"
    insert_text = "meta_tx_v0_syscall(${1:address}, ${2:entry_point_selector}, ${3:calldata}, ${4:signature})"
    text_edits = ["""
    use starknet::syscalls::meta_tx_v0_syscall;

    """]

    [[completions]]
    completion_label = "metaprogramming"
    completion_label_path = "(use core::metaprogramming)"
    text_edits = ["""
    use core::metaprogramming;

    """]

    [[completions]]
    completion_label = "min(...)"
    completion_label_path = "(use core::cmp::min)"
    completion_label_type_info = "fn(a: T, b: T) -> T"
    insert_text = "min(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::min;

    """]

    [[completions]]
    completion_label = "minmax(...)"
    completion_label_path = "(use core::cmp::minmax)"
    completion_label_type_info = "fn(a: T, b: T) -> (T, T)"
    insert_text = "minmax(${1:a}, ${2:b})"
    text_edits = ["""
    use core::cmp::minmax;

    """]

    [[completions]]
    completion_label = "never"
    completion_label_path = "(use core::never)"
    text_edits = ["""
    use core::never;

    """]

    [[completions]]
    completion_label = "null(...)"
    completion_label_path = "(use core::nullable::null)"
    completion_label_type_info = "fn() -> Nullable<T> nopanic"
    insert_text = "null()"
    text_edits = ["""
    use core::nullable::null;

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
    completion_label = "panic_with_byte_array(...)"
    completion_label_path = "(use core::panics::panic_with_byte_array)"
    completion_label_type_info = "fn(err: @ByteArray) -> crate::never"
    insert_text = "panic_with_byte_array(${1:err})"
    text_edits = ["""
    use core::panics::panic_with_byte_array;

    """]

    [[completions]]
    completion_label = "panic_with_const_felt252(...)"
    completion_label_path = "(use core::panic_with_const_felt252)"
    completion_label_type_info = "fn() -> never"
    insert_text = "panic_with_const_felt252()"
    text_edits = ["""
    use core::panic_with_const_felt252;

    """]

    [[completions]]
    completion_label = "panic_with_felt252(...)"
    completion_label_path = "(use core::panic_with_felt252)"
    completion_label_type_info = "fn(err_code: felt252) -> never"
    insert_text = "panic_with_felt252(${1:err_code})"
    text_edits = ["""
    use core::panic_with_felt252;

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
    completion_label = "pedersen(...)"
    completion_label_path = "(use core::pedersen::pedersen)"
    completion_label_type_info = "fn(a: felt252, b: felt252) -> felt252 nopanic"
    insert_text = "pedersen(${1:a}, ${2:b})"
    text_edits = ["""
    use core::pedersen::pedersen;

    """]

    [[completions]]
    completion_label = "pop_l2_to_l1_message(...)"
    completion_label_path = "(use starknet::testing::pop_l2_to_l1_message)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(felt252, Span<felt252>)>"
    insert_text = "pop_l2_to_l1_message(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_l2_to_l1_message;

    """]

    [[completions]]
    completion_label = "pop_log(...)"
    completion_label_path = "(use starknet::testing::pop_log)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<T>"
    insert_text = "pop_log(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log;

    """]

    [[completions]]
    completion_label = "pop_log_raw(...)"
    completion_label_path = "(use starknet::testing::pop_log_raw)"
    completion_label_type_info = "fn(address: ContractAddress) -> Option<(Span<felt252>, Span<felt252>)>"
    insert_text = "pop_log_raw(${1:address})"
    text_edits = ["""
    use starknet::testing::pop_log_raw;

    """]

    [[completions]]
    completion_label = "poseidon"
    completion_label_path = "(use core::poseidon)"
    text_edits = ["""
    use core::poseidon;

    """]

    [[completions]]
    completion_label = "poseidon_hash_span(...)"
    completion_label_path = "(use core::poseidon::poseidon_hash_span)"
    completion_label_type_info = "fn(span: Span<felt252>) -> felt252"
    insert_text = "poseidon_hash_span(${1:span})"
    text_edits = ["""
    use core::poseidon::poseidon_hash_span;

    """]

    [[completions]]
    completion_label = "pow"
    completion_label_path = "(use core::num::traits::ops::pow)"
    text_edits = ["""
    use core::num::traits::ops::pow;

    """]

    [[completions]]
    completion_label = "print_byte_array_as_string(...)"
    completion_label_path = "(use core::debug::print_byte_array_as_string)"
    completion_label_type_info = "fn(self: @ByteArray) -> ()"
    insert_text = "print_byte_array_as_string()"
    text_edits = ["""
    use core::debug::print_byte_array_as_string;

    """]

    [[completions]]
    completion_label = "public_key_point_to_eth_address(...)"
    completion_label_path = "(use starknet::eth_signature::public_key_point_to_eth_address)"
    completion_label_type_info = "fn(public_key_point: Secp256Point) -> EthAddress"
    insert_text = "public_key_point_to_eth_address(${1:public_key_point})"
    text_edits = ["""
    use starknet::eth_signature::public_key_point_to_eth_address;

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
    completion_label = "qm31_const(...)"
    completion_label_path = "(use core::qm31::qm31_const)"
    completion_label_type_info = "fn() -> qm31 nopanic"
    insert_text = "qm31_const()"
    text_edits = ["""
    use core::qm31::qm31_const;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use core::ecdsa::recover_public_key)"
    completion_label_type_info = "fn(message_hash: felt252, signature_r: felt252, signature_s: felt252, y_parity: bool) -> Option<felt252>"
    insert_text = "recover_public_key(${1:message_hash}, ${2:signature_r}, ${3:signature_s}, ${4:y_parity})"
    text_edits = ["""
    use core::ecdsa::recover_public_key;

    """]

    [[completions]]
    completion_label = "recover_public_key(...)"
    completion_label_path = "(use starknet::secp256_trait::recover_public_key)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature) -> Option<Secp256Point>"
    insert_text = "recover_public_key(${1:msg_hash}, ${2:signature})"
    text_edits = ["""
    use starknet::secp256_trait::recover_public_key;

    """]

    [[completions]]
    completion_label = "redeposit_gas(...)"
    completion_label_path = "(use core::gas::redeposit_gas)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "redeposit_gas()"
    text_edits = ["""
    use core::gas::redeposit_gas;

    """]

    [[completions]]
    completion_label = "replace_class_syscall(...)"
    completion_label_path = "(use starknet::syscalls::replace_class_syscall)"
    completion_label_type_info = "fn(class_hash: ClassHash) -> Result<(), Array<felt252>> nopanic"
    insert_text = "replace_class_syscall(${1:class_hash})"
    text_edits = ["""
    use starknet::syscalls::replace_class_syscall;

    """]

    [[completions]]
    completion_label = "require_implicit(...)"
    completion_label_path = "(use core::internal::require_implicit)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "require_implicit()"
    text_edits = ["""
    use core::internal::require_implicit;

    """]

    [[completions]]
    completion_label = "result"
    completion_label_path = "(use core::result)"
    text_edits = ["""
    use core::result;

    """]

    [[completions]]
    completion_label = "revoke_ap_tracking(...)"
    completion_label_path = "(use core::internal::revoke_ap_tracking)"
    completion_label_type_info = "fn() -> () nopanic"
    insert_text = "revoke_ap_tracking()"
    text_edits = ["""
    use core::internal::revoke_ap_tracking;

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
    completion_label = "send_message_to_l1_syscall(...)"
    completion_label_path = "(use starknet::syscalls::send_message_to_l1_syscall)"
    completion_label_type_info = "fn(to_address: felt252, payload: Span<felt252>) -> Result<(), Array<felt252>> nopanic"
    insert_text = "send_message_to_l1_syscall(${1:to_address}, ${2:payload})"
    text_edits = ["""
    use starknet::syscalls::send_message_to_l1_syscall;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "set_account_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_account_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_account_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_account_contract_address;

    """]

    [[completions]]
    completion_label = "set_block_hash(...)"
    completion_label_path = "(use starknet::testing::set_block_hash)"
    completion_label_type_info = "fn(block_number: u64, value: felt252) -> ()"
    insert_text = "set_block_hash(${1:block_number}, ${2:value})"
    text_edits = ["""
    use starknet::testing::set_block_hash;

    """]

    [[completions]]
    completion_label = "set_block_number(...)"
    completion_label_path = "(use starknet::testing::set_block_number)"
    completion_label_type_info = "fn(block_number: u64) -> ()"
    insert_text = "set_block_number(${1:block_number})"
    text_edits = ["""
    use starknet::testing::set_block_number;

    """]

    [[completions]]
    completion_label = "set_block_timestamp(...)"
    completion_label_path = "(use starknet::testing::set_block_timestamp)"
    completion_label_type_info = "fn(block_timestamp: u64) -> ()"
    insert_text = "set_block_timestamp(${1:block_timestamp})"
    text_edits = ["""
    use starknet::testing::set_block_timestamp;

    """]

    [[completions]]
    completion_label = "set_caller_address(...)"
    completion_label_path = "(use starknet::testing::set_caller_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_caller_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_caller_address;

    """]

    [[completions]]
    completion_label = "set_chain_id(...)"
    completion_label_path = "(use starknet::testing::set_chain_id)"
    completion_label_type_info = "fn(chain_id: felt252) -> ()"
    insert_text = "set_chain_id(${1:chain_id})"
    text_edits = ["""
    use starknet::testing::set_chain_id;

    """]

    [[completions]]
    completion_label = "set_contract_address(...)"
    completion_label_path = "(use starknet::testing::set_contract_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_contract_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_contract_address;

    """]

    [[completions]]
    completion_label = "set_max_fee(...)"
    completion_label_path = "(use starknet::testing::set_max_fee)"
    completion_label_type_info = "fn(fee: u128) -> ()"
    insert_text = "set_max_fee(${1:fee})"
    text_edits = ["""
    use starknet::testing::set_max_fee;

    """]

    [[completions]]
    completion_label = "set_nonce(...)"
    completion_label_path = "(use starknet::testing::set_nonce)"
    completion_label_type_info = "fn(nonce: felt252) -> ()"
    insert_text = "set_nonce(${1:nonce})"
    text_edits = ["""
    use starknet::testing::set_nonce;

    """]

    [[completions]]
    completion_label = "set_sequencer_address(...)"
    completion_label_path = "(use starknet::testing::set_sequencer_address)"
    completion_label_type_info = "fn(address: ContractAddress) -> ()"
    insert_text = "set_sequencer_address(${1:address})"
    text_edits = ["""
    use starknet::testing::set_sequencer_address;

    """]

    [[completions]]
    completion_label = "set_signature(...)"
    completion_label_path = "(use starknet::testing::set_signature)"
    completion_label_type_info = "fn(signature: Span<felt252>) -> ()"
    insert_text = "set_signature(${1:signature})"
    text_edits = ["""
    use starknet::testing::set_signature;

    """]

    [[completions]]
    completion_label = "set_transaction_hash(...)"
    completion_label_path = "(use starknet::testing::set_transaction_hash)"
    completion_label_type_info = "fn(hash: felt252) -> ()"
    insert_text = "set_transaction_hash(${1:hash})"
    text_edits = ["""
    use starknet::testing::set_transaction_hash;

    """]

    [[completions]]
    completion_label = "set_version(...)"
    completion_label_path = "(use starknet::testing::set_version)"
    completion_label_type_info = "fn(version: felt252) -> ()"
    insert_text = "set_version(${1:version})"
    text_edits = ["""
    use starknet::testing::set_version;

    """]

    [[completions]]
    completion_label = "sha256"
    completion_label_path = "(use core::sha256)"
    text_edits = ["""
    use core::sha256;

    """]

    [[completions]]
    completion_label = "sha256_process_block_syscall(...)"
    completion_label_path = "(use starknet::syscalls::sha256_process_block_syscall)"
    completion_label_type_info = "fn(state: crate::sha256::Sha256StateHandle, input: Box<[u32; 16]>) -> Result<crate::sha256::Sha256StateHandle, Array<felt252>> nopanic"
    insert_text = "sha256_process_block_syscall(${1:state}, ${2:input})"
    text_edits = ["""
    use starknet::syscalls::sha256_process_block_syscall;

    """]

    [[completions]]
    completion_label = "signature_from_vrs(...)"
    completion_label_path = "(use starknet::secp256_trait::signature_from_vrs)"
    completion_label_type_info = "fn(v: u32, r: u256, s: u256) -> Signature"
    insert_text = "signature_from_vrs(${1:v}, ${2:r}, ${3:s})"
    text_edits = ["""
    use starknet::secp256_trait::signature_from_vrs;

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
    completion_label = "storage_address_from_base(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base)"
    completion_label_type_info = "fn(base: StorageBaseAddress) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base(${1:base})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base;

    """]

    [[completions]]
    completion_label = "storage_address_from_base_and_offset(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_from_base_and_offset)"
    completion_label_type_info = "fn(base: StorageBaseAddress, offset: u8) -> StorageAddress nopanic"
    insert_text = "storage_address_from_base_and_offset(${1:base}, ${2:offset})"
    text_edits = ["""
    use starknet::storage_access::storage_address_from_base_and_offset;

    """]

    [[completions]]
    completion_label = "storage_address_to_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_to_felt252)"
    completion_label_type_info = "fn(address: StorageAddress) -> felt252 nopanic"
    insert_text = "storage_address_to_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_to_felt252;

    """]

    [[completions]]
    completion_label = "storage_address_try_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_address_try_from_felt252)"
    completion_label_type_info = "fn(address: felt252) -> Option<StorageAddress> nopanic"
    insert_text = "storage_address_try_from_felt252(${1:address})"
    text_edits = ["""
    use starknet::storage_access::storage_address_try_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_base_address_const(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_const)"
    completion_label_type_info = "fn() -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_const()"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_const;

    """]

    [[completions]]
    completion_label = "storage_base_address_from_felt252(...)"
    completion_label_path = "(use starknet::storage_access::storage_base_address_from_felt252)"
    completion_label_type_info = "fn(addr: felt252) -> StorageBaseAddress nopanic"
    insert_text = "storage_base_address_from_felt252(${1:addr})"
    text_edits = ["""
    use starknet::storage_access::storage_base_address_from_felt252;

    """]

    [[completions]]
    completion_label = "storage_read_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_read_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress) -> Result<felt252, Array<felt252>> nopanic"
    insert_text = "storage_read_syscall(${1:address_domain}, ${2:address})"
    text_edits = ["""
    use starknet::syscalls::storage_read_syscall;

    """]

    [[completions]]
    completion_label = "storage_write_syscall(...)"
    completion_label_path = "(use starknet::syscalls::storage_write_syscall)"
    completion_label_type_info = "fn(address_domain: u32, address: StorageAddress, value: felt252) -> Result<(), Array<felt252>> nopanic"
    insert_text = "storage_write_syscall(${1:address_domain}, ${2:address}, ${3:value})"
    text_edits = ["""
    use starknet::syscalls::storage_write_syscall;

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
    completion_label = "u128_byte_reverse(...)"
    completion_label_path = "(use core::integer::u128_byte_reverse)"
    completion_label_type_info = "fn(input: u128) -> u128 nopanic"
    insert_text = "u128_byte_reverse(${1:input})"
    text_edits = ["""
    use core::integer::u128_byte_reverse;

    """]

    [[completions]]
    completion_label = "u128_overflowing_add(...)"
    completion_label_path = "(use core::integer::u128_overflowing_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_add;

    """]

    [[completions]]
    completion_label = "u128_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u128_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> (u128, bool) nopanic"
    insert_text = "u128_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u128_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u128_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> Result<u128, u128> nopanic"
    insert_text = "u128_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u128_safe_divmod(...)"
    completion_label_path = "(use core::integer::u128_safe_divmod)"
    completion_label_type_info = "fn(lhs: u128, rhs: NonZero<u128>) -> (u128, u128) nopanic"
    insert_text = "u128_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_safe_divmod;

    """]

    [[completions]]
    completion_label = "u128_sqrt(...)"
    completion_label_path = "(use core::integer::u128_sqrt)"
    completion_label_type_info = "fn(value: u128) -> u64 nopanic"
    insert_text = "u128_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u128_sqrt;

    """]

    [[completions]]
    completion_label = "u128_wide_mul(...)"
    completion_label_path = "(use core::integer::u128_wide_mul)"
    completion_label_type_info = "fn(a: u128, b: u128) -> (u128, u128) nopanic"
    insert_text = "u128_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wide_mul;

    """]

    [[completions]]
    completion_label = "u128_wrapping_add(...)"
    completion_label_path = "(use core::integer::u128_wrapping_add)"
    completion_label_type_info = "fn(lhs: u128, rhs: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u128_wrapping_add;

    """]

    [[completions]]
    completion_label = "u128_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u128_wrapping_sub)"
    completion_label_type_info = "fn(a: u128, b: u128) -> u128 nopanic"
    insert_text = "u128_wrapping_sub(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u128_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u16_overflowing_add(...)"
    completion_label_path = "(use core::integer::u16_overflowing_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_add;

    """]

    [[completions]]
    completion_label = "u16_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u16_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> Result<u16, u16> nopanic"
    insert_text = "u16_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u16_safe_divmod(...)"
    completion_label_path = "(use core::integer::u16_safe_divmod)"
    completion_label_type_info = "fn(lhs: u16, rhs: NonZero<u16>) -> (u16, u16) nopanic"
    insert_text = "u16_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_safe_divmod;

    """]

    [[completions]]
    completion_label = "u16_sqrt(...)"
    completion_label_path = "(use core::integer::u16_sqrt)"
    completion_label_type_info = "fn(value: u16) -> u8 nopanic"
    insert_text = "u16_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u16_sqrt;

    """]

    [[completions]]
    completion_label = "u16_wide_mul(...)"
    completion_label_path = "(use core::integer::u16_wide_mul)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u32 nopanic"
    insert_text = "u16_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wide_mul;

    """]

    [[completions]]
    completion_label = "u16_wrapping_add(...)"
    completion_label_path = "(use core::integer::u16_wrapping_add)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_add;

    """]

    [[completions]]
    completion_label = "u16_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u16_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u16, rhs: u16) -> u16 nopanic"
    insert_text = "u16_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u16_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u2"
    completion_label_path = "(use core::sha256::u2)"
    text_edits = ["""
    use core::sha256::u2;

    """]

    [[completions]]
    completion_label = "u256_div_mod_n(...)"
    completion_label_path = "(use core::math::u256_div_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> Option<u256>"
    insert_text = "u256_div_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_div_mod_n;

    """]

    [[completions]]
    completion_label = "u256_inv_mod(...)"
    completion_label_path = "(use core::math::u256_inv_mod)"
    completion_label_type_info = "fn(a: u256, n: NonZero<u256>) -> Option<NonZero<u256>>"
    insert_text = "u256_inv_mod(${1:a}, ${2:n})"
    text_edits = ["""
    use core::math::u256_inv_mod;

    """]

    [[completions]]
    completion_label = "u256_mul_mod_n(...)"
    completion_label_path = "(use core::math::u256_mul_mod_n)"
    completion_label_type_info = "fn(a: u256, b: u256, n: NonZero<u256>) -> u256"
    insert_text = "u256_mul_mod_n(${1:a}, ${2:b}, ${3:n})"
    text_edits = ["""
    use core::math::u256_mul_mod_n;

    """]

    [[completions]]
    completion_label = "u256_overflow_mul(...)"
    completion_label_path = "(use core::integer::u256_overflow_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflow_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_mul;

    """]

    [[completions]]
    completion_label = "u256_overflow_sub(...)"
    completion_label_path = "(use core::integer::u256_overflow_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflow_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflow_sub;

    """]

    [[completions]]
    completion_label = "u256_overflowing_add(...)"
    completion_label_path = "(use core::integer::u256_overflowing_add)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_add;

    """]

    [[completions]]
    completion_label = "u256_overflowing_mul(...)"
    completion_label_path = "(use core::integer::u256_overflowing_mul)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool)"
    insert_text = "u256_overflowing_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_mul;

    """]

    [[completions]]
    completion_label = "u256_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u256_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u256, rhs: u256) -> (u256, bool) nopanic"
    insert_text = "u256_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u256_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u256_sqrt(...)"
    completion_label_path = "(use core::integer::u256_sqrt)"
    completion_label_type_info = "fn(a: u256) -> u128 nopanic"
    insert_text = "u256_sqrt(${1:a})"
    text_edits = ["""
    use core::integer::u256_sqrt;

    """]

    [[completions]]
    completion_label = "u256_wide_mul(...)"
    completion_label_path = "(use core::integer::u256_wide_mul)"
    completion_label_type_info = "fn(a: u256, b: u256) -> u512 nopanic"
    insert_text = "u256_wide_mul(${1:a}, ${2:b})"
    text_edits = ["""
    use core::integer::u256_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_overflowing_add(...)"
    completion_label_path = "(use core::integer::u32_overflowing_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_add;

    """]

    [[completions]]
    completion_label = "u32_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u32_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> Result<u32, u32> nopanic"
    insert_text = "u32_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u32_safe_divmod(...)"
    completion_label_path = "(use core::integer::u32_safe_divmod)"
    completion_label_type_info = "fn(lhs: u32, rhs: NonZero<u32>) -> (u32, u32) nopanic"
    insert_text = "u32_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_safe_divmod;

    """]

    [[completions]]
    completion_label = "u32_sqrt(...)"
    completion_label_path = "(use core::integer::u32_sqrt)"
    completion_label_type_info = "fn(value: u32) -> u16 nopanic"
    insert_text = "u32_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u32_sqrt;

    """]

    [[completions]]
    completion_label = "u32_wide_mul(...)"
    completion_label_path = "(use core::integer::u32_wide_mul)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u64 nopanic"
    insert_text = "u32_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wide_mul;

    """]

    [[completions]]
    completion_label = "u32_wrapping_add(...)"
    completion_label_path = "(use core::integer::u32_wrapping_add)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_add;

    """]

    [[completions]]
    completion_label = "u32_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u32_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u32, rhs: u32) -> u32 nopanic"
    insert_text = "u32_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u32_wrapping_sub;

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
    completion_label = "u512_safe_div_rem_by_u256(...)"
    completion_label_path = "(use core::integer::u512_safe_div_rem_by_u256)"
    completion_label_type_info = "fn(lhs: u512, rhs: NonZero<u256>) -> (u512, u256) nopanic"
    insert_text = "u512_safe_div_rem_by_u256(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u512_safe_div_rem_by_u256;

    """]

    [[completions]]
    completion_label = "u64_overflowing_add(...)"
    completion_label_path = "(use core::integer::u64_overflowing_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_add;

    """]

    [[completions]]
    completion_label = "u64_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u64_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> Result<u64, u64> nopanic"
    insert_text = "u64_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u64_safe_divmod(...)"
    completion_label_path = "(use core::integer::u64_safe_divmod)"
    completion_label_type_info = "fn(lhs: u64, rhs: NonZero<u64>) -> (u64, u64) nopanic"
    insert_text = "u64_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_safe_divmod;

    """]

    [[completions]]
    completion_label = "u64_sqrt(...)"
    completion_label_path = "(use core::integer::u64_sqrt)"
    completion_label_type_info = "fn(value: u64) -> u32 nopanic"
    insert_text = "u64_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u64_sqrt;

    """]

    [[completions]]
    completion_label = "u64_wide_mul(...)"
    completion_label_path = "(use core::integer::u64_wide_mul)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u128 nopanic"
    insert_text = "u64_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wide_mul;

    """]

    [[completions]]
    completion_label = "u64_wrapping_add(...)"
    completion_label_path = "(use core::integer::u64_wrapping_add)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_add;

    """]

    [[completions]]
    completion_label = "u64_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u64_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u64, rhs: u64) -> u64 nopanic"
    insert_text = "u64_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u64_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u8_overflowing_add(...)"
    completion_label_path = "(use core::integer::u8_overflowing_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_add;

    """]

    [[completions]]
    completion_label = "u8_overflowing_sub(...)"
    completion_label_path = "(use core::integer::u8_overflowing_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> Result<u8, u8> nopanic"
    insert_text = "u8_overflowing_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_overflowing_sub;

    """]

    [[completions]]
    completion_label = "u8_safe_divmod(...)"
    completion_label_path = "(use core::integer::u8_safe_divmod)"
    completion_label_type_info = "fn(lhs: u8, rhs: NonZero<u8>) -> (u8, u8) nopanic"
    insert_text = "u8_safe_divmod(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_safe_divmod;

    """]

    [[completions]]
    completion_label = "u8_sqrt(...)"
    completion_label_path = "(use core::integer::u8_sqrt)"
    completion_label_type_info = "fn(value: u8) -> u8 nopanic"
    insert_text = "u8_sqrt(${1:value})"
    text_edits = ["""
    use core::integer::u8_sqrt;

    """]

    [[completions]]
    completion_label = "u8_wide_mul(...)"
    completion_label_path = "(use core::integer::u8_wide_mul)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u16 nopanic"
    insert_text = "u8_wide_mul(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wide_mul;

    """]

    [[completions]]
    completion_label = "u8_wrapping_add(...)"
    completion_label_path = "(use core::integer::u8_wrapping_add)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_add(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_add;

    """]

    [[completions]]
    completion_label = "u8_wrapping_sub(...)"
    completion_label_path = "(use core::integer::u8_wrapping_sub)"
    completion_label_type_info = "fn(lhs: u8, rhs: u8) -> u8 nopanic"
    insert_text = "u8_wrapping_sub(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::integer::u8_wrapping_sub;

    """]

    [[completions]]
    completion_label = "u96"
    completion_label_path = "(use core::circuit::u96)"
    text_edits = ["""
    use core::circuit::u96;

    """]

    [[completions]]
    completion_label = "upcast(...)"
    completion_label_path = "(use core::internal::bounded_int::upcast)"
    completion_label_type_info = "fn(x: FromType) -> ToType nopanic"
    insert_text = "upcast(${1:x})"
    text_edits = ["""
    use core::internal::bounded_int::upcast;

    """]

    [[completions]]
    completion_label = "verify_eth_signature(...)"
    completion_label_path = "(use starknet::eth_signature::verify_eth_signature)"
    completion_label_type_info = "fn(msg_hash: u256, signature: Signature, eth_address: EthAddress) -> ()"
    insert_text = "verify_eth_signature(${1:msg_hash}, ${2:signature}, ${3:eth_address})"
    text_edits = ["""
    use starknet::eth_signature::verify_eth_signature;

    """]

    [[completions]]
    completion_label = "withdraw_gas(...)"
    completion_label_path = "(use core::gas::withdraw_gas)"
    completion_label_type_info = "fn() -> Option<()> nopanic"
    insert_text = "withdraw_gas()"
    text_edits = ["""
    use core::gas::withdraw_gas;

    """]

    [[completions]]
    completion_label = "withdraw_gas_all(...)"
    completion_label_path = "(use core::gas::withdraw_gas_all)"
    completion_label_type_info = "fn(costs: BuiltinCosts) -> Option<()> nopanic"
    insert_text = "withdraw_gas_all(${1:costs})"
    text_edits = ["""
    use core::gas::withdraw_gas_all;

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

    [[completions]]
    completion_label = "zip(...)"
    completion_label_path = "(use core::iter::zip)"
    completion_label_type_info = "fn(a: A, b: B) -> Zip<AIntoIter::IntoIter, BIntoIter::IntoIter>"
    insert_text = "zip(${1:a}, ${2:b})"
    text_edits = ["""
    use core::iter::zip;

    """]
    "#);
}
