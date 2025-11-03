use cairo_lang_defs::ids::LookupItemId::ModuleItem;
use cairo_lang_defs::ids::{ModuleId, ModuleItemId};
use cairo_lang_doc::db::DocGroup;
use cairo_lang_doc::documentable_item::DocumentableItemId;
use cairo_lang_semantic::corelib::CorelibSemantic;
use cairo_lang_semantic::helper::ModuleHelper;
use cairo_lang_syntax::node::kind::SyntaxKind;

use crate::ide::markdown::{RULE, fenced_code_block};
use crate::lang::db::AnalysisDatabase;

pub fn keyword(db: &AnalysisDatabase, kind: SyntaxKind) -> Option<String> {
    let keyword = CairoKeyword::try_from_kind(&kind)?;
    let keyword_docs = keyword.get_docs(db)?;

    let mut md = String::new();

    md += &fenced_code_block(keyword.token.as_str());
    md += RULE;
    md += "\n";

    md += &keyword_docs;

    Some(md)
}

struct CairoKeyword {
    mod_name: String,
    token: String,
}

impl CairoKeyword {
    fn try_from_kind(kind: &SyntaxKind) -> Option<Self> {
        if !kind.is_keyword_token() {
            return None;
        }

        let keyword = keyword_token_from_kind(kind);
        let mod_name = format!("keyword_{}", keyword);

        Some(Self { mod_name, token: keyword.to_string() })
    }

    fn get_docs(&self, db: &AnalysisDatabase) -> Option<String> {
        let core_info_struct = db.core_info();
        let keywords_module_id = core_info_struct.keyword_docs_submodule;

        let keyword_submodule =
            ModuleHelper { db, id: keywords_module_id }.submodule(&self.mod_name).id;

        let ModuleId::Submodule(keyword_submodule_id) = keyword_submodule else {
            panic!("Could not find keyword submodule: {}", self.mod_name);
        };

        db.get_item_documentation(DocumentableItemId::LookupItem(ModuleItem(
            ModuleItemId::Submodule(keyword_submodule_id),
        )))
    }
}

fn keyword_token_from_kind(kind: &SyntaxKind) -> &str {
    match kind {
        SyntaxKind::TokenAs => "as",
        SyntaxKind::TokenConst => "const",
        SyntaxKind::TokenElse => "else",
        SyntaxKind::TokenEnum => "enum",
        SyntaxKind::TokenExtern => "extern",
        SyntaxKind::TokenFalse => "false",
        SyntaxKind::TokenFunction => "fn",
        SyntaxKind::TokenIf => "if",
        SyntaxKind::TokenWhile => "while",
        SyntaxKind::TokenFor => "for",
        SyntaxKind::TokenLoop => "loop",
        SyntaxKind::TokenImpl => "impl",
        SyntaxKind::TokenImplicits => "implicits",
        SyntaxKind::TokenLet => "let",
        SyntaxKind::TokenMacro => "macro",
        SyntaxKind::TokenMatch => "match",
        SyntaxKind::TokenModule => "mod",
        SyntaxKind::TokenMut => "mut",
        SyntaxKind::TokenNoPanic => "nopanic",
        SyntaxKind::TokenOf => "of",
        SyntaxKind::TokenRef => "ref",
        SyntaxKind::TokenContinue => "continue",
        SyntaxKind::TokenReturn => "return",
        SyntaxKind::TokenBreak => "break",
        SyntaxKind::TokenStruct => "struct",
        SyntaxKind::TokenTrait => "trait",
        SyntaxKind::TokenTrue => "true",
        SyntaxKind::TokenType => "type",
        SyntaxKind::TokenUse => "use",
        SyntaxKind::TokenPub => "pub",
        _ => panic!("Unknown keyword kind: {:?}", kind),
    }
}
