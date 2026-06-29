use std::collections::HashMap;

use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::span::TextSpan;
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionParams, Diagnostic, DocumentChanges, OneOf,
    OptionalVersionedTextDocumentIdentifier, Range, TextDocumentEdit, TextEdit, Url, WorkspaceEdit,
};

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::{LsProtoGroup, ToLsp};

mod dependency_git_path_ambiguous;
mod dependency_git_ref_without_git;
mod dependency_git_reference_ambiguous;
mod dependency_git_registry_ambiguous;
mod inlining_strategy_conflict;
mod patch_not_in_workspace_root;
mod profile_inheritance_invalid;
mod toml;
mod unknown_field;

use self::toml::{move_patch_to_workspace_root, remove_key_path};
use super::extract_code;

struct ManifestActionContext<'a> {
    db: &'a AnalysisDatabase,
    file_id: cairo_lang_filesystem::ids::FileId<'a>,
    uri: &'a Url,
    raw_toml: &'a str,
    diagnostic: &'a Diagnostic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScarbManifestCode {
    UnknownField,
    ProfileInheritanceInvalid,
    InliningStrategyConflict,
    DependencyGitRefWithoutGit,
    DependencyGitReferenceAmbiguous,
    DependencyGitPathAmbiguous,
    DependencyGitRegistryAmbiguous,
    PatchNotInWorkspaceRoot,
}

impl ScarbManifestCode {
    fn parse(code: &str) -> Option<Self> {
        match code {
            "SE0002" => Some(Self::UnknownField),
            "SE0004" => Some(Self::ProfileInheritanceInvalid),
            "SE0005" => Some(Self::InliningStrategyConflict),
            "SE0007" => Some(Self::DependencyGitRefWithoutGit),
            "SE0008" => Some(Self::DependencyGitReferenceAmbiguous),
            "SE0010" => Some(Self::DependencyGitPathAmbiguous),
            "SE0011" => Some(Self::DependencyGitRegistryAmbiguous),
            "SE0012" => Some(Self::PatchNotInWorkspaceRoot),
            _ => None,
        }
    }

    fn from_diagnostic(diagnostic: &Diagnostic) -> Option<Self> {
        Self::parse(extract_code(diagnostic)?)
    }

    fn build_actions(self, ctx: &ManifestActionContext<'_>) -> Vec<CodeAction> {
        match self {
            Self::UnknownField => unknown_field::build(ctx),
            Self::ProfileInheritanceInvalid => profile_inheritance_invalid::build(ctx),
            Self::InliningStrategyConflict => inlining_strategy_conflict::build(ctx),
            Self::DependencyGitRefWithoutGit => dependency_git_ref_without_git::build(ctx),
            Self::DependencyGitReferenceAmbiguous => dependency_git_reference_ambiguous::build(ctx),
            Self::DependencyGitPathAmbiguous => dependency_git_path_ambiguous::build(ctx),
            Self::DependencyGitRegistryAmbiguous => dependency_git_registry_ambiguous::build(ctx),
            Self::PatchNotInWorkspaceRoot => patch_not_in_workspace_root::build(ctx),
        }
    }
}

pub fn code_actions(params: &CodeActionParams, db: &AnalysisDatabase) -> Vec<CodeAction> {
    let uri = &params.text_document.uri;
    let Some(file_id) = db.file_for_url(uri) else {
        return vec![];
    };
    let Some(raw_toml) = db.file_content(file_id) else {
        return vec![];
    };

    params
        .context
        .diagnostics
        .iter()
        .filter_map(|diagnostic| {
            let code = ScarbManifestCode::from_diagnostic(diagnostic)?;
            Some((code, diagnostic))
        })
        .flat_map(|(code, diagnostic)| {
            let ctx = ManifestActionContext { db, file_id, uri, raw_toml, diagnostic };
            code.build_actions(&ctx)
        })
        .collect()
}

pub fn is_manifest_diagnostic(diagnostic: &Diagnostic) -> bool {
    ScarbManifestCode::from_diagnostic(diagnostic).is_some()
}

fn remove_key_from_diagnostic_data_action(
    ctx: &ManifestActionContext<'_>,
    title_prefix: &str,
) -> Option<CodeAction> {
    let path = diagnostic_string_array(ctx.diagnostic, "field_path")?;
    let key = path.last()?.clone();
    let new_text = remove_key_path(ctx.raw_toml, &path)?;

    Some(replace_manifest_action(
        ctx,
        new_text,
        format!("{title_prefix} `{key}` field"),
        ctx.diagnostic.clone(),
    ))
}

fn replace_manifest_action(
    ctx: &ManifestActionContext<'_>,
    new_text: String,
    title: String,
    diagnostic: Diagnostic,
) -> CodeAction {
    CodeAction {
        title,
        kind: Some(CodeActionKind::QUICKFIX),
        is_preferred: Some(true),
        edit: Some(WorkspaceEdit {
            changes: Some(HashMap::from_iter([(
                ctx.uri.clone(),
                vec![TextEdit {
                    range: full_document_range(ctx.db, ctx.file_id, ctx.raw_toml),
                    new_text,
                }],
            )])),
            document_changes: None,
            change_annotations: None,
        }),
        diagnostics: Some(vec![diagnostic]),
        ..Default::default()
    }
}

fn move_patch_to_workspace_root_action(
    ctx: &ManifestActionContext<'_>,
    workspace_root_uri: Url,
    workspace_root_file_id: cairo_lang_filesystem::ids::FileId<'_>,
    workspace_root_raw_toml: &str,
) -> Option<CodeAction> {
    let (member_new_text, workspace_root_new_text) =
        move_patch_to_workspace_root(ctx.raw_toml, workspace_root_raw_toml)?;

    let member_edit = TextDocumentEdit {
        text_document: OptionalVersionedTextDocumentIdentifier {
            uri: ctx.uri.clone(),
            version: None,
        },
        edits: vec![OneOf::Left(TextEdit {
            range: full_document_range(ctx.db, ctx.file_id, ctx.raw_toml),
            new_text: member_new_text,
        })],
    };
    let workspace_root_edit = TextDocumentEdit {
        text_document: OptionalVersionedTextDocumentIdentifier {
            uri: workspace_root_uri,
            version: None,
        },
        edits: vec![OneOf::Left(TextEdit {
            range: full_document_range(ctx.db, workspace_root_file_id, workspace_root_raw_toml),
            new_text: workspace_root_new_text,
        })],
    };

    Some(CodeAction {
        title: "Move `[patch]` to workspace root manifest".to_string(),
        kind: Some(CodeActionKind::QUICKFIX),
        is_preferred: Some(true),
        edit: Some(WorkspaceEdit {
            changes: None,
            document_changes: Some(DocumentChanges::Edits(vec![member_edit, workspace_root_edit])),
            change_annotations: None,
        }),
        diagnostics: Some(vec![ctx.diagnostic.clone()]),
        ..Default::default()
    })
}

fn full_document_range(
    db: &AnalysisDatabase,
    file_id: cairo_lang_filesystem::ids::FileId<'_>,
    raw_toml: &str,
) -> Range {
    TextSpan::from_str(raw_toml)
        .position_in_file(db, file_id)
        .map(|span| span.to_lsp())
        .unwrap_or_default()
}

fn diagnostic_string_array(diagnostic: &Diagnostic, key: &str) -> Option<Vec<String>> {
    diagnostic
        .data
        .as_ref()?
        .get(key)?
        .as_array()?
        .iter()
        .map(|value| value.as_str().map(str::to_string))
        .collect()
}
