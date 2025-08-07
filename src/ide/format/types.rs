use cairo_lang_defs::ids::{GenericTypeId, ImportableId};
use cairo_lang_semantic::{
    GenericArgumentId, TypeId, TypeLongId,
    items::{
        constant::ConstValueId,
        imp::{ImplId, ImplLongId},
    },
};
use cairo_lang_utils::ordered_hash_map::OrderedHashMap;
use itertools::Itertools;

use crate::lang::db::AnalysisDatabase;

/// Returns a textual representation of a type with the given [`TypeId`],
/// consisting of name, path and generic parameters.
/// Precedes the type name with a shortest path allowed by `importables`.
pub fn format_type<'db>(
    db: &'db AnalysisDatabase,
    ty: TypeId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> String {
    match ty.long(db) {
        TypeLongId::Concrete(concrete_type) => {
            let importable = match concrete_type.generic_type(db) {
                GenericTypeId::Enum(enum_id) => ImportableId::Enum(enum_id),
                GenericTypeId::Struct(struct_id) => ImportableId::Struct(struct_id),
                GenericTypeId::Extern(extern_id) => ImportableId::ExternType(extern_id),
            };
            let path = importables
                .get(&importable)
                .cloned()
                .unwrap_or_else(|| concrete_type.generic_type(db).format(db));
            let generics = concrete_type.generic_args(db);

            if generics.is_empty() {
                path
            } else {
                let generics_list = generics
                    .into_iter()
                    .map(|generic| {
                        InferredValue::try_from_generic_arg_id(generic)
                            .map(|value| value.format(db, importables))
                            .unwrap_or_else(|| generic.format(db))
                    })
                    .join(", ");

                format!("{path}<{generics_list}>",)
            }
        }
        TypeLongId::Tuple(types) => {
            format!("({})", types.iter().map(|ty| format_type(db, *ty, importables)).join(", "))
        }
        TypeLongId::Snapshot(ty) => {
            format!("@{}", format_type(db, *ty, importables))
        }
        TypeLongId::FixedSizeArray { type_id, size } => {
            format!("[{}; {}]", format_type(db, *type_id, importables), size.format(db))
        }
        TypeLongId::Closure(closure) => {
            format!(
                "fn ({}) -> {}",
                closure.param_tys.iter().map(|ty| format_type(db, *ty, importables)).join(", "),
                format_type(db, closure.ret_ty, importables)
            )
        }
        TypeLongId::Missing(_) => "?".to_string(),
        _ => ty.format(db),
    }
}

pub enum InferredValue<'db> {
    Type(TypeId<'db>),
    Constant(ConstValueId<'db>),
    Impl(ImplId<'db>),
}

impl<'db> InferredValue<'db> {
    pub fn format(
        &self,
        db: &'db AnalysisDatabase,
        importables: &OrderedHashMap<ImportableId<'db>, String>,
    ) -> String {
        match *self {
            InferredValue::Type(ty) => format_type(db, ty, importables),
            InferredValue::Constant(const_id) => const_id.format(db),
            InferredValue::Impl(impl_id) => format_impl(db, impl_id),
        }
    }

    pub fn try_from_generic_arg_id(generic: GenericArgumentId<'db>) -> Option<Self> {
        match generic {
            GenericArgumentId::Type(ty) => Some(InferredValue::Type(ty)),
            GenericArgumentId::Constant(const_id) => Some(InferredValue::Constant(const_id)),
            GenericArgumentId::Impl(impl_id) => Some(InferredValue::Impl(impl_id)),
            GenericArgumentId::NegImpl => None,
        }
    }
}

fn format_impl<'db>(db: &'db AnalysisDatabase, impl_id: ImplId<'db>) -> String {
    // Translate unresolved impl to `<?>` instead of printing its salsa ID.
    if matches!(impl_id.long(db), ImplLongId::ImplVar(_)) {
        "<?>".to_string()
    } else {
        impl_id.format(db)
    }
}
