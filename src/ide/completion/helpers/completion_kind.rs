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
    }
}
