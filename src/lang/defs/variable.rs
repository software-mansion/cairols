use cairo_lang_defs::ids::VarId;
use cairo_lang_semantic::{Binding, Mutability};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::Upcast;
use cairo_lang_utils::smol_str::SmolStr;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

/// Information about the definition of a variable (local, function parameter).
#[derive(Eq, PartialEq)]
pub struct VariableDef {
    var_id: VarId,
    identifier: ast::TerminalIdentifier,
}

impl VariableDef {
    /// Constructs a new [`VariableDef`] instance.
    pub(super) fn new(db: &AnalysisDatabase, var_id: VarId, definition_node: SyntaxNode) -> Self {
        let identifier = ast::TerminalIdentifier::from_syntax_node(db, definition_node);
        Self { var_id, identifier }
    }

    /// Gets the syntax node which defines this symbol.
    pub fn definition_node(&self) -> SyntaxNode {
        self.identifier.as_syntax_node()
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId {
        self.identifier.stable_ptr().untyped()
    }

    /// Gets variable signature, which tries to resemble the way how it is defined in code.
    pub fn signature(&self, db: &AnalysisDatabase) -> Option<String> {
        let name = self.name(db);
        let binding = db.lookup_binding(self.var_id)?;

        let prefix = match &binding {
            Binding::LocalVar(_) => "let ",
            Binding::LocalItem(_) => "const ",
            Binding::Param(_) => "",
        };

        let mutability = match &binding {
            Binding::LocalVar(local) => {
                if local.is_mut {
                    "mut "
                } else {
                    ""
                }
            }
            Binding::LocalItem(_) => "",
            Binding::Param(param) => match param.mutability {
                Mutability::Immutable => "",
                Mutability::Mutable => "mut ",
                Mutability::Reference => "ref ",
            },
        };

        let ty = binding.ty().format(db.upcast());

        Some(format!("{prefix}{mutability}{name}: {ty}"))
    }

    /// Gets this variable's name.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        self.identifier.text(db)
    }
}
