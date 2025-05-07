use cairo_lang_defs::ids::{ImportableId, VarId};
use cairo_lang_semantic::{Binding, Mutability};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::smol_str::SmolStr;

use crate::ide::ty::format_type;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;

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
    pub fn definition_stable_ptr(&self, db: &dyn SyntaxGroup) -> SyntaxStablePtrId {
        self.identifier.stable_ptr(db).untyped()
    }

    /// Gets variable signature, which tries to resemble the way how it is defined in code.
    pub fn signature(
        &self,
        db: &AnalysisDatabase,
        importables: &OrderedHashMap<ImportableId, String>,
    ) -> Option<String> {
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

        let ty = format_type(db, binding.ty(), importables);

        Some(format!("{prefix}{mutability}{name}: {ty}"))
    }

    /// Gets this variable's name.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        self.identifier.text(db)
    }
}
