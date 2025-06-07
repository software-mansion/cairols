use super::inlining::{ExpandMacroInliningStrategy, FileWithOrigin};
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
pub fn expanded_macros(
    db: &AnalysisDatabase,
    item: ModuleItem,
    crate_id: CrateId,
    metadata: &MacroPluginMetadata<'_>,
    files: &mut Vec<FileWithOrigin>,
) -> Option<()> {
    let item_files = expand_module_item(db, item, crate_id, metadata)?;

    files.extend(item_files.iter().cloned());

    for file_with_origin in item_files {
        if file_with_origin.strategy == ExpandMacroInliningStrategy::Append {
            continue;
        }

        let syntax = db.file_module_syntax(file_with_origin.file).ok()?;

        for item in syntax.items(db).elements(db) {
            expanded_macros(db, item, crate_id, metadata, files);
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
) -> Option<Vec<FileWithOrigin>> {
    let mut files = vec![];

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

            files.push(FileWithOrigin {
                file: new_file,
                generated_from: item.as_syntax_node(),
                strategy: ExpandMacroInliningStrategy::from_remove_original_item(
                    result.remove_original_item,
                ),
            });
        }

        if result.remove_original_item {
            break;
        }
    }

    Some(files)
}
