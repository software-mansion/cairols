use super::inlining::FileWithOrigin;
use crate::lang::db::AnalysisDatabase;
use cairo_lang_defs::{
    db::DefsGroup,
    plugin::{MacroPlugin, MacroPluginMetadata},
};
use cairo_lang_filesystem::ids::{CrateId, FileKind, FileLongId, VirtualFile};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast::ModuleItem};
use cairo_lang_utils::Intern;

// Resursively expand module item.
pub fn expand_module_item_macros(
    db: &AnalysisDatabase,
    item: ModuleItem,
    crate_id: CrateId,
    metadata: &MacroPluginMetadata<'_>,
    files: &mut Vec<FileWithOrigin>,
    extra_files: &mut Vec<FileWithOrigin>,
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
fn expand_module_item(
    db: &AnalysisDatabase,
    item: ModuleItem,
    crate_id: CrateId,
    metadata: &MacroPluginMetadata<'_>,
) -> Option<(Vec<FileWithOrigin>, Vec<FileWithOrigin>)> {
    let mut files = vec![];
    let mut extra_files = vec![];

    for &plugin_id in db.crate_macro_plugins(crate_id).iter() {
        let result =
            db.lookup_intern_macro_plugin(plugin_id).generate_code(db, item.clone(), metadata);

        if let Some(generated) = result.code {
            let new_file = FileLongId::Virtual(VirtualFile {
                parent: Some(item.stable_ptr(db).untyped().file_id(db)),
                name: generated.name,
                content: generated.content.into(),
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
