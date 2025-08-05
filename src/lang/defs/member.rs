use cairo_lang_defs::ids::{MemberId, NamedLanguageElementId};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};

use crate::lang::db::AnalysisDatabase;
use crate::lang::defs::ItemDef;

/// Information about a struct member.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MemberDef<'db> {
    member_id: MemberId<'db>,
    struct_item: ItemDef<'db>,
    definition_stable_ptr: SyntaxStablePtrId<'db>,
}

impl<'db> MemberDef<'db> {
    /// Constructs a new [`MemberDef`] instance.
    pub(super) fn new(
        db: &'db AnalysisDatabase,
        member_id: MemberId<'db>,
        definition_node: SyntaxNode<'db>,
    ) -> Option<Self> {
        let struct_ast = definition_node.ancestor_of_type::<ast::ItemStruct>(db)?;
        let struct_item = ItemDef::new(db, struct_ast.name(db).as_syntax_node())?;
        Some(Self { member_id, struct_item, definition_stable_ptr: definition_node.stable_ptr(db) })
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId<'db> {
        self.definition_stable_ptr
    }

    /// Gets [`MemberId`] associated with this symbol.
    pub fn member_id(&self) -> MemberId<'db> {
        self.member_id
    }

    /// Gets a definition of the structure which this symbol is a member of.
    pub fn struct_item(&self) -> &ItemDef<'db> {
        &self.struct_item
    }

    /// Gets member's name.
    pub fn name(&self, db: &'db AnalysisDatabase) -> &'db str {
        self.member_id.name(db)
    }
}
