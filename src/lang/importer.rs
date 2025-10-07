use std::fmt::Display;

use cairo_lang_defs::{
    db::DefsGroup,
    ids::{LanguageElementId, ModuleId},
};
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_syntax::node::{
    TypedStablePtr, TypedSyntaxNode, ast::ItemUse, ids::SyntaxStablePtrId,
};
use lsp_types::{Position, Range, TextEdit};

use super::{analysis_context::AnalysisContext, db::AnalysisDatabase};
use crate::lang::lsp::ToLsp;

/// Returns a TextEdit to import the given trait if it is not already in scope.
/// The decision is based on visibility from the current module in `ctx`.
pub fn import_edit_for_trait_if_needed<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    trait_id: cairo_lang_defs::ids::TraitId<'db>,
) -> Option<TextEdit> {
    let trait_path = db.visible_traits_from_module(ctx.module_file_id)?.get(&trait_id)?.clone();
    // If the path contains '::', it means it is not currently in scope and needs an import.
    if trait_path.contains("::") { new_import_edit(db, ctx, trait_path) } else { None }
}

pub fn new_import_edit<'db>(
    db: &'db AnalysisDatabase,
    ctx: &AnalysisContext<'db>,
    import_path: impl Display,
) -> Option<TextEdit> {
    let use_position = use_position(db, ctx)?;

    let mut new_text = format!("use {import_path};\n");

    if !use_position.is_sticking {
        new_text.push('\n');
    }

    Some(TextEdit { range: use_position.range(), new_text })
}

fn use_position<'db>(db: &'db AnalysisDatabase, ctx: &AnalysisContext<'db>) -> Option<UsePosition> {
    db.module_uses_ids(ctx.module_file_id.0)
        .ok()
        .and_then(|uses| {
            let module_main_file = db.module_main_file(ctx.module_file_id.0).ok()?;

            uses.iter()
                .find(|use_statement| {
                    use_statement.stable_ptr(db).untyped().file_id(db) == module_main_file
                })
                .map(|use_statement| {
                    use_statement
                        .stable_ptr(db)
                        .untyped()
                        .lookup(db)
                        .ancestor_of_type::<ItemUse>(db)
                        .expect("use leaf must be in use item")
                        .use_kw(db)
                        .token(db)
                        .stable_ptr(db)
                        .untyped()
                })
                .map(|stable_ptr| UsePosition::new(db, stable_ptr, true))
                .map(Some)
                .unwrap_or_else(|| {
                    ctx.module_file_id
                        .0
                        .module_data(db)
                        .ok()?
                        .items(db)
                        .first()
                        .map(|item| UsePosition::new(db, item.untyped_stable_ptr(db), false))
                })
        })
        .unwrap_or_else(|| {
            let ModuleId::Submodule(submodule) = ctx.module_file_id.0 else { unreachable!() };

            UsePosition::new(db, submodule.untyped_stable_ptr(db), false)
        })
}

#[derive(Debug)]
struct UsePosition {
    position: Position,
    pub is_sticking: bool,
}

impl UsePosition {
    pub fn new<'db>(
        db: &'db AnalysisDatabase,
        ptr: SyntaxStablePtrId<'db>,
        is_sticking: bool,
    ) -> Option<Self> {
        let node = ptr.lookup(db);

        Some(Self {
            position: node
                .span_start_without_trivia(db)
                .position_in_file(db, ptr.file_id(db))?
                .to_lsp(),
            is_sticking,
        })
    }

    pub fn range(&self) -> Range {
        Range::new(self.position, self.position)
    }
}
