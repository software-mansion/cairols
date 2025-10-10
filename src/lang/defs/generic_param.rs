use cairo_lang_defs::ids::{GenericParamId, LanguageElementId};
use cairo_lang_semantic::lsp_helpers::LspHelpers;
use cairo_lang_semantic::{ConcreteTraitId, GenericParam, TypeId};
use cairo_lang_syntax::node::{TypedStablePtr, ids::SyntaxStablePtrId};

use crate::{
    ide::format::{traits::format_trait_path, types::format_type},
    lang::db::AnalysisDatabase,
};

/// A representation of a named generic parameter (type parameter, generic const, named impl).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericParamDef<'db> {
    id: GenericParamId<'db>,
    name: String,
    definition_stable_ptr: SyntaxStablePtrId<'db>,
    semantic: GenericParamSemantic<'db>,
}

/// Information about the parameter from its semantic model, corresponding to its kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericParamSemantic<'db> {
    Type,
    Const { ty: TypeId<'db> },
    Impl { concrete_trait: Option<ConcreteTraitId<'db>> },
}

impl<'db> GenericParamDef<'db> {
    /// Constructs a new instance of [`GenericParamDef`]
    /// if the given [`GenericParam`] refers to an appropriate item (type parameter, generic const, named impl).
    /// Returns `None` for anonymous and negative impls.
    pub(super) fn new(
        db: &'db AnalysisDatabase,
        generic_param: &GenericParam<'db>,
    ) -> Option<Self> {
        let details = match generic_param {
            GenericParam::Type(_generic_param_type) => GenericParamSemantic::Type,
            GenericParam::Const(generic_param_const) => {
                GenericParamSemantic::Const { ty: generic_param_const.ty }
            }
            GenericParam::Impl(generic_param_impl) => GenericParamSemantic::Impl {
                concrete_trait: generic_param_impl.concrete_trait.ok(),
            },
            GenericParam::NegImpl(_) => return None,
        };

        // Name exists for types, consts and named impls.
        let name = generic_param.id().name(db)?;

        Some(Self {
            id: generic_param.id(),
            name: name.to_string(db),
            definition_stable_ptr: generic_param.stable_ptr(db).untyped(),
            semantic: details,
        })
    }

    /// Gets the stable pointer to the syntax node which defines this symbol.
    pub fn definition_stable_ptr(&self) -> SyntaxStablePtrId<'db> {
        self.definition_stable_ptr
    }

    pub fn name(&self, _db: &'db AnalysisDatabase) -> &str {
        &self.name
    }

    pub fn signature(&self, db: &AnalysisDatabase) -> String {
        let importables = || {
            let module_id = self.id.module_id(db);
            db.visible_importables_from_module(module_id).unwrap_or_default()
        };

        match self.semantic {
            GenericParamSemantic::Type => self.name(db).to_string(),
            GenericParamSemantic::Const { ty } => {
                let formatted_type = format_type(db, ty, &importables(), None);
                format!("const {}: {}", self.name(db), formatted_type)
            }
            GenericParamSemantic::Impl { concrete_trait } => {
                let trait_path = concrete_trait
                    .map(|concrete_trait_id| {
                        format_trait_path(db, concrete_trait_id.trait_id(db), &importables())
                    })
                    .unwrap_or_else(|| "?".to_string());
                format!("impl {}: {}", self.name(db), trait_path)
            }
        }
    }
}
