use crate::find_references::find_references;
use crate::support::insta::test_transform;

#[test]
fn test_extern_type_in_alias_as_type() {
    test_transform!(find_references, r#"
    type TypeAlias = u32<caret>;
    "#, @r"
    // found several references in the core crate
    type TypeAlias = <sel>u32</sel>;
    ")
}

#[test]
fn test_extern_type_in_alias_as_type_parameter() {
    test_transform!(find_references, r#"
    type TypeAlias = Array<u32<caret>>;
    "#, @r"
    // found several references in the core crate
    type TypeAlias = Array<<sel>u32</sel>>;
    ")
}

#[test]
fn test_extern_type_in_variable_as_type() {
    test_transform!(find_references, r#"
    fn foo() {
        let x: u32<caret> = 0x0;
    }
    "#, @r"
    // found several references in the core crate
    fn foo() {
        let x: <sel>u32</sel> = 0x0;
    }
    ")
}

#[test]
fn test_extern_type_in_variable_as_type_parameter() {
    test_transform!(find_references, r#"
    fn foo() {
        let x: Array<u32<caret>> = 0x0;
    }
    "#, @r"
    // found several references in the core crate
    fn foo() {
        let x: Array<<sel>u32</sel>> = 0x0;
    }
    ")
}

#[test]
fn test_extern_type_in_function_argument_as_type() {
    test_transform!(find_references, r#"
    fn foo(x: u32<caret>) {}
    "#, @r"
    // found several references in the core crate
    fn foo(x: <sel>u32</sel>) {}
    ")
}

#[test]
fn test_extern_type_in_function_argument_as_type_parameter() {
    test_transform!(find_references, r#"
    fn foo(x: Array<u32<caret>>) {}
    "#, @r"
    // found several references in the core crate
    fn foo(x: Array<<sel>u32</sel>>) {}
    ")
}

#[test]
fn test_extern_type_in_return_type_as_type() {
    test_transform!(find_references, r#"
    fn foo() -> u32<caret> { 0x0 }
    "#, @r"
    // found several references in the core crate
    fn foo() -> <sel>u32</sel> { 0x0 }
    ")
}

#[test]
fn test_extern_type_in_return_type_as_type_parameter() {
    test_transform!(find_references, r#"
    fn foo() -> Array<u32<caret>> { 0x0 }
    "#, @r"
    // found several references in the core crate
    fn foo() -> Array<<sel>u32</sel>> { 0x0 }
    ")
}

#[test]
fn test_extern_type_in_struct_field_as_type() {
    test_transform!(find_references, r#"
    struct Struct {
        x: u32<caret>
    }
    "#, @r"
    // found several references in the core crate
    struct Struct {
        x: <sel>u32</sel>
    }
    ")
}

#[test]
fn test_extern_type_in_struct_field_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {
        x: Array<u32<caret>>
    }
    "#, @r"
    // found several references in the core crate
    struct Struct {
        x: Array<<sel>u32</sel>>
    }
    ")
}

#[test]
fn test_extern_type_in_turbofish_enum_as_type() {
    test_transform!(find_references, r#"
    fn foo() {
        let x = Result::<u32<caret>>::Err(());
    }
    "#, @r"
    // found several references in the core crate
    fn foo() {
        let x = Result::<<sel>u32</sel>>::Err(());
    }
    ")
}

#[test]
fn test_extern_type_in_turbofish_enum_as_type_parameter() {
    test_transform!(find_references, r#"
    fn foo() {
        let x = Result::<Array<u32<caret>>>::Err(());
    }
    "#, @r"
    // found several references in the core crate
    fn foo() {
        let x = Result::<Array<<sel>u32</sel>>>::Err(());
    }
    "
    )
}

// FIXME(#404)
#[test]
fn test_extern_type_in_trait_associated_type_as_type() {
    test_transform!(find_references, r#"
    trait Trait {
        type Type = u32<caret>
    }
    "#, @"none response")
}

// FIXME(#404)
#[test]
fn test_extern_type_in_trait_associated_type_as_type_parameter() {
    test_transform!(find_references, r#"
    trait Trait {
        type Type = Array<u32<caret>>
    }
    "#, @"none response")
}

#[test]
fn test_extern_type_in_trait_associated_const_as_type() {
    test_transform!(find_references, r#"
    trait Trait {
        const Const: u32<caret>;
    }
    "#, @r"
    // found several references in the core crate
    trait Trait {
        const Const: <sel>u32</sel>;
    }
    ")
}

#[test]
fn test_extern_type_in_trait_associated_const_as_type_parameter() {
    test_transform!(find_references, r#"
    trait Trait {
        const Const: Array<u32<caret>>;
    }
    "#, @r"
    // found several references in the core crate
    trait Trait {
        const Const: Array<<sel>u32</sel>>;
    }
    ")
}

// FIXME(#51)
#[test]
fn test_extern_type_in_trait_generic_bound_as_type() {
    test_transform!(find_references, r#"
    trait Trait<T, +Into<u32<caret>, T>> {}
    "#, @"none response")
}

// FIXME(#51)
#[test]
fn test_extern_type_in_trait_generic_bound_as_type_parameter() {
    test_transform!(find_references, r#"
    trait Trait<T, +Into<Array<u32<caret>>, T>> {}
    "#, @"none response")
}

// FIXME(#405)
#[test]
fn test_extern_type_in_impl_associated_type_as_type() {
    test_transform!(find_references, r#"
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = u32<caret>;
    }
    "#, @r"
    trait Trait {
        type <sel=declaration>Type</sel>;
    }
    impl Impl of Trait {
        type Type = u32;
    }
    ")
}

// FIXME(#405)
#[test]
fn test_extern_type_in_impl_associated_type_as_type_parameter() {
    test_transform!(find_references, r#"
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<u32<caret>>;
    }
    "#, @r"
    trait Trait {
        type <sel=declaration>Type</sel>;
    }
    impl Impl of Trait {
        type Type = Array<u32>;
    }
    ")
}

// FIXME(#405)
#[test]
fn test_extern_type_in_impl_associated_const_as_type() {
    test_transform!(find_references, r#"
    trait Trait {
        const Const: u32;
    }
    impl Impl of Trait {
        const Const: u32<caret> = 0x0;
    }
    "#, @r"
    trait Trait {
        const <sel=declaration>Const</sel>: u32;
    }
    impl Impl of Trait {
        const <sel>Const</sel>: u32 = 0x0;
    }
    ")
}

// FIXME(#405)
#[test]
fn test_extern_type_in_impl_associated_const_as_type_parameter() {
    test_transform!(find_references, r#"
    trait Trait {
        const Const: u32;
    }
    impl Impl of Trait {
        const Const: Array<u32<caret>> = 0x0;
    }
    "#, @r"
    trait Trait {
        const <sel=declaration>Const</sel>: u32;
    }
    impl Impl of Trait {
        const <sel>Const</sel>: Array<u32> = 0x0;
    }
    ")
}

// FIXME(#51)
#[test]
fn test_extern_type_in_impl_generic_bound_as_type() {
    test_transform!(find_references, r#"
    trait Trait<T, +Into<u32, T>> {}
    impl<T, +Into<u32<caret>, T>> Impl of Trait<T> {}
    "#, @"none response")
}

// FIXME(#51)
#[test]
fn test_extern_type_in_impl_generic_bound_as_type_parameter() {
    test_transform!(find_references, r#"
    trait Trait<T, +Into<u32, T>> {}
    impl<T, +Into<Array<u32<caret>>, T>> Impl of Trait<T> {}
    "#, @"none response")
}

#[test]
fn test_type_alias_in_use() {
    test_transform!(find_references, r#"
    mod mod1 {
        struct Struct {}
        type TypeAlias = Struct;
    }
    use mod1::TypeAlias<caret>;
    "#, @r"
    mod mod1 {
        struct Struct {}
        type <sel=declaration>TypeAlias</sel> = Struct;
    }
    use mod1::<sel>TypeAlias</sel>;
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_alias_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    type TypeAlias = SomeTypeAlias<caret>;
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    type TypeAlias = SomeTypeAlias;
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_alias_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    type TypeAlias = Array<SomeTypeAlias<caret>>;
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    type TypeAlias = Array<SomeTypeAlias>;
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_variable_as_type() {
    test_transform!(find_references,
    r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo() {
        let x: SomeTypeAlias<caret> = 0x0;
    }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo() {
        let x: SomeTypeAlias = 0x0;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_variable_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo() {
        let x: Array<SomeTypeAlias<caret>> = 0x0;
    }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo() {
        let x: Array<SomeTypeAlias> = 0x0;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_function_argument_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo(x: SomeTypeAlias<caret>) {}
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo(x: SomeTypeAlias) {}
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_function_argument_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo(x: Array<SomeTypeAlias<caret>>) {}
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo(x: Array<SomeTypeAlias>) {}
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_return_type_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo() -> SomeTypeAlias<caret> { 0x0 }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo() -> SomeTypeAlias { 0x0 }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_return_type_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo() -> Array<SomeTypeAlias<caret>> { 0x0 }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo() -> Array<SomeTypeAlias> { 0x0 }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_struct_field_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    struct Struct {
        x: SomeTypeAlias<caret>
    }
    "#, @r"
    struct Struct {}
    type SomeTypeAlias = <sel>Struct</sel>;
    struct <sel=declaration>Struct</sel> {
        x: SomeTypeAlias
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_struct_field_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    struct Struct {
        x: Array<SomeTypeAlias<caret>>
    }
    "#, @r"
    struct Struct {}
    type SomeTypeAlias = <sel>Struct</sel>;
    struct <sel=declaration>Struct</sel> {
        x: Array<SomeTypeAlias>
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_turbofish_enum_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo() {
        let x = Result::<SomeTypeAlias<caret>>::Err(());
    }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo() {
        let x = Result::<SomeTypeAlias>::Err(());
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_turbofish_enum_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    fn foo() {
        let x = Result::<Array<SomeTypeAlias<caret>>>::Err(());
    }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    fn foo() {
        let x = Result::<Array<SomeTypeAlias>>::Err(());
    }
    ")
}

// FIXME(#404)
#[test]
fn test_type_alias_in_trait_associated_type_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        type Type = SomeTypeAlias<caret>
    }
    "#, @"none response")
}

// FIXME(#404)
#[test]
fn test_type_alias_in_trait_associated_type_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        type Type = Array<SomeTypeAlias<caret>>
    }
    "#, @"none response")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_trait_associated_const_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        const Const: SomeTypeAlias<caret>;
    }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    trait Trait {
        const Const: SomeTypeAlias;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_trait_associated_const_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        const Const: Array<SomeTypeAlias<caret>>;
    }
    "#, @r"
    struct <sel=declaration>Struct</sel> {}
    type SomeTypeAlias = <sel>Struct</sel>;
    trait Trait {
        const Const: Array<SomeTypeAlias>;
    }
    ")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_trait_generic_bound_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait<T, +Into<SomeTypeAlias<caret>, T>> {}
    "#, @"none response")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_trait_generic_bound_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait<T, +Into<Array<SomeTypeAlias<caret>>, T>> {}
    "#, @"none response")
}

// FIXME(#405)
#[test]
fn test_type_alias_in_impl_associated_type_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = SomeTypeAlias<caret>;
    }
    "#, @r"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        type <sel=declaration>Type</sel>;
    }
    impl Impl of Trait {
        type Type = SomeTypeAlias;
    }
    ")
}

// FIXME(#405)
#[test]
fn test_type_alias_in_impl_associated_type_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<SomeTypeAlias<caret>>;
    }
    "#, @r"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        type <sel=declaration>Type</sel>;
    }
    impl Impl of Trait {
        type Type = Array<SomeTypeAlias>;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_impl_associated_const_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        const Const: SomeTypeAlias;
    }
    impl Impl of Trait {
        const Const: SomeTypeAlias<caret> = 0x0;
    }
    "#, @r"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        const <sel=declaration>Const</sel>: SomeTypeAlias;
    }
    impl Impl of Trait {
        const <sel>Const</sel>: SomeTypeAlias = 0x0;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_impl_associated_const_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        const Const: Array<SomeTypeAlias>;
    }
    impl Impl of Trait {
        const Const: Array<SomeTypeAlias<caret>> = 0x0;
    }
    "#, @r"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait {
        const <sel=declaration>Const</sel>: Array<SomeTypeAlias>;
    }
    impl Impl of Trait {
        const <sel>Const</sel>: Array<SomeTypeAlias> = 0x0;
    }
    ")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_impl_generic_bound_as_type() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait<T, +Into<SomeTypeAlias, T>> {}
    impl<T, +Into<SomeTypeAlias<caret>, T>> Impl of Trait<T> {}
    "#, @"none response")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_impl_generic_bound_as_type_parameter() {
    test_transform!(find_references, r#"
    struct Struct {}
    type SomeTypeAlias = Struct;
    trait Trait<T, +Into<SomeTypeAlias, T>> {}
    impl<T, +Into<Array<SomeTypeAlias<caret>>, T>> Impl of Trait<T> {}
    "#, @"none response")
}
