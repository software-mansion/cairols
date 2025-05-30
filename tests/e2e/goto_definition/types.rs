use lsp_types::request::GotoDefinition;

use crate::support::insta::test_transform_and_macros;

#[test]
fn test_extern_type_in_alias_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    type TypeAlias = u32<caret>;
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_alias_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    type TypeAlias = Array<u32<caret>>;
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_variable_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x: u32<caret> = 0x0;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_variable_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x: Array<u32<caret>> = 0x0;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_function_argument_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(x: u32<caret>) {}
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_function_argument_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(x: Array<u32<caret>>) {}
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_return_type_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() -> u32<caret> { 0x0 }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_return_type_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() -> Array<u32<caret>> { 0x0 }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_struct_field_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    struct Struct {
        x: u32<caret>
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_struct_field_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    struct Struct {
        x: Array<u32<caret>>
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_turbofish_enum_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x = Result::<u32<caret>>::Err(());
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_turbofish_enum_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x = Result::<Array<u32<caret>>>::Err(());
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_trait_associated_type_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = u32<caret>;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_trait_associated_type_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = Arrat<u32<caret>>;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_trait_associated_const_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: u32<caret>;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_trait_associated_const_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: Array<u32<caret>>;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_trait_generic_bound_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<u32<caret>, T>> {}
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_trait_generic_bound_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<Array<u32<caret>>, T>> {}
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_impl_associated_type_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = u32<caret>;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_impl_associated_type_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = Array<u32<caret>>;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_impl_associated_const_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: u32;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        const Const: u32<caret> = 0x0;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_impl_associated_const_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: u32;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        const Const: Array<u32<caret>> = 0x0;
    }
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_impl_generic_bound_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<u32, T>> {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl<T, +Into<u32<caret>, T>> Impl of Trait<T> {}
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_impl_generic_bound_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<u32, T>> {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl<T, +Into<Array<u32<caret>>, T>> Impl of Trait<T> {}
    "#, @r"
    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;

    ==============================

    // → core/src/integer.cairo
    pub extern type <sel>u32</sel>;
    ")
}

#[test]
fn test_builtin_alias_in_use() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    use core::circuit::u96<caret>;
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_alias_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    type TypeAlias = u96<caret>
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_alias_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    type TypeAlias = Array<u96<caret>>
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_variable_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x: u96<caret> = 0x0;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_variable_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x: Array<u96<caret>> = 0x0;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_function_argument_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(x: u96<caret>) {}
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_function_argument_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo(x: Array<u96<caret>>) {}
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_return_type_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() -> u96<caret> { 0x0 }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_return_type_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() -> Array<u96<caret>> { 0x0 }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_struct_field_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    struct Struct {
        x: u96<caret>
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_struct_field_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    struct Struct {
        x: Array<u96<caret>>
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_turbofish_enum_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x = Result::<u96<caret>>::Err(());
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_turbofish_enum_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let x = Result::<Array<u96<caret>>>::Err(());
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_trait_associated_type_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl TraitImpl of Trait {
        type Type = u96<caret>;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_associated_type_via_usage() {
    test_transform_and_macros!(GotoDefinition, r#"
    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl TraitImpl of Trait {
        type Type = felt252;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    fn foo() {
        let _v: TraitImpl::Ty<caret>pe = 123;
    }
    "#, @r"
    trait Trait {
        type Type;
    }

    impl TraitImpl of Trait {
        type <sel>Type</sel> = felt252;
    }

    fn foo() {
        let _v: TraitImpl::Type = 123;
    }

    ==============================

    #[complex_attribute_macro_v2]
    trait Trait {
        type Type;
    }

    #[complex_attribute_macro_v2]
    impl TraitImpl of Trait {
        type <sel>Type</sel> = felt252;
    }

    #[complex_attribute_macro_v2]
    fn foo() {
        let _v: TraitImpl::Type = 123;
    }
    ")
}

#[test]
fn test_builtin_alias_in_trait_associated_type_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = Array<u96<caret>>;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_trait_associated_const_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: u96<caret>;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_trait_associated_const_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: Array<u96<caret>>;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_trait_generic_bound_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<u96<caret>, T>> {}
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_trait_generic_bound_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<Array<u96<caret>>, T>> {}
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_impl_associated_type_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = u96<caret>;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_impl_associated_type_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        type Type;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        type Type = Array<u96<caret>>;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_impl_associated_const_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: u96;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        const Const: u96<caret> = 0x0;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_impl_associated_const_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait {
        const Const: u96;
    }

    <macro>#[complex_attribute_macro_v2]</macro>
    impl Impl of Trait {
        const Const: Array<u96<caret>> = 0x0;
    }
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_impl_generic_bound_as_type() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<u96, T>> {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl<T, +Into<u96<caret>, T>> Impl of Trait<T> {}
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}

#[test]
fn test_builtin_alias_in_impl_generic_bound_as_type_parameter() {
    test_transform_and_macros!(GotoDefinition, r#"
    use core::circuit::u96;

    <macro>#[complex_attribute_macro_v2]</macro>
    trait Trait<T, +Into<u96, T>> {}

    <macro>#[complex_attribute_macro_v2]</macro>
    impl<T, +Into<Array<u96<caret>>, T>> Impl of Trait<T> {}
    "#, @r"
    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =

    ==============================

    // → core/src/circuit.cairo
    pub type <sel>u96</sel> =
    ")
}
