use super::{Report, test_completions_text_edits_inner};
use crate::support::cairo_project_toml::CAIRO_PROJECT_TOML_2024_07;
use crate::support::insta::test_transform;
use crate::support::sandbox;

fn lib_cairo(cairo_code: &str) -> Report {
    test_completions_text_edits_inner(cairo_code, "src/lib.cairo", |cairo| {
        sandbox! {
            files {
                "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
                "src/lib.cairo" => cairo,
                "src/aaaa.cairo" => "",
                "src/bbbb.cairo" => "",
                "src/cccc.cairo" => "",
            }
        }
    })
}

#[test]
fn lib_cairo_with_body() {
    test_transform!(lib_cairo,
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
    test_transform!(lib_cairo,
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
    test_transform!(lib_cairo,
    "mod cccc; mod <caret>",
    @r#"
    caret = """
    mod cccc; mod <caret>
    """

    [[completions]]
    completion_label = "aaaa;"

    [[completions]]
    completion_label = "bbbb;"
    "#);
}

#[test]
fn lib_cairo_with_partial_name_with_semicolon() {
    test_transform!(lib_cairo,
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
    test_transform!(lib_cairo,
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}

fn other_top_level_file(cairo_code: &str) -> Report {
    test_completions_text_edits_inner(cairo_code, "src/aaaa.cairo", |cairo| {
        sandbox! {
            files {
                "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
                "src/lib.cairo" => "mod aaaa;",
                "src/aaaa.cairo" => cairo,
                "src/aaaa/bbbb.cairo" => "",
                "src/bbbb.cairo" => "",
                "src/aaaa/cccc.cairo" => "",
                "src/cccc.cairo" => "",
            }
        }
    })
}

#[test]
fn other_top_level_file_with_body() {
    test_transform!(other_top_level_file,
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
    test_transform!(other_top_level_file,
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
    test_transform!(other_top_level_file,
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
    test_transform!(other_top_level_file,
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
    test_transform!(other_top_level_file,
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}

fn nested_file(cairo_code: &str) -> Report {
    test_completions_text_edits_inner(cairo_code, "src/x/d.cairo", |cairo| {
        sandbox! {
            files {
                "cairo_project.toml" => CAIRO_PROJECT_TOML_2024_07,
                "src/lib.cairo" => "mod x;",
                "src/x/d/aaaa.cairo" => "",
                "src/x/d/bbbb.cairo" => "",
                "src/x/d/cccc.cairo" => "",
                "src/x/d.cairo" => cairo,
                "src/x.cairo" => "mod d;",
                "src/dddd.cairo" => "",
            }
        }
    })
}

#[test]
fn nested_file_with_body() {
    test_transform!(nested_file,
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
    test_transform!(nested_file,
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
    test_transform!(nested_file,
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
    test_transform!(nested_file,
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
    test_transform!(nested_file,
    "mod cccc; mod bbbb<caret>;",
    @r#"
    caret = """
    mod cccc; mod bbbb<caret>;
    """
    completions = []
    "#);
}
