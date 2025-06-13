use cairo_lang_defs::ids::NamedLanguageElementId;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem, ResolverData};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{TypedSyntaxNode, ast};
use cairo_lang_utils::smol_str::SmolStr;

pub use self::finder::ResolvedItem;
pub use self::finder::{find_declaration, find_definition};
pub use self::item::ItemDef;
pub use self::member::MemberDef;
pub use self::module::ModuleDef;
pub use self::variable::VariableDef;
pub use self::variant::VariantDef;
use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
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
#[derive(Eq, PartialEq, Debug)]
pub enum SymbolDef {
    Item(ItemDef),
    Variable(VariableDef),
    ExprInlineMacro(SmolStr),
    Member(MemberDef),
    Variant(VariantDef),
    Module(ModuleDef),
}

/// An instance of Search (for definition or declaration).
/// Keeps result (def) and context information about the conducted search.
pub struct SymbolSearch {
    /// Result of the search
    pub def: SymbolDef,
    /// Intermediate result, used to construct `def`
    pub resolved_item: ResolvedItem,
    /// State of the resolver after this search
    pub resolver_data: Option<ResolverData>,
}

impl SymbolSearch {
    /// Finds definition of the symbol referred to by the given identifier.
    /// Gets you to the "nearest" point upwards in terms of top-down code breakdown
    /// (most likely you would use it to get this: usage -> impl)
    pub fn find_definition(
        db: &AnalysisDatabase,
        identifier: &ast::TerminalIdentifier,
    ) -> Option<Self> {
        // Get the resolved item info and the syntax node of the definition.
        let lookup_items =
            db.collect_lookup_items_with_parent_files(identifier.as_syntax_node())?;
        let mut resolver_data = None;
        let resolved_item = find_definition(db, identifier, &lookup_items, &mut resolver_data)?;

        Self::from_resolved_item(db, resolved_item, resolver_data)
    }

    /// Finds declaration of the symbol referred to by given identifier.
    /// This is always the same as definition except for any identifier referring to an impl item
    /// - for which it will return a corresponding item in the definition of a trait.
    pub fn find_declaration(
        db: &AnalysisDatabase,
        identifier: &ast::TerminalIdentifier,
    ) -> Option<Self> {
        // Get the resolved item info and the syntax node of the definition.
        let lookup_items =
            db.collect_lookup_items_with_parent_files(identifier.as_syntax_node())?;
        let mut resolver_data = None;
        let resolved_item = find_declaration(db, identifier, &lookup_items, &mut resolver_data)?;

        Self::from_resolved_item(db, resolved_item, resolver_data)
    }

    fn from_resolved_item(
        db: &AnalysisDatabase,
        resolved_item: ResolvedItem,
        resolver_data: Option<ResolverData>,
    ) -> Option<Self> {
        match resolved_item {
            ResolvedItem::Generic(ResolvedGenericItem::GenericConstant(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericFunction(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericType(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericTypeAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericImplAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Trait(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Impl(_))
            | ResolvedItem::Generic(ResolvedGenericItem::TraitItem(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Constant(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Function(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Type(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Trait(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Impl(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::SelfTrait(_))
            | ResolvedItem::ImplItem(_) => {
                ItemDef::new(db, resolved_item.definition_node(db)?).map(SymbolDef::Item)
            }

            ResolvedItem::Generic(ResolvedGenericItem::Module(id))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Module(id)) => {
                Some(SymbolDef::Module(ModuleDef::new(db, id, resolved_item.definition_node(db)?)))
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variable(var_id)) => {
                Some(SymbolDef::Variable(VariableDef::new(
                    db,
                    var_id,
                    resolved_item.definition_node(db)?,
                )))
            }

            ResolvedItem::Member(member_id) => {
                MemberDef::new(db, member_id, resolved_item.definition_node(db)?)
                    .map(SymbolDef::Member)
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variant(ref variant)) => {
                VariantDef::new(db, variant.id, resolved_item.definition_node(db)?)
                    .map(SymbolDef::Variant)
            }

            ResolvedItem::Concrete(ResolvedConcreteItem::Variant(ref concrete_variant)) => {
                VariantDef::new(db, concrete_variant.id, resolved_item.definition_node(db)?)
                    .map(SymbolDef::Variant)
            }

            ResolvedItem::ExprInlineMacro(ref inline_macro) => {
                Some(SymbolDef::ExprInlineMacro(inline_macro.clone()))
            }

            ResolvedItem::Concrete(ResolvedConcreteItem::Macro(ref inline_macro)) => {
                Some(SymbolDef::ExprInlineMacro(inline_macro.name(db)))
            }

            ResolvedItem::Generic(ResolvedGenericItem::Macro(ref inline_macro)) => {
                Some(SymbolDef::ExprInlineMacro(inline_macro.name(db)))
            }
        }
        .map(|def| Self { def, resolved_item, resolver_data })
    }

    /// Starts a find-usages search for this symbol.
    pub fn usages<'a>(&'a self, db: &'a AnalysisDatabase) -> FindUsages<'a> {
        FindUsages::new(&self.def, &self.resolved_item, db)
    }
}

impl SymbolDef {
    /// Gets the [`FileId`] and [`TextSpan`] of symbol's definition node's originating location.
    pub fn definition_location(&self, db: &AnalysisDatabase) -> Option<(FileId, TextSpan)> {
        let stable_ptr = self.definition_stable_ptr(db)?;
        let node = stable_ptr.lookup(db);
        let found_file = stable_ptr.file_id(db);
        let span = node.span_without_trivia(db);
        Some(get_originating_location(db, found_file, span, None))
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
                    // Add generated virtual files to the search scope if such files could be
                    // generated from this function;
                    // i.e. if there is an attribute on the function or an inline macro inside it.
                    // Otherwise, search only the function scope.
                    if owning_function
                        .ancestor_of_kinds(
                            db,
                            &[SyntaxKind::AttributeList, SyntaxKind::ExprInlineMacro],
                        )
                        .is_some()
                    {
                        let file_id = owning_function.stable_ptr(db).file_id(db);
                        let files_spans = db
                            .file_and_subfiles_with_corresponding_modules(file_id)
                            .unwrap_or_default()
                            .0
                            .into_iter()
                            .map(|f| {
                                if f == file_id {
                                    (f, Some(owning_function.span(db)))
                                } else {
                                    (f, None)
                                }
                            })
                            .collect();

                        SearchScope::files_spans(files_spans)
                    } else {
                        SearchScope::file_span(
                            owning_function.stable_ptr(db).file_id(db),
                            owning_function.span(db),
                        )
                    }
                } else {
                    SearchScope::file(var.definition_stable_ptr(db).file_id(db))
                }
            }

            // TODO(#195): Use visibility information to narrow down search scopes.
            _ => SearchScope::everything(db),
        }
    }

    /// Gets a stable pointer to the "most interesting" syntax node of the symbol.
    ///
    /// Typically, this is this symbol's name node.
    pub fn definition_stable_ptr(&self, db: &dyn SyntaxGroup) -> Option<SyntaxStablePtrId> {
        match self {
            Self::Item(d) => Some(d.definition_stable_ptr()),
            Self::Variable(d) => Some(d.definition_stable_ptr(db)),
            Self::ExprInlineMacro(_) => None,
            Self::Member(d) => Some(d.definition_stable_ptr()),
            Self::Variant(d) => Some(d.definition_stable_ptr()),
            Self::Module(d) => Some(d.definition_stable_ptr()),
        }
    }
}
