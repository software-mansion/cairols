use indoc::indoc;
use lsp_types::{
    CodeAction, CodeActionContext, CodeActionOrCommand, CodeActionParams, Diagnostic,
    NumberOrString, Position, Range, lsp_request,
};

use crate::support::sandbox;

#[test]
fn manifest_unknown_field_code_action_endpoint_returns_fix() {
    assert_manifest_quick_fix(
        indoc! {r#"
            [package]
            name = "test_package"
            version = "0.1.0"
            edition = "2025_12"
            typo_field = "oops"
        "#},
        "unknown manifest field `package.typo_field`",
        "SE0002",
        "Remove unknown manifest field `package.typo_field`",
        "typo_field",
        r#"edition = "2025_12""#,
    );
}

#[test]
fn manifest_inlining_strategy_conflict_code_action_endpoint_returns_fixes() {
    let manifest = indoc! {r#"
        [package]
        name = "test_package"
        version = "0.1.0"
        edition = "2025_12"

        [profile.dev.cairo]
        inlining-strategy = "default"
        skip-optimizations = true
    "#};

    let diagnostic = manifest_diagnostic(
        manifest,
        "`inlining-strategy` has no effect when `skip-optimizations = true`; remove it, set it to `\"avoid\"`, or disable `skip-optimizations`",
        "SE0005",
    );

    let code_actions = manifest_code_actions(manifest, diagnostic.clone());

    let remove_action =
        find_code_action(code_actions.clone(), "Remove conflicting `inlining-strategy` field");
    let remove_text = only_manifest_edit(&remove_action);
    assert!(!remove_text.contains("inlining-strategy"), "{remove_text}");
    assert!(remove_text.contains("skip-optimizations = true"), "{remove_text}");

    let avoid_action =
        find_code_action(code_actions.clone(), "Set `inlining-strategy` to `\"avoid\"`");
    let avoid_text = only_manifest_edit(&avoid_action);
    assert!(avoid_text.contains(r#"inlining-strategy = "avoid""#), "{avoid_text}");
    assert!(avoid_text.contains("skip-optimizations = true"), "{avoid_text}");

    let skip_action = find_code_action(code_actions, "Set `skip-optimizations` to `false`");
    let skip_text = only_manifest_edit(&skip_action);
    assert!(skip_text.contains(r#"inlining-strategy = "default""#), "{skip_text}");
    assert!(skip_text.contains("skip-optimizations = false"), "{skip_text}");
}

#[test]
fn manifest_profile_inheritance_invalid_code_action_endpoint_returns_fixes() {
    let manifest = indoc! {r#"
        [package]
        name = "test_package"
        version = "0.1.0"
        edition = "2025_12"

        [profile.custom]
        inherits = "invalid"
    "#};

    let diagnostic = manifest_diagnostic(
        manifest,
        "profile can inherit from `dev` or `release` only, found `invalid`",
        "SE0004",
    );

    let code_actions = manifest_code_actions(manifest, diagnostic);

    let remove_action = find_code_action(code_actions.clone(), "Remove invalid `inherits` field");
    let remove_text = only_manifest_edit(&remove_action);
    assert!(!remove_text.contains("inherits"), "{remove_text}");
    assert!(remove_text.contains("[profile.custom]"), "{remove_text}");

    let dev_action = find_code_action(code_actions.clone(), "Set `inherits` to `\"dev\"`");
    let dev_text = only_manifest_edit(&dev_action);
    assert!(dev_text.contains(r#"inherits = "dev""#), "{dev_text}");
    assert!(!dev_text.contains(r#"inherits = "invalid""#), "{dev_text}");

    let release_action = find_code_action(code_actions, "Set `inherits` to `\"release\"`");
    let release_text = only_manifest_edit(&release_action);
    assert!(release_text.contains(r#"inherits = "release""#), "{release_text}");
    assert!(!release_text.contains(r#"inherits = "invalid""#), "{release_text}");
}

#[test]
fn manifest_dependency_git_ref_without_git_code_action_endpoint_returns_fix() {
    assert_manifest_quick_fix(
        indoc! {r#"
            [package]
            name = "test_package"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            foo = { path = "../foo", branch = "main" }
        "#},
        "dependency (foo) is non-Git, but provides `branch`, `tag` or `rev`",
        "SE0007",
        "Remove unsupported `branch` field",
        r#"branch = "main""#,
        r#"path = "../foo""#,
    );
}

#[test]
fn manifest_dependency_git_reference_ambiguous_code_action_endpoint_returns_fix() {
    assert_manifest_quick_fix(
        indoc! {r#"
            [package]
            name = "test_package"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            foo = { git = "https://example.com", branch = "main", tag = "v1" }
        "#},
        "dependency (foo) specification is ambiguous, only one of `branch`, `tag` or `rev` is allowed",
        "SE0008",
        "Remove conflicting `branch` field",
        r#"branch = "main""#,
        r#"tag = "v1""#,
    );
}

#[test]
fn manifest_dependency_git_path_ambiguous_code_action_endpoint_returns_fix() {
    assert_manifest_quick_fix(
        indoc! {r#"
            [package]
            name = "test_package"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            foo = { git = "https://example.com", path = "../foo" }
        "#},
        "dependency (foo) specification is ambiguous, only one of `git` or `path` is allowed",
        "SE0010",
        "Remove conflicting `git` field",
        r#"git = "https://example.com""#,
        r#"path = "../foo""#,
    );
}

#[test]
fn manifest_dependency_git_registry_ambiguous_code_action_endpoint_returns_fix() {
    assert_manifest_quick_fix(
        indoc! {r#"
            [package]
            name = "test_package"
            version = "0.1.0"
            edition = "2025_12"

            [dependencies]
            foo = { git = "https://example.com", registry = "custom" }
        "#},
        "dependency (foo) specification is ambiguous, only one of `git` or `registry` is allowed",
        "SE0011",
        "Remove conflicting `git` field",
        r#"git = "https://example.com""#,
        r#"registry = "custom""#,
    );
}

fn assert_manifest_quick_fix(
    manifest: &str,
    diagnostic_message: &str,
    code: &str,
    expected_title: &str,
    removed_text: &str,
    kept_text: &str,
) {
    let diagnostic = manifest_diagnostic(manifest, diagnostic_message, code);
    let code_actions = manifest_code_actions(manifest, diagnostic);

    let action = find_code_action(code_actions, expected_title);

    let new_text = only_manifest_edit(&action);
    assert!(!new_text.contains(removed_text), "{new_text}");
    assert!(new_text.contains(kept_text), "{new_text}");
}

fn manifest_code_actions(manifest: &str, diagnostic: Diagnostic) -> Vec<CodeActionOrCommand> {
    let mut ls = sandbox! {
        files {
            "Scarb.toml" => manifest,
        }
    };

    ls.open_and_wait_for_project_update("Scarb.toml");

    ls.send_request::<lsp_request!("textDocument/codeAction")>(CodeActionParams {
        text_document: ls.doc_id("Scarb.toml"),
        range: Range { start: diagnostic.range.start, end: diagnostic.range.end },
        context: CodeActionContext {
            diagnostics: vec![diagnostic],
            only: None,
            trigger_kind: None,
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    })
    .expect("code actions request failed")
}

fn manifest_diagnostic(manifest: &str, message: &str, code: &str) -> Diagnostic {
    let anchor = diagnostic_anchor(code);
    let start = manifest
        .find(anchor)
        .unwrap_or_else(|| panic!("missing anchor `{anchor}` in:\n{manifest}"));
    let end = start + anchor.len();

    Diagnostic {
        range: Range { start: position_at(manifest, start), end: position_at(manifest, end) },
        severity: None,
        code: Some(NumberOrString::String(code.to_string())),
        code_description: None,
        source: Some("scarb".to_string()),
        message: message.to_string(),
        related_information: None,
        tags: None,
        data: diagnostic_data(code),
    }
}

fn diagnostic_data(code: &str) -> Option<serde_json::Value> {
    match code {
        "SE0002" => Some(serde_json::json!({
            "field_path": ["package", "typo_field"],
        })),
        "SE0004" => Some(serde_json::json!({
            "profile": "custom",
            "field_path": ["profile", "custom", "inherits"],
            "valid_values": ["dev", "release"],
        })),
        "SE0005" => Some(serde_json::json!({
            "profile": "dev",
            "inlining_strategy_path": ["profile", "dev", "cairo", "inlining-strategy"],
            "skip_optimizations_path": ["profile", "dev", "cairo", "skip-optimizations"],
        })),
        "SE0007" => Some(serde_json::json!({
            "name": "foo",
            "table": "dependencies",
            "field": "branch",
            "field_path": ["dependencies", "foo", "branch"],
            "fields": ["branch"],
        })),
        "SE0008" => Some(serde_json::json!({
            "name": "foo",
            "table": "dependencies",
            "field": "branch",
            "field_path": ["dependencies", "foo", "branch"],
            "fields": ["branch", "tag"],
        })),
        "SE0010" => Some(serde_json::json!({
            "name": "foo",
            "table": "dependencies",
            "field": "git",
            "field_path": ["dependencies", "foo", "git"],
            "fields": ["git", "path"],
        })),
        "SE0011" => Some(serde_json::json!({
            "name": "foo",
            "table": "dependencies",
            "field": "git",
            "field_path": ["dependencies", "foo", "git"],
            "fields": ["git", "registry"],
        })),
        _ => None,
    }
}

fn diagnostic_anchor(code: &str) -> &'static str {
    match code {
        "SE0002" => "typo_field",
        "SE0005" => "inlining-strategy",
        "SE0004" => "inherits",
        "SE0007" => "branch",
        "SE0008" => "branch",
        "SE0010" => "git",
        "SE0011" => "git",
        _ => panic!("unsupported manifest diagnostic code: {code}"),
    }
}

fn position_at(text: &str, offset: usize) -> Position {
    let mut line = 0;
    let mut character = 0;

    for ch in text[..offset].chars() {
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += 1;
        }
    }

    Position::new(line, character)
}

fn find_code_action(code_actions: Vec<CodeActionOrCommand>, expected_title: &str) -> CodeAction {
    code_actions
        .into_iter()
        .map(|item| match item {
            CodeActionOrCommand::CodeAction(action) => action,
            CodeActionOrCommand::Command(command) => {
                panic!("expected code action, got command: {command:#?}")
            }
        })
        .find(|action| action.title == expected_title)
        .unwrap_or_else(|| panic!("missing code action `{expected_title}`"))
}

fn only_manifest_edit(action: &CodeAction) -> String {
    let changes = action.edit.as_ref().and_then(|edit| edit.changes.as_ref()).unwrap();
    assert_eq!(changes.len(), 1, "{changes:#?}");

    let edits = changes.values().next().unwrap();
    assert_eq!(edits.len(), 1, "{edits:#?}");

    edits[0].new_text.clone()
}
