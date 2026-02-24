use crate::hover::HoverManifest;
use crate::support::fixture::fixture;
use crate::support::insta::test_transform_plain;

#[test]
fn manifest_hover_package_name() {
    let fixture = fixture! {
        "src/lib.cairo" => "fn main() {}",
    };

    test_transform_plain!(HoverManifest, fixture, r#"
        [package]
        na<caret>me = "my_pkg"
        version = "0.1.0"
        edition = "2025_12"

        [dependencies]
        starknet = "*"
        "#, @r#"
        source_context = """
        na<caret>me = "my_pkg"
        """
        highlight = """
        <sel>name</sel> = "my_pkg"
        """
        popover = """
        The package name is a valid Cairo identifier used to refer to the package.
        It is used when listed as a dependency in another package, and as the default name of targets.

        The name must use only ASCII lowercase alphanumeric characters or _, and cannot be empty. It also must not be a valid Cairo keyword or a wildcard pattern (_).
        - See official documentation at: <https://docs.swmansion.com/scarb/docs/reference/manifest.html#name>"""
        "#)
}

#[test]
fn manifest_hover_workspace_members() {
    let fixture = fixture! {
        "src/lib.cairo" => "fn main() {}",
    };

    test_transform_plain!(HoverManifest, fixture, r#"
        [workspace]
        members = [
            "crates<caret>/*",
        ]
        "#, @r#"
        source_context = """
            "crates<caret>/*",
        """
        highlight = """
        members = <sel>[
            "crates/*",
        ]</sel>
        """
        popover = """
        List of workspace member package paths (relative to the workspace root).
        Supports globs to match multiple paths, using typical filename glob patterns like * and ?.
        - See official documentation at: <https://docs.swmansion.com/scarb/docs/reference/workspaces.html#members>"""
        "#)
}

#[test]
fn manifest_hover_not_existing_field() {
    let fixture = fixture! {
        "src/lib.cairo" => "fn main() {}",
    };

    test_transform_plain!(HoverManifest, fixture, r#"
        [package]
        name = "hello"
        version = "0.1.0"
        not_existing_field = "x<caret>d"
        "#, @r#"
        source_context = """
        not_existing_field = "x<caret>d"
        """
"#)
}

#[test]
fn manifest_hover_package_edition_workspace() {
    let fixture = fixture! {
        "src/lib.cairo" => "fn main() {}",
    };

    test_transform_plain!(HoverManifest, fixture, r#"
        [package]
        name = "starknet_hello_world"
        edition.works<caret>pace = true
        "#, @r#"
        source_context = """
        edition.works<caret>pace = true
        """
        highlight = """
        edition.<sel>workspace</sel> = true
        """
        popover = "Allows inheriting keys from a workspace."
"#)
}
