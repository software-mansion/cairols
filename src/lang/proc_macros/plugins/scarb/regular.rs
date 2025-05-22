use cairo_lang_defs::patcher::{PatchBuilder, RewriteNode};
use cairo_lang_defs::plugin::{PluginDiagnostic, PluginGeneratedFile, PluginResult};
use cairo_lang_filesystem::db::Edition;
use cairo_lang_filesystem::ids::{CodeMapping, CodeOrigin};
use std::collections::HashSet;

use super::into_cairo_diagnostics;
use crate::lang::db::AnalysisDatabase;
use crate::lang::proc_macros::db::{get_attribute_expansion, get_derive_expansion};
use crate::lang::proc_macros::plugins::scarb::conversion::{
    CallSiteLocation, code_mapping_from_proc_macro_server,
};
use crate::lang::proc_macros::plugins::scarb::types::TokenStreamBuilder;
use cairo_lang_filesystem::span::TextSpan as CairoTextSpan;
use cairo_lang_macro::{AllocationContext, TextSpan, TokenStream, TokenStreamMetadata};
use cairo_lang_syntax::attribute::structured::{AttributeArgVariant, AttributeStructurize};
use cairo_lang_syntax::node::ast::{
    self, Expr, ImplItem, MaybeImplBody, MaybeTraitBody, PathSegment,
};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{SyntaxNode, Terminal, TypedStablePtr, TypedSyntaxNode};
use convert_case::{Case, Casing};
use itertools::Itertools;
use scarb_proc_macro_server_types::methods::ProcMacroResult;
use scarb_proc_macro_server_types::methods::expand::{ExpandAttributeParams, ExpandDeriveParams};
use scarb_proc_macro_server_types::scope::ProcMacroScope;
use scarb_stable_hash::StableHasher;

const DERIVE_ATTR: &str = "derive";

/// Copied from: <https://github.com/software-mansion/scarb/blob/4e81d1c4498137f80e211c6e2c6a5a6de01c66f2/scarb/src/compiler/plugin/proc_macro/host.rs#L893>
/// Modified scarb code with replaced dylib calls in favour of [`ProcMacroGroup`] calls. Also
/// removed `aux_data`.
pub fn macro_generate_code(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    item_ast: ast::ModuleItem,
    defined_attributes: &[String],
    defined_derives: &[String],
    metadata: &cairo_lang_defs::plugin::MacroPluginMetadata<'_>,
) -> PluginResult {
    let stream_metadata = calculate_metadata(db, item_ast.clone(), metadata.edition);

    // Handle inner functions.
    if let InnerAttrExpansionResult::Some(result) =
        expand_inner_attr(db, expansion_context.clone(), defined_attributes, item_ast.clone())
    {
        return result;
    }

    // Expand first attribute.
    // Note that we only expand the first attribute, as we assume that the rest of the attributes
    // will be handled by a subsequent call to this function.
    let ctx = AllocationContext::default();
    let (input, body) = parse_attribute(db, Vec::from(defined_attributes), item_ast.clone(), &ctx);

    if let Some(result) = match input {
        AttrExpansionFound::Last(AttrExpansionArgs { name, args, call_site }) => {
            Some((name, args, call_site, true))
        }
        AttrExpansionFound::Some(AttrExpansionArgs { name, args, call_site }) => {
            Some((name, args, call_site, false))
        }
        AttrExpansionFound::None => None,
    }
    .map(|(name, args, call_site, last)| {
        let token_stream = body.with_metadata(stream_metadata.clone());
        expand_attribute(
            db,
            expansion_context.clone(),
            name,
            last,
            args,
            token_stream,
            call_site,
            item_ast.as_syntax_node(),
        )
    }) {
        return result;
    }

    // Expand all derives.
    // Note that all proc macro attributes should be already expanded at this point.
    if let Some(result) = expand_derives(
        db,
        expansion_context.clone(),
        defined_derives,
        item_ast.clone(),
        stream_metadata.clone(),
    ) {
        return result;
    }

    // No expansions can be applied.
    PluginResult { code: None, diagnostics: Vec::new(), remove_original_item: false }
}

fn expand_inner_attr(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    defined_attributes: &[String],
    item_ast: ast::ModuleItem,
) -> InnerAttrExpansionResult {
    let mut context = InnerAttrExpansionContext::new();
    let mut item_builder = PatchBuilder::new(db, &item_ast);
    let mut used_attr_names: HashSet<String> = Default::default();
    let mut all_none = true;
    let ctx = AllocationContext::default();

    match item_ast.clone() {
        ast::ModuleItem::Trait(trait_ast) => {
            item_builder.add_node(trait_ast.attributes(db).as_syntax_node());
            item_builder.add_node(trait_ast.visibility(db).as_syntax_node());
            item_builder.add_node(trait_ast.trait_kw(db).as_syntax_node());
            item_builder.add_node(trait_ast.name(db).as_syntax_node());
            item_builder.add_node(trait_ast.generic_params(db).as_syntax_node());

            // Parser attributes for inner functions.
            match trait_ast.body(db) {
                MaybeTraitBody::None(terminal) => {
                    item_builder.add_node(terminal.as_syntax_node());
                    InnerAttrExpansionResult::None
                }
                MaybeTraitBody::Some(body) => {
                    item_builder.add_node(body.lbrace(db).as_syntax_node());

                    let item_list = body.items(db);
                    for item in item_list.elements(db).iter() {
                        let ast::TraitItem::Function(func) = item else {
                            item_builder.add_node(item.as_syntax_node());
                            continue;
                        };

                        let mut token_stream_builder = TokenStreamBuilder::new(db);
                        let attrs = func.attributes(db).elements(db);
                        let found = parse_attrs(
                            db,
                            defined_attributes,
                            &mut token_stream_builder,
                            attrs,
                            &ctx,
                        );
                        if let Some(name) = found.as_name() {
                            used_attr_names.insert(name);
                        }
                        token_stream_builder.add_node(func.declaration(db).as_syntax_node());
                        token_stream_builder.add_node(func.body(db).as_syntax_node());
                        let token_stream = token_stream_builder.build(&ctx);

                        all_none = all_none
                            && do_expand_inner_attr(
                                db,
                                &mut context,
                                expansion_context.clone(),
                                &mut item_builder,
                                found,
                                func,
                                token_stream,
                            );
                    }

                    item_builder.add_node(body.rbrace(db).as_syntax_node());

                    if all_none {
                        InnerAttrExpansionResult::None
                    } else {
                        let (code, mappings) = item_builder.build();
                        InnerAttrExpansionResult::Some(context.into_result(
                            code,
                            mappings,
                            used_attr_names.into_iter().collect(),
                        ))
                    }
                }
            }
        }

        ast::ModuleItem::Impl(impl_ast) => {
            item_builder.add_node(impl_ast.attributes(db).as_syntax_node());
            item_builder.add_node(impl_ast.visibility(db).as_syntax_node());
            item_builder.add_node(impl_ast.impl_kw(db).as_syntax_node());
            item_builder.add_node(impl_ast.name(db).as_syntax_node());
            item_builder.add_node(impl_ast.generic_params(db).as_syntax_node());
            item_builder.add_node(impl_ast.of_kw(db).as_syntax_node());
            item_builder.add_node(impl_ast.trait_path(db).as_syntax_node());

            match impl_ast.body(db) {
                MaybeImplBody::None(terminal) => {
                    item_builder.add_node(terminal.as_syntax_node());
                    InnerAttrExpansionResult::None
                }
                MaybeImplBody::Some(body) => {
                    item_builder.add_node(body.lbrace(db).as_syntax_node());

                    let items = body.items(db);
                    for item in items.elements(db) {
                        let ImplItem::Function(func) = item else {
                            item_builder.add_node(item.as_syntax_node());
                            continue;
                        };

                        let mut token_stream_builder = TokenStreamBuilder::new(db);
                        let attrs = func.attributes(db).elements(db);
                        let found = parse_attrs(
                            db,
                            defined_attributes,
                            &mut token_stream_builder,
                            attrs,
                            &ctx,
                        );
                        if let Some(name) = found.as_name() {
                            used_attr_names.insert(name);
                        }
                        token_stream_builder.add_node(func.visibility(db).as_syntax_node());
                        token_stream_builder.add_node(func.declaration(db).as_syntax_node());
                        token_stream_builder.add_node(func.body(db).as_syntax_node());
                        let token_stream = token_stream_builder.build(&ctx);
                        all_none = all_none
                            && do_expand_inner_attr(
                                db,
                                &mut context,
                                expansion_context.clone(),
                                &mut item_builder,
                                found,
                                &func,
                                token_stream,
                            );
                    }

                    item_builder.add_node(body.rbrace(db).as_syntax_node());

                    if all_none {
                        InnerAttrExpansionResult::None
                    } else {
                        let (code, mappings) = item_builder.build();
                        InnerAttrExpansionResult::Some(context.into_result(
                            code,
                            mappings,
                            used_attr_names.into_iter().collect(),
                        ))
                    }
                }
            }
        }
        _ => InnerAttrExpansionResult::None,
    }
}

fn do_expand_inner_attr(
    db: &AnalysisDatabase,
    context: &mut InnerAttrExpansionContext,
    expansion_context: ProcMacroScope,
    item_builder: &mut PatchBuilder<'_>,
    found: AttrExpansionFound,
    func: &impl TypedSyntaxNode,
    token_stream: TokenStream,
) -> bool {
    let mut all_none = true;
    let (name, args, call_site) = match found {
        AttrExpansionFound::Last(AttrExpansionArgs { name, args, call_site }) => {
            all_none = false;
            (name, args, call_site)
        }
        AttrExpansionFound::Some(AttrExpansionArgs { name, args, call_site }) => {
            all_none = false;
            (name, args, call_site)
        }
        AttrExpansionFound::None => {
            item_builder.add_node(func.as_syntax_node());
            return all_none;
        }
    };

    let result = get_attribute_expansion(
        db,
        ExpandAttributeParams {
            context: expansion_context,
            attr: name,
            args: args.clone(),
            item: token_stream.clone(),
            call_site: call_site.span,
        },
    );

    let expanded =
        context.register_result(db, token_stream.to_string(), result, call_site.stable_ptr);
    item_builder.add_modified(RewriteNode::Mapped {
        origin: func.as_syntax_node().span(db),
        node: Box::new(RewriteNode::Text(expanded.to_string())),
    });

    all_none
}

struct InnerAttrExpansionContext {
    // Metadata returned for expansions.
    diagnostics: Vec<PluginDiagnostic>,
    any_changed: bool,
}

impl InnerAttrExpansionContext {
    pub fn new() -> Self {
        Self { diagnostics: Vec::new(), any_changed: false }
    }

    pub fn register_result(
        &mut self,
        db: &dyn SyntaxGroup,
        original: String,
        result: ProcMacroResult,
        stable_ptr: SyntaxStablePtrId,
    ) -> String {
        let expanded = result.token_stream.to_string();
        let changed = expanded.as_str() != original;

        self.diagnostics.extend(into_cairo_diagnostics(db, result.diagnostics, stable_ptr));

        self.any_changed = self.any_changed || changed;

        expanded
    }
    pub fn into_result(
        self,
        expanded: String,
        code_mappings: Vec<CodeMapping>,
        attr_names: Vec<String>,
    ) -> PluginResult {
        let msg = if attr_names.len() == 1 {
            "the attribute macro"
        } else {
            "one of the attribute macros"
        };
        let derive_names = attr_names.iter().map(ToString::to_string).join("`, `");
        let note = format!("this error originates in {msg}: `{derive_names}`");

        PluginResult {
            code: Some(PluginGeneratedFile {
                name: "proc_attr_inner".into(),
                content: expanded,
                aux_data: None,
                code_mappings,
                diagnostics_note: Some(note),
            }),
            diagnostics: self.diagnostics,
            remove_original_item: true,
        }
    }
}

enum InnerAttrExpansionResult {
    None,
    Some(PluginResult),
}

pub enum AttrExpansionFound {
    Some(AttrExpansionArgs),
    Last(AttrExpansionArgs),
    None,
}

pub struct AttrExpansionArgs {
    pub name: String,
    pub args: TokenStream,
    pub call_site: CallSiteLocation,
}

impl AttrExpansionFound {
    pub fn as_name(&self) -> Option<String> {
        match self {
            AttrExpansionFound::Some(AttrExpansionArgs { name, .. })
            | AttrExpansionFound::Last(AttrExpansionArgs { name, .. }) => Some(name.clone()),
            AttrExpansionFound::None => None,
        }
    }
}

/// Find first attribute procedural macros that should be expanded.
///
/// Remove the attribute from the code.
pub(crate) fn parse_attribute(
    db: &dyn SyntaxGroup,
    defined_attributes: Vec<String>,
    item_ast: ast::ModuleItem,
    ctx: &AllocationContext,
) -> (AttrExpansionFound, TokenStream) {
    let mut token_stream_builder = TokenStreamBuilder::new(db);
    let input = match item_ast.clone() {
        ast::ModuleItem::Trait(trait_ast) => {
            let attrs = trait_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(trait_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(trait_ast.trait_kw(db).as_syntax_node());
            token_stream_builder.add_node(trait_ast.name(db).as_syntax_node());
            token_stream_builder.add_node(trait_ast.generic_params(db).as_syntax_node());
            token_stream_builder.add_node(trait_ast.body(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::Impl(impl_ast) => {
            let attrs = impl_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(impl_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(impl_ast.impl_kw(db).as_syntax_node());
            token_stream_builder.add_node(impl_ast.name(db).as_syntax_node());
            token_stream_builder.add_node(impl_ast.generic_params(db).as_syntax_node());
            token_stream_builder.add_node(impl_ast.of_kw(db).as_syntax_node());
            token_stream_builder.add_node(impl_ast.trait_path(db).as_syntax_node());
            token_stream_builder.add_node(impl_ast.body(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::Module(module_ast) => {
            let attrs = module_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(module_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(module_ast.module_kw(db).as_syntax_node());
            token_stream_builder.add_node(module_ast.name(db).as_syntax_node());
            token_stream_builder.add_node(module_ast.body(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::FreeFunction(free_func_ast) => {
            let attrs = free_func_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(free_func_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(free_func_ast.declaration(db).as_syntax_node());
            token_stream_builder.add_node(free_func_ast.body(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::ExternFunction(extern_func_ast) => {
            let attrs = extern_func_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(extern_func_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(extern_func_ast.extern_kw(db).as_syntax_node());
            token_stream_builder.add_node(extern_func_ast.declaration(db).as_syntax_node());
            token_stream_builder.add_node(extern_func_ast.semicolon(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::ExternType(extern_type_ast) => {
            let attrs = extern_type_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(extern_type_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(extern_type_ast.extern_kw(db).as_syntax_node());
            token_stream_builder.add_node(extern_type_ast.type_kw(db).as_syntax_node());
            token_stream_builder.add_node(extern_type_ast.name(db).as_syntax_node());
            token_stream_builder.add_node(extern_type_ast.generic_params(db).as_syntax_node());
            token_stream_builder.add_node(extern_type_ast.semicolon(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::Struct(struct_ast) => {
            let attrs = struct_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(struct_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(struct_ast.struct_kw(db).as_syntax_node());
            token_stream_builder.add_node(struct_ast.name(db).as_syntax_node());
            token_stream_builder.add_node(struct_ast.generic_params(db).as_syntax_node());
            token_stream_builder.add_node(struct_ast.lbrace(db).as_syntax_node());
            token_stream_builder.add_node(struct_ast.members(db).as_syntax_node());
            token_stream_builder.add_node(struct_ast.rbrace(db).as_syntax_node());
            expansion
        }
        ast::ModuleItem::Enum(enum_ast) => {
            let attrs = enum_ast.attributes(db).elements(db);
            let expansion =
                parse_attrs(db, &defined_attributes, &mut token_stream_builder, attrs, ctx);
            token_stream_builder.add_node(enum_ast.visibility(db).as_syntax_node());
            token_stream_builder.add_node(enum_ast.enum_kw(db).as_syntax_node());
            token_stream_builder.add_node(enum_ast.name(db).as_syntax_node());
            token_stream_builder.add_node(enum_ast.generic_params(db).as_syntax_node());
            token_stream_builder.add_node(enum_ast.lbrace(db).as_syntax_node());
            token_stream_builder.add_node(enum_ast.variants(db).as_syntax_node());
            token_stream_builder.add_node(enum_ast.rbrace(db).as_syntax_node());
            expansion
        }
        _ => AttrExpansionFound::None,
    };
    let token_stream = token_stream_builder.build(ctx);
    (input, token_stream)
}

fn parse_attrs(
    db: &dyn SyntaxGroup,
    defined_attributes: &[String],
    builder: &mut TokenStreamBuilder<'_>,
    attrs: Vec<ast::Attribute>,
    ctx: &AllocationContext,
) -> AttrExpansionFound {
    // This function parses attributes of the item,
    // checking if those attributes correspond to a procedural macro that should be fired.
    // The proc macro attribute found is removed from attributes list,
    // while other attributes are appended to the `PathBuilder` passed as an argument.

    // Note this function does not affect the executable attributes,
    // as it only pulls `ExpansionKind::Attr` from the plugin.
    // This means that executable attributes will neither be removed from the item,
    // nor will they cause the item to be rewritten.
    let mut expansion = None;
    let mut last = true;
    for attr in attrs {
        // We ensure that this flag is changed *after* the expansion is found.
        if last {
            let structured_attr = attr.clone().structurize(db);
            let found = defined_attributes.contains(&structured_attr.id.into());
            if found {
                if expansion.is_none() {
                    let mut args_builder = TokenStreamBuilder::new(db);
                    args_builder.add_node(attr.arguments(db).as_syntax_node());
                    let args = args_builder.build(ctx);
                    expansion = Some(AttrExpansionArgs {
                        name: attr.attr(db).as_syntax_node().get_text_without_trivia(db),
                        args,
                        call_site: CallSiteLocation::new(&attr, db),
                    });
                    // Do not add the attribute for found expansion.
                    continue;
                } else {
                    last = false;
                }
            }
        }
        builder.add_node(attr.as_syntax_node());
    }
    match (expansion, last) {
        (Some(args), true) => AttrExpansionFound::Last(args),
        (Some(args), false) => AttrExpansionFound::Some(args),
        (None, _) => AttrExpansionFound::None,
    }
}

/// Handle `#[derive(...)]` attribute.
///
/// Returns a list of expansions that this plugin should apply.
fn parse_derive(
    db: &dyn SyntaxGroup,
    defined_derives: &[String],
    item_ast: ast::ModuleItem,
) -> Vec<(String, CallSiteLocation)> {
    let attrs = match item_ast {
        ast::ModuleItem::Struct(struct_ast) => Some(struct_ast.query_attr(db, DERIVE_ATTR)),
        ast::ModuleItem::Enum(enum_ast) => Some(enum_ast.query_attr(db, DERIVE_ATTR)),
        _ => None,
    };

    attrs
        .unwrap_or_default()
        .iter()
        .map(|attr| attr.clone().structurize(db))
        .flat_map(|attr| attr.args.into_iter())
        .filter_map(|attr| {
            let AttributeArgVariant::Unnamed(value) = attr.clone().variant else {
                return None;
            };
            let Expr::Path(path) = value else {
                return None;
            };
            let path = path.segments(db).elements(db);
            let path = path.last()?;
            let PathSegment::Simple(segment) = path else {
                return None;
            };
            let ident = segment.ident(db);
            let value = ident.text(db).to_string();

            let matching_derive = defined_derives
                .iter()
                .find(|derive| derive.to_case(Case::Pascal) == value)
                .cloned()?;

            Some((matching_derive, CallSiteLocation::new(segment, db)))
        })
        .collect()
}

fn expand_derives(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    defined_derives: &[String],
    item_ast: ast::ModuleItem,
    stream_metadata: TokenStreamMetadata,
) -> Option<PluginResult> {
    let mut token_stream_builder = TokenStreamBuilder::new(db);
    token_stream_builder.add_node(item_ast.as_syntax_node());
    token_stream_builder.with_metadata(stream_metadata.clone());
    let ctx = AllocationContext::default();
    let token_stream = token_stream_builder.build(&ctx);

    // All derives to be applied.
    let derives = parse_derive(db, defined_derives, item_ast.clone());

    if derives.is_empty() {
        // No derives found - returning early.
        return None;
    }

    let stable_ptr = derives[0].1.stable_ptr;
    let span_db = stable_ptr.lookup(db).span(db);
    let call_site = TextSpan { start: span_db.start.as_u32(), end: span_db.end.as_u32() };

    let derive_names: Vec<String> = derives.into_iter().map(|a| a.0).collect();
    // region: Modified scarb code
    let result = get_derive_expansion(
        db,
        ExpandDeriveParams {
            context: expansion_context,
            derives: derive_names.clone(),
            item: token_stream,
            call_site,
        },
    );
    // endregion

    Some(PluginResult {
        code: if result.token_stream.is_empty() {
            None
        } else {
            let content = result.token_stream.to_string();
            let msg = if derive_names.len() == 1 {
                "the derive macro"
            } else {
                "one of the derive macros"
            };
            let derive_names = derive_names.iter().join("`, `");
            let note = format!("this error originates in {msg}: `{derive_names}`");

            let code_mappings = result
                .code_mappings
                .map(|x| x.into_iter().map(code_mapping_from_proc_macro_server).collect())
                .unwrap_or_default();

            Some(PluginGeneratedFile {
                name: "proc_macro_derive".into(),
                code_mappings,
                content,
                aux_data: None,
                diagnostics_note: Some(note),
            })
        },
        diagnostics: into_cairo_diagnostics(db, result.diagnostics, stable_ptr),
        // Note that we don't remove the original item here, unlike for attributes.
        // We do not add the original code to the generated file either.
        remove_original_item: false,
    })
}

#[allow(clippy::too_many_arguments)]
fn expand_attribute(
    db: &AnalysisDatabase,
    expansion_context: ProcMacroScope,
    name: String,
    last: bool,
    args: TokenStream,
    token_stream: TokenStream,
    call_site: CallSiteLocation,
    original_node: SyntaxNode,
) -> PluginResult {
    // region: Modified scarb code
    let result = get_attribute_expansion(
        db,
        ExpandAttributeParams {
            context: expansion_context,
            args,
            attr: name.clone(),
            item: token_stream.clone(),
            call_site: call_site.span,
        },
    );
    // endregion

    // Handle token stream.
    if result.token_stream.is_empty() {
        // Remove original code
        return PluginResult {
            diagnostics: into_cairo_diagnostics(db, result.diagnostics, call_site.stable_ptr),
            code: None,
            remove_original_item: true,
        };
    }

    // This is a minor optimization.
    // If the expanded macro attribute is the only one that will be expanded by `ProcMacroHost`
    // in this `generate_code` call (i.e. all the other macro attributes has been expanded by
    // previous calls), and the expansion did not produce any changes, we can skip rewriting the
    // expanded node by simply returning no generated code, and leaving the original item as is.
    // However, if we have other macro attributes to expand, we must rewrite the node even if no
    // changes have been produced, so that we can parse the attributes once again and expand them.
    // In essence, `code: None, remove_original_item: false` means `ProcMacroHost` will not be
    // called again for this AST item.
    // This optimization limits the number of generated nodes a bit.
    if last && token_stream.to_string() == result.token_stream.to_string() {
        return PluginResult {
            code: None,
            remove_original_item: false,
            diagnostics: into_cairo_diagnostics(db, result.diagnostics, call_site.stable_ptr),
        };
    }

    let file_name = format!("proc_macro_{}", name);
    let content = result.token_stream.to_string();
    PluginResult {
        code: Some(PluginGeneratedFile {
            name: file_name.into(),
            code_mappings: result
                .code_mappings
                .map(|x| x.into_iter().map(code_mapping_from_proc_macro_server).collect())
                .unwrap_or_else(|| {
                    vec![CodeMapping {
                        origin: CodeOrigin::Span(original_node.span_without_trivia(db)),
                        span: CairoTextSpan::from_str(&content),
                    }]
                }),
            content,
            aux_data: None,
            diagnostics_note: Some(format!(
                "this error originates in the attribute macro: `{}`",
                name
            )),
        }),
        diagnostics: into_cairo_diagnostics(db, result.diagnostics, call_site.stable_ptr),
        remove_original_item: true,
    }
}

fn calculate_metadata(
    db: &dyn SyntaxGroup,
    item_ast: ast::ModuleItem,
    edition: Edition,
) -> TokenStreamMetadata {
    fn short_hash(hashable: impl std::hash::Hash) -> String {
        let mut hasher = StableHasher::new();
        hashable.hash(&mut hasher);
        hasher.finish_as_short_hash()
    }

    let stable_ptr = item_ast.clone().stable_ptr(db).untyped();
    let file_path = stable_ptr.file_id(db).full_path(db);
    let file_id = short_hash(file_path.clone());
    let edition = serde_json::to_value(edition).unwrap();
    TokenStreamMetadata::new(file_path, file_id, edition)
}
