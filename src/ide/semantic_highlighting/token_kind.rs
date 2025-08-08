use cairo_lang_defs::ids::TraitItemId;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::keyword::{CRATE_KW, SELF_TYPE_KW, SUPER_KW};
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_syntax::node::ast::{TerminalIdentifier, TerminalIdentifierPtr};
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedSyntaxNode, ast};
use lsp_types::SemanticTokenType;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};

pub enum SemanticTokenKind {
    Namespace,
    Class,
    Enum,
    Interface,
    Struct,
    TypeParameter,
    Type,
    Parameter,
    Variable,
    #[allow(dead_code)]
    Property,
    EnumMember,
    Function,
    Comment,
    Keyword,
    Operator,
    Number,
    String,
    Field,
    Annotation,
    InlineMacro,
    GenericParamImpl,
}
impl SemanticTokenKind {
    pub fn from_syntax_node(db: &AnalysisDatabase, node: SyntaxNode) -> Option<Self> {
        let node_kind = node.kind(db);
        // Simple tokens.
        match node_kind {
            SyntaxKind::TokenIdentifier => {}
            kind if kind.is_keyword_token() => return Some(SemanticTokenKind::Keyword),
            SyntaxKind::TokenLiteralNumber => return Some(SemanticTokenKind::Number),
            SyntaxKind::TokenNot
                if matches!(
                    node.grandparent_kind(db),
                    Some(SyntaxKind::ExprInlineMacro | SyntaxKind::ItemInlineMacro)
                ) =>
            {
                return Some(SemanticTokenKind::InlineMacro);
            }
            SyntaxKind::TokenPlus
                if matches!(
                    node.grandparent_kind(db),
                    Some(SyntaxKind::GenericParamImplAnonymous)
                ) =>
            {
                return Some(SemanticTokenKind::GenericParamImpl);
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
            | SyntaxKind::TokenMod => return Some(SemanticTokenKind::Operator),
            SyntaxKind::TokenSingleLineComment => return Some(SemanticTokenKind::Comment),
            SyntaxKind::TokenShortString | SyntaxKind::TokenString => {
                return Some(SemanticTokenKind::String);
            }
            _ => return None,
        };

        assert_eq!(node_kind, SyntaxKind::TokenIdentifier);

        let identifier = node.ancestor_of_type::<ast::TerminalIdentifier>(db)?;

        // Non-keyword keywords.
        if [SUPER_KW, SELF_TYPE_KW, CRATE_KW].contains(&identifier.text(db).as_str()) {
            return Some(SemanticTokenKind::Keyword);
        }

        let identifier_parent = identifier.as_syntax_node().parent(db)?;
        match identifier_parent.kind(db) {
            SyntaxKind::ItemInlineMacro => return Some(SemanticTokenKind::InlineMacro),
            SyntaxKind::AliasClause => return Some(SemanticTokenKind::Class),
            SyntaxKind::ItemConstant | SyntaxKind::TraitItemConstant => {
                return Some(SemanticTokenKind::EnumMember);
            }
            kind if ast::ModuleItem::is_variant(kind) => return Some(SemanticTokenKind::Class),
            SyntaxKind::StructArgSingle => return Some(SemanticTokenKind::Field),
            SyntaxKind::FunctionDeclaration => return Some(SemanticTokenKind::Function),
            SyntaxKind::GenericParamType => return Some(SemanticTokenKind::TypeParameter),
            SyntaxKind::PathSegmentSimple | SyntaxKind::PathSegmentWithGenericArgs => {
                match identifier_parent.grandparent_kind(db) {
                    Some(SyntaxKind::GenericParamImplAnonymous) => {
                        return Some(SemanticTokenKind::GenericParamImpl);
                    }
                    Some(
                        SyntaxKind::GenericArgNamed
                        | SyntaxKind::GenericArgUnnamed
                        | SyntaxKind::GenericArgValueExpr,
                    ) => {
                        return Some(SemanticTokenKind::TypeParameter);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        // Identifier.
        let mut expr_path_ptr = None;

        for node in identifier_parent.ancestors_with_self(db) {
            if is_inline_macro(db, node) {
                return Some(SemanticTokenKind::InlineMacro);
            }
            match node.kind(db) {
                SyntaxKind::ExprInlineMacro => return Some(SemanticTokenKind::InlineMacro),
                SyntaxKind::ExprPath => {
                    expr_path_ptr = Some(ast::ExprPath::from_syntax_node(db, node).stable_ptr(db));
                }
                SyntaxKind::Member => return Some(SemanticTokenKind::Variable),
                SyntaxKind::PatternIdentifier => return Some(SemanticTokenKind::Variable),
                SyntaxKind::Variant => return Some(SemanticTokenKind::EnumMember),
                SyntaxKind::Attribute => return Some(SemanticTokenKind::Annotation),
                _ => {}
            };

            // We use resultants here to get semantics of the actual node that is generated
            for (resultant, terminal_ptr) in
                get_resultants_and_closest_terminals(db, identifier.as_syntax_node())
            {
                if let Some(lookup_item_id) = db.find_lookup_item(resultant) {
                    if let Some(item) =
                        db.lookup_resolved_generic_item_by_ptr(lookup_item_id, terminal_ptr)
                    {
                        return Some(match item {
                            ResolvedGenericItem::GenericConstant(_) => {
                                SemanticTokenKind::EnumMember
                            }
                            ResolvedGenericItem::Module(_) => SemanticTokenKind::Namespace,
                            ResolvedGenericItem::GenericFunction(_) => SemanticTokenKind::Function,
                            ResolvedGenericItem::GenericType(_)
                            | ResolvedGenericItem::GenericTypeAlias(_) => SemanticTokenKind::Type,
                            ResolvedGenericItem::Variant(_) => SemanticTokenKind::EnumMember,
                            ResolvedGenericItem::Trait(_) => SemanticTokenKind::Interface,
                            ResolvedGenericItem::Impl(_)
                            | ResolvedGenericItem::GenericImplAlias(_) => SemanticTokenKind::Class,
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
                    if let Some(item) =
                        db.lookup_resolved_concrete_item_by_ptr(lookup_item_id, terminal_ptr)
                    {
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
                }
            }
        }
        None
    }
    pub fn as_u32(&self) -> u32 {
        match self {
            SemanticTokenKind::Namespace => 0,
            SemanticTokenKind::Class => 1,
            SemanticTokenKind::Enum => 2,
            SemanticTokenKind::Interface => 3,
            SemanticTokenKind::Struct => 4,
            SemanticTokenKind::TypeParameter => 5,
            SemanticTokenKind::Type => 6,
            SemanticTokenKind::Parameter => 7,
            SemanticTokenKind::Variable => 8,
            SemanticTokenKind::Property => 9,
            SemanticTokenKind::EnumMember => 10,
            SemanticTokenKind::Function => 11,
            SemanticTokenKind::Comment => 12,
            SemanticTokenKind::Keyword => 13,
            SemanticTokenKind::Operator => 14,
            SemanticTokenKind::Number => 15,
            SemanticTokenKind::String => 16,
            SemanticTokenKind::Field => 17,
            SemanticTokenKind::Annotation => 18,
            SemanticTokenKind::InlineMacro => 19,
            SemanticTokenKind::GenericParamImpl => 20,
        }
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
}

// Retrieves the most-likely-usable resultant, and the terminal ptr we can use for semantic lookup
fn get_resultants_and_closest_terminals(
    db: &AnalysisDatabase,
    node: SyntaxNode,
) -> Vec<(SyntaxNode, TerminalIdentifierPtr)> {
    let Some(resultants) = db.get_node_resultants(node) else {
        return vec![];
    };

    resultants
        .into_iter()
        .filter_map(|resultant| {
            let terminal = if resultant.kind(db).is_terminal() {
                Some(resultant)
            } else if resultant.kind(db).is_token() {
                resultant.ancestors(db).find(|ancestor| ancestor.kind(db).is_terminal())
            } else {
                None
            }?;

            Some((resultant, TerminalIdentifier::cast(db, terminal)?.stable_ptr(db)))
        })
        .collect()
}

/// Checks whether the given node is an inline macro invocation and not just the simple path segment.
fn is_inline_macro(db: &AnalysisDatabase, node: SyntaxNode) -> bool {
    if let Some(path_node) = node.ancestor_of_kind(db, SyntaxKind::ExprPath)
        && let Some(maybe_macro) = path_node.parent(db)
    {
        let kind = maybe_macro.kind(db);
        return kind == SyntaxKind::ExprInlineMacro || kind == SyntaxKind::ItemInlineMacro;
    }
    false
}
