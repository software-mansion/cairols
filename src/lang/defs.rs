use std::iter;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{
    LanguageElementId, LocalVarLongId, LookupItemId, MemberId, ModuleId, ModuleItemId,
    NamedLanguageElementId, SubmoduleLongId, TopLevelLanguageElementId, TraitItemId, VarId,
};
use cairo_lang_diagnostics::ToOption;
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::items::functions::{GenericFunctionId, ImplGenericFunctionId};
use cairo_lang_semantic::items::generics::generic_params_to_args;
use cairo_lang_semantic::items::imp::ImplLongId;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{Binding, ConcreteTraitLongId, Expr, Mutability, TypeLongId};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::{Intern, LookupIntern, Upcast};
use itertools::Itertools;
use smol_str::SmolStr;
use tracing::error;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup};
use crate::lang::syntax::SyntaxNodeExt;
use crate::lang::usages::FindUsages;
use crate::lang::usages::search_scope::SearchScope;

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
        let (definition_item, definition_node) = {
            let lookup_items = db.collect_lookup_items_stack(&identifier.as_syntax_node())?;
            let (resolved_item, stable_ptr) = find_definition(db, identifier, &lookup_items)?;
            let node = stable_ptr.lookup(db.upcast());
            (resolved_item, node)
        };

        match definition_item {
            ResolvedItem::Generic(ResolvedGenericItem::GenericConstant(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericFunction(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericType(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericTypeAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericImplAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Variant(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Trait(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Impl(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Constant(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Function(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Type(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Variant(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Trait(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Impl(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::SelfTrait(_)) => {
                ItemDef::new(db, &definition_node).map(Self::Item)
            }

            ResolvedItem::Generic(ResolvedGenericItem::Module(id)) => {
                Some(Self::Module(ModuleDef::new(db, id, definition_node)))
            }

            ResolvedItem::Concrete(ResolvedConcreteItem::Module(id)) => {
                Some(Self::Module(ModuleDef::new(db, id, definition_node)))
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variable(var_id)) => {
                Some(Self::Variable(VariableDef::new(db, var_id, definition_node)))
            }

            ResolvedItem::Member(member_id) => {
                MemberDef::new(db, member_id, definition_node).map(Self::Member)
            }
        }
    }

    /// Gets a stable pointer to the "most interesting" syntax node of the symbol.
    ///
    /// Typically, this is this symbol's name node.
    pub fn definition_stable_ptr(&self) -> Option<SyntaxStablePtrId> {
        match self {
            Self::Item(d) => Some(d.definition_stable_ptr),
            Self::Variable(d) => Some(d.definition_stable_ptr()),
            Self::ExprInlineMacro(_) => None,
            Self::Member(d) => Some(d.definition_stable_ptr),
            Self::Module(d) => Some(d.definition_stable_ptr),
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
            Self::Module(it) => it.name(db),
        }
    }

    /// Builds a search scope for finding usages of this symbol.
    #[tracing::instrument(skip_all)]
    pub fn search_scope(&self, db: &AnalysisDatabase) -> SearchScope {
        match &self {
            Self::Variable(var) => {
                if let Some(owning_function) = var.definition_node().parent_of_kinds(db, &[
                    SyntaxKind::FunctionWithBody,
                    SyntaxKind::TraitItemFunction,
                ]) {
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

/// Information about the definition of an item (function, trait, impl, module, etc.).
#[derive(Eq, PartialEq)]
pub struct ItemDef {
    /// The [`LookupItemId`] associated with the item.
    lookup_item_id: LookupItemId,

    /// Parent item to use as context when building signatures, etc.
    ///
    /// Sometimes, a signature of an item, it might contain parts that are defined elsewhere.
    /// For example, for trait/impl items,
    /// signature may refer to generic params defined in the defining trait/impl.
    /// This reference allows including simplified signatures of such contexts alongside
    /// the signature of this item.
    context_items: Vec<LookupItemId>,

    definition_stable_ptr: SyntaxStablePtrId,
}

impl ItemDef {
    /// Constructs new [`ItemDef`] instance.
    fn new(db: &AnalysisDatabase, definition_node: &SyntaxNode) -> Option<Self> {
        let mut lookup_item_ids = db.collect_lookup_items_stack(definition_node)?.into_iter();

        // Pull the lookup item representing the defining item.
        let lookup_item_id = lookup_item_ids.next()?;

        // Collect context items.
        let context_items = lookup_item_ids
            .take_while(|item| {
                matches!(
                    item,
                    LookupItemId::ModuleItem(ModuleItemId::Struct(_))
                        | LookupItemId::ModuleItem(ModuleItemId::Enum(_))
                        | LookupItemId::ModuleItem(ModuleItemId::Trait(_))
                        | LookupItemId::ModuleItem(ModuleItemId::Impl(_))
                        | LookupItemId::TraitItem(TraitItemId::Impl(_))
                )
            })
            .collect();

        Some(Self {
            lookup_item_id,
            context_items,
            definition_stable_ptr: definition_node.stable_ptr(),
        })
    }

    /// Get item signature without its body including signatures of its contexts.
    pub fn signature(&self, db: &AnalysisDatabase) -> String {
        let contexts = self.context_items.iter().copied().rev();
        let this = iter::once(self.lookup_item_id);
        contexts.chain(this).map(|item| db.get_item_signature(item.into())).join("\n")
    }

    /// Gets item documentation in a final form usable for display.
    pub fn documentation(&self, db: &AnalysisDatabase) -> Option<String> {
        db.get_item_documentation(self.lookup_item_id.into())
    }

    /// Gets the full path (including crate name and defining trait/impl if applicable)
    /// to the module containing the item.
    pub fn definition_path(&self, db: &AnalysisDatabase) -> String {
        let defs_db = db.upcast();
        match self.lookup_item_id {
            LookupItemId::ModuleItem(item) => item.parent_module(defs_db).full_path(defs_db),
            LookupItemId::TraitItem(item) => item.trait_id(defs_db).full_path(defs_db),
            LookupItemId::ImplItem(item) => item.impl_def_id(defs_db).full_path(defs_db),
        }
    }

    /// Gets the name of the item.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        let defs_db = db.upcast();
        match self.lookup_item_id {
            LookupItemId::ModuleItem(item) => item.name(defs_db),
            LookupItemId::TraitItem(item) => item.name(defs_db),
            LookupItemId::ImplItem(item) => item.name(defs_db),
        }
    }
}

/// Information about the definition of a variable (local, function parameter).
#[derive(Eq, PartialEq)]
pub struct VariableDef {
    var_id: VarId,
    identifier: ast::TerminalIdentifier,
}

impl VariableDef {
    /// Constructs a new [`VariableDef`] instance.
    fn new(db: &AnalysisDatabase, var_id: VarId, definition_node: SyntaxNode) -> Self {
        let identifier = ast::TerminalIdentifier::from_syntax_node(db, definition_node);
        Self { var_id, identifier }
    }

    /// Gets the syntax node which defines this variable.
    pub fn definition_node(&self) -> SyntaxNode {
        self.identifier.as_syntax_node()
    }

    /// Gets the stable pointer to the syntax node which defines this variable.
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

/// Information about a struct member.
#[derive(Eq, PartialEq)]
pub struct MemberDef {
    member_id: MemberId,
    structure: ItemDef,
    definition_stable_ptr: SyntaxStablePtrId,
}

impl MemberDef {
    /// Constructs a new [`MemberDef`] instance.
    pub fn new(
        db: &AnalysisDatabase,
        member_id: MemberId,
        definition_node: SyntaxNode,
    ) -> Option<Self> {
        let structure = ItemDef::new(db, &definition_node)?;
        Some(Self { member_id, structure, definition_stable_ptr: definition_node.stable_ptr() })
    }

    /// Gets [`MemberId`] associated with this symbol.
    pub fn member_id(&self) -> MemberId {
        self.member_id
    }

    /// Gets a definition of the structure which this symbol is a member of.
    pub fn structure(&self) -> &ItemDef {
        &self.structure
    }

    /// Gets member's name.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        self.member_id.name(db)
    }
}

/// Information about the definition of a module.
#[derive(Eq, PartialEq)]
pub struct ModuleDef {
    id: ModuleId,
    /// A full path to the parent module if [`ModuleId`] points to a submodule,
    /// None otherwise (i.e. for a crate root).
    parent_full_path: Option<String>,
    definition_stable_ptr: SyntaxStablePtrId,
}

impl ModuleDef {
    /// Constructs a new [`ModuleDef`] instance.
    pub fn new(db: &AnalysisDatabase, id: ModuleId, definition_node: SyntaxNode) -> Self {
        let parent_full_path = id
            .full_path(db)
            .strip_suffix(id.name(db).as_str())
            .unwrap()
            // Fails when the path lacks `::`, i.e. when we import from a crate root.
            .strip_suffix("::")
            .map(String::from);

        ModuleDef { id, parent_full_path, definition_stable_ptr: definition_node.stable_ptr() }
    }

    /// Gets the module signature: a name preceded by a qualifier: "mod" for submodule
    /// and "crate" for crate root.
    pub fn signature(&self, db: &AnalysisDatabase) -> String {
        let prefix = if self.parent_full_path.is_some() { "mod" } else { "crate" };
        format!("{prefix} {}", self.id.name(db))
    }

    /// Gets the full path of the parent module.
    pub fn definition_path(&self) -> String {
        self.parent_full_path.clone().unwrap_or_default()
    }

    /// Gets the module's documentation if it's available.
    pub fn documentation(&self, db: &AnalysisDatabase) -> Option<String> {
        let doc_id = match self.id {
            ModuleId::CrateRoot(id) => DocumentableItemId::Crate(id),
            ModuleId::Submodule(id) => DocumentableItemId::LookupItem(LookupItemId::ModuleItem(
                ModuleItemId::Submodule(id),
            )),
        };

        db.get_item_documentation(doc_id)
    }

    /// Gets the name of the module.
    pub fn name(&self, db: &AnalysisDatabase) -> SmolStr {
        self.id.name(db)
    }
}

/// Either [`ResolvedGenericItem`], [`ResolvedConcreteItem`] or [`MemberId`].
enum ResolvedItem {
    Generic(ResolvedGenericItem),
    Concrete(ResolvedConcreteItem),
    Member(MemberId),
}

fn find_definition(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<(ResolvedItem, SyntaxStablePtrId)> {
    // The lookup_resolved_*_item_by_ptr queries tend to sometimes return an encompassing item
    // instead of actually resolving the identifier.
    // The following series of heuristics resolve the identifier in alternative ways for such cases.

    if let Some(parent) = identifier.as_syntax_node().parent() {
        if parent.kind(db) == SyntaxKind::ItemModule {
            let Some(containing_module_file_id) = db.find_module_file_containing_node(&parent)
            else {
                error!("`find_definition` failed: could not find module");
                return None;
            };

            let submodule_id = SubmoduleLongId(
                containing_module_file_id,
                ast::ItemModule::from_syntax_node(db, parent).stable_ptr(),
            )
            .intern(db);
            let item = ResolvedGenericItem::Module(ModuleId::Submodule(submodule_id));
            return Some((
                ResolvedItem::Generic(item.clone()),
                resolved_generic_item_def(db, item)?,
            ));
        }
    }

    if let Some(member_id) = try_extract_member(db, identifier, lookup_items)
        .or_else(|| try_extract_member_from_constructor(db, identifier, lookup_items))
    {
        return Some((ResolvedItem::Member(member_id), member_id.untyped_stable_ptr(db)));
    }

    if let Some(var_id) = try_extract_variable_declaration(db, identifier, lookup_items) {
        let item = ResolvedGenericItem::Variable(var_id);
        return Some((ResolvedItem::Generic(item.clone()), resolved_generic_item_def(db, item)?));
    }

    for &lookup_item_id in lookup_items {
        if let Some(item) =
            db.lookup_resolved_generic_item_by_ptr(lookup_item_id, identifier.stable_ptr())
        {
            return Some((
                ResolvedItem::Generic(item.clone()),
                resolved_generic_item_def(db, item)?,
            ));
        }

        if let Some(item) =
            db.lookup_resolved_concrete_item_by_ptr(lookup_item_id, identifier.stable_ptr())
        {
            let stable_ptr = resolved_concrete_item_def(db.upcast(), item.clone())?;
            return Some((ResolvedItem::Concrete(item), stable_ptr));
        }
    }

    // FIXME(mkaput): This logic always kicks in if we're finding definition of undefined symbol
    //   which is very wrong in such cases.
    let item = match lookup_items.first().copied()? {
        LookupItemId::ModuleItem(item) => {
            ResolvedGenericItem::from_module_item(db, item).to_option()?
        }
        LookupItemId::TraitItem(trait_item) => {
            if let TraitItemId::Function(trait_function_id) = trait_item {
                let parent_trait = trait_item.trait_id(db);
                let generic_parameters = db.trait_generic_params(parent_trait).to_option()?;
                let concrete_trait = ConcreteTraitLongId {
                    trait_id: parent_trait,
                    generic_args: generic_params_to_args(&generic_parameters, db),
                };
                let concrete_trait = db.intern_concrete_trait(concrete_trait);

                ResolvedGenericItem::GenericFunction(GenericFunctionId::Impl(
                    ImplGenericFunctionId {
                        impl_id: ImplLongId::SelfImpl(concrete_trait).intern(db),
                        function: trait_function_id,
                    },
                ))
            } else {
                ResolvedGenericItem::Trait(trait_item.trait_id(db))
            }
        }
        LookupItemId::ImplItem(impl_item) => ResolvedGenericItem::Impl(impl_item.impl_def_id(db)),
    };

    Some((ResolvedItem::Generic(item.clone()), resolved_generic_item_def(db, item)?))
}

/// Extracts [`MemberId`] if the [`TerminalIdentifier`] is used as a struct member
/// in [`ast::ExprStructCtorCall`].
fn try_extract_member_from_constructor(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<MemberId> {
    let function_id = lookup_items.first()?.function_with_body()?;

    let identifier_node = identifier.as_syntax_node();

    let constructor_expr = identifier_node.parent_of_type::<ast::ExprStructCtorCall>(db)?;
    let constructor_expr_id =
        db.lookup_expr_by_ptr(function_id, constructor_expr.stable_ptr().into()).ok()?;

    let Expr::StructCtor(constructor_expr_semantic) =
        db.expr_semantic(function_id, constructor_expr_id)
    else {
        return None;
    };

    let struct_member = identifier_node.parent_of_type::<ast::StructArgSingle>(db)?;

    let struct_member_name =
        struct_member.identifier(db).as_syntax_node().get_text_without_trivia(db);

    constructor_expr_semantic
        .members
        .iter()
        .find_map(|(id, _)| struct_member_name.eq(id.name(db).as_str()).then_some(*id))
}

/// Extracts [`MemberId`] if the [`TerminalIdentifier`] points to
/// right-hand side of access member expression e.g., to `xyz` in `self.xyz`.
fn try_extract_member(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<MemberId> {
    let syntax_node = identifier.as_syntax_node();
    let binary_expr = syntax_node.parent_of_type::<ast::ExprBinary>(db)?;

    let function_with_body = lookup_items.first()?.function_with_body()?;

    let expr_id =
        db.lookup_expr_by_ptr(function_with_body, binary_expr.stable_ptr().into()).ok()?;
    let semantic_expr = db.expr_semantic(function_with_body, expr_id);

    if let Expr::MemberAccess(expr_member_access) = semantic_expr {
        let pointer_to_rhs = binary_expr.rhs(db).stable_ptr().untyped();

        let mut current_node = syntax_node;
        // Check if the terminal identifier points to a member, not a struct variable.
        while pointer_to_rhs != current_node.stable_ptr() {
            // If we found the node with the binary expression, then we are sure we won't find the
            // node with the member.
            if current_node.stable_ptr() == binary_expr.stable_ptr().untyped() {
                return None;
            }
            current_node = current_node.parent().unwrap();
        }

        Some(expr_member_access.member)
    } else {
        None
    }
}

/// Lookups if the identifier is a declaration of a variable/param in one of the lookup items.
///
/// Declaration identifiers aren't kept in `ResolvedData`, which is searched for by
/// `lookup_resolved_generic_item_by_ptr` and `lookup_resolved_concrete_item_by_ptr`.
/// Therefore, we have to look for these ourselves.
fn try_extract_variable_declaration(
    db: &AnalysisDatabase,
    identifier: &ast::TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<VarId> {
    let function_id = lookup_items.first()?.function_with_body()?;

    // Look at function parameters.
    if let Some(param) = identifier.as_syntax_node().parent_of_kind(db, SyntaxKind::Param) {
        // Closures have different semantic model structures than regular functions.
        let params = if let Some(expr_closure_ast) = param.parent_of_type::<ast::ExprClosure>(db) {
            let expr_id =
                db.lookup_expr_by_ptr(function_id, expr_closure_ast.stable_ptr().into()).ok()?;
            let Expr::ExprClosure(expr_closure_semantic) = db.expr_semantic(function_id, expr_id)
            else {
                unreachable!("expected semantic for ast::ExprClosure to be Expr::ExprClosure");
            };
            expr_closure_semantic.params
        } else {
            let signature = db.function_with_body_signature(function_id).ok()?;
            signature.params
        };

        if let Some(param) =
            params.into_iter().find(|param| param.stable_ptr == identifier.stable_ptr())
        {
            return Some(VarId::Param(param.id));
        }
    }

    // Look at patterns in the function body.
    if let Some(pattern) = identifier.as_syntax_node().parent_of_type::<ast::Pattern>(db) {
        // Bail out if the pattern happens to not exist in the semantic model.
        // We don't need semantics for returning, though, due to the way how local variables are
        // identified there, so we're happily ignoring the result value.
        db.lookup_pattern_by_ptr(function_id, pattern.stable_ptr()).ok()?;

        return Some(VarId::Local(
            LocalVarLongId(function_id.module_file_id(db), identifier.stable_ptr()).intern(db),
        ));
    }

    None
}

fn resolved_concrete_item_def(
    db: &AnalysisDatabase,
    item: ResolvedConcreteItem,
) -> Option<SyntaxStablePtrId> {
    match item {
        ResolvedConcreteItem::Type(ty) => {
            if let TypeLongId::GenericParameter(param) = ty.lookup_intern(db) {
                Some(param.untyped_stable_ptr(db.upcast()))
            } else {
                None
            }
        }
        ResolvedConcreteItem::Impl(imp) => {
            if let ImplLongId::GenericParameter(param) = imp.lookup_intern(db) {
                Some(param.untyped_stable_ptr(db.upcast()))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn resolved_generic_item_def(
    db: &AnalysisDatabase,
    item: ResolvedGenericItem,
) -> Option<SyntaxStablePtrId> {
    Some(match item {
        ResolvedGenericItem::GenericConstant(item) => item.untyped_stable_ptr(db.upcast()),

        ResolvedGenericItem::Module(module_id) => {
            match module_id {
                ModuleId::CrateRoot(_) => {
                    // For crate root files (src/lib.cairo), the definition node is the file itself.
                    let module_file = db.module_main_file(module_id).ok()?;
                    let file_syntax = db.file_module_syntax(module_file).ok()?;
                    file_syntax.as_syntax_node().stable_ptr()
                }
                ModuleId::Submodule(submodule_id) => {
                    // For submodules, the definition node is the identifier in `mod <ident> .*`.
                    submodule_id
                        .stable_ptr(db.upcast())
                        .lookup(db.upcast())
                        .name(db.upcast())
                        .stable_ptr()
                        .untyped()
                }
            }
        }

        ResolvedGenericItem::GenericFunction(item) => {
            let declaration: ast::FunctionDeclaration = match item {
                GenericFunctionId::Free(id) => {
                    id.stable_ptr(db.upcast()).lookup(db.upcast()).declaration(db.upcast())
                }
                GenericFunctionId::Extern(id) => {
                    id.stable_ptr(db.upcast()).lookup(db.upcast()).declaration(db.upcast())
                }
                GenericFunctionId::Impl(id) => match id.impl_function(db.upcast()) {
                    Ok(Some(id)) => {
                        id.stable_ptr(db.upcast()).lookup(db.upcast()).declaration(db.upcast())
                    }
                    // It is possible (Marek didn't find out how it happens), that we hop into a
                    // situation where concrete impl is not inferred yet, so we can't find the
                    // declaration. Fall back to trait function in such cases.
                    _ => id
                        .function
                        .stable_ptr(db.upcast())
                        .lookup(db.upcast())
                        .declaration(db.upcast()),
                },
            };
            declaration.name(db.upcast()).stable_ptr().untyped()
        }

        ResolvedGenericItem::GenericType(generic_type) => {
            generic_type.untyped_stable_ptr(db.upcast())
        }

        ResolvedGenericItem::GenericTypeAlias(type_alias) => {
            type_alias.untyped_stable_ptr(db.upcast())
        }

        ResolvedGenericItem::GenericImplAlias(impl_alias) => {
            impl_alias.untyped_stable_ptr(db.upcast())
        }

        ResolvedGenericItem::Variant(variant) => variant.id.stable_ptr(db.upcast()).untyped(),

        ResolvedGenericItem::Trait(trt) => trt.stable_ptr(db.upcast()).untyped(),

        ResolvedGenericItem::Impl(imp) => imp.stable_ptr(db.upcast()).untyped(),

        ResolvedGenericItem::Variable(var) => match var {
            VarId::Param(param) => param
                .stable_ptr(db.upcast())
                .lookup(db.upcast())
                .name(db.upcast())
                .stable_ptr()
                .untyped(),
            VarId::Local(var) => var.untyped_stable_ptr(db.upcast()),
            VarId::Item(item) => item.name_stable_ptr(db.upcast()),
        },
    })
}
