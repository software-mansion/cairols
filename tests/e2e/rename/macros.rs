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
