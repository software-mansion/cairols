use crate::rename::rename;
use crate::support::insta::test_transform;

#[test]
fn test_type_alias_in_use() {
    test_transform!(rename, r#"
    mod mod1 {
        struct Struct {};
        type TypeAlias = Struct;
    }
    use mod1::TypeAlias<caret>;
    "#, @r"
    mod mod1 {
        struct Struct {};
        type RENAMED = Struct;
    }
    use mod1::RENAMED;
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_alias_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    type AnotherTypeAlias = TypeAlias<caret>;
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    type AnotherTypeAlias = TypeAlias;
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_alias_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    type AnotherTypeAlias = Array<TypeAlias<caret>>;
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    type AnotherTypeAlias = Array<TypeAlias>;
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_variable_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x: TypeAlias<caret> = 0x0;
    }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo() {
        let x: TypeAlias = 0x0;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_variable_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x: Array<TypeAlias<caret>> = 0x0;
    }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo() {
        let x: Array<TypeAlias> = 0x0;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_function_argument_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo(x: TypeAlias<caret>) {}
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo(x: TypeAlias) {}
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_function_argument_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo(x: Array<TypeAlias<caret>>) {}
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo(x: Array<TypeAlias>) {}
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_return_type_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() -> TypeAlias<caret> { 0x0 }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo() -> TypeAlias { 0x0 }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_return_type_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() -> Array<TypeAlias<caret>> { 0x0 }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo() -> Array<TypeAlias> { 0x0 }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_struct_field_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    struct Struct {
        x: TypeAlias<caret>
    }
    "#, @r"
    struct Struct {};
    type TypeAlias = RENAMED;
    struct RENAMED {
        x: TypeAlias
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_struct_field_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    struct Struct {
        x: Array<TypeAlias<caret>>
    }
    "#, @r"
    struct Struct {};
    type TypeAlias = RENAMED;
    struct RENAMED {
        x: Array<TypeAlias>
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_turbofish_enum_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x = Result::<TypeAlias<caret>>::Err(());
    }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo() {
        let x = Result::<TypeAlias>::Err(());
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_turbofish_enum_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    fn foo() {
        let x = Result::<Array<TypeAlias<caret>>>::Err(());
    }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    fn foo() {
        let x = Result::<Array<TypeAlias>>::Err(());
    }
    ")
}

// FIXME(#404)
#[test]
fn test_type_alias_in_trait_associated_type_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type = TypeAlias<caret>
    }
    "#, @"none response")
}

// FIXME(#404)
#[test]
fn test_type_alias_in_trait_associated_type_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type = Array<TypeAlias<caret>>
    }
    "#, @"none response")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_trait_associated_const_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: TypeAlias<caret>;
    }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    trait Trait {
        const Const: TypeAlias;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_trait_associated_const_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: Array<TypeAlias<caret>>;
    }
    "#, @r"
    struct RENAMED {};
    type TypeAlias = RENAMED;
    trait Trait {
        const Const: Array<TypeAlias>;
    }
    ")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_trait_generic_bound_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<TypeAlias<caret>, T>> {}
    "#, @"none response")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_trait_generic_bound_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<Array<TypeAlias<caret>>, T>> {}
    "#, @"none response")
}

// FIXME(#405)
#[test]
fn test_type_alias_in_impl_associated_type_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = TypeAlias<caret>;
    }
    "#, @r"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type RENAMED;
    }
    impl Impl of Trait {
        type Type = TypeAlias;
    }
    ")
}

// FIXME(#405)
#[test]
fn test_type_alias_in_impl_associated_type_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type Type;
    }
    impl Impl of Trait {
        type Type = Array<TypeAlias<caret>>;
    }
    "#, @r"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        type RENAMED;
    }
    impl Impl of Trait {
        type Type = Array<TypeAlias>;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_impl_associated_const_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: TypeAlias;
    }
    impl Impl of Trait {
        const Const: TypeAlias<caret> = 0x0;
    }
    "#, @r"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const RENAMED: TypeAlias;
    }
    impl Impl of Trait {
        const RENAMED: TypeAlias = 0x0;
    }
    ")
}

// FIXME(#466)
#[test]
fn test_type_alias_in_impl_associated_const_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const Const: TypeAlias;
    }
    impl Impl of Trait {
        const Const: Array<TypeAlias<caret>> = 0x0;
    }
    "#, @r"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait {
        const RENAMED: TypeAlias;
    }
    impl Impl of Trait {
        const RENAMED: Array<TypeAlias> = 0x0;
    }
    ")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_impl_generic_bound_as_type() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<TypeAlias, T>> {}
    impl<T, +Into<TypeAlias<caret>, T>> Impl of Trait<T> {}
    "#, @"none response")
}

// FIXME(#51)
#[test]
fn test_type_alias_in_impl_generic_bound_as_type_parameter() {
    test_transform!(rename, r#"
    struct Struct {};
    type TypeAlias = Struct;
    trait Trait<T, +Into<TypeAlias, T>> {}
    impl<T, +Into<Array<TypeAlias<caret>>, T>> Impl of Trait<T> {}
    "#, @"none response")
}
