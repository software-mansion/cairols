use cairo_lang_defs::ids::{ImportableId, TraitItemId};
use cairo_lang_semantic::resolve::ResolvedGenericItem;
use lsp_types::CompletionItemKind;

pub fn resolved_generic_item_completion_kind(item: ResolvedGenericItem) -> CompletionItemKind {
    match item {
        ResolvedGenericItem::GenericConstant(_) => CompletionItemKind::CONSTANT,
        ResolvedGenericItem::Module(_) => CompletionItemKind::MODULE,
        ResolvedGenericItem::GenericFunction(_) => CompletionItemKind::FUNCTION,
        ResolvedGenericItem::GenericType(_) | ResolvedGenericItem::GenericTypeAlias(_) => {
            CompletionItemKind::CLASS
        }
        ResolvedGenericItem::Impl(_) | ResolvedGenericItem::GenericImplAlias(_) => {
            CompletionItemKind::CLASS
        }
        ResolvedGenericItem::Variant(_) => CompletionItemKind::ENUM_MEMBER,
        ResolvedGenericItem::Trait(_) => CompletionItemKind::INTERFACE,
        ResolvedGenericItem::Variable(_) => CompletionItemKind::VARIABLE,
        ResolvedGenericItem::TraitItem(trait_item) => match trait_item {
            TraitItemId::Function(_) => CompletionItemKind::FUNCTION,
            TraitItemId::Type(_) => CompletionItemKind::CLASS,
            TraitItemId::Constant(_) => CompletionItemKind::CONSTANT,
            TraitItemId::Impl(_) => CompletionItemKind::CLASS,
        },
    }
}

pub fn importable_completion_kind(item: ImportableId) -> CompletionItemKind {
    match item {
        ImportableId::Constant(_) => CompletionItemKind::CONSTANT,
        ImportableId::Submodule(_) => CompletionItemKind::MODULE,
        ImportableId::ExternFunction(_) | ImportableId::FreeFunction(_) => {
            CompletionItemKind::FUNCTION
        }
        ImportableId::ExternType(_)
        | ImportableId::TypeAlias(_)
        | ImportableId::Impl(_)
        | ImportableId::ImplAlias(_)
        | ImportableId::Struct(_) => CompletionItemKind::CLASS,
        ImportableId::Variant(_) => CompletionItemKind::ENUM_MEMBER,
        ImportableId::Trait(_) => CompletionItemKind::INTERFACE,
        ImportableId::Enum(_) => CompletionItemKind::ENUM,
    }
}
