use cairo_lang_defs::ids::{LookupItemId, ModuleId, ModuleItemId, SubmoduleId};
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;

use crate::lang::db::AnalysisDatabase;

/// Information about the definition of a module.
/// It uses [`NonMacroModuleId`] instead of [`ModuleId`] since [`ModuleId::MacroCall`] is resolved to
/// [`super::ItemDef`] to make sure it points to the macro declaration.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ModuleDef<'db> {
    id: NonMacroModuleId<'db>,
    /// A full path to the parent module if [`ModuleId`] points to a submodule,
    /// None otherwise (i.e. for a crate root).
    parent_full_path: Option<String>,
    definition_stable_ptr: SyntaxStablePtrId<'db>,
}

/// [`ModuleId`] without [`ModuleId::MacroCall`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NonMacroModuleId<'db> {
    CrateRoot(CrateId<'db>),
    Submodule(SubmoduleId<'db>),
}

impl<'db> TryFrom<ModuleId<'db>> for NonMacroModuleId<'db> {
    type Error = ();

    fn try_from(value: ModuleId<'db>) -> Result<Self, Self::Error> {
        match value {
            ModuleId::CrateRoot(id) => Ok(Self::CrateRoot(id)),
            ModuleId::Submodule(id) => Ok(Self::Submodule(id)),
            ModuleId::MacroCall { .. } => Err(()),
        }
    }
}

impl<'db> From<NonMacroModuleId<'db>> for ModuleId<'db> {
    fn from(value: NonMacroModuleId<'db>) -> Self {
        match value {
            NonMacroModuleId::CrateRoot(id) => Self::CrateRoot(id),
            NonMacroModuleId::Submodule(id) => Self::Submodule(id),
        }
    }
}

impl<'db> ModuleDef<'db> {
    /// Constructs a new [`ModuleDef`] instance.
    pub(super) fn new(
        db: &'db AnalysisDatabase,
        id: NonMacroModuleId<'db>,
        definition_node: SyntaxNode<'db>,
    ) -> Self {
        let module_id: ModuleId = id.into();
        let parent_full_path = module_id
            .full_path(db)
            .strip_suffix(module_id.name(db))
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
        format!("{prefix} {}", self.name(db))
    }

    /// Gets the full path of the parent module.
    pub fn definition_path(&self) -> String {
        self.parent_full_path.clone().unwrap_or_default()
    }

    /// Gets the module's documentation if it is available.
    pub fn documentation(&self, db: &'db AnalysisDatabase) -> Option<String> {
        let doc_id = match self.id {
            NonMacroModuleId::CrateRoot(id) => DocumentableItemId::Crate(id),
            NonMacroModuleId::Submodule(id) => DocumentableItemId::LookupItem(
                LookupItemId::ModuleItem(ModuleItemId::Submodule(id)),
            ),
        };

        db.get_item_documentation(doc_id)
    }

    /// Gets the name of the module.
    pub fn name(&self, db: &'db AnalysisDatabase) -> &'db str {
        self.module_id().name(db)
    }

    /// Gets the id of the module.
    pub fn non_macro_module_id(&self) -> NonMacroModuleId<'db> {
        self.id
    }

    /// Gets the id of the module.
    pub fn module_id(&self) -> ModuleId<'db> {
        self.id.into()
    }
}
