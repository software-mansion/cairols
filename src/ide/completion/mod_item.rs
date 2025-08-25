use std::collections::HashSet;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::ast::{ItemModule, MaybeModuleBody, TerminalIdentifier};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode};
use lsp_types::{CompletionItem, CompletionItemKind, Url};

use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::text_matching::text_matches;

pub fn mod_completions<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
) -> Vec<CompletionItem> {
    let file_id = node.stable_ptr(db).file_id(db);

    if let Some(ident) = TerminalIdentifier::cast(db, node)
        && let Some(module_item) = node.parent_of_type::<ItemModule>(db)
        // We are in nested mod, we should not show completions for file modules.
        && module_item.as_syntax_node().ancestor_of_kind(db, SyntaxKind::ItemModule).is_none()
        && let Some(mod_names_completions) =
            mod_completions_ex(db, module_item, file_id, ident.text(db))
    {
        return mod_names_completions;
    }

    // if there is no name `mod <cursor>` we will be on `mod`.
    if node.kind(db) == SyntaxKind::TerminalModule
        && let Some(module_item) = node.parent_of_type::<ItemModule>(db)
        // We are in nested mod, we should not show completions for file modules.
        && module_item.as_syntax_node().ancestor_of_kind(db, SyntaxKind::ItemModule).is_none()
        // use "" as typed text in this case.
        && let Some(mod_names_completions) = mod_completions_ex(db, module_item, file_id, "")
    {
        return mod_names_completions;
    }

    vec![]
}
pub fn mod_completions_ex<'db>(
    db: &'db AnalysisDatabase,
    module: ItemModule<'db>,
    file: FileId<'db>,
    typed_module_name: &str,
) -> Option<Vec<CompletionItem>> {
    let semicolon_missing = match module.body(db) {
        MaybeModuleBody::None(semicolon) => {
            semicolon.token(db).as_syntax_node().kind(db) == SyntaxKind::TokenMissing
        }
        // If this module has body (ie. { /* some code */ }) we should not propose file names as
        // completion.
        MaybeModuleBody::Some(_) => return None,
    };

    let mut url = db.url_for_file(file)?;

    let current_file = url.path().to_string();

    let file_name = pop_path(&mut url)?;

    let module_files = db.file_modules(file).ok()?;

    let mut existing_modules_files = collect_existing_modules(db, &module_files)?;
    existing_modules_files.insert(current_file);

    let current_dir = url.to_file_path().ok()?;

    let search_dir = if is_crate_root(&module_files) {
        current_dir.clone()
    } else {
        current_dir.join(file_name.strip_suffix(".cairo").unwrap_or(&file_name))
    };

    let mut result: Vec<_> = Default::default();

    for cairo_file in read_dir(&search_dir)? {
        let file = cairo_file.iter().next_back()?.to_string_lossy().to_string();

        if !existing_modules_files.contains(&cairo_file.to_string_lossy().to_string()) {
            let file_name = file.strip_suffix(".cairo").unwrap_or(&file);

            if text_matches(file_name, typed_module_name) {
                let label = file_name.strip_suffix(".cairo").unwrap_or(file_name);
                let semicolon =
                    if semicolon_missing { ";".to_string() } else { Default::default() };

                result.push(CompletionItem {
                    label: format!("{label}{semicolon}"),
                    kind: Some(CompletionItemKind::MODULE),
                    ..Default::default()
                });
            }
        }
    }

    Some(result)
}

fn pop_path(url: &mut Url) -> Option<String> {
    let file_name = url.path_segments()?.next_back()?.to_string();

    if let Ok(mut path) = url.path_segments_mut() {
        path.pop();
    }

    Some(file_name)
}

fn collect_existing_modules<'db>(
    db: &'db AnalysisDatabase,
    module_files: &[ModuleId<'db>],
) -> Option<HashSet<String>> {
    let mut existing_modules_files = HashSet::<_>::default();

    for module in module_files
        .iter()
        .filter_map(|module| db.module_submodules_ids(*module).ok())
        .flat_map(|submodule_ids| {
            submodule_ids.iter().copied().map(ModuleId::Submodule).collect::<Vec<_>>()
        })
    {
        // This sometimes returns paths like `[ROOT_DIR]/src/.cairo`.
        // It means we are on `mod <caret>;`
        let path = db.module_main_file(module).ok()?.full_path(db);

        if path.ends_with("/.cairo") {
            continue;
        }

        existing_modules_files.insert(path);
    }

    Some(existing_modules_files)
}

fn is_crate_root(module_files: &[ModuleId]) -> bool {
    module_files.iter().any(|module| matches!(module, ModuleId::CrateRoot(_)))
}

fn read_dir(dir: &Path) -> Option<Vec<PathBuf>> {
    let mut result = vec![];

    for dir in std::fs::read_dir(dir).ok()? {
        let dir = dir.ok()?;

        let file_type = dir.file_type().ok()?;

        if file_type.is_file()
            && let Some(path) = handle_file(&dir)?
        {
            result.push(path);
        }
    }

    Some(result)
}

fn handle_file(dir: &DirEntry) -> Option<Option<PathBuf>> {
    Some(if dir.file_name().to_str()?.ends_with(".cairo") { Some(dir.path()) } else { None })
}
