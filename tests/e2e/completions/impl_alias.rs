use lsp_types::request::Completion;

use crate::{
    completions::{completion_fixture, sorting_dep_fixture},
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn empty_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}
    fn foo_fn() {}

    impl MyAlias = <caret>;
    ", @r#"
    caret = """
    impl MyAlias = <caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "dep"

    [[completions]]
    completion_label = "hello"

    [[completions]]
    completion_label = "core"

    [[completions]]
    completion_label = "starknet"

    [[completions]]
    completion_label = "AddInputResultImpl"
    completion_label_path = "(use core::circuit::AddInputResultImpl)"
    text_edits = ["""
    use core::circuit::AddInputResultImpl;

    """]

    [[completions]]
    completion_label = "ArrayImpl"
    completion_label_path = "(use core::array::ArrayImpl)"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "BoolImpl"
    completion_label_path = "(use core::boolean::BoolImpl)"
    text_edits = ["""
    use core::boolean::BoolImpl;

    """]

    [[completions]]
    completion_label = "BoxImpl"
    completion_label_path = "(use core::box::BoxImpl)"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "ByteArrayImpl"
    completion_label_path = "(use core::byte_array::ByteArrayImpl)"
    text_edits = ["""
    use core::byte_array::ByteArrayImpl;

    """]

    [[completions]]
    completion_label = "ByteSpanImpl"
    completion_label_path = "(use core::byte_array::ByteSpanImpl)"
    text_edits = ["""
    use core::byte_array::ByteSpanImpl;

    """]

    [[completions]]
    completion_label = "Bytes31Impl"
    completion_label_path = "(use core::bytes_31::Bytes31Impl)"
    text_edits = ["""
    use core::bytes_31::Bytes31Impl;

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
    completion_label = "CircuitInputsImpl"
    completion_label_path = "(use core::circuit::CircuitInputsImpl)"
    text_edits = ["""
    use core::circuit::CircuitInputsImpl;

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
    completion_label = "DebugImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::DebugImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

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
    completion_label = "EcPointImpl"
    completion_label_path = "(use core::ec::EcPointImpl)"
    text_edits = ["""
    use core::ec::EcPointImpl;

    """]

    [[completions]]
    completion_label = "EcStateImpl"
    completion_label_path = "(use core::ec::EcStateImpl)"
    text_edits = ["""
    use core::ec::EcStateImpl;

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
    completion_label = "HashImpl"
    completion_label_path = "(use core::hash::into_felt252_based::HashImpl)"
    text_edits = ["""
    use core::hash::into_felt252_based::HashImpl;

    """]

    [[completions]]
    completion_label = "LowerHexImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::LowerHexImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::LowerHexImpl;

    """]

    [[completions]]
    completion_label = "NullableImpl"
    completion_label_path = "(use core::nullable::NullableImpl)"
    text_edits = ["""
    use core::nullable::NullableImpl;

    """]

    [[completions]]
    completion_label = "OptionTraitImpl"
    completion_label_path = "(use core::option::OptionTraitImpl)"
    text_edits = ["""
    use core::option::OptionTraitImpl;

    """]

    [[completions]]
    completion_label = "PedersenImpl"
    completion_label_path = "(use core::pedersen::PedersenImpl)"
    text_edits = ["""
    use core::pedersen::PedersenImpl;

    """]

    [[completions]]
    completion_label = "PoseidonImpl"
    completion_label_path = "(use core::poseidon::PoseidonImpl)"
    text_edits = ["""
    use core::poseidon::PoseidonImpl;

    """]

    [[completions]]
    completion_label = "ResultTraitImpl"
    completion_label_path = "(use core::result::ResultTraitImpl)"
    text_edits = ["""
    use core::result::ResultTraitImpl;

    """]

    [[completions]]
    completion_label = "Secp256k1Impl"
    completion_label_path = "(use starknet::secp256k1::Secp256k1Impl)"
    text_edits = ["""
    use starknet::secp256k1::Secp256k1Impl;

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
    completion_label = "Secp256r1PointImpl"
    completion_label_path = "(use starknet::secp256r1::Secp256r1PointImpl)"
    text_edits = ["""
    use starknet::secp256r1::Secp256r1PointImpl;

    """]

    [[completions]]
    completion_label = "SerdeImpl"
    completion_label_path = "(use core::serde::into_felt252_based::SerdeImpl)"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

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
    completion_label = "SquashedFelt252DictImpl"
    completion_label_path = "(use core::dict::SquashedFelt252DictImpl)"
    text_edits = ["""
    use core::dict::SquashedFelt252DictImpl;

    """]

    [[completions]]
    completion_label = "StorableStoragePointerReadAccess"
    completion_label_path = "(use starknet::storage::StorableStoragePointerReadAccess)"
    text_edits = ["""
    use starknet::storage::StorableStoragePointerReadAccess;

    """]

    [[completions]]
    completion_label = "StorageNodeDeref"
    completion_label_path = "(use starknet::storage::StorageNodeDeref)"
    text_edits = ["""
    use starknet::storage::StorageNodeDeref;

    """]

    [[completions]]
    completion_label = "StorageNodeMutDeref"
    completion_label_path = "(use starknet::storage::StorageNodeMutDeref)"
    text_edits = ["""
    use starknet::storage::StorageNodeMutDeref;

    """]

    [[completions]]
    completion_label = "SubPointersDeref"
    completion_label_path = "(use starknet::storage::SubPointersDeref)"
    text_edits = ["""
    use starknet::storage::SubPointersDeref;

    """]

    [[completions]]
    completion_label = "SubPointersMutDeref"
    completion_label_path = "(use starknet::storage::SubPointersMutDeref)"
    text_edits = ["""
    use starknet::storage::SubPointersMutDeref;

    """]

    [[completions]]
    completion_label = "U256IntoEthAddress"
    completion_label_path = "(use starknet::eth_address::U256IntoEthAddress)"
    text_edits = ["""
    use starknet::eth_address::U256IntoEthAddress;

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
fn partial_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}
    impl FooAlias = FooFelt252;
    fn foo_fn() {}
    const FOO_CONST: felt252 = 0;

    impl MyAlias = Foo<caret>;
    ", @r#"
    caret = """
    impl MyAlias = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooAlias"

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn after_colon_colon() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub mod nested {}
        pub trait Foo<T> {}
        pub struct FooStruct {}
        pub impl FooFelt252 of Foo<felt252> {}
        pub fn foo_fn() {}
    }

    impl MyAlias = inner::<caret>;
    ", @r#"
    caret = """
    impl MyAlias = inner::<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"
    completion_label_path = "(use inner::FooFelt252)"

    [[completions]]
    completion_label = "nested"
    completion_label_path = "(use inner::nested)"
    "#);
}

#[test]
fn generic_impl() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    impl FooImpl<T> of Foo<T> {}

    impl MyAlias<T> = FooI<caret><T>;
    ", @r#"
    caret = """
    impl MyAlias<T> = FooI<caret><T>;
    """

    [[completions]]
    completion_label = "FooImpl"
    "#);
}

#[test]
fn generic_arg_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooImpl<T> of Foo<T> {}

    impl MyAlias = FooImpl<FooS<caret>>;
    ", @r#"
    caret = """
    impl MyAlias = FooImpl<FooS<caret>>;
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "FooImpl"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn type_alias_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct FooStruct {}
    impl FooImpl of Drop<FooStruct> {}

    type MyAlias = FooS<caret>;
    ", @r#"
    caret = """
    type MyAlias = FooS<caret>;
    """

    [[completions]]
    completion_label = "FooImpl"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn associated_impl_in_impl_body() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    trait Bar {
        impl FooImpl: Foo<felt252>;
    }

    impl BarImpl of Bar {
        impl FooImpl = Foo<caret>;
    }
    ", @r#"
    caret = """
        impl FooImpl = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn module_prefix() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub trait Foo<T> {}
        pub impl FooFelt252 of Foo<felt252> {}
    }

    impl MyAlias = inn<caret>;
    ", @r#"
    caret = """
    impl MyAlias = inn<caret>;
    """

    [[completions]]
    completion_label = "inner"

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
    "#);
}

#[test]
fn self_is_excluded() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    impl FooFelt252 of Foo<felt252> {}
    impl MyAliasOther = FooFelt252;

    impl MyAlias = MyAl<caret>;
    ", @r#"
    caret = """
    impl MyAlias = MyAl<caret>;
    """

    [[completions]]
    completion_label = "MyAliasOther"
    "#);
}

#[test]
fn enclosing_impl_is_excluded() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    impl BarFelt252 of Foo<felt252> {}

    trait Bar {
        impl FooImpl: Foo<felt252>;
    }

    impl BarImpl of Bar {
        impl FooImpl = Bar<caret>;
    }
    ", @r#"
    caret = """
        impl FooImpl = Bar<caret>;
    """

    [[completions]]
    completion_label = "BarFelt252"
    "#);
}

#[test]
fn without_semicolon() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    impl MyAlias = Foo<caret>
    ", @r#"
    caret = """
    impl MyAlias = Foo<caret>
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn empty_generic_arg_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T, U> {}
    struct FooStruct {}
    impl FooImpl<T, U> of Foo<T, U> {}

    impl MyAlias = FooImpl<u32, <caret>>;
    ", @r#"
    caret = """
    impl MyAlias = FooImpl<u32, <caret>>;
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "FooImpl"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn partial_name_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    trait Bar {
        impl FooImpl: Foo<felt252>;
    }

    #[complex_attribute_macro_v2]
    impl BarImpl of Bar {
        impl FooImpl = Foo<caret>;
    }
    ", @r#"
    caret = """
        impl FooImpl = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn pub_impl_alias() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    pub impl MyAlias = Foo<caret>;
    ", @r#"
    caret = """
    pub impl MyAlias = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn generic_impl_alias() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooImpl<T, +Drop<T>> of Foo<T> {}

    impl MyAlias<T, +Drop<T>> = Foo<caret>;
    ", @r#"
    caret = """
    impl MyAlias<T, +Drop<T>> = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooImpl"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn cursor_mid_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub trait Foo<T> {}
        pub impl FooFelt252 of Foo<felt252> {}
    }
    struct inner_struct {}

    impl MyAlias = inn<caret>er::FooFelt252;
    ", @r#"
    caret = """
    impl MyAlias = inn<caret>er::FooFelt252;
    """

    [[completions]]
    completion_label = "inner"

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
    completion_label = "iter"
    completion_label_path = "(use core::iter)"
    text_edits = ["""
    use core::iter;

    """]
    "#);
}

#[test]
fn nested_module_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub mod nested {
            pub mod deeper {}
            pub trait Foo<T> {}
            pub struct FooStruct {}
            pub impl FooFelt252 of Foo<felt252> {}
            pub const FOO: felt252 = 0;
        }
    }

    impl MyAlias = inner::nested::<caret>;
    ", @r#"
    caret = """
    impl MyAlias = inner::nested::<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"
    completion_label_path = "(use inner::nested::FooFelt252)"

    [[completions]]
    completion_label = "deeper"
    completion_label_path = "(use inner::nested::deeper)"
    "#);
}

#[test]
fn nested_module_path_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub mod nested {
            pub trait Foo<T> {}
            pub struct FooStruct {}
            pub impl FooFelt252 of Foo<felt252> {}
            pub impl BarFelt252 of Foo<felt252> {}
        }
    }

    impl MyAlias = inner::nested::Fo<caret>;
    ", @r#"
    caret = """
    impl MyAlias = inner::nested::Fo<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"
    completion_label_path = "(use inner::nested::FooFelt252)"
    "#);
}

#[test]
fn private_impl_is_not_proposed() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub trait Foo<T> {}
        impl FooPrivate of Foo<felt252> {}
        pub impl FooPublic of Foo<u32> {}
    }

    impl MyAlias = inner::Foo<caret>;
    ", @r#"
    caret = """
    impl MyAlias = inner::Foo<caret>;
    """

    [[completions]]
    completion_label = "FooPublic"
    completion_label_path = "(use inner::FooPublic)"
    "#);
}

#[test]
fn impl_from_dependency_with_import() {
    test_transform_plain!(Completion, sorting_dep_fixture(), "
    impl MyAlias = MergeS<caret>;
    ", @r#"
    caret = """
    impl MyAlias = MergeS<caret>;
    """

    [[completions]]
    completion_label = "MergeSort"
    completion_label_path = "(use alexandria_sorting::MergeSort)"
    text_edits = ["""
    use alexandria_sorting::MergeSort;

    """]

    [[completions]]
    completion_label = "serde"
    completion_label_path = "(use core::serde)"
    text_edits = ["""
    use core::serde;

    """]
    "#);
}

#[test]
fn dependency_crate_path() {
    test_transform_plain!(Completion, sorting_dep_fixture(), "
    impl MyAlias = alexandria_sorting::<caret>;
    ", @r#"
    caret = """
    impl MyAlias = alexandria_sorting::<caret>;
    """

    [[completions]]
    completion_label = "MergeSort"
    completion_label_path = "(use alexandria_sorting::MergeSort)"

    [[completions]]
    completion_label = "interface"
    completion_label_path = "(use alexandria_sorting::interface)"

    [[completions]]
    completion_label = "merge_sort"
    completion_label_path = "(use alexandria_sorting::merge_sort)"
    "#);
}

#[test]
fn alias_in_nested_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    mod inner {
        impl MyAlias = Foo<caret>;
    }
    ", @r#"
    caret = """
        impl MyAlias = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"
    completion_label_path = "(use crate::FooFelt252)"
    text_edits = ["""
    use crate::FooFelt252;

    """]

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn associated_impl_qualified_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub trait Foo<T> {}
        pub struct FooStruct {}
        pub impl FooFelt252 of Foo<felt252> {}
    }

    trait Bar {
        impl FooImpl: inner::Foo<felt252>;
    }

    impl BarImpl of Bar {
        impl FooImpl = inner::<caret>;
    }
    ", @r#"
    caret = """
        impl FooImpl = inner::<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"
    completion_label_path = "(use inner::FooFelt252)"
    "#);
}

#[test]
fn without_semicolon_before_item() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    impl MyAlias = Foo<caret>
    fn foo_fn() {}
    ", @r#"
    caret = """
    impl MyAlias = Foo<caret>
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}

#[test]
fn qualified_path_without_semicolon() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod inner {
        pub trait Foo<T> {}
        pub struct FooStruct {}
        pub impl FooFelt252 of Foo<felt252> {}
    }

    impl MyAlias = inner::<caret>
    ", @r#"
    caret = """
    impl MyAlias = inner::<caret>
    """

    [[completions]]
    completion_label = "FooFelt252"
    completion_label_path = "(use inner::FooFelt252)"
    "#);
}

#[test]
fn alias_name_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    impl Foo<caret> = FooFelt252;
    ", @r#"
    caret = """
    impl Foo<caret> = FooFelt252;
    """
    completions = []
    "#);
}

#[test]
fn alias_generic_params_are_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooImpl<T, impl TFoo: Foo<T>> of Foo<Array<T>> {}

    impl MyAlias<T, impl TFoo: Fo<caret><T>> = FooImpl<T, TFoo>;
    ", @r#"
    caret = """
    impl MyAlias<T, impl TFoo: Fo<caret><T>> = FooImpl<T, TFoo>;
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn turbofish_generic_arg_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooImpl<T> of Foo<T> {}

    impl MyAlias = FooImpl::<FooS<caret>>;
    ", @r#"
    caret = """
    impl MyAlias = FooImpl::<FooS<caret>>;
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "FooImpl"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn nested_generic_arg_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct<T> {}
    impl FooImpl<T> of Foo<T> {}

    impl MyAlias = FooImpl<FooStruct<FooS<caret>>>;
    ", @r#"
    caret = """
    impl MyAlias = FooImpl<FooStruct<FooS<caret>>>;
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "FooImpl"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn trait_associated_impl_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    trait Bar {
        impl FooImpl: Foo<caret><felt252>;
    }
    ", @r#"
    caret = """
        impl FooImpl: Foo<caret><felt252>;
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "Felt252DictTrait"

    [[completions]]
    completion_label = "Felt252DictValue"

    [[completions]]
    completion_label = "felt252"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

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
    completion_label = "felt252_div(...)"
    completion_label_path = "(use core::felt252_div)"
    completion_label_type_info = "fn(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic"
    insert_text = "felt252_div(${1:lhs}, ${2:rhs})"
    text_edits = ["""
    use core::felt252_div;

    """]
    "#);
}

#[test]
fn impl_declaration_is_not_filtered() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait Foo<T> {}
    struct FooStruct {}
    impl FooFelt252 of Foo<felt252> {}

    impl FooU32 of Foo<caret><u32> {}
    ", @r#"
    caret = """
    impl FooU32 of Foo<caret><u32> {}
    """

    [[completions]]
    completion_label = "Foo"

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "FooStruct"

    [[completions]]
    completion_label = "FooU32"

    [[completions]]
    completion_label = "Foo"
    completion_label_path = "(use dep::Foo)"
    text_edits = ["""
    use dep::Foo;

    """]
    "#);
}

#[test]
fn partial_name_in_module_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    #[complex_attribute_macro_v2]
    mod my_mod {
        pub trait Foo<T> {}
        pub struct FooStruct {}
        pub impl FooFelt252 of Foo<felt252> {}

        impl MyAlias = Foo<caret>;
    }
    ", @r#"
    caret = """
        impl MyAlias = Foo<caret>;
    """

    [[completions]]
    completion_label = "FooFelt252"

    [[completions]]
    completion_label = "Felt252TryIntoContractAddress"
    completion_label_path = "(use starknet::contract_address::Felt252TryIntoContractAddress)"
    text_edits = ["""
    use starknet::contract_address::Felt252TryIntoContractAddress;

    """]
    "#);
}
