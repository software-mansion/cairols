use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_defs::plugin::InlineMacroExprPlugin;
use cairo_lang_defs::plugin::MacroPluginMetadata;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_semantic::diagnostic::NotFoundItemType;
use cairo_lang_semantic::expr::compute::Environment;
use cairo_lang_semantic::expr::inference::InferenceId;
use cairo_lang_semantic::items::macro_declaration::MacroDeclarationSemantic;
use cairo_lang_semantic::items::macro_declaration::MatcherContext;
use cairo_lang_semantic::items::macro_declaration::expand_macro_rule;
use cairo_lang_semantic::items::macro_declaration::is_macro_rule_match;
use cairo_lang_semantic::resolve::ResolutionContext;
use cairo_lang_semantic::resolve::ResolvedGenericItem;
use cairo_lang_semantic::resolve::Resolver;
use cairo_lang_syntax::node::TypedSyntaxNode;
use cairo_lang_syntax::node::ast;
use cairo_lang_syntax::node::helpers::GetIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use salsa::Database;

/// Modified https://github.com/starkware-libs/cairo/blob/176d303f5f62958a4615baf1991f049e7427b3e6/crates/cairo-lang-semantic/src/expr/compute.rs#L728
pub fn expand_single_inline_macro_no_context<'db>(
    db: &dyn Database,
    syntax: &ast::ExprInlineMacro<'db>,
    module_id: ModuleId<'_>,
) -> Option<String> {
    let crate_id = module_id.owning_crate(db);

    let mut resolver = Resolver::new(db, module_id, InferenceId::NoContext);
    let cfg_set = db
        .crate_config(crate_id)
        .and_then(|cfg| cfg.settings.cfg_set.as_ref())
        .unwrap_or(db.cfg_set());
    let edition = db.crate_config(crate_id).map(|cfg| cfg.settings.edition).unwrap_or_default();

    let metadata = MacroPluginMetadata {
        cfg_set,
        declared_derives: db.declared_derives(crate_id),
        allowed_features: &Default::default(),
        edition,
    };

    let macro_name = syntax.path(db).identifier(db);
    // Skipping expanding an inline macro if it had a parser error.
    if syntax.as_syntax_node().descendants(db).any(|node| {
        matches!(
            node.kind(db),
            SyntaxKind::ExprMissing
                | SyntaxKind::WrappedArgListMissing
                | SyntaxKind::StatementMissing
                | SyntaxKind::ModuleItemMissing
                | SyntaxKind::TraitItemMissing
                | SyntaxKind::ImplItemMissing
                | SyntaxKind::TokenMissing
                | SyntaxKind::TokenSkipped
                | SyntaxKind::WrappedTokenTreeMissing
        )
    }) {
        return None;
    }
    // We call the resolver with a new diagnostics, since the diagnostics should not be reported
    // if the macro was found as a plugin.
    let user_defined_macro = resolver.resolve_generic_path(
        &mut Default::default(),
        &syntax.path(db),
        NotFoundItemType::Macro,
        ResolutionContext::Statement(&mut Environment::empty()),
    );
    if let Ok(ResolvedGenericItem::Macro(macro_declaration_id)) = user_defined_macro {
        let macro_rules = db.macro_declaration_rules(macro_declaration_id).ok()?;
        let (rule, (captures, placeholder_to_rep_id)) = macro_rules.iter().find_map(|rule| {
            is_macro_rule_match(db, rule, &syntax.arguments(db)).map(|res| (rule, res))
        })?;
        let mut matcher_ctx =
            MatcherContext { captures, placeholder_to_rep_id, ..Default::default() };
        expand_macro_rule(db, rule, &mut matcher_ctx)
            .ok()
            .map(|expanded_code| expanded_code.text.to_string())
    } else if let Some(macro_plugin_id) =
        db.crate_inline_macro_plugins(crate_id).get(&macro_name.to_string(db)).cloned()
    {
        let macro_plugin = macro_plugin_id.long(db);
        macro_plugin.generate_code(db, syntax, &metadata).code.map(|file| file.content)
    } else {
        None
    }
}
