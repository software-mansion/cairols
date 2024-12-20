use std::iter;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{
    FunctionTitleId, LanguageElementId, LookupItemId, MemberId, ModuleId, ModuleItemId,
    NamedLanguageElementId, SubmoduleLongId, TopLevelLanguageElementId, TraitItemId,
};
use cairo_lang_diagnostics::ToOption;
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_filesystem::db::get_originating_location;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_filesystem::span::TextSpan;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::expr::pattern::QueryPatternVariablesFromDb;
use cairo_lang_semantic::items::function_with_body::SemanticExprLookup;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::items::imp::ImplLongId;
use cairo_lang_semantic::lookup_item::LookupItemEx;
use cairo_lang_semantic::resolve::{ResolvedConcreteItem, ResolvedGenericItem};
use cairo_lang_semantic::{Binding, Expr, Mutability, TypeLongId};
use cairo_lang_syntax::node::ast::{ExprPath, Param, PatternIdentifier, TerminalIdentifier};
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::utils::{is_grandparent_of_kind, is_parent_of_kind};
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode, ast};
use cairo_lang_utils::{Intern, LookupIntern, Upcast};
use itertools::Itertools;
use smol_str::SmolStr;
use tracing::error;

use crate::lang::db::{AnalysisDatabase, LsSemanticGroup, LsSyntaxGroup};
use crate::lang::inspect::usages::FindUsages;

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
    pub fn find(db: &AnalysisDatabase, identifier: &TerminalIdentifier) -> Option<Self> {
        if let Some(parent) = identifier.as_syntax_node().parent() {
            if parent.kind(db.upcast()) == SyntaxKind::PathSegmentSimple
                && is_grandparent_of_kind(db, &parent, SyntaxKind::ExprInlineMacro)
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
            | ResolvedItem::Generic(ResolvedGenericItem::TraitFunction(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericType(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericTypeAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::GenericImplAlias(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Variant(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Trait(_))
            | ResolvedItem::Generic(ResolvedGenericItem::Impl(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Constant(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Function(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::TraitFunction(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Type(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Variant(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Trait(_))
            | ResolvedItem::Concrete(ResolvedConcreteItem::Impl(_)) => {
                ItemDef::new(db, &definition_node).map(Self::Item)
            }

            ResolvedItem::Generic(ResolvedGenericItem::Module(id)) => {
                Some(Self::Module(ModuleDef::new(db, id, definition_node)))
            }

            ResolvedItem::Concrete(ResolvedConcreteItem::Module(id)) => {
                Some(Self::Module(ModuleDef::new(db, id, definition_node)))
            }

            ResolvedItem::Generic(ResolvedGenericItem::Variable(_)) => {
                VariableDef::new(db, definition_node).map(Self::Variable)
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
            Self::Variable(d) => Some(d.definition_stable_ptr),
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
            SymbolDef::Item(it) => it.name(db),
            SymbolDef::Variable(it) => it.name(),
            SymbolDef::ExprInlineMacro(name) => name.clone(),
            SymbolDef::Member(it) => it.name(db),
            SymbolDef::Module(it) => it.name(db),
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
    name: SmolStr,
    var: Binding,
    definition_stable_ptr: SyntaxStablePtrId,
}

impl VariableDef {
    /// Constructs a new [`VariableDef`] instance.
    fn new(db: &AnalysisDatabase, definition_node: SyntaxNode) -> Option<Self> {
        match definition_node.kind(db.upcast()) {
            SyntaxKind::TerminalIdentifier => {
                let pattern_node = definition_node.parent()?;
                let kind = pattern_node.kind(db.upcast());
                if let SyntaxKind::PatternIdentifier = kind {
                    let pattern_identifier =
                        PatternIdentifier::from_syntax_node(db.upcast(), pattern_node);
                    Self::new_pattern_identifier(db, pattern_identifier)
                } else if is_parent_of_kind(db, &pattern_node, SyntaxKind::ExprPath) {
                    let expr_path = ExprPath::from_syntax_node(
                        db.upcast(),
                        pattern_node.parent().expect("Grandparent already exists"),
                    );
                    let terminal_identifier =
                        TerminalIdentifier::from_syntax_node(db.upcast(), definition_node);
                    Self::new_expr_path(db, expr_path, terminal_identifier)
                } else {
                    error!(
                        "variable definition node parent is not an pattern identifier: {kind:?}"
                    );
                    None
                }
            }

            SyntaxKind::Param => {
                let param = Param::from_syntax_node(db.upcast(), definition_node);
                Self::new_param(db, param)
            }

            kind => {
                error!("variable definition node is not an identifier nor param: {kind:?}");
                None
            }
        }
    }

    /// Constructs a new [`VariableDef`] instance for [`PatternIdentifier`].
    fn new_pattern_identifier(
        db: &AnalysisDatabase,
        pattern_identifier: PatternIdentifier,
    ) -> Option<Self> {
        let name = pattern_identifier.name(db.upcast()).text(db.upcast());

        let definition_stable_ptr = pattern_identifier.stable_ptr().untyped();

        let var =
            lookup_binding_for_pattern(db, ast::Pattern::Identifier(pattern_identifier), &name)?;

        Some(Self { name, var, definition_stable_ptr })
    }

    /// Constructs a new [`VariableDef`] instance for a [`TerminalIdentifier`]
    /// that is a segment of an [`ExprPath`].
    fn new_expr_path(
        db: &AnalysisDatabase,
        expr_path: ExprPath,
        terminal_identifier: TerminalIdentifier,
    ) -> Option<Self> {
        let name = terminal_identifier.text(db.upcast());

        let definition_stable_ptr = terminal_identifier.stable_ptr().untyped();

        let var = lookup_binding_for_pattern(db, ast::Pattern::Path(expr_path), &name)?;

        Some(Self { name, var, definition_stable_ptr })
    }

    /// Constructs new [`VariableDef`] instance for [`Param`].
    fn new_param(db: &AnalysisDatabase, param: Param) -> Option<Self> {
        let name = param.name(db.upcast()).text(db.upcast());

        // Get the function which contains the variable/parameter.
        let function_id = db.find_lookup_item(&param.as_syntax_node())?.function_with_body()?;

        // Get function signature.
        let signature = db.function_with_body_signature(function_id).ok()?;

        // Extract parameter semantic from the found signature.
        let var = signature.params.into_iter().find(|p| p.name == name)?.into();

        Some(Self { name, var, definition_stable_ptr: param.stable_ptr().untyped() })
    }

    /// Gets variable signature, which tries to resemble the way how it is defined in code.
    pub fn signature(&self, db: &AnalysisDatabase) -> String {
        let Self { name, var, .. } = self;

        let prefix = match var {
            Binding::LocalVar(_) => "let ",
            Binding::LocalItem(_) => "const ",
            Binding::Param(_) => "",
        };

        let mutability = match var {
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

        let ty = var.ty().format(db.upcast());

        format!("{prefix}{mutability}{name}: {ty}")
    }

    /// Gets this variable's name.
    pub fn name(&self) -> SmolStr {
        self.name.clone()
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
    identifier: &TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<(ResolvedItem, SyntaxStablePtrId)> {
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

    for lookup_item_id in lookup_items.iter().copied() {
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

    // Skip variable definition, otherwise we would get parent ModuleItem for variable.
    if db.first_ancestor_of_kind(identifier.as_syntax_node(), SyntaxKind::StatementLet).is_none() {
        let item = match lookup_items.first().copied()? {
            LookupItemId::ModuleItem(item) => {
                ResolvedGenericItem::from_module_item(db, item).to_option()?
            }
            LookupItemId::TraitItem(trait_item) => {
                if let TraitItemId::Function(trait_fn) = trait_item {
                    ResolvedGenericItem::TraitFunction(trait_fn)
                } else {
                    ResolvedGenericItem::Trait(trait_item.trait_id(db))
                }
            }
            LookupItemId::ImplItem(impl_item) => {
                ResolvedGenericItem::Impl(impl_item.impl_def_id(db))
            }
        };

        Some((ResolvedItem::Generic(item.clone()), resolved_generic_item_def(db, item)?))
    } else {
        None
    }
}

/// Extracts [`MemberId`] if the [`TerminalIdentifier`] is used as a struct member
/// in [`ast::ExprStructCtorCall`].
fn try_extract_member_from_constructor(
    db: &AnalysisDatabase,
    identifier: &TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<MemberId> {
    let function_id = lookup_items.first()?.function_with_body()?;

    let identifier_node = identifier.as_syntax_node();

    let constructor =
        db.first_ancestor_of_kind(identifier_node.clone(), SyntaxKind::ExprStructCtorCall)?;
    let constructor_expr = ast::ExprStructCtorCall::from_syntax_node(db, constructor);
    let constructor_expr_id =
        db.lookup_expr_by_ptr(function_id, constructor_expr.stable_ptr().into()).ok()?;

    let Expr::StructCtor(constructor_expr_semantic) =
        db.expr_semantic(function_id, constructor_expr_id)
    else {
        return None;
    };

    let struct_member = db.first_ancestor_of_kind(identifier_node, SyntaxKind::StructArgSingle)?;
    let struct_member = ast::StructArgSingle::from_syntax_node(db, struct_member);

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
    identifier: &TerminalIdentifier,
    lookup_items: &[LookupItemId],
) -> Option<MemberId> {
    let syntax_node = identifier.as_syntax_node();
    let binary_expr_syntax_node =
        db.first_ancestor_of_kind(syntax_node.clone(), SyntaxKind::ExprBinary)?;
    let binary_expr = ast::ExprBinary::from_syntax_node(db, binary_expr_syntax_node);

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
    let defs_db = db.upcast();
    Some(match item {
        ResolvedGenericItem::GenericConstant(item) => item.untyped_stable_ptr(defs_db),
        ResolvedGenericItem::Module(module_id) => {
            // Check if the module is an inline submodule.
            if let ModuleId::Submodule(submodule_id) = module_id {
                if let ast::MaybeModuleBody::Some(submodule_id) =
                    submodule_id.stable_ptr(defs_db).lookup(db.upcast()).body(db.upcast())
                {
                    // Inline module.
                    return Some(submodule_id.stable_ptr().untyped());
                }
            }
            let module_file = db.module_main_file(module_id).ok()?;
            let file_syntax = db.file_module_syntax(module_file).ok()?;
            file_syntax.as_syntax_node().stable_ptr()
        }
        ResolvedGenericItem::GenericFunction(item) => {
            let title = match item {
                GenericFunctionId::Free(id) => FunctionTitleId::Free(id),
                GenericFunctionId::Extern(id) => FunctionTitleId::Extern(id),
                GenericFunctionId::Impl(id) => {
                    // Note: Only the trait title is returned.
                    FunctionTitleId::Trait(id.function)
                }
                GenericFunctionId::Trait(id) => FunctionTitleId::Trait(id.trait_function(db)),
            };
            title.untyped_stable_ptr(defs_db)
        }
        ResolvedGenericItem::GenericType(generic_type) => generic_type.untyped_stable_ptr(defs_db),
        ResolvedGenericItem::GenericTypeAlias(type_alias) => type_alias.untyped_stable_ptr(defs_db),
        ResolvedGenericItem::GenericImplAlias(impl_alias) => impl_alias.untyped_stable_ptr(defs_db),
        ResolvedGenericItem::Variant(variant) => variant.id.stable_ptr(defs_db).untyped(),
        ResolvedGenericItem::Trait(trt) => trt.stable_ptr(defs_db).untyped(),
        ResolvedGenericItem::Impl(imp) => imp.stable_ptr(defs_db).untyped(),
        ResolvedGenericItem::TraitFunction(trait_function) => {
            trait_function.stable_ptr(defs_db).untyped()
        }
        ResolvedGenericItem::Variable(var) => var.untyped_stable_ptr(defs_db),
    })
}

/// Extracts variable semantic (a binding) from the given pattern syntax node.
///
/// The `name` has to be the name of the variable (which is always known in advance when calling
/// this function but hard to reverse lookup).
fn lookup_binding_for_pattern(
    db: &AnalysisDatabase,
    pattern: ast::Pattern,
    name: &str,
) -> Option<Binding> {
    // Get the function which contains the variable/parameter.
    let function_id = db.find_lookup_item(&pattern.as_syntax_node())?.function_with_body()?;

    // Get the semantic model for the pattern.
    let pattern = db.pattern_semantic(
        function_id,
        db.lookup_pattern_by_ptr(function_id, pattern.stable_ptr()).ok()?,
    );

    // Extract the binding from the found pattern.
    let binding = pattern
        .variables(&QueryPatternVariablesFromDb(db.upcast(), function_id))
        .into_iter()
        .find(|pv| pv.name == name)?
        .var
        .into();

    Some(binding)
}
