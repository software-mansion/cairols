use lsp_types::request::Completion;

use crate::{
    completions::completion_fixture,
    support::insta::{test_transform_plain, test_transform_with_macros},
};

#[test]
fn single_element_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    struct ByteA_ActuallyNotByteArray {}

    fn a() {
        ByteA<caret>
    }
    ",@r#"
    caret = """
        ByteA<caret>
    """

    [[completions]]
    completion_label = "ByteA_ActuallyNotByteArray {...}"
    completion_label_path = "(use ByteA_ActuallyNotByteArray)"
    completion_label_type_info = "ByteA_ActuallyNotByteArray {}"
    insert_text = "ByteA_ActuallyNotByteArray {}"

    [[completions]]
    completion_label = "ByteArray"

    [[completions]]
    completion_label = "ByteArrayTrait"

    [[completions]]
    completion_label = "Bytes31Trait"

    [[completions]]
    completion_label = "System"

    [[completions]]
    completion_label = "BitAnd"
    completion_label_path = "(use core::traits::BitAnd)"
    text_edits = ["""
    use core::traits::BitAnd;

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
    "#);
}

#[test]
fn multi_segment_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod foo {
        struct Bar {}
        pub struct Baz {}
    }

    fn a() {
        foo::B<caret>
    }
    ",@r#"
    caret = """
        foo::B<caret>
    """

    [[completions]]
    completion_label = "Baz {...}"
    completion_label_path = "(use foo::Baz)"
    completion_label_type_info = "Baz {}"
    insert_text = "Baz {}"
    "#);
}

#[test]
fn multi_segment_path_partial() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod foo {
        pub mod bar {
            pub struct Baz {}
        }
        pub struct Boo {}
    }

    fn a() {
        bar::B<caret>
    }
    ",@r#"
    caret = """
        bar::B<caret>
    """

    [[completions]]
    completion_label = "Baz {...}"
    completion_label_path = "(use foo::bar::Baz)"
    completion_label_type_info = "Baz {}"
    insert_text = "Baz {}"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn multi_segment_path_partial_macro() {
    test_transform_with_macros!(Completion, completion_fixture(), "
    mod foo {
        pub mod bar {
            pub struct Baz {}
        }
        pub struct Boo {}
    }

    #[complex_attribute_macro_v2]
    fn a() {
        bar::B<caret>
    }
    ",@r#"
    caret = """
        bar::B<caret>
    """

    [[completions]]
    completion_label = "Baz {...}"
    completion_label_path = "(use foo::bar::Baz)"
    completion_label_type_info = "Baz {}"
    insert_text = "Baz {}"
    text_edits = ["""
    use foo::bar;

    """]
    "#);
}

#[test]
fn enum_variant() {
    test_transform_plain!(Completion, completion_fixture(), "
    enum Enumik {
        A,
        B,
    }

    fn func() {
        let x = Enumik::<caret>
    }
    ",@r#"
    caret = """
        let x = Enumik::<caret>
    """

    [[completions]]
    completion_label = "A"
    completion_label_path = "(use Enumik::A)"

    [[completions]]
    completion_label = "B"
    completion_label_path = "(use Enumik::B)"
    "#);
}

#[test]
fn type_annotation() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x: module::<caret> = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret> = 0x0;
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn type_annotation_with_dangling_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;

        pub const CONST: u32 = 0;

        pub mod nested_module {
            pub type T = u32;
        }
    }
    fn foo() -> u32 {
        let x: module::<caret>
            nested_module::T = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret>
    """

    [[completions]]
    completion_label = "CONST"
    completion_label_path = "(use module::CONST)"

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"

    [[completions]]
    completion_label = "nested_module"
    completion_label_path = "(use module::nested_module)"
    "#);
}

#[test]
fn type_annotation_with_trivia() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x: module::<caret> // comment
            = 0x0;
    }
    ",@r#"
    caret = """
        let x: module::<caret> // comment
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn generic_parameter() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x = Into::<module::<caret>, u32>(0);
    }
    ",@r#"
    caret = """
        let x = Into::<module::<caret>, u32>(0);
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn generic_parameter_with_trivia() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod module {
        pub type felt = felt252;
        pub type int = i32;
        type priv_int = i32;
    }
    fn foo() {
        let x = Into::<module::<caret>//comment
        , u32>(0);
    }
    ",@r#"
    caret = """
        let x = Into::<module::<caret>//comment
    """

    [[completions]]
    completion_label = "felt"
    completion_label_path = "(use module::felt)"

    [[completions]]
    completion_label = "int"
    completion_label_path = "(use module::int)"
    "#);
}

#[test]
fn function_implicit_parameter() {
    test_transform_plain!(Completion, completion_fixture(), "
    fn foo() implicits(core::Range<caret>) {}
    ",@r#"
    caret = """
    fn foo() implicits(core::Range<caret>) {}
    """

    [[completions]]
    completion_label = "RangeCheck"
    completion_label_path = "(use core::RangeCheck)"
    "#);
}

#[test]
fn simple_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod a {
        pub fn xyz() {}
    }

    fn foo() {
        xy<caret>
    }
    ",@r#"
    caret = """
        xy<caret>
    """

    [[completions]]
    completion_label = "xyz(...)"
    completion_label_path = "(use a::xyz)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "xyz()"
    text_edits = ["""
    use a::xyz;

    """]
    "#);
}

#[test]
fn duplicated_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod a {
        pub fn xyz() {}
    }

    mod b {
        pub fn xyz() {}
    }

    fn foo() {
        xy<caret>
    }
    ",@r#"
    caret = """
        xy<caret>
    """

    [[completions]]
    completion_label = "xyz(...)"
    completion_label_path = "(use a::xyz)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "xyz()"
    text_edits = ["""
    use a::xyz;

    """]

    [[completions]]
    completion_label = "xyz(...)"
    completion_label_path = "(use b::xyz)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "xyz()"
    text_edits = ["""
    use b::xyz;

    """]
    "#);
}

// FIXME(#957)
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
    completions = []
    "#);
}

#[test]
fn no_text_last_segment_in_function_context() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
       pub const MY_CONST: u8 = 5;
       pub fn my_func() {}
    }

    fn a() {
        my_mod::<caret>
    }
    ",@r#"
    caret = """
        my_mod::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_path = "(use my_mod::MY_CONST)"

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_path = "(use my_mod::my_func)"
    completion_label_type_info = "fn() -> ()"
    insert_text = "my_func()"
    "#);
}

#[test]
fn simple_declarative_macro_completion() {
    test_transform_plain!(Completion, completion_fixture(), "
    macro my_own_macro {
        ($x:ident) => {
            1
        };
    }

    fn foo() {
        let _a = my_own<caret>
    }
    ",@r#"
    caret = """
        let _a = my_own<caret>
    """

    [[completions]]
    completion_label = "my_own_macro!"
    completion_label_path = "(use my_own_macro)"
    insert_text = "my_own_macro!($1)"
    "#);
}

#[test]
fn declarative_macro_completion_without_explicit_path() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
        pub macro my_own_macro {
            ($x:ident) => {
                1
            };
        }
    }

    fn foo() {
        let _a = my_own<caret>
    }
    ",@r#"
    caret = """
        let _a = my_own<caret>
    """

    [[completions]]
    completion_label = "my_mod"

    [[completions]]
    completion_label = "my_own_macro!"
    completion_label_path = "(use my_mod::my_own_macro)"
    insert_text = "my_own_macro!($1)"
    text_edits = ["""
    use my_mod::my_own_macro;

    """]
    "#);
}

#[test]
fn trait_prefix_with_function() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "my_func()"
    "#);
}

#[test]
fn trait_prefix_with_type() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        type MyType;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "MyType"
    "#);
}

#[test]
fn trait_prefix_with_constant() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        const MY_CONST: u32;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_type_info = "u32"
    "#);
}

#[test]
fn trait_prefix_with_all_items() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
        type MyType;
        const MY_CONST: u32;
    }

    fn test() {
        MyTrait::<caret>
    }
    ",@r#"
    caret = """
        MyTrait::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "MyType"

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "my_func()"
    "#);
}

#[test]
fn impl_prefix_with_all_items() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
        type MyType;
        const MY_CONST: u32;
    }

    impl MyImpl of MyTrait {
        fn my_func() -> u32 { 0 }
        type MyType = u32;
        const MY_CONST: u32 = 5;
    }

    fn test() {
        MyImpl::<caret>
    }
    ",@r#"
    caret = """
        MyImpl::<caret>
    """

    [[completions]]
    completion_label = "MY_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "MyType"

    [[completions]]
    completion_label = "my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "my_func()"
    "#);
}

#[test]
fn impl_item_suffix_by_impl_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait MyTrait {
        fn my_func() -> u32;
        type MyType;
        const MY_CONST: u32;
    }

    impl MyImpl of MyTrait {
        fn my_func() -> u32 { 0 }
        type MyType = u32;
        const MY_CONST: u32 = 5;
    }

    fn test() {
        MyImpl<caret>
    }
    ",@r#"
    caret = """
        MyImpl<caret>
    """

    [[completions]]
    completion_label = "MyImpl"

    [[completions]]
    completion_label = "MyImpl::MY_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "MyImpl::MyType"

    [[completions]]
    completion_label = "MyImpl::my_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "MyImpl::my_func()"

    [[completions]]
    completion_label = "MyTrait::MyType"

    [[completions]]
    completion_label = "ArrayImpl"
    completion_label_path = "(use core::array::ArrayImpl)"
    text_edits = ["""
    use core::array::ArrayImpl;

    """]

    [[completions]]
    completion_label = "BoxImpl"
    completion_label_path = "(use core::box::BoxImpl)"
    text_edits = ["""
    use core::box::BoxImpl;

    """]

    [[completions]]
    completion_label = "DebugImpl"
    completion_label_path = "(use core::fmt::into_felt252_based::DebugImpl)"
    text_edits = ["""
    use core::fmt::into_felt252_based::DebugImpl;

    """]

    [[completions]]
    completion_label = "HashImpl"
    completion_label_path = "(use core::hash::into_felt252_based::HashImpl)"
    text_edits = ["""
    use core::hash::into_felt252_based::HashImpl;

    """]

    [[completions]]
    completion_label = "SerdeImpl"
    completion_label_path = "(use core::serde::into_felt252_based::SerdeImpl)"
    text_edits = ["""
    use core::serde::into_felt252_based::SerdeImpl;

    """]
    "#);
}

#[test]
fn trait_item_suffix_by_trait_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait UniqueXyzTrait {
        fn unique_xyz_func() -> u32;
        type UniqueXyzType;
        const UNIQUE_XYZ_CONST: u32;
    }

    fn test() {
        UniqueXyz<caret>
    }
    ",@r#"
    caret = """
        UniqueXyz<caret>
    """

    [[completions]]
    completion_label = "UniqueXyzTrait"

    [[completions]]
    completion_label = "UniqueXyzTrait::UNIQUE_XYZ_CONST"
    completion_label_type_info = "u32"

    [[completions]]
    completion_label = "UniqueXyzTrait::UniqueXyzType"

    [[completions]]
    completion_label = "UniqueXyzTrait::unique_xyz_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "UniqueXyzTrait::unique_xyz_func()"

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]
    "#);
}

#[test]
fn trait_item_suffix_by_item_name() {
    test_transform_plain!(Completion, completion_fixture(), "
    trait UniqueXyzTrait {
        fn unique_xyz_func() -> u32;
    }

    fn test() {
        unique_xyz<caret>
    }
    ",@r#"
    caret = """
        unique_xyz<caret>
    """

    [[completions]]
    completion_label = "UniqueXyzTrait::unique_xyz_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "UniqueXyzTrait::unique_xyz_func()"
    "#);
}

#[test]
fn trait_item_suffix_from_other_module() {
    test_transform_plain!(Completion, completion_fixture(), "
    mod my_mod {
        pub trait UniqueXyzTrait {
            fn unique_xyz_func() -> u32;
        }
    }

    fn test() {
        UniqueXyz<caret>
    }
    ",@r#"
    caret = """
        UniqueXyz<caret>
    """

    [[completions]]
    completion_label = "UniqueXyzTrait"
    completion_label_path = "(use my_mod::UniqueXyzTrait)"
    text_edits = ["""
    use my_mod::UniqueXyzTrait;

    """]

    [[completions]]
    completion_label = "UniqueXyzTrait::unique_xyz_func(...)"
    completion_label_type_info = "fn() -> u32"
    insert_text = "UniqueXyzTrait::unique_xyz_func()"
    text_edits = ["""
    use my_mod::UniqueXyzTrait;

    """]

    [[completions]]
    completion_label = "UnitInt"
    completion_label_path = "(use core::internal::bounded_int::UnitInt)"
    text_edits = ["""
    use core::internal::bounded_int::UnitInt;

    """]
    "#);
}
