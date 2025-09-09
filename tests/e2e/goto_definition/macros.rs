use lsp_types::request::GotoDefinition;

use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn inline_macro() {
    test_transform_plain!(GotoDefinition, r#"
    fn main() {
        prin<caret>t!("Hello, world!");
    }
    "#, @"none response")
}

#[test]
fn inline_macro_with_macros() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    fn main() {
        prin<caret>t!("Hello, world!");
    }
    "#, @"none response")
}

#[test]
fn declarative_macro_local_definition() {
    test_transform_plain!(GotoDefinition, r#"
    pub macro add_one {
        ($x:expr) => { $x + 1 };
    }
    fn main() {
        let y = add_<caret>one!(41);
    }
    "#, @r"
    pub macro <sel>add_one</sel> {
        ($x:expr) => { $x + 1 };
    }
    fn main() {
        let y = add_one!(41);
    }
    ")
}

#[test]
fn declarative_macro_local_definition_with_macros() {
    test_transform_with_macros!(GotoDefinition, r#"
    #[complex_attribute_macro_v2]
    pub macro add_one {
        ($x:expr) => { $x + 1 };
    }

    #[complex_attribute_macro_v2]
    fn main() {
        let y = add_<caret>one!(41);
    }
    "#, @r"
    #[complex_attribute_macro_v2]
    pub macro <sel>add_one</sel> {
        ($x:expr) => { $x + 1 };
    }

    #[complex_attribute_macro_v2]
    fn main() {
        let y = add_one!(41);
    }
    ")
}

#[test]
fn declarative_macro_on_definition() {
    test_transform_plain!(GotoDefinition, r#"
    pub macro ad<caret>d_one {
        ($x:expr) => { $x + 1 };
    }

    fn xyz() {
        add_one!(1);
    }
    "#, @r"
    pub macro <sel>add_one</sel> {
        ($x:expr) => { $x + 1 };
    }

    fn xyz() {
        add_one!(1);
    }
    ")
}

#[test]
fn declarative_macro_in_nested_module() {
    test_transform_plain!(GotoDefinition, r#"
    mod math {
        pub macro add_two {
            ($x:expr) => { $x + 2 };
        }
    }
    fn main() {
        let z = math::add_<caret>two!(40);
    }
    "#, @r"
    mod math {
        pub macro <sel>add_two</sel> {
            ($x:expr) => { $x + 2 };
        }
    }
    fn main() {
        let z = math::add_two!(40);
    }
    ")
}

#[test]
fn declarative_macro_reexported() {
    test_transform_plain!(GotoDefinition, r#"
    mod inner {
        pub macro inc {
            ($x:expr) => { $x + 1 };
        }
    }
    use inner::in<caret>c;
    "#, @r"
    mod inner {
        pub macro <sel>inc</sel> {
            ($x:expr) => { $x + 1 };
        }
    }
    use inner::inc;
    ")
}

#[test]
fn declarative_macros_with_same_name_in_different_modules() {
    test_transform_plain!(GotoDefinition, r#"
    mod a {
        pub macro make {
            () => { 1 };
        }
    }
    mod b {
        pub macro make {
            () => { 2 };
        }
    }
    fn main() {
        let x = a::ma<caret>ke!();
    }
    "#, @r"
    mod a {
        pub macro <sel>make</sel> {
            () => { 1 };
        }
    }
    mod b {
        pub macro make {
            () => { 2 };
        }
    }
    fn main() {
        let x = a::make!();
    }
    ")
}
