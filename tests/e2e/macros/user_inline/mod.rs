use indoc::indoc;

use crate::macros::{
    fixtures::ProjectWithUserDefinedInlineMacros, test_macro_expansion_and_diagnostics,
};

#[test]
fn test_simple_expr_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithUserDefinedInlineMacros,
        cwd = "test_package",
        files {
            "test_package/src/a.cairo" => indoc!(r#"
                use test_package::add_one;

                fn foo() {
                    let x = 1;
                    let _ = add_on<caret>e!(x);
                }
            "#)
        }
    );
}

#[test]
fn test_multiple_branch_expr_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithUserDefinedInlineMacros,
        cwd = "test_package",
        files {
            "test_package/src/a.cairo" => indoc!(r#"
                use test_package::add_many;

                fn foo() {
                    let x = 1;
                    let y = 1;
                    let z = add_many<caret>!(x, y);
                    let _ = add_many<caret>!(x, y, z);
                }
            "#)
        }
    );
}

#[test]
fn test_repetition_expr_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithUserDefinedInlineMacros,
        cwd = "test_package",
        files {
            "test_package/src/a.cairo" => indoc!(r#"
                use test_package::build_array;

                fn foo() {
                    let _full = build_arr<caret>ay!(1, 2, 3);
                    let _empty: Array<felt252> = build_arr<caret>ay!();
                }
            "#)
        }
    );
}

#[test]
fn test_statement_macros() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithUserDefinedInlineMacros,
        cwd = "test_package",
        files {
            "test_package/src/a.cairo" => indoc!(r#"
                use test_package::{append_twice, declare_two};

                fn foo() {
                    declare_t<caret>wo!(10);
                    let mut data: Array<felt252> = ArrayTrait::new();
                    append_t<caret>wice!(data, 1);
                }
            "#)
        }
    );
}

// Two independent item-level macro invocations on adjacent lines: expanding one
// must not leak fragments of the other expansion into the result.
//
// Note: the item-level macros are defined in the same module they are invoked in,
// like in the compiler test suite. Importing a macro with `use` and invoking it
// at the top level of the importing module results in `E2025: Cycle detected
// while resolving 'use' items`.
#[test]
fn test_adjacent_top_level_item_macros() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithUserDefinedInlineMacros,
        cwd = "test_package",
        files {
            "test_package/src/a.cairo" => indoc!(r#"
                macro define_fn {
                    ($name:ident) => {
                        expose! {
                            fn $name() -> felt252 {
                                42
                            }
                        }
                    };
                }

                macro define_struct {
                    ($name:ident) => {
                        expose! {
                            struct $name {
                                pub value: felt252,
                            }
                        }
                    };
                }

                define_f<caret>n!(the_answer);
                define_str<caret>uct!(Wrapper);

                fn foo() -> felt252 {
                    let w = Wrapper { value: the_answer() };
                    w.value
                }
            "#)
        }
    );
}

// Note: the item-level macro is defined in the same module it is invoked in,
// like in the compiler test suite. Importing a macro with `use` and invoking it
// at the top level of the importing module results in `E2025: Cycle detected
// while resolving 'use' items`.
#[test]
fn test_top_level_item_macro() {
    test_macro_expansion_and_diagnostics!(
        project = ProjectWithUserDefinedInlineMacros,
        cwd = "test_package",
        files {
            "test_package/src/a.cairo" => indoc!(r#"
                macro define_fn_and_struct {
                    ($name:ident) => {
                        expose! {
                            fn $name() -> felt252 {
                                42
                            }

                            struct Wrapper {
                                pub value: felt252,
                            }

                            fn get_value(s: Wrapper) -> felt252 {
                                s.value
                            }
                        }
                    };
                }

                define_fn_an<caret>d_struct!(the_answer);

                fn foo() -> felt252 {
                    let w = Wrapper { value: the_answer() };
                    get_value(w)
                }
            "#)
        }
    );
}
