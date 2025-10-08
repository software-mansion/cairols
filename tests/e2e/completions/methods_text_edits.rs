use indoc::indoc;
use lsp_types::request::Completion;

use crate::completions::completion_fixture;
use crate::support::fixture;
use crate::support::fixture::Fixture;
use crate::support::insta::{test_transform_plain, test_transform_with_macros};

#[test]
fn simple_trait_macro() {
    test_transform_with_macros!(Completion, completion_fixture(),"
    #[complex_attribute_macro_v2]
    mod hidden_trait {
        pub trait ATrait1<T> {
            fn some_method(self: @T);
        }
        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }

    use hidden_trait::ATrait1;

    #[complex_attribute_macro_v2]
    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>
        }
    }
    ",@r#"
    caret = """
            x.some_me<caret>
    """

    [[completions]]
    completion_label = "some_method()"
    detail = "fn(self: @T) -> ()"
    insert_text = "some_method()"
    text_edits = ["""
    use crate::ATrait1;

    """]
    "#)
}

#[test]
fn simple_trait() {
    test_transform_plain!(Completion, completion_fixture(),"
    mod hidden_trait {
        pub trait ATrait1<T> {
            fn some_method(self: @T);
        }
        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }

    use hidden_trait::ATrait1;

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>
        }
    }
    ",@r#"
    caret = """
            x.some_me<caret>
    """

    [[completions]]
    completion_label = "some_method()"
    detail = "fn(self: @T) -> ()"
    insert_text = "some_method()"
    text_edits = ["""
    use crate::ATrait1;

    """]
    "#);
}

#[test]
fn non_directly_visible_trait() {
    test_transform_plain!(Completion, completion_fixture(),"
    mod hidden_trait {
        pub trait ATrait1<T> {
            fn some_method(self: @T);
        }

        impl Felt252ATraitImpl of ATrait1<felt252> {
            fn some_method(self: @felt252) {}
        }
    }

    use hidden_trait::ATrait1;

    mod inner_mod {
        fn main() {
            let x = 5_felt252;
            x.some_me<caret>
        }
    }
    ",@r#"
    caret = """
            x.some_me<caret>
    """

    [[completions]]
    completion_label = "some_method()"
    detail = "fn(self: @T) -> ()"
    insert_text = "some_method()"
    text_edits = ["""
    use crate::ATrait1;

    """]
    "#);
}

fn only_dependencies_methods_included_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc! {r#"
            [crate_roots]
            this = "src"
            dep = "dep"

            [config.override.this]
            edition = "2024_07"

            [config.override.this.dependencies]
            dep = { discriminator = "dep" }
        "#},
        "dep/lib.cairo" =>  indoc! {
        r#"
            pub trait X<T> {
                fn some_method(self: @T);
            }
            impl MyImpl of X<felt252> {
                fn some_method(self: @felt252) {}
            }
        "#}
    }
}

#[test]
fn methods_from_deps_included() {
    test_transform_plain!(Completion, only_dependencies_methods_included_fixture(), "
    fn func() {
        let x = 5_felt252;
        x.some_metho<caret>
    }
    ",@r#"
    caret = """
        x.some_metho<caret>
    """

    [[completions]]
    completion_label = "some_method()"
    detail = "fn(self: @T) -> ()"
    insert_text = "some_method()"
    text_edits = ["""
    use dep::X;

    """]
    "#);
}

fn only_dependencies_methods_excluded_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => indoc! {r#"
            [crate_roots]
            this = "src"
            dep = "dep"
        "#},
        "dep/lib.cairo" =>  indoc! {
        r#"
            pub trait X<T> {
                fn some_method(self: @T);
            }
            impl MyImpl of X<felt252> {
                fn some_method(self: @felt252) {}
            }
        "#}
    }
}

#[test]
fn methods_from_non_deps_excluded() {
    test_transform_plain!(Completion, only_dependencies_methods_excluded_fixture(), "
    fn func() {
        let x = 5_felt252;
        x.some_method<caret>
    }
    ",@r#"
    caret = """
        x.some_method<caret>
    """
    completions = []
    "#);
}
