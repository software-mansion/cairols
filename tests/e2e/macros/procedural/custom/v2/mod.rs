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
fn inline_with_location_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    error<caret>_inline_with_location_macro_v2!(10 +<caret> 10);
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
fn attribute_with_location_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[error_attribute_with_location_macro_v2]<caret>
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
fn derive_with_location_error_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Error<caret>DeriveWithLocationMacroV2)]
                struct Empty<caret>Struct {<caret>}
            "#)
        }
    );
}

#[test]
fn inline_simple_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = simple<caret>_inline_macro_v2!(10_u8 +<caret> 5_felt252);
                }
            "#)
        }
    );
}

#[test]
fn inline_complex_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = complex<caret>_inline_macro_v2!([1_u8, 2_felt252]);
                }
            "#)
        }
    );
}

#[test]
fn inline_improper_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    let _x = improper<caret>_inline_macro_v2!({ valid_syntax_btw = 10 });
                }
            "#)
        }
    );
}

#[test]
fn inline_with_error_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                fn foo() {
                    error<caret>_inline_macro_v2!(10 +<caret> "wrong");
                }
            "#)
        }
    );
}

#[test]
fn inline_simple_module_level_v2() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                mod fu {
                    simple_module_level_inline_macro_v2!()<caret>;
                }
                fn main() {
                    fu::foo();
                }
            "#)
        }
    );
}

#[test]
fn attribute_simple_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[simple_attribute_macro_v2]<caret>
                fn <caret>foo() { a = 5; <caret>}
            "#)
        }
    );
}

#[test]
fn attribute_complex_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[complex_attribute_macro_v2]<caret>
                fn <caret>foo() { let x: u8 = 5_u32; <caret>}
            "#)
        }
    );
}

#[test]
fn attribute_improper_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[improper_attribute_macro_v2]<caret>
                fn f<caret>oo() { 5_u8 + 5_felt252; <caret>}
            "#)
        }
    );
}

#[test]
fn attribute_with_error_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[error_attribute_macro_v2]<caret>
                fn f<caret>oo() { "a" + "b"; <caret>}
            "#)
        }
    );
}

#[test]
fn attribute_with_mod_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[mod_attribute_macro_v2]<caret>
                fn f<caret>oo() { "a" + "b"; <caret>}
            "#)
        }
    );
}

#[test]
fn derive_simple_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Simple<caret>DeriveMacroV2)]
                struct Empty<caret>Struct { x: unknown_type <caret>}
            "#)
        }
    );
}

#[test]
fn derive_complex_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Complex<caret>DeriveMacroV2)]
                struct Empty<caret>Struct { x: u8, x: u8 <caret>}
            "#)
        }
    );
}

#[test]
fn derive_improper_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Improper<caret>DeriveMacroV2)]
                struct Empty<caret>Struct { a: b,<caret>}
            "#)
        }
    );
}

#[test]
fn derive_with_error_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Error<caret>DeriveMacroV2)]
                struct Empty<caret>Struct { a: b, <caret>}
            "#)
        }
    );
}

#[test]
fn derive_with_mod_v2_with_user_error() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithCustomMacrosV2,
        cwd = "test_package",
        files {
            "test_package/src/lib.cairo" => indoc!(r#"
                #[derive<caret>(Mod<caret>DeriveMacroV2)]
                struct My<caret>Struct { a: b, <caret>}
            "#)
        }
    );
}
