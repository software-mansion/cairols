use super::{db::AnalysisDatabase, lsp::LsProtoGroup};
use crate::{
    lang::{defs::SymbolDef, lsp::ToLsp},
    server::is_cairo_file_path,
};
use cairo_lang_defs::{
    db::DefsGroup,
    ids::{LanguageElementId, ModuleId},
};
use cairo_lang_filesystem::{
    db::{CrateConfiguration, FilesGroup},
    ids::Directory,
};
use cairo_lang_syntax::node::Token;
use lsp_types::{FileRename, RenameFilesParams, TextEdit, Url, WorkspaceEdit};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn rename_file(db: &AnalysisDatabase, params: RenameFilesParams) -> Option<WorkspaceEdit> {
    let mut changes: HashMap<Url, Vec<TextEdit>> = Default::default();

    for rename in params.files {
        handle_rename(db, rename, &mut changes);
    }

    Some(WorkspaceEdit { changes: Some(changes), ..Default::default() })
}

fn handle_rename(
    db: &AnalysisDatabase,
    rename: FileRename,
    changes: &mut HashMap<Url, Vec<TextEdit>>,
) -> Option<()> {
    let old_uri = Url::parse(&rename.old_uri).ok()?;
    let new_uri = Url::parse(&rename.new_uri).ok()?;

    if !is_cairo_file_path(&old_uri) || !is_cairo_file_path(&new_uri) {
        return None;
    }

    let file = db.file_for_url(&old_uri)?;

    let first = *db.file_modules(file).ok()?.first()?;

    let submodule = match first {
        ModuleId::CrateRoot(_) => {
            // If renamed file was src/lib.cairo there is nothing we can do.
            return None;
        }
        ModuleId::Submodule(submodule) => submodule,
    };

    let Some(CrateConfiguration { root: Directory::Real(root), .. }) =
        db.crate_config(first.owning_crate(db))
    else {
        return None;
    };

    let old_path = PathBuf::from(old_uri.path());
    let new_path = PathBuf::from(new_uri.path());

    assert!(old_path.starts_with(&root));

    if !new_path.starts_with(root) {
        return None;
    }

    let mut prefix = PathBuf::new();
    for (comp1, comp2) in old_path.components().zip(new_path.components()) {
        if comp1 == comp2 {
            prefix.push(comp1);
        } else {
            break;
        }
    }

    let old_file_name = assert_single_component(old_path.strip_prefix(&prefix).unwrap())?;
    let new_file_name = assert_single_component(new_path.strip_prefix(&prefix).unwrap())?;

    let mod_name = db
        .module_submodules(submodule.parent_module(db))
        .unwrap()
        .get(&submodule)
        .unwrap()
        .name(db);

    assert_eq!(old_file_name, mod_name.token(db).text(db));

    for usage in
        SymbolDef::find(db, &mod_name).unwrap().usages(db).include_declaration(true).collect()
    {
        let file = db.url_for_file(usage.file)?;
        let range = usage.span.position_in_file(db, usage.file)?;

        changes
            .entry(file)
            .or_default()
            .push(TextEdit { range: range.to_lsp(), new_text: new_file_name.clone() });
    }

    Some(())
}

fn assert_single_component(path: &Path) -> Option<String> {
    let mut components = path.components();
    let result = components
        .next()
        .map(|c| c.as_os_str().to_string_lossy().split(".").next().unwrap().to_string());

    if components.next().is_some() { None } else { result }
}
