use cairo_lang_defs::ids::{NamedLanguageElementId, VariantId};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::smol_str::SmolStr;

use crate::lang::db::AnalysisDatabase;
use crate::lang::defs::ItemDef;

/// Information about an enum variant.
#[derive(Eq, PartialEq)]
pub struct VariantDef {
    variant_id: VariantId,
    enum_item: ItemDef,
    definition_stable_ptr: SyntaxStablePtrId,
}

impl VariantDef {
    /// Constructs a new [`VariantDef`] instance.
    pub(super) fn new(
        db: &AnalysisDatabase,
        variant_id: VariantId,
        definition_node: SyntaxNode,
    ) -> Option<Self> {
        let enum_ast = definition_node.ancestor_of_type::<ast::ItemEnum>(db)?;
        let enum_item = ItemDef::new(db, &enum_ast.name(db).as_syntax_node())?;
        Some(Self { variant_id, enum_item, definition_stable_ptr: definition_node.stable_ptr() })
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId {
        self.definition_stable_ptr
    }

    /// Gets [`VariantId`] associated with this symbol.
    pub fn variant_id(&self) -> VariantId {
        self.variant_id
    }

    /// Gets a definition of the enum which this symbol is a variant of.
    pub fn enum_item(&self) -> &ItemDef {
        &self.enum_item
    }

    /// Gets variant's name.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        self.variant_id.name(db)
    }
}
