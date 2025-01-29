use std::iter;

use cairo_lang_syntax::node::SyntaxNode;

pub trait SyntaxNodeExt {
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
}

impl SyntaxNodeExt for SyntaxNode {
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
}
