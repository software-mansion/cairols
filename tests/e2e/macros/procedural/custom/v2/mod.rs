use indoc::indoc;

use crate::macros::{fixtures::ProjectWithCustomMacrosV2, test_macro_expansion_and_diagnostics};

#[test]
fn inline_simple_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = simple<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn inline_complex_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = complex<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn inline_improper_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = improper<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

#[test]
fn inline_with_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    error<caret>_inline_macro_v2!(10 +<caret> 10);
                }
            "#)
        }
    );
}

#[test]
fn attribute_simple_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[simple_attribute_macro_v2]<caret>
                fn <caret>foo() {<caret>}
            "#)
        }
    );
}

#[test]
fn attribute_complex_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[complex_attribute_macro_v2]<caret>
                fn <caret>foo() {<caret>}
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn attribute_improper_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[improper_attribute_macro_v2]<caret>
                fn f<caret>oo() {<caret>}
            "#)
        }
    );
}

#[test]
fn attribute_with_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[error_attribute_macro_v2]<caret>
                fn f<caret>oo() {<caret>}
            "#)
        }
    );
}

#[test]
fn derive_simple_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Simple<caret>DeriveMacroV2)]
                struct Empty<caret>Struct {<caret>}
            "#)
        }
    );
}

#[test]
fn derive_complex_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Complex<caret>DeriveMacroV2)]
                struct Empty<caret>Struct {<caret>}
            "#)
        }
    );
}

// TODO(#535): Diagnostics from V2 wrongly mapped
#[ignore = "Wrong diagnostics mappings from scarb nightly"]
#[test]
fn derive_improper_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Improper<caret>DeriveMacroV2)]
                struct Empty<caret>Struct {<caret>}
            "#)
        }
    );
}

#[test]
fn derive_with_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Error<caret>DeriveMacroV2)]
                struct Empty<caret>Struct {<caret>}
            "#)
        }
    );
}

#[test]
fn attribute_with_quote_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {"test_package/src/lib.cairo" => indoc!(r#"
            #[wrap_with_module]
            fn funksia<caret>() -> felt252 {
                123
            }
        "#)
        }
    );
}

#[test]
fn attribute_with_quote_user_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[wrap_with_module]
                fn funk<caret>sia() -> felt252 {
                    abc = 123
                    abc
                }
            "#)
        }
    );
}
