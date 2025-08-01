use cairo_lang_defs::ids::{LookupItemId, ModuleId, ModuleItemId};
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;

use crate::lang::db::AnalysisDatabase;

/// Information about the definition of a module.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ModuleDef<'db> {
    id: ModuleId<'db>,
    /// A full path to the parent module if [`ModuleId`] points to a submodule,
    /// None otherwise (i.e. for a crate root).
    parent_full_path: Option<String>,
    definition_stable_ptr: SyntaxStablePtrId<'db>,
}

impl<'db> ModuleDef<'db> {
    /// Constructs a new [`ModuleDef`] instance.
    pub(super) fn new(
        db: &'db AnalysisDatabase,
        id: ModuleId<'db>,
        definition_node: SyntaxNode<'db>,
    ) -> Self {
        let parent_full_path = id
            .full_path(db)
            .strip_suffix(id.name(db))
            .unwrap()
            // Fails when the path lacks `::`, i.e. when we import from a crate root.
            .strip_suffix("::")
            .map(String::from);

        ModuleDef { id, parent_full_path, definition_stable_ptr: definition_node.stable_ptr(db) }
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId<'db> {
        self.definition_stable_ptr
    }

    /// Gets the module signature: a name preceded by a qualifier: "mod" for submodule
    /// and "crate" for crate root.
    pub fn signature(&self, db: &'db AnalysisDatabase) -> String {
        let prefix = if self.parent_full_path.is_some() { "mod" } else { "crate" };
        format!("{prefix} {}", self.id.name(db))
    }

    /// Gets the full path of the parent module.
    pub fn definition_path(&self) -> String {
        self.parent_full_path.clone().unwrap_or_default()
    }

    /// Gets the module's documentation if it is available.
    pub fn documentation(&self, db: &'db AnalysisDatabase) -> Option<String> {
        let doc_id = match self.id {
            ModuleId::CrateRoot(id) => DocumentableItemId::Crate(id),
            ModuleId::Submodule(id) => DocumentableItemId::LookupItem(LookupItemId::ModuleItem(
                ModuleItemId::Submodule(id),
            )),
            ModuleId::MacroCall { id: _, generated_file_id: _ } => {
                return None;
            }
        };

        db.get_item_documentation(doc_id)
    }

    /// Gets the name of the module.
    pub fn name(&self, db: &'db AnalysisDatabase) -> &'db str {
        self.id.name(db)
    }

    /// Gets the id of the module.
    pub fn module_id(&self) -> ModuleId<'db> {
        self.id
    }
}
