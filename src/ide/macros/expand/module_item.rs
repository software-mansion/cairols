use cairo_lang_defs::{
    db::DefsGroup,
    plugin::{MacroPlugin, MacroPluginMetadata},
};
use cairo_lang_filesystem::ids::{CrateId, FileKind, FileLongId, SmolStrId, VirtualFile};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast::ModuleItem};
use cairo_lang_utils::Intern;

use super::inlining::FileWithOrigin;
use crate::lang::db::AnalysisDatabase;

// Resursively expand module item.
pub fn expand_module_item_macros<'db>(
    db: &'db AnalysisDatabase,
    item: ModuleItem<'db>,
    crate_id: CrateId<'db>,
    metadata: &MacroPluginMetadata<'_>,
    files: &mut Vec<FileWithOrigin<'db>>,
    extra_files: &mut Vec<FileWithOrigin<'db>>,
) -> Option<()> {
    let (new_files, new_extra_files) = expand_module_item(db, item, crate_id, metadata)?;

    files.extend(new_files.iter().cloned());
    extra_files.extend(new_extra_files);

    for file_with_origin in new_files {
        let syntax = db.file_module_syntax(file_with_origin.file).ok()?;

        for item in syntax.items(db).elements(db) {
            expand_module_item_macros(db, item, crate_id, metadata, files, extra_files);
        }
    }

    Some(())
}

// Generate files for single module item expansion.
fn expand_module_item<'db>(
    db: &'db AnalysisDatabase,
    item: ModuleItem<'db>,
    crate_id: CrateId<'db>,
    metadata: &MacroPluginMetadata<'_>,
) -> Option<(Vec<FileWithOrigin<'db>>, Vec<FileWithOrigin<'db>>)> {
    let mut files = vec![];
    let mut extra_files = vec![];

    for &plugin_id in db.crate_macro_plugins(crate_id).iter() {
        let result = plugin_id.long(db).generate_code(db, item.clone(), metadata);

        if let Some(generated) = result.code {
            let new_file = FileLongId::Virtual(VirtualFile {
                parent: Some(item.stable_ptr(db).untyped().file_id(db)),
                name: SmolStrId::from(db, generated.name.as_str()),
                content: SmolStrId::from(db, generated.content.as_str()),
                code_mappings: generated.code_mappings.into(),
                kind: FileKind::Module,
                original_item_removed: false,
            })
            .intern(db);

            let file_with_origin =
                FileWithOrigin { file: new_file, generated_from: item.as_syntax_node() };

            if result.remove_original_item {
                files.push(file_with_origin);
            } else {
                extra_files.push(file_with_origin);
            }
        }

        if result.remove_original_item {
            break;
        }
    }

    Some((files, extra_files))
}
