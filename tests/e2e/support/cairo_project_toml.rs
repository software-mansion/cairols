use indoc::indoc;

pub const CAIRO_PROJECT_TOML: &str = indoc! {r#"
    [crate_roots]
    hello = "src"
"#};

pub const CAIRO_PROJECT_TOML_2024_07: &str = indoc! {r#"
    [crate_roots]
    hello = "src"

    [config.global]
    edition = "2024_07"
"#};

pub const CAIRO_PROJECT_TOML_2023_11: &str = indoc! {r#"
    [crate_roots]
    hello = "src"

    [config.global]
    edition = "2023_11"
"#};

pub const WELL_KNOWN_CAIRO_PROJECT_TOMLS: &[&str] =
    &[CAIRO_PROJECT_TOML, CAIRO_PROJECT_TOML_2023_11, CAIRO_PROJECT_TOML_2024_07];
