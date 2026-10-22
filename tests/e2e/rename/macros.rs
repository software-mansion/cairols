use lsp_types::request::Rename;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
#[should_panic(expected = "not supported")]
fn plugin_inline_macro() {
    test_transform_plain!(Rename, r#"
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }
    fn bar() {
        let forty_two = array![42];
    }
    "#, @"");
}

#[test]
#[should_panic(expected = "not supported")]
fn plugin_inline_macro_with_macros() {
    test_transform_with_macros!(Rename, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        print!("Hello world!");
        let arr = arr<caret>ay![1, 2, 3, 4, 5];
    }

    #[complex_attribute_macro_v2]
    fn bar() {
        let forty_two = array![42];
    }
    "#, @"");
}

#[test]
fn declarative_inline_macro_on_usage() {
    test_transform_plain!(Rename, r#"
    fn main() {
        let x = i<caret>nc!(0);
    }

    use modzik::inc;

    mod modzik {
        /// Increment number by one.
        pub macro inc {
            ($x:expr) => { $x + 1 };
        }
    }
    "#, @r"
    fn main() {
        let x = RENAMED!(0);
    }

    use modzik::RENAMED;

    mod modzik {
        /// Increment number by one.
        pub macro RENAMED {
            ($x:expr) => { $x + 1 };
        }
    }
    ");
}

#[test]
fn declarative_inline_macro_on_definition() {
    test_transform_plain!(Rename, r#"
    fn main() {
        let x = inc!(0);
    }

    use modzik::inc;

    mod modzik {
        /// Increment number by one.
        pub macro i<caret>nc {
            ($x:expr) => { $x + 1 };
        }
    }
    "#, @r"
    fn main() {
        let x = RENAMED!(0);
    }

    use modzik::RENAMED;

    mod modzik {
        /// Increment number by one.
        pub macro RENAMED {
            ($x:expr) => { $x + 1 };
        }
    }
    ");
}

#[test]
fn declarative_inline_macro_on_definition_with_macros() {
    test_transform_with_macros!(Rename, r#"
    fn main() {
        let x = inc!(0);
    }

    use modzik::inc;

    #[complex_attribute_macro_v2]
    mod modzik {
        /// Increment number by one.
        pub macro i<caret>nc {
            ($x:expr) => { $x + 1 };
        }
    }
    "#, @r"
    fn main() {
        let x = RENAMED!(0);
    }

    use modzik::RENAMED;

    #[complex_attribute_macro_v2]
    mod modzik {
        /// Increment number by one.
        pub macro RENAMED {
            ($x:expr) => { $x + 1 };
        }
    }
    ");
}

#[test]
fn top_level_declarative_macro_on_definition() {
    test_transform_plain!(Rename, r#"
    pub macro decl<caret>are_mod {
        ($name:ident) => { mod $name {} };
    }

    declare_mod!(modzik);
    "#, @r"
    pub macro RENAMED {
        ($name:ident) => { mod $name {} };
    }

    RENAMED!(modzik);
    ")
}

#[test]
fn top_level_declarative_macro_on_usage() {
    test_transform_plain!(Rename, r#"
    pub macro declare_mod {
        ($name:ident) => { mod $name {} };
    }

    decla<caret>re_mod!(modzik);
    "#, @r"
    pub macro RENAMED {
        ($name:ident) => { mod $name {} };
    }

    RENAMED!(modzik);
    ")
}

#[test]
fn struct_constructor_shorthand_in_declarative_macro() {
    test_transform_plain!(Rename, r"
    #[derive(Drop)]
    struct Foo { field: felt252 }

    pub macro my_macro {
        ($x:expr) => { $x };
    }

    fn main() {
        let field = 42;
        let x = my_macro!(Foo { fie<caret>ld });
    }
    ", @"")
}

#[test]
fn struct_member_via_definition_with_shorthand_in_declarative_macro() {
    test_transform_plain!(Rename, r"
    #[derive(Drop)]
    struct Foo { fie<caret>ld: felt252 }

    pub macro my_macro {
        ($x:expr) => { $x };
    }

    fn main() {
        let field = 42;
        let x = my_macro!(Foo { field });
    }
    ", @"
    #[derive(Drop)]
    struct Foo { RENAMED: felt252 }

    pub macro my_macro {
        ($x:expr) => { $x };
    }

    fn main() {
        let field = 42;
        let x = my_macro!(Foo { field });
    }
    ")
}

#[test]
fn var_via_binding_with_shorthand_in_declarative_macro() {
    test_transform_plain!(Rename, r"
    #[derive(Drop)]
    struct Foo { field: felt252 }

    pub macro my_macro {
        ($x:expr) => { $x };
    }

    fn main() {
        let fie<caret>ld = 42;
        let x = my_macro!(Foo { field });
    }
    ", @"
    #[derive(Drop)]
    struct Foo { field: felt252 }

    pub macro my_macro {
        ($x:expr) => { $x };
    }

    fn main() {
        let RENAMED = 42;
        let x = my_macro!(Foo { field });
    }
    ")
}

#[test]
fn struct_constructor_shorthand_in_plugin_inline_macro() {
    test_transform_plain!(Rename, r"
    #[derive(Drop)]
    struct Foo { field: felt252 }
    fn main() {
        let field = 42;
        let x = array![Foo { fie<caret>ld }];
    }
    ", @"
    #[derive(Drop)]
    struct Foo { field: felt252 }
    fn main() {
        let RENAMED = 42;
        let x = array![Foo { field: RENAMED }];
    }
    ")
}

#[test]
fn struct_constructor_shorthand_in_declarative_macro_with_macros() {
    test_transform_with_macros!(Rename, r"
    #[derive(Drop)]
    struct Foo { field: felt252 }

    pub macro my_macro {
        ($x:expr) => { $x };
    }

    #[complex_attribute_macro_v2]
    fn main() {
        let field = 42;
        let x = my_macro!(Foo { fie<caret>ld });
    }
    ", @"")
}
