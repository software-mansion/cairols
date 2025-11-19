use cairo_language_server::lsp;
use indoc::indoc;
use lsp_types::NumberOrString;

use crate::support::normalize::normalize;
use crate::support::sandbox;

#[test]
fn test_simple_deps() {
    let mut ls = sandbox! {
        files {
            "a/Scarb.toml" => indoc! (r#"
                [package]
                name = "a"
                version = "0.1.0"
                edition = "2024_07"

                [dependencies]
                b = { path = "../b" }
            "#),
            "a/src/lib.cairo" => indoc!(r#"
                use b::Foo;

                fn main() {
                    let foo = Foo::Bar;
                    match foo {
                        Foo::Baz => {},
                        _ => {}
                    }
                }
            "#),
            "b/Scarb.toml" => indoc!(r#"
                [package]
                name = "b"
                version = "0.1.0"
                edition = "2024_07"
            "#),
            "b/src/lib.cairo" => indoc!(r#"
                pub enum Foo {
                    Bar,
                    Baz,
                }

                mod non_existent;
            "#),
        }
    };

    assert!(ls.open_and_wait_for_diagnostics("a/src/lib.cairo").is_empty());

    // Check if opening `a` triggers calculating diagnostics for `b`.
    let diagnostics_from_b = ls.get_diagnostics_for_file("b/src/lib.cairo");
    assert_eq!(diagnostics_from_b.len(), 1);
    assert_eq!(diagnostics_from_b[0].code, Some(NumberOrString::String("E0005".to_string())));

    let analyzed_crates = ls.send_request::<lsp::ext::ViewAnalyzedCrates>(());

    insta::assert_snapshot!(normalize(&ls, analyzed_crates))
}
