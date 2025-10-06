use cairo_lang_defs::ids::{GenericTypeId, ImportableId, TopLevelLanguageElementId, TraitId};
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
/// If provided a context of `trait_id`, replaces it with `Self` in the output.
pub fn format_type<'db>(
    db: &'db AnalysisDatabase,
    ty: TypeId<'db>,
    importables: &OrderedHashMap<ImportableId<'db>, String>,
    trait_id: Option<TraitId>,
) -> String {
    let raw_type = match ty.long(db) {
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
                let formatted_generics_list = format_generic_args(db, &generics, importables);
                format!("{path}{formatted_generics_list}",)
            }
        }
        TypeLongId::Tuple(types) => {
            format!(
                "({})",
                types.iter().map(|ty| format_type(db, *ty, importables, trait_id)).join(", ")
            )
        }
        TypeLongId::Snapshot(ty) => {
            format!("@{}", format_type(db, *ty, importables, trait_id))
        }
        TypeLongId::FixedSizeArray { type_id, size } => {
            format!("[{}; {}]", format_type(db, *type_id, importables, trait_id), size.format(db))
        }
        TypeLongId::Closure(closure) => {
            format!(
                "fn ({}) -> {}",
                closure
                    .param_tys
                    .iter()
                    .map(|ty| format_type(db, *ty, importables, trait_id))
                    .join(", "),
                format_type(db, closure.ret_ty, importables, trait_id)
            )
        }
        TypeLongId::Missing(_) => "?".to_string(),
        _ => ty.format(db),
    };

    // FIXME(#631): We could place this logic in specific arms that would require it, when all relevant ones are implemented.
    if let Some(trait_id) = trait_id {
        replace_self_type(&raw_type, &trait_id.full_path(db))
    } else {
        raw_type
    }
}

/// Formats the provided generic arguments as a list wrapped with angle brackets.
pub fn format_generic_args<'db>(
    db: &'db AnalysisDatabase,
    args: &[GenericArgumentId<'db>],
    importables: &OrderedHashMap<ImportableId<'db>, String>,
) -> String {
    let arg_list = args
        .iter()
        .map(|&arg| {
            InferredValue::try_from_generic_arg_id(arg)
                .map(|value| value.format(db, importables))
                .unwrap_or_else(|| arg.format(db))
        })
        .join(", ");

    format!("<{arg_list}>")
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
            InferredValue::Type(ty) => format_type(db, ty, importables, None),
            InferredValue::Constant(const_id) => const_id.format(db),
            InferredValue::Impl(impl_id) => format_impl(db, impl_id),
        }
    }

    pub fn try_from_generic_arg_id(generic: GenericArgumentId<'db>) -> Option<Self> {
        match generic {
            GenericArgumentId::Type(ty) => Some(InferredValue::Type(ty)),
            GenericArgumentId::Constant(const_id) => Some(InferredValue::Constant(const_id)),
            GenericArgumentId::Impl(impl_id) => Some(InferredValue::Impl(impl_id)),
            GenericArgumentId::NegImpl(_) => None,
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

fn replace_self_type(input: &str, self_type: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let self_type_len = self_type.len();
    let bytes = input.as_bytes();

    while i < input.len() {
        // Detect the start of the expected type followed by turbofish
        if i + self_type_len + 2 < bytes.len()
            && &input[i..i + self_type_len] == self_type
            && &input[i + self_type_len..i + self_type_len + 2] == "::"
        {
            // Ensure it's followed by < indicating turbofish
            let generics_start = i + self_type_len + 2;
            if input.as_bytes().get(generics_start) == Some(&b'<') {
                // Append 'Self' instead of the original type and turbofish
                result.push_str("Self");
                i = skip_generics(input, generics_start);
                continue;
            }
        }

        // Append the current character to the result and continue
        result.push(input.as_bytes()[i] as char);
        i += 1;
    }

    result
}

// Skips the generics part and returns new position just after closing '>'
fn skip_generics(input: &str, start: usize) -> usize {
    let mut nesting = 0;
    let mut j = start;
    let bytes = input.as_bytes();

    while j < input.len() {
        match bytes[j] {
            b'<' => nesting += 1,
            b'>' => {
                nesting -= 1;
                if nesting == 0 {
                    return j + 1;
                }
            }
            _ => (),
        }
        j += 1;
    }

    j
}
