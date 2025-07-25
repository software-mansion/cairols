use lsp_types::ClientCapabilities;
use lsp_types::request::Completion;

use crate::completions::transform;
use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::Cursors;
use crate::support::fixture::Fixture;
use crate::support::insta::{test_transform_plain, test_transform_with_macros};
use crate::support::transform::Transformer;
use crate::support::{MockClient, fixture};

fn lib_cairo_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/aaaa.cairo" => "",
        "src/bbbb.cairo" => "",
        "src/cccc.cairo" => "",
    }
}

#[test]
fn lib_cairo_with_body() {
    test_transform_plain!(Completion, lib_cairo_fixture(),
    "mod cccc; mod <caret>{}",
    @r#"
    caret = """
    mod cccc; mod <caret>{}
    """
    completions = []
    "#);
}

#[test]
fn lib_cairo_without_name_with_semicolon() {
    test_transform_plain!(Completion, lib_cairo_fixture(),
    "mod cccc; mod <caret>;",
    @r#"
    caret = """
    mod cccc; mod <caret>;
    """

    [[completions]]
    completion_label = "aaaa"

    [[completions]]
    completion_label = "bbbb"
    "#);
}

#[test]
fn lib_cairo_without_name_with_semicolon_macro() {
    test_transform_with_macros!(Completion, lib_cairo_fixture(),
    "#[complex_attribute_macro_v2] mod cccc;#[complex_attribute_macro_v2] mod <caret>;",
    @r#"
    caret = """
    #[complex_attribute_macro_v2] mod cccc;#[complex_attribute_macro_v2] mod <caret>;
    """

    [[completions]]
    completion_label = "aaaa"

    [[completions]]
    completion_label = "bbbb"
    "#);
}

#[test]
fn lib_cairo_without_name_without_semicolon() {
    test_transform_plain!(Completion, lib_cairo_fixture(),
    "mod cccc; mod <caret>",
    @r#"
    caret = """
    mod cccc; mod<caret>
    """

    [[completions]]
    completion_label = "aaaa;"

    [[completions]]
    completion_label = "bbbb;"
    "#);
}

#[test]
fn lib_cairo_with_partial_name_with_semicolon() {
    test_transform_plain!(Completion, lib_cairo_fixture(),
    "mod cccc; mod aa<caret>;",
    @r#"
    caret = """
    mod cccc; mod aa<caret>;
    """

    [[completions]]
    completion_label = "aaaa"
    "#);
}

#[test]
fn lib_cairo_with_full_name_with_semicolon() {
    test_transform_plain!(Completion, lib_cairo_fixture(),
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}

struct OtherTopLevelFile;

impl Transformer for OtherTopLevelFile {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        Completion::capabilities(base)
    }

    fn transform(ls: MockClient, cursors: Cursors, _config: Option<serde_json::Value>) -> String {
        transform(ls, cursors, Self::main_file())
    }

    fn main_file() -> &'static str {
        "src/aaaa.cairo"
    }
}

fn other_top_level_file_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => "mod aaaa;",
        "src/aaaa/bbbb.cairo" => "",
        "src/bbbb.cairo" => "",
        "src/aaaa/cccc.cairo" => "",
        "src/cccc.cairo" => "",
    }
}

#[test]
fn other_top_level_file_with_body() {
    test_transform_plain!(OtherTopLevelFile, other_top_level_file_fixture(),
    "mod cccc; mod <caret>{}",
    @r#"
    caret = """
    mod cccc; mod <caret>{}
    """
    completions = []
    "#);
}

#[test]
fn other_top_level_file_without_name_with_semicolon() {
    test_transform_plain!(OtherTopLevelFile, other_top_level_file_fixture(),
    "mod cccc; mod <caret>;",
    @r#"
    caret = """
    mod cccc; mod <caret>;
    """

    [[completions]]
    completion_label = "bbbb"
    "#);
}

#[test]
fn other_top_level_file_with_partial_name_with_semicolon() {
    test_transform_plain!(OtherTopLevelFile, other_top_level_file_fixture(),
    "mod cccc; mod aa<caret>;",
    @r#"
    caret = """
    mod cccc; mod aa<caret>;
    """
    completions = []
    "#);
}

#[test]
fn other_top_level_file_with_partial_name_without_semicolon() {
    test_transform_plain!(OtherTopLevelFile, other_top_level_file_fixture(),
    "mod cccc; mod aa<caret>",
    @r#"
    caret = """
    mod cccc; mod aa<caret>
    """
    completions = []
    "#);
}

#[test]
fn other_top_level_file_with_full_name_with_semicolon() {
    test_transform_plain!(OtherTopLevelFile, other_top_level_file_fixture(),
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}

struct NestedFile;

impl Transformer for NestedFile {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        Completion::capabilities(base)
    }

    fn transform(ls: MockClient, cursors: Cursors, _config: Option<serde_json::Value>) -> String {
        transform(ls, cursors, Self::main_file())
    }

    fn main_file() -> &'static str {
        "src/x/d.cairo"
    }
}

fn nested_file_fixture() -> Fixture {
    fixture! {
        "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
        "src/lib.cairo" => "mod x;",
        "src/x/d/aaaa.cairo" => "",
        "src/x/d/bbbb.cairo" => "",
        "src/x/d/cccc.cairo" => "",
        "src/x.cairo" => "mod d;",
        "src/dddd.cairo" => "",
    }
}

#[test]
fn nested_file_with_body() {
    test_transform_plain!(NestedFile, nested_file_fixture(),
    "mod cccc; mod <caret>{}",
    @r#"
    caret = """
    mod cccc; mod <caret>{}
    """
    completions = []
    "#);
}

#[test]
fn nested_file_without_name_with_semicolon() {
    test_transform_plain!(NestedFile, nested_file_fixture(),
    "mod cccc; mod <caret>;",
    @r#"
    caret = """
    mod cccc; mod <caret>;
    """

    [[completions]]
    completion_label = "aaaa"

    [[completions]]
    completion_label = "bbbb"
    "#);
}

#[test]
fn nested_file_with_partial_name_without_semicolon() {
    test_transform_plain!(NestedFile, nested_file_fixture(),
    "mod cccc; mod aa<caret>",
    @r#"
    caret = """
    mod cccc; mod aa<caret>
    """

    [[completions]]
    completion_label = "aaaa;"
    "#);
}

#[test]
fn nested_file_with_partial_name_with_semicolon() {
    test_transform_plain!(NestedFile, nested_file_fixture(),
    "mod cccc; mod aa<caret>;",
    @r#"
    caret = """
    mod cccc; mod aa<caret>;
    """

    [[completions]]
    completion_label = "aaaa"
    "#);
}

#[test]
fn nested_file_with_full_name_with_semicolon() {
    test_transform_plain!(NestedFile, nested_file_fixture(),
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}
