use crate::completions::transform;
use crate::support::MockClient;
use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::cursor::Cursors;
use crate::support::fixture::Fixture;
use crate::support::insta::test_transform_plain;
use crate::support::transform::Transformer;
use lsp_types::ClientCapabilities;
use lsp_types::request::Completion;

struct LibCairo;

impl Transformer for LibCairo {
    fn capabilities(base: ClientCapabilities) -> ClientCapabilities {
        Completion::capabilities(base)
    }

    fn transform(ls: MockClient, cursors: Cursors) -> String {
        Completion::transform(ls, cursors)
    }

    fn files(fixture: &mut Fixture) {
        fixture
            .add_file("cairo_project.toml", CAIRO_PROJECT_TOML_2024_07)
            .add_file("src/aaaa.cairo", "")
            .add_file("src/bbbb.cairo", "")
            .add_file("src/cccc.cairo", "");
    }
}

#[test]
fn lib_cairo_with_body() {
    test_transform_plain!(LibCairo,
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
    test_transform_plain!(LibCairo,
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
fn lib_cairo_without_name_without_semicolon() {
    test_transform_plain!(LibCairo,
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
    test_transform_plain!(LibCairo,
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
    test_transform_plain!(LibCairo,
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

    fn transform(ls: MockClient, cursors: Cursors) -> String {
        transform(ls, cursors, Self::main_file())
    }

    fn files(fixture: &mut Fixture) {
        fixture
            .add_file("cairo_project.toml", CAIRO_PROJECT_TOML_2024_07)
            .add_file("src/lib.cairo", "mod aaaa;")
            .add_file("src/aaaa/bbbb.cairo", "")
            .add_file("src/bbbb.cairo", "")
            .add_file("src/aaaa/cccc.cairo", "")
            .add_file("src/cccc.cairo", "");
    }

    fn main_file() -> &'static str {
        "src/aaaa.cairo"
    }
}

#[test]
fn other_top_level_file_with_body() {
    test_transform_plain!(OtherTopLevelFile,
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
    test_transform_plain!(OtherTopLevelFile,
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
    test_transform_plain!(OtherTopLevelFile,
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
    test_transform_plain!(OtherTopLevelFile,
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
    test_transform_plain!(OtherTopLevelFile,
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

    fn transform(ls: MockClient, cursors: Cursors) -> String {
        transform(ls, cursors, Self::main_file())
    }

    fn files(fixture: &mut Fixture) {
        fixture
            .add_file("cairo_project.toml", CAIRO_PROJECT_TOML_2024_07)
            .add_file("src/lib.cairo", "mod x;")
            .add_file("src/x/d/aaaa.cairo", "")
            .add_file("src/x/d/bbbb.cairo", "")
            .add_file("src/x/d/cccc.cairo", "")
            .add_file("src/x.cairo", "mod d;")
            .add_file("src/dddd.cairo", "");
    }

    fn main_file() -> &'static str {
        "src/x/d.cairo"
    }
}

#[test]
fn nested_file_with_body() {
    test_transform_plain!(NestedFile,
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
    test_transform_plain!(NestedFile,
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
    test_transform_plain!(NestedFile,
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
    test_transform_plain!(NestedFile,
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
    test_transform_plain!(NestedFile,
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}
