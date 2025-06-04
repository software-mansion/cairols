use cairo_lang_defs::ids::{LanguageElementId, ModuleId};
use cairo_lang_syntax::node::ast::{ItemModule, MaybeModuleBody};
use cairo_lang_syntax::node::{SyntaxNode, Terminal};
use lsp_types::{
    CodeAction, CodeActionKind, CreateFile, DocumentChangeOperation, DocumentChanges, ResourceOp,
    Url, WorkspaceEdit,
};

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Code actions for missing module file.
pub fn create_module_file(
    db: &AnalysisDatabase,
    node: SyntaxNode,
    mut url: Url,
) -> Option<CodeAction> {
    let item_module = node.ancestor_of_type::<ItemModule>(db)?;

    if !matches!(item_module.body(db), MaybeModuleBody::None(_)) {
        return None;
    }

    let mut is_crate_root = false;

    if let ModuleId::Submodule(submodule) = db.find_module_containing_node(node)? {
        if matches!(submodule.parent_module(db), ModuleId::CrateRoot(_)) {
            is_crate_root = true;
        }
    };

    let module_name = item_module.name(db).text(db);
    if module_name.is_empty() {
        return None;
    }

    let file = url.path_segments()?.next_back()?;
    let extra_folder = file.strip_suffix(".cairo").unwrap_or(file).to_owned();

    if let Ok(mut path) = url.path_segments_mut() {
        path.pop();

        if !is_crate_root {
            path.push(&extra_folder);
        }

        path.push(&format!("{module_name}.cairo"));
    }

    Some(CodeAction {
        title: format!("Create module file `{module_name}`"),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            document_changes: Some(DocumentChanges::Operations(vec![DocumentChangeOperation::Op(
                ResourceOp::Create(CreateFile { uri: url, annotation_id: None, options: None }),
            )])),
            ..Default::default()
        }),
        ..Default::default()
    })
}
