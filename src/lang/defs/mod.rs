use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use cairo_lang_utils::Upcast;
use smol_str::SmolStr;

use self::finder::{ResolvedItem, find_definition};
pub use self::item::ItemDef;
pub use self::member::MemberDef;
pub use self::module::ModuleDef;
pub use self::variable::VariableDef;
pub use self::variant::VariantDef;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::syntax::SyntaxNodeExt;
use crate::lang::usages::FindUsages;
use crate::lang::usages::search_scope::SearchScope;

mod finder;
mod item;
mod member;
mod module;
mod variable;
mod variant;

/// Keeps information about the symbol that is being searched for/inspected.
///
/// This is an ephemeral data structure.
/// Do not store it in any kind of state.
#[derive(Eq, PartialEq)]
pub enum SymbolDef {
    Item(ItemDef),
    Variable(VariableDef),
    ExprInlineMacro(SmolStr),
    Member(MemberDef),
    Variant(VariantDef),
    Module(ModuleDef),
}

impl SymbolDef {
    /// Finds definition of the symbol referred to by the given identifier.
    pub fn find(db: &AnalysisDatabase, identifier: &ast::TerminalIdentifier) -> Option<Self> {
        if let Some(parent) = identifier.as_syntax_node().parent() {
            if parent.kind(db.upcast()) == SyntaxKind::PathSegmentSimple
                && parent.grandparent_kind(db) == Some(SyntaxKind::ExprInlineMacro)
            {
                return Some(Self::ExprInlineMacro(
                    parent
                        .parent()
                        .expect("Grandparent already exists")
                        .get_text_without_trivia(db.upcast())
                        .into(),
                ));
            }
        }
        // Get the resolved item info and the syntax node of the definition.
        let lookup_items = db.collect_lookup_items_stack(&identifier.as_syntax_node())?;
        let resolved_item = find_definition(db, identifier, &lookup_items)?;
        let definition_node = resolved_item.definition_node(db)?;

        match resolved_item {
            ResolvedItem::Generic(ResolvedGenericItem::GenericConstant(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericFunction(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericType(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericTypeAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericImplAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Trait(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Impl(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Constant(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Function(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Type(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Trait(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Impl(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::SelfTrait(_)) => {
                ItemDef::new(db, &definition_node).map(Self::Item)
            }

            ResolvedItem::Generic(ResolvedGenericItem::Module(id))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Module(id)) => {
                Some(Self::Module(ModuleDef::new(db, id, definition_node)))
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variable(var_id)) => {
                Some(Self::Variable(VariableDef::new(db, var_id, definition_node)))
            }

            ResolvedItem::Member(member_id) => {
                MemberDef::new(db, member_id, definition_node).map(Self::Member)
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variant(variant)) => {
                VariantDef::new(db, variant.id, definition_node).map(Self::Variant)
            }

            ResolvedItem::Concrete(ResolvedConcreteItem::Variant(concrete_variant)) => {
                VariantDef::new(db, concrete_variant.id, definition_node).map(Self::Variant)
            }
        }
    }

    /// Gets a stable pointer to the "most interesting" syntax node of the symbol.
    ///
    /// Typically, this is this symbol's name node.
    pub fn definition_stable_ptr(&self) -> Option<SyntaxStablePtrId> {
        match self {
            Self::Item(d) => Some(d.definition_stable_ptr()),
            Self::Variable(d) => Some(d.definition_stable_ptr()),
            Self::ExprInlineMacro(_) => None,
            Self::Member(d) => Some(d.definition_stable_ptr()),
            Self::Variant(d) => Some(d.definition_stable_ptr()),
            Self::Module(d) => Some(d.definition_stable_ptr()),
        }
    }

    /// Gets the [`FileId`] and [`TextSpan`] of symbol's definition node's originating location.
    pub fn definition_location(&self, db: &AnalysisDatabase) -> Option<(FileId, TextSpan)> {
        let stable_ptr = self.definition_stable_ptr()?;
        let node = stable_ptr.lookup(db.upcast());
        let found_file = stable_ptr.file_id(db.upcast());
        let span = node.span_without_trivia(db.upcast());
        let width = span.width();
        let (file_id, mut span) =
            get_originating_location(db.upcast(), found_file, span.start_only(), None);
        span.end = span.end.add_width(width);
        Some((file_id, span))
    }

    /// Gets the name of the symbol.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        match self {
            Self::Item(it) => it.name(db),
            Self::Variable(it) => it.name(db),
            Self::ExprInlineMacro(name) => name.clone(),
            Self::Member(it) => it.name(db),
            Self::Variant(it) => it.name(db),
            Self::Module(it) => it.name(db),
        }
    }

    /// Builds a search scope for finding usages of this symbol.
    #[tracing::instrument(skip_all)]
    pub fn search_scope(&self, db: &AnalysisDatabase) -> SearchScope {
        match &self {
            Self::Variable(var) => {
                if let Some(owning_function) = var.definition_node().ancestor_of_kinds(
                    db,
                    &[SyntaxKind::FunctionWithBody, SyntaxKind::TraitItemFunction],
                ) {
                    SearchScope::file_span(
                        owning_function.stable_ptr().file_id(db.upcast()),
                        owning_function.span(db.upcast()),
                    )
                } else {
                    SearchScope::file(var.definition_stable_ptr().file_id(db.upcast()))
                }
            }

            // TODO(#195): Use visibility information to narrow down search scopes.
            _ => SearchScope::everything(db),
        }
    }

    /// Starts a find-usages search for this symbol.
    pub fn usages<'a>(&'a self, db: &'a AnalysisDatabase) -> FindUsages<'a> {
        FindUsages::new(self, db)
    }
}
