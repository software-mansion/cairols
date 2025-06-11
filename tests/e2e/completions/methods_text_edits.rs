use indoc::indoc;

use crate::completions::completion_fixture;
use crate::support::fixture;
use crate::support::fixture::Fixture;
use crate::support::insta::test_transform_plain;
use lsp_types::request::Completion;

#[test]
fn simple_trait() {
    test_transform_plain!(Completion, completion_fixture(),"
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
            x.some_me<caret>
        }
    }
    ",@r#"
    caret = """
            x.some_me<caret>
    """

    [[completions]]
    completion_label = "Some"

    [[completions]]
    completion_label = "Some"
    text_edits = ["""
    use core::internal::OptionRev::Some;

    """]

    [[completions]]
    completion_label = "add_assign()"
    detail = "core::ops::arith::AddAssign"
    insert_text = "add_assign($1)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "add_eq()"
    detail = "core::traits::AddEq"
    insert_text = "add_eq($1)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "append_formatted_to_byte_array()"
    detail = "core::to_byte_array::AppendFormattedToByteArray"
    insert_text = "append_formatted_to_byte_array($1)"
    text_edits = ["""
    use core::to_byte_array::AppendFormattedToByteArray;

    """]

    [[completions]]
    completion_label = "clone()"
    detail = "core::clone::Clone"
    insert_text = "clone($1)"

    [[completions]]
    completion_label = "destruct()"
    detail = "core::traits::Destruct"
    insert_text = "destruct($1)"

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Display"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Debug"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::LowerHex"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::LowerHex;

    """]

    [[completions]]
    completion_label = "format_as_byte_array()"
    detail = "core::to_byte_array::FormatAsByteArray"
    insert_text = "format_as_byte_array($1)"
    text_edits = ["""
    use core::to_byte_array::FormatAsByteArray;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "is_non_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_non_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_non_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_non_zero($1)"

    [[completions]]
    completion_label = "is_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_zero($1)"

    [[completions]]
    completion_label = "mul_assign()"
    detail = "core::ops::arith::MulAssign"
    insert_text = "mul_assign($1)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "mul_eq()"
    detail = "core::traits::MulEq"
    insert_text = "mul_eq($1)"
    text_edits = ["""
    use core::traits::MulEq;

    """]

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
    completion_label = "pow()"
    detail = "core::num::traits::ops::pow::Pow"
    insert_text = "pow($1)"
    text_edits = ["""
    use core::num::traits::Pow;

    """]

    [[completions]]
    completion_label = "print()"
    detail = "core::debug::PrintTrait"
    insert_text = "print($1)"

    [[completions]]
    completion_label = "serde"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "serialize()"
    detail = "core::serde::Serde"
    insert_text = "serialize($1)"

    [[completions]]
    completion_label = "set_nonce"
    text_edits = ["""
    use starknet::testing::set_nonce;

    """]

    [[completions]]
    completion_label = "some_method()"
    detail = "hello::hidden_trait::ATrait1"
    insert_text = "some_method($1)"
    text_edits = ["""
    use crate::ATrait1;

    """]

    [[completions]]
    completion_label = "storage_access"
    text_edits = ["""
    use starknet::storage_access;

    """]

    [[completions]]
    completion_label = "sub_assign()"
    detail = "core::ops::arith::SubAssign"
    insert_text = "sub_assign($1)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]

    [[completions]]
    completion_label = "sub_eq()"
    detail = "core::traits::SubEq"
    insert_text = "sub_eq($1)"
    text_edits = ["""
    use core::traits::SubEq;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
    "#);
}

#[test]
fn non_directly_visible_trait() {
    test_transform_plain!(Completion, completion_fixture(),"
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
            x.some_me<caret>
        }
    }
    ",@r#"
    caret = """
            x.some_me<caret>
    """

    [[completions]]
    completion_label = "Some"

    [[completions]]
    completion_label = "Some"
    text_edits = ["""
    use core::internal::OptionRev::Some;

    """]

    [[completions]]
    completion_label = "add_assign()"
    detail = "core::ops::arith::AddAssign"
    insert_text = "add_assign($1)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "add_eq()"
    detail = "core::traits::AddEq"
    insert_text = "add_eq($1)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "append_formatted_to_byte_array()"
    detail = "core::to_byte_array::AppendFormattedToByteArray"
    insert_text = "append_formatted_to_byte_array($1)"
    text_edits = ["""
    use core::to_byte_array::AppendFormattedToByteArray;

    """]

    [[completions]]
    completion_label = "clone()"
    detail = "core::clone::Clone"
    insert_text = "clone($1)"

    [[completions]]
    completion_label = "destruct()"
    detail = "core::traits::Destruct"
    insert_text = "destruct($1)"

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Display"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Debug"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::LowerHex"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::LowerHex;

    """]

    [[completions]]
    completion_label = "format_as_byte_array()"
    detail = "core::to_byte_array::FormatAsByteArray"
    insert_text = "format_as_byte_array($1)"
    text_edits = ["""
    use core::to_byte_array::FormatAsByteArray;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "is_non_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_non_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_non_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_non_zero($1)"

    [[completions]]
    completion_label = "is_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_zero($1)"

    [[completions]]
    completion_label = "mul_assign()"
    detail = "core::ops::arith::MulAssign"
    insert_text = "mul_assign($1)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "mul_eq()"
    detail = "core::traits::MulEq"
    insert_text = "mul_eq($1)"
    text_edits = ["""
    use core::traits::MulEq;

    """]

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
    completion_label = "pow()"
    detail = "core::num::traits::ops::pow::Pow"
    insert_text = "pow($1)"
    text_edits = ["""
    use core::num::traits::Pow;

    """]

    [[completions]]
    completion_label = "print()"
    detail = "core::debug::PrintTrait"
    insert_text = "print($1)"

    [[completions]]
    completion_label = "serde"
    text_edits = ["""
    use core::serde;

    """]

    [[completions]]
    completion_label = "serialize()"
    detail = "core::serde::Serde"
    insert_text = "serialize($1)"

    [[completions]]
    completion_label = "set_nonce"
    text_edits = ["""
    use starknet::testing::set_nonce;

    """]

    [[completions]]
    completion_label = "some_method()"
    detail = "hello::hidden_trait::ATrait1"
    insert_text = "some_method($1)"
    text_edits = ["""
    use crate::ATrait1;

    """]

    [[completions]]
    completion_label = "storage_access"
    text_edits = ["""
    use starknet::storage_access;

    """]

    [[completions]]
    completion_label = "sub_assign()"
    detail = "core::ops::arith::SubAssign"
    insert_text = "sub_assign($1)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]

    [[completions]]
    completion_label = "sub_eq()"
    detail = "core::traits::SubEq"
    insert_text = "sub_eq($1)"
    text_edits = ["""
    use core::traits::SubEq;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
    "#);
}

fn only_dependencies_methods_included_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc! {r#"
            [crate_roots]
            this = "src"
            dep = "dep"

            [config.override.this]
            edition = "2024_07"

            [config.override.this.dependencies]
            dep = { discriminator = "dep" }
        "#},
        "dep/lib.cairo" =>  indoc! {
        r#"
            pub trait X<T> {
                fn some_method(self: @T);
            }
            impl MyImpl of X<felt252> {
                fn some_method(self: @felt252) {}
            }
        "#}
    }
}

#[test]
fn methods_from_deps_included() {
    test_transform_plain!(Completion, only_dependencies_methods_included_fixture(), "
    fn func() {
        let x = 5_felt252;
        x.some_metho<caret>
    }
    ",@r#"
    caret = """
        x.some_metho<caret>
    """

    [[completions]]
    completion_label = "add_assign()"
    detail = "core::ops::arith::AddAssign"
    insert_text = "add_assign($1)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "add_eq()"
    detail = "core::traits::AddEq"
    insert_text = "add_eq($1)"
    text_edits = ["""
    use core::traits::AddEq;

    """]

    [[completions]]
    completion_label = "append_formatted_to_byte_array()"
    detail = "core::to_byte_array::AppendFormattedToByteArray"
    insert_text = "append_formatted_to_byte_array($1)"
    text_edits = ["""
    use core::to_byte_array::AppendFormattedToByteArray;

    """]

    [[completions]]
    completion_label = "clone()"
    detail = "core::clone::Clone"
    insert_text = "clone($1)"

    [[completions]]
    completion_label = "destruct()"
    detail = "core::traits::Destruct"
    insert_text = "destruct($1)"

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Display"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Debug"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::LowerHex"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::LowerHex;

    """]

    [[completions]]
    completion_label = "format_as_byte_array()"
    detail = "core::to_byte_array::FormatAsByteArray"
    insert_text = "format_as_byte_array($1)"
    text_edits = ["""
    use core::to_byte_array::FormatAsByteArray;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "is_non_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_non_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_non_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_non_zero($1)"

    [[completions]]
    completion_label = "is_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_zero($1)"

    [[completions]]
    completion_label = "mul_assign()"
    detail = "core::ops::arith::MulAssign"
    insert_text = "mul_assign($1)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "mul_eq()"
    detail = "core::traits::MulEq"
    insert_text = "mul_eq($1)"
    text_edits = ["""
    use core::traits::MulEq;

    """]

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
    completion_label = "pow()"
    detail = "core::num::traits::ops::pow::Pow"
    insert_text = "pow($1)"
    text_edits = ["""
    use core::num::traits::Pow;

    """]

    [[completions]]
    completion_label = "print()"
    detail = "core::debug::PrintTrait"
    insert_text = "print($1)"

    [[completions]]
    completion_label = "selector!"
    insert_text = 'selector!("$1")'

    [[completions]]
    completion_label = "serialize()"
    detail = "core::serde::Serde"
    insert_text = "serialize($1)"

    [[completions]]
    completion_label = "some_method()"
    detail = "dep::X"
    insert_text = "some_method($1)"
    text_edits = ["""
    use dep::X;

    """]

    [[completions]]
    completion_label = "sub_assign()"
    detail = "core::ops::arith::SubAssign"
    insert_text = "sub_assign($1)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]

    [[completions]]
    completion_label = "sub_eq()"
    detail = "core::traits::SubEq"
    insert_text = "sub_eq($1)"
    text_edits = ["""
    use core::traits::SubEq;

    """]

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
    "#);
}

fn only_dependencies_methods_excluded_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc! {r#"
            [crate_roots]
            this = "src"
            dep = "dep"
        "#},
        "dep/lib.cairo" =>  indoc! {
        r#"
            pub trait X<T> {
                fn some_method(self: @T);
            }
            impl MyImpl of X<felt252> {
                fn some_method(self: @felt252) {}
            }
        "#}
    }
}

#[test]
fn methods_from_non_deps_excluded() {
    test_transform_plain!(Completion, only_dependencies_methods_excluded_fixture(), "
    fn func() {
        let x = 5_felt252;
        x.some_method<caret>
    }
    ",@r#"
    caret = """
        x.some_method<caret>
    """

    [[completions]]
    completion_label = "add_assign()"
    detail = "core::ops::arith::AddAssign"
    insert_text = "add_assign($1)"
    text_edits = ["""
    use core::ops::AddAssign;

    """]

    [[completions]]
    completion_label = "add_eq()"
    detail = "core::traits::AddEq"
    insert_text = "add_eq($1)"

    [[completions]]
    completion_label = "append_formatted_to_byte_array()"
    detail = "core::to_byte_array::AppendFormattedToByteArray"
    insert_text = "append_formatted_to_byte_array($1)"
    text_edits = ["""
    use to_byte_array::AppendFormattedToByteArray;

    """]

    [[completions]]
    completion_label = "clone()"
    detail = "core::clone::Clone"
    insert_text = "clone($1)"

    [[completions]]
    completion_label = "destruct()"
    detail = "core::traits::Destruct"
    insert_text = "destruct($1)"

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Display"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Display;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::Debug"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::Debug;

    """]

    [[completions]]
    completion_label = "fmt()"
    detail = "core::fmt::LowerHex"
    insert_text = "fmt($1)"
    text_edits = ["""
    use core::fmt::LowerHex;

    """]

    [[completions]]
    completion_label = "format_as_byte_array()"
    detail = "core::to_byte_array::FormatAsByteArray"
    insert_text = "format_as_byte_array($1)"
    text_edits = ["""
    use to_byte_array::FormatAsByteArray;

    """]

    [[completions]]
    completion_label = "get_descriptor()"
    detail = "core::circuit::GetCircuitDescriptor"
    insert_text = "get_descriptor($1)"
    text_edits = ["""
    use core::circuit::GetCircuitDescriptor;

    """]

    [[completions]]
    completion_label = "into()"
    detail = "core::traits::Into"
    insert_text = "into($1)"

    [[completions]]
    completion_label = "is_non_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_non_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_non_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_non_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_non_zero($1)"

    [[completions]]
    completion_label = "is_one()"
    detail = "core::num::traits::one::One"
    insert_text = "is_one($1)"
    text_edits = ["""
    use core::num::traits::One;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::num::traits::zero::Zero"
    insert_text = "is_zero($1)"
    text_edits = ["""
    use core::num::traits::Zero;

    """]

    [[completions]]
    completion_label = "is_zero()"
    detail = "core::zeroable::Zeroable"
    insert_text = "is_zero($1)"

    [[completions]]
    completion_label = "mul_assign()"
    detail = "core::ops::arith::MulAssign"
    insert_text = "mul_assign($1)"
    text_edits = ["""
    use core::ops::MulAssign;

    """]

    [[completions]]
    completion_label = "mul_eq()"
    detail = "core::traits::MulEq"
    insert_text = "mul_eq($1)"

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
    completion_label = "pow()"
    detail = "core::num::traits::ops::pow::Pow"
    insert_text = "pow($1)"
    text_edits = ["""
    use core::num::traits::Pow;

    """]

    [[completions]]
    completion_label = "print()"
    detail = "core::debug::PrintTrait"
    insert_text = "print($1)"
    text_edits = ["""
    use debug::PrintTrait;

    """]

    [[completions]]
    completion_label = "selector!"
    insert_text = 'selector!("$1")'

    [[completions]]
    completion_label = "serialize()"
    detail = "core::serde::Serde"
    insert_text = "serialize($1)"

    [[completions]]
    completion_label = "storage_node"
    text_edits = ["""
    use core::starknet::storage::storage_node;

    """]

    [[completions]]
    completion_label = "sub_assign()"
    detail = "core::ops::arith::SubAssign"
    insert_text = "sub_assign($1)"
    text_edits = ["""
    use core::ops::SubAssign;

    """]

    [[completions]]
    completion_label = "sub_eq()"
    detail = "core::traits::SubEq"
    insert_text = "sub_eq($1)"

    [[completions]]
    completion_label = "try_into()"
    detail = "core::traits::TryInto"
    insert_text = "try_into($1)"
    "#);
}
