use cairo_lang_defs::ids::TraitItemId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::keyword::{CRATE_KW, SELF_TYPE_KW, SUPER_KW};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_syntax::node::ast::{ExprPathPtr, TerminalIdentifier};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode, ast};
use lsp_types::SemanticTokenType;

use crate::ide::semantic_highlighting::is_inline_macro;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

#[derive(Clone, Copy)]
pub enum SemanticTokenKind {
    Namespace = 0,
    Class = 1,
    Enum = 2,
    Interface = 3,
    Struct = 4,
    TypeParameter = 5,
    Type = 6,
    Parameter = 7,
    Variable = 8,
    #[allow(dead_code)]
    Property = 9,
    EnumMember = 10,
    Function = 11,
    Comment = 12,
    Keyword = 13,
    Operator = 14,
    Number = 15,
    String = 16,
    Field = 17,
    Annotation = 18,
    InlineMacro = 19,
    GenericParamImpl = 20,
}
impl SemanticTokenKind {
    pub fn from_syntax_node<'db>(db: &'db AnalysisDatabase, node: SyntaxNode<'db>) -> Option<Self> {
        let node_kind = node.kind(db);

        // Simple tokens.
        if !matches!(node_kind, SyntaxKind::TokenIdentifier) {
            return Self::from_simple_token_kind(db, &node);
        }

        let identifier = node.ancestor_of_type::<ast::TerminalIdentifier>(db)?;

        // Non-keyword keywords.
        if [SUPER_KW, SELF_TYPE_KW, CRATE_KW].contains(&identifier.text(db)) {
            return Some(SemanticTokenKind::Keyword);
        }

        let identifier_parent = identifier.as_syntax_node().parent(db)?;
        if let Some(kind) = Self::from_identifier(db, &identifier) {
            return Some(kind);
        }

        let mut expr_path_ptr = None;

        for node in identifier_parent.ancestors_with_self(db) {
            if is_inline_macro(db, node) {
                return Some(SemanticTokenKind::InlineMacro);
            }

            match node.kind(db) {
                SyntaxKind::ExprPath => {
                    expr_path_ptr = Some(ast::ExprPath::from_syntax_node(db, node).stable_ptr(db));
                }
                SyntaxKind::Member => return Some(SemanticTokenKind::Variable),
                SyntaxKind::PatternIdentifier => return Some(SemanticTokenKind::Variable),
                SyntaxKind::Variant => return Some(SemanticTokenKind::EnumMember),
                SyntaxKind::Attribute => return Some(SemanticTokenKind::Annotation),
                _ => {}
            };

            for resultant in db.get_node_resultants(identifier.as_syntax_node()).unwrap_or_default()
            {
                if let Some(kind) = Self::from_resultant(db, resultant, expr_path_ptr) {
                    return Some(kind);
                }
            }
        }
        None
    }

    pub fn legend() -> Vec<SemanticTokenType> {
        vec![
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::CLASS,
            SemanticTokenType::ENUM,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::STRUCT,
            SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenType::TYPE,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::COMMENT,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::NUMBER,
            SemanticTokenType::STRING,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::DECORATOR,
            SemanticTokenType::MACRO,
            SemanticTokenType::INTERFACE,
        ]
    }

    /// Returns a semantic token kind for a simple token kind.
    /// Returns `None` if the token kind has no corresponding semantic token kind.
    fn from_simple_token_kind(db: &AnalysisDatabase, node: &SyntaxNode) -> Option<Self> {
        let node_kind = node.kind(db);
        let grandparent_kind = node.grandparent_kind(db);
        match node_kind {
            kind if kind.is_keyword_token() => Some(SemanticTokenKind::Keyword),
            SyntaxKind::TokenLiteralNumber => Some(SemanticTokenKind::Number),
            SyntaxKind::TokenNot
                if matches!(
                    grandparent_kind,
                    Some(SyntaxKind::ExprInlineMacro | SyntaxKind::ItemInlineMacro)
                ) =>
            {
                Some(SemanticTokenKind::InlineMacro)
            }
            SyntaxKind::TokenPlus
                if matches!(grandparent_kind, Some(SyntaxKind::GenericParamImplAnonymous)) =>
            {
                Some(SemanticTokenKind::GenericParamImpl)
            }
            SyntaxKind::TokenAnd
            | SyntaxKind::TokenAndAnd
            | SyntaxKind::TokenOr
            | SyntaxKind::TokenOrOr
            | SyntaxKind::TokenEqEq
            | SyntaxKind::TokenNeq
            | SyntaxKind::TokenGE
            | SyntaxKind::TokenGT
            | SyntaxKind::TokenLE
            | SyntaxKind::TokenLT
            | SyntaxKind::TokenNot
            | SyntaxKind::TokenPlus
            | SyntaxKind::TokenMinus
            | SyntaxKind::TokenMul
            | SyntaxKind::TokenDiv
            | SyntaxKind::TokenMod => Some(SemanticTokenKind::Operator),
            SyntaxKind::TokenSingleLineComment => Some(SemanticTokenKind::Comment),
            SyntaxKind::TokenShortString | SyntaxKind::TokenString => {
                Some(SemanticTokenKind::String)
            }
            _ => None,
        }
    }

    /// Returns a semantic token kind based on identifier.
    /// Returns `None` if the token kind has no corresponding semantic token kind.
    fn from_identifier(db: &AnalysisDatabase, identifier: &TerminalIdentifier) -> Option<Self> {
        let identifier_node = identifier.as_syntax_node();
        let parent_node = identifier_node.parent(db)?;
        let parent_kind = parent_node.kind(db);
        let grandparent_kind = parent_node.grandparent_kind(db);
        match parent_kind {
            SyntaxKind::ItemInlineMacro => Some(SemanticTokenKind::InlineMacro),
            SyntaxKind::AliasClause => Some(SemanticTokenKind::Class),
            SyntaxKind::ItemConstant | SyntaxKind::TraitItemConstant => {
                Some(SemanticTokenKind::EnumMember)
            }
            kind if ast::ModuleItem::is_variant(kind) => Some(SemanticTokenKind::Class),
            SyntaxKind::StructArgSingle => Some(SemanticTokenKind::Field),
            SyntaxKind::FunctionDeclaration => Some(SemanticTokenKind::Function),
            SyntaxKind::GenericParamType => Some(SemanticTokenKind::TypeParameter),
            SyntaxKind::PathSegmentSimple | SyntaxKind::PathSegmentWithGenericArgs => {
                match grandparent_kind {
                    Some(SyntaxKind::GenericParamImplAnonymous) => {
                        Some(SemanticTokenKind::GenericParamImpl)
                    }
                    Some(
                        SyntaxKind::GenericArgNamed
                        | SyntaxKind::GenericArgUnnamed
                        | SyntaxKind::GenericArgValueExpr,
                    ) => Some(SemanticTokenKind::TypeParameter),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    ///   Arguments:
    /// - `db`: The database to use for lookup.
    /// - `resultant`: The resultant syntax node.
    /// - `expr_path_ptr`: Optional expression path pointer.
    ///   Returns
    /// - `Some(SemanticTokenKind)` if a corresponding semantic token kind is found, otherwise returns `None`.
    fn from_resultant(
        db: &AnalysisDatabase,
        resultant: SyntaxNode,
        expr_path_ptr: Option<ExprPathPtr>,
    ) -> Option<SemanticTokenKind> {
        let terminal_ptr = db.find_closest_terminal(resultant)?;
        let lookup_item_id = db.find_lookup_item(resultant)?;

        if let Some(item) = db.lookup_resolved_generic_item_by_ptr(lookup_item_id, terminal_ptr) {
            return Some(match item {
                ResolvedGenericItem::GenericConstant(_) => SemanticTokenKind::EnumMember,
                ResolvedGenericItem::Module(_) => SemanticTokenKind::Namespace,
                ResolvedGenericItem::GenericFunction(_) => SemanticTokenKind::Function,
                ResolvedGenericItem::GenericType(_) | ResolvedGenericItem::GenericTypeAlias(_) => {
                    SemanticTokenKind::Type
                }
                ResolvedGenericItem::Variant(_) => SemanticTokenKind::EnumMember,
                ResolvedGenericItem::Trait(_) => SemanticTokenKind::Interface,
                ResolvedGenericItem::Impl(_) | ResolvedGenericItem::GenericImplAlias(_) => {
                    SemanticTokenKind::Class
                }
                ResolvedGenericItem::Variable(_) => SemanticTokenKind::Variable,
                ResolvedGenericItem::TraitItem(trait_item) => match trait_item {
                    TraitItemId::Function(_) => SemanticTokenKind::Function,
                    TraitItemId::Type(_) => SemanticTokenKind::Interface,
                    TraitItemId::Constant(_) => SemanticTokenKind::EnumMember,
                    TraitItemId::Impl(_) => SemanticTokenKind::Class,
                },
                ResolvedGenericItem::Macro(_) => SemanticTokenKind::InlineMacro,
            });
        }

        if let Some(item) = db.lookup_resolved_concrete_item_by_ptr(lookup_item_id, terminal_ptr) {
            return Some(match item {
                ResolvedConcreteItem::Constant(_) => SemanticTokenKind::EnumMember,
                ResolvedConcreteItem::Module(_) => SemanticTokenKind::Namespace,
                ResolvedConcreteItem::Function(_) => SemanticTokenKind::Function,
                ResolvedConcreteItem::Type(_) => SemanticTokenKind::Type,
                ResolvedConcreteItem::Variant(_) => SemanticTokenKind::EnumMember,
                ResolvedConcreteItem::Trait(_) | ResolvedConcreteItem::SelfTrait(_) => {
                    SemanticTokenKind::Interface
                }
                ResolvedConcreteItem::Impl(_) => SemanticTokenKind::Class,
                ResolvedConcreteItem::Macro(_) => SemanticTokenKind::InlineMacro,
            });
        }

        // Exprs and patterns.
        if let Some(function_id) = lookup_item_id.function_with_body()
            && let Some(expr_path_ptr) = expr_path_ptr
            && db.lookup_pattern_by_ptr(function_id, expr_path_ptr.into()).is_ok()
        {
            return Some(SemanticTokenKind::Variable);
        }

        None
    }
}
