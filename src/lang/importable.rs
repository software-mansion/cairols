use cairo_lang_defs::ids::ImportableId;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode};

use crate::lang::db::AnalysisDatabase;

pub fn importable_crate_id<'db>(
    db: &'db AnalysisDatabase,
    importable: ImportableId<'db>,
) -> CrateId<'db> {
    match importable {
        ImportableId::Crate(crate_id) => crate_id,
        _ => {
            let importable_node = importable_syntax_node(db, importable)
                .expect("Importable should have a syntax node.");
            let module = db
                .find_module_containing_node(importable_node)
                .expect("A node should be contained in a module");
            module.owning_crate(db)
        }
    }
}

// TODO: Upstream this function to compiler.
pub fn importable_syntax_node<'db>(
    db: &'db AnalysisDatabase,
    importable: ImportableId<'db>,
) -> Option<SyntaxNode<'db>> {
    match importable {
        ImportableId::Constant(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Submodule(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::ExternFunction(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::FreeFunction(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::ExternType(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::TypeAlias(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Impl(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::ImplAlias(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Struct(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Variant(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Trait(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Enum(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::MacroDeclaration(id) => Some(id.stable_ptr(db).lookup(db).as_syntax_node()),
        ImportableId::Crate(_) => None,
    }
}
