use cairo_lang_defs::ids::TraitId;
use cairo_lang_semantic::Signature;
use cairo_lang_semantic::diagnostic::SemanticDiagnostics;
use cairo_lang_semantic::items::visibility::Visibility;
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast::{
    Expr, ExprBinary, ExprFunctionCall, ExprPath, ItemStruct, StatementExpr, StatementLet,
    TypeClause,
};
use cairo_lang_syntax::node::kind::SyntaxKind;
use itertools::Itertools;

use crate::ide::completion::helpers::formatting::generate_abbreviated_signature;
use crate::lang::analysis_context::AnalysisContext;
use crate::lang::db::AnalysisDatabase;
use crate::lang::visibility::peek_visible_in_with_edition;

#[derive(Clone)]
pub struct TypedSnippet {
    pub lsp_snippet: String,
    pub type_hint: Option<String>,
}

impl TypedSnippet {
    pub fn struct_initialization<'db>(
        db: &'db AnalysisDatabase,
        ctx: &AnalysisContext<'db>,
        struct_node: ItemStruct<'db>,
    ) -> Option<TypedSnippet> {
        if (ctx.node.ancestor_of_type::<StatementLet>(db).is_none()
            && ctx.node.ancestor_of_type::<StatementExpr>(db).is_none())
            || ctx.node.ancestor_of_type::<ExprPath>(db).is_none()
            || ctx.node.ancestor_of_type::<TypeClause>(db).is_some()
        {
            return None;
        }

        let struct_parent_module_id =
            db.find_module_containing_node(struct_node.as_syntax_node())?;

        let mut diagnostics = SemanticDiagnostics::new(ctx.module_id);

        // If any field of the struct is not visible, we should not propose initialization.
        if !struct_node.members(db).elements(db).all(|member| {
            peek_visible_in_with_edition(
                db,
                Visibility::from_ast(db, &mut diagnostics, &member.visibility(db)),
                struct_parent_module_id,
                ctx.module_id,
            )
        }) {
            return None;
        }

        let struct_name =
            struct_node.name(db).as_syntax_node().get_text_without_trivia(db).long(db).as_str();

        let type_hint = struct_node
            .members(db)
            .elements(db)
            .map(|member| {
                let member_name =
                    member.name(db).as_syntax_node().get_text_without_trivia(db).long(db).as_str();
                let member_type = member
                    .type_clause(db)
                    .as_syntax_node()
                    .get_text_without_trivia(db)
                    .long(db)
                    .as_str();

                format!("{}{}", member_name, member_type)
            })
            .join(", ");

        let args = struct_node
            .members(db)
            .elements(db)
            .enumerate()
            .map(|(index, member)| {
                format!(
                    "{}: ${}",
                    member.name(db).as_syntax_node().get_text_without_trivia(db).long(db).as_str(),
                    // We use 1-based indexing for snippet placeholders, as the `0` is reserved for the final cursor position.
                    index + 1,
                )
            })
            .join(", ");

        Some(if args.is_empty() {
            let empty_initializer = format!("{} {{}}", struct_name);
            TypedSnippet {
                lsp_snippet: empty_initializer.clone(),
                type_hint: Some(empty_initializer),
            }
        } else {
            TypedSnippet {
                lsp_snippet: format!("{} {{ {} }}", struct_name, args),
                type_hint: Some(format!("{} {{ {} }}", struct_name, type_hint)),
            }
        })
    }

    /// Creates an LSP snippet for the function with the given name the signature.
    ///
    /// Example: for a function with a signature
    /// ```cairo
    /// fn xyz(a: u8, b: ByteArray) -> felt252 {}
    /// ```
    /// returns a string "xyz({$1:a}, {$2:b})".
    /// If the call already has an argument list (e.g. `xy<caret>(1, 2)`), returns only "xyz",
    /// so the existing parentheses and arguments are left untouched.
    pub fn function_call<'db>(
        db: &'db AnalysisDatabase,
        ctx: &AnalysisContext<'db>,
        function_name: &str,
        signature: &Signature,
        trait_id: Option<TraitId>,
    ) -> TypedSnippet {
        let type_hint = Some(generate_abbreviated_signature(db, signature, trait_id));

        if has_argument_list(db, ctx) {
            return TypedSnippet { lsp_snippet: function_name.to_string(), type_hint };
        }

        let params_snippet = signature
            .params
            .iter()
            .map(|param| param.name.to_string(db))
            .filter(|name| name != "self")
            .enumerate()
            .map(|(index, name)| format!("${{{}:{}}}", index + 1, name))
            .join(", ");
        TypedSnippet { lsp_snippet: format!("{function_name}({params_snippet})"), type_hint }
    }

    pub fn macro_call(last_segment: &str) -> TypedSnippet {
        TypedSnippet { lsp_snippet: format!("{}!($1)", last_segment), type_hint: None }
    }
}

/// Checks if the function being completed already has an argument list after it.
fn has_argument_list<'db>(db: &'db AnalysisDatabase, ctx: &AnalysisContext<'db>) -> bool {
    // `fo<caret>(1, 2)` or `foo.ba<caret>(1, 2)`.
    let is_function_of_call = ctx
        .node
        .ancestor_of_type::<ExprPath>(db)
        .and_then(|path| path.as_syntax_node().parent_of_type::<ExprFunctionCall>(db))
        .is_some_and(|call| !call.arguments(db).rparen(db).as_syntax_node().contains_missing(db));

    // With no method name typed after the dot, the parentheses become the rhs of the dot
    // expression: `foo.<caret>()` and `foo.<caret>(x, y)` are parsed as `Expr::Tuple`,
    // `foo.<caret>(x)` as `Expr::Parenthesized`.
    let is_parens_after_dot = ctx.node.kind(db) == SyntaxKind::TerminalDot
        && ctx
            .node
            .parent_of_type::<ExprBinary>(db)
            .is_some_and(|expr| matches!(expr.rhs(db), Expr::Tuple(_) | Expr::Parenthesized(_)));

    is_function_of_call || is_parens_after_dot
}
