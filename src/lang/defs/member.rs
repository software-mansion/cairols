use cairo_lang_defs::ids::{MemberId, NamedLanguageElementId};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::smol_str::SmolStr;

use crate::lang::db::AnalysisDatabase;
use crate::lang::defs::ItemDef;

/// Information about a struct member.
#[derive(Eq, PartialEq)]
pub struct MemberDef {
    member_id: MemberId,
    struct_item: ItemDef,
    definition_stable_ptr: SyntaxStablePtrId,
}

impl MemberDef {
    /// Constructs a new [`MemberDef`] instance.
    pub(super) fn new(
        db: &AnalysisDatabase,
        member_id: MemberId,
        definition_node: SyntaxNode,
    ) -> Option<Self> {
        let struct_ast = definition_node.ancestor_of_type::<ast::ItemStruct>(db)?;
        let struct_item = ItemDef::new(db, &struct_ast.name(db).as_syntax_node())?;
        Some(Self { member_id, struct_item, definition_stable_ptr: definition_node.stable_ptr() })
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId {
        self.definition_stable_ptr
    }

    /// Gets [`MemberId`] associated with this symbol.
    pub fn member_id(&self) -> MemberId {
        self.member_id
    }

    /// Gets a definition of the structure which this symbol is a member of.
    pub fn struct_item(&self) -> &ItemDef {
        &self.struct_item
    }

    /// Gets member's name.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        self.member_id.name(db)
    }
}
