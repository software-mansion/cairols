use crate::macros::fixtures::ProjectWithCustomMacrosV1AndV2;
use crate::macros::test_macro_expansion_and_diagnostics;
use indoc::indoc;

#[test]
fn mixed_inline_simple() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = simple<caret>_inline_macro!(10 +<caret> 10);
                    let _y = simple<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#),
        }
    );
}

#[test]
fn mixed_inline_complex() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = complex<caret>_inline_macro!(10 +<caret> 10);
                    let _y = complex<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn mixed_inline_improper() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = improper<caret>_inline_macro!(10 +<caret> 10);
                }

                fn bar() {
                    let _y = improper<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

#[test]
fn mixed_inline_with_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    error<caret>_inline_macro!(10 +<caret> 10);
                    error<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn mixed_attributes_simple() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[simple_attribute_macro]<caret>
                fn <caret>foo() {<caret>}

                #[simple_attribute_macro_v2]<caret>
                fn <caret>foo_v2() {<caret>}
            "#),

        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn mixed_attribute_complex() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[complex_attribute_macro]<caret>
                fn <caret>foo() {<caret>}

                #[complex_attribute_macro_v2]<caret>
                fn <caret>bar() {<caret>}
            "#)

        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn mixed_attribute_improper() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[improper_attribute_macro]<caret>
                fn f<caret>oo() {<caret>}

                #[improper_attribute_macro_v2]<caret>
                fn b<caret>ar() {<caret>}
            "#)
        }
    );
}

#[test]
fn mixed_attribute_with_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[error_attribute_macro]<caret>
                fn f<caret>oo() {<caret>}

                #[error_attribute_macro_v2]<caret>
                fn b<caret>ar() {<caret>}
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn mixed_derive_simple() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Simple<caret>DeriveMacro)]
                struct Empty<caret>Struct {<caret>}

                #[derive<caret>(Simple<caret>DeriveMacroV2)]
                struct Empty<caret>StructV2 {<caret>}
            "#),
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[test]
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
fn mixed_derive_complex() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Complex<caret>DeriveMacro)]
                struct Empty<caret>Struct {<caret>}

                #[derive<caret>(Complex<caret>DeriveMacroV2)]
                struct Empty<caret>StructV2 {<caret>}
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[test]
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
fn mixed_derive_improper() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Improper<caret>DeriveMacro)]
                struct Empty<caret>Struct {<caret>}

                #[derive<caret>(Improper<caret>DeriveMacroV2)]
                struct Empty<caret>StructV2 {<caret>}
            "#)
        }
    );
}

#[test]
fn mixed_derive_with_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV1AndV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Error<caret>DeriveMacro)]
                struct Empty<caret>Struct {<caret>}

                #[derive<caret>(Error<caret>DeriveMacroV2)]
                struct Empty<caret>StructV2 {<caret>}
            "#)
        }
    );
}
