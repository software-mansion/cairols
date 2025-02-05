use std::iter;

use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};

pub trait SyntaxNodeExt {
    /// Mirror of [`TypedSyntaxNode::cast`].
    fn cast<T: TypedSyntaxNode>(self, db: &dyn SyntaxGroup) -> Option<T>;

    /// Creates an iterator that yields ancestors of this syntax node.
    fn ancestors(&self) -> impl Iterator<Item = SyntaxNode>;

    /// Creates an iterator that yields this syntax node and walks up its ancestors.
    fn ancestors_with_self(&self) -> impl Iterator<Item = SyntaxNode>;

    /// Checks whether this syntax node is strictly above the given syntax node in the syntax tree.
    fn is_ancestor(&self, node: &SyntaxNode) -> bool;

    /// Checks whether this syntax node is strictly under the given syntax node in the syntax tree.
    fn is_descendant(&self, node: &SyntaxNode) -> bool;

    /// Checks whether this syntax node is or is above the given syntax node in the syntax tree.
    fn is_ancestor_or_self(&self, node: &SyntaxNode) -> bool;

    /// Checks whether this syntax node is or is under the given syntax node in the syntax tree.
    fn is_descendant_or_self(&self, node: &SyntaxNode) -> bool;

    /// Finds the first ancestor of a given kind.
    fn ancestor_of_kind(&self, db: &dyn SyntaxGroup, kind: SyntaxKind) -> Option<SyntaxNode>;

    /// Finds the first ancestor of a given kind and returns it in typed form.
    fn ancestor_of_type<T: TypedSyntaxNode>(&self, db: &dyn SyntaxGroup) -> Option<T>;

    /// Finds the parent of a given kind.
    #[allow(dead_code)]
    fn parent_of_kind(&self, db: &dyn SyntaxGroup, kind: SyntaxKind) -> Option<SyntaxNode>;

    /// Finds the parent of a given kind and returns it in typed form.
    #[allow(dead_code)]
    fn parent_of_type<T: TypedSyntaxNode>(&self, db: &dyn SyntaxGroup) -> Option<T>;

    /// Finds the first parent of one of the kinds.
    fn ancestor_of_kinds(&self, db: &dyn SyntaxGroup, kinds: &[SyntaxKind]) -> Option<SyntaxNode>;

    /// Gets the kind of the given node's parent if it exists.
    fn parent_kind(&self, db: &dyn SyntaxGroup) -> Option<SyntaxKind>;

    /// Gets the kind of the given node's grandparent if it exists.
    fn grandparent_kind(&self, db: &dyn SyntaxGroup) -> Option<SyntaxKind>;
}

impl SyntaxNodeExt for SyntaxNode {
    fn cast<T: TypedSyntaxNode>(self, db: &dyn SyntaxGroup) -> Option<T> {
        T::cast(db, self)
    }

    fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
        // We aren't reusing `ancestors_with_self` here to avoid cloning this node.
        iter::successors(self.parent(), SyntaxNode::parent)
    }

    fn ancestors_with_self(&self) -> impl Iterator<Item = SyntaxNode> {
        iter::successors(Some(self.clone()), SyntaxNode::parent)
    }

    fn is_ancestor(&self, node: &SyntaxNode) -> bool {
        node.ancestors().any(|n| n == *self)
    }

    fn is_descendant(&self, node: &SyntaxNode) -> bool {
        node.is_ancestor(self)
    }

    fn is_ancestor_or_self(&self, node: &SyntaxNode) -> bool {
        node.ancestors_with_self().any(|n| n == *self)
    }

    fn is_descendant_or_self(&self, node: &SyntaxNode) -> bool {
        node.is_ancestor_or_self(self)
    }

    fn ancestor_of_kind(&self, db: &dyn SyntaxGroup, kind: SyntaxKind) -> Option<SyntaxNode> {
        self.ancestors().find(|node| node.kind(db) == kind)
    }

    fn ancestor_of_type<T: TypedSyntaxNode>(&self, db: &dyn SyntaxGroup) -> Option<T> {
        self.ancestors().find_map(|node| T::cast(db, node))
    }

    fn parent_of_kind(&self, db: &dyn SyntaxGroup, kind: SyntaxKind) -> Option<SyntaxNode> {
        self.parent().filter(|node| node.kind(db) == kind)
    }

    fn parent_of_type<T: TypedSyntaxNode>(&self, db: &dyn SyntaxGroup) -> Option<T> {
        self.parent().and_then(|node| T::cast(db, node))
    }

    fn ancestor_of_kinds(&self, db: &dyn SyntaxGroup, kinds: &[SyntaxKind]) -> Option<SyntaxNode> {
        self.ancestors().find(|node| kinds.contains(&node.kind(db)))
    }

    fn parent_kind(&self, db: &dyn SyntaxGroup) -> Option<SyntaxKind> {
        Some(self.parent()?.kind(db))
    }

    fn grandparent_kind(&self, db: &dyn SyntaxGroup) -> Option<SyntaxKind> {
        Some(self.parent()?.parent()?.kind(db))
    }
}
