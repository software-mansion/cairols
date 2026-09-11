use cairo_lang_defs::ids::{FunctionWithBodyId, MemberId, NamedLanguageElementId};
use cairo_lang_semantic::items::function_with_body::{
    FunctionWithBodySemantic, SemanticExprLookup,
};
use cairo_lang_semantic::{Expr, MemberAccessKind};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, TypedStablePtr, TypedSyntaxNode, ast};

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
    pub fn name(&self, db: &'db AnalysisDatabase) -> String {
        self.member_id.name(db).to_string(db)
    }
}

/// Resolves the member accessed by a member access expression (e.g. `self.<ident>`) containing
/// `node`, if `node` is a part of the right-hand side of that expression.
pub fn resolve_accessed_member<'db>(
    db: &'db AnalysisDatabase,
    node: SyntaxNode<'db>,
    function_with_body: FunctionWithBodyId<'db>,
) -> Option<MemberId<'db>> {
    let binary_expr = node.ancestor_of_type::<ast::ExprBinary>(db)?;

    let expr_id =
        db.lookup_expr_by_ptr(function_with_body, binary_expr.stable_ptr(db).into()).ok()?;

    // Accessing a member through a snapshot wraps the member access in snapshot and desnap
    // expressions (e.g. `self.x` where `self: @Point` is a desnap of the member access),
    // so unwrap them to get to the member access itself.
    let mut semantic_expr = db.expr_semantic(function_with_body, expr_id);
    let expr_member_access = loop {
        semantic_expr = match semantic_expr {
            Expr::MemberAccess(expr_member_access) => break expr_member_access,
            Expr::Snapshot(expr_snapshot) => {
                db.expr_semantic(function_with_body, expr_snapshot.inner)
            }
            Expr::Desnap(expr_desnap) => db.expr_semantic(function_with_body, expr_desnap.inner),
            _ => return None,
        };
    };

    let pointer_to_rhs = binary_expr.rhs(db).stable_ptr(db).untyped();

    let mut current_node = node;
    // Check if the node points to a member, not a struct variable.
    while pointer_to_rhs != current_node.stable_ptr(db) {
        // If we found the node with the binary expression, then we're sure we won't find the
        // node with the member.
        if current_node.stable_ptr(db) == binary_expr.stable_ptr(db).untyped() {
            return None;
        }
        current_node = current_node.parent(db)?;
    }

    let MemberAccessKind::Struct { member_id, .. } = expr_member_access.kind else {
        return None;
    };
    Some(member_id)
}
