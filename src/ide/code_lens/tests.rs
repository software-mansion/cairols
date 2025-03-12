use super::CodeLensProvider;
use crate::config::Config;
use crate::config::TestRunner;
use crate::lang::db::AnalysisDatabase;
use crate::lang::db::LsSemanticGroup;
use crate::lang::db::LsSyntaxGroup;
use crate::lang::lsp::LsProtoGroup;
use crate::lang::lsp::ToCairo;
use crate::lang::lsp::ToLsp;
use crate::lsp::ext::ExecuteInTerminal;
use crate::lsp::ext::ExecuteInTerminalParams;
use crate::server::client::Notifier;
use crate::state::State;
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::FreeFunctionLongId;
use cairo_lang_defs::ids::ModuleFileId;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::ids::SubmoduleLongId;
use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_syntax::node::{
    TypedStablePtr, TypedSyntaxNode,
    ast::{MaybeModuleBody, ModuleItem},
    helpers::QueryAttrs,
};
use cairo_lang_test_plugin::TestPlugin;
use cairo_lang_utils::Intern;
use cairo_lang_utils::LookupIntern;
use lsp_types::Command;
use lsp_types::Range;
use lsp_types::TextDocumentIdentifier;
use lsp_types::TextDocumentPositionParams;
use lsp_types::{CodeLens, Url};
use serde_json::Number;
use serde_json::Value;
use std::fmt::Display;

pub struct TestCodeLensProvider;

impl CodeLensProvider for TestCodeLensProvider {
    fn calculate_code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
    ) -> Option<Vec<CodeLens>> {
        let file = db.file_for_url(&url)?;

        let is_runner_available = config
            .test_runner
            .command(
                TestFullQualifiedPath::default(), // We can substitute with anything here.
                AvailableTestRunners::new(
                    db,
                    db.file_modules(file).ok()?.first()?.owning_crate(db),
                )?,
                &config.run_test_command,
            )
            .is_some();

        let mut result = vec![];

        if is_runner_available {
            collect_tests(
                db,
                db.file_module_syntax(file).ok()?.items(db).elements(db),
                file,
                url,
                &mut result,
            );
        }

        Some(result)
    }

    fn execute_code_lens(
        &self,
        state: &State,
        notifier: Notifier,
        url: Url,
        code_lens: CodeLens,
    ) -> Option<()> {
        let db = &state.db;

        let position = code_lens.range.start.to_cairo();

        let file = db.file_for_url(&url)?;
        let file_path = url.to_file_path().ok()?;

        let node = db.find_syntax_node_at_position(file, position)?;

        let module_item = node.ancestor_of_type::<ModuleItem>(db)?;
        let module_file_id = db.find_module_file_containing_node(&module_item.as_syntax_node())?;

        let command = state.config.test_runner.command(
            TestFullQualifiedPath::new(db, module_item, module_file_id)?,
            AvailableTestRunners::new(db, module_file_id.0.owning_crate(db))?,
            &state.config.run_test_command,
        )?;

        notifier.notify::<ExecuteInTerminal>(ExecuteInTerminalParams {
            cwd: state
                .project_controller
                .manifests_registry()
                .manifests_dirs()
                .find(|dir| file_path.starts_with(dir))?, // TODO(#484)
            command,
        });

        Some(())
    }
}

#[derive(Default)]
struct TestFullQualifiedPath(String);

impl Display for TestFullQualifiedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for TestFullQualifiedPath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TestFullQualifiedPath {
    pub fn new(
        db: &AnalysisDatabase,
        module_item: ModuleItem,
        module_file_id: ModuleFileId,
    ) -> Option<Self> {
        let full_path = match module_item {
            ModuleItem::FreeFunction(function_with_body) => ModuleItemId::FreeFunction(
                FreeFunctionLongId(module_file_id, function_with_body.stable_ptr()).intern(db),
            )
            .full_path(db),
            ModuleItem::Module(item_module) => ModuleItemId::Submodule(
                SubmoduleLongId(module_file_id, item_module.stable_ptr()).intern(db),
            )
            .full_path(db),
            _ => return None,
        };

        Some(Self(full_path))
    }
}

struct AvailableTestRunners {
    cairo_test: bool,
    snforge: bool,
}

impl AvailableTestRunners {
    pub fn new(db: &AnalysisDatabase, crate_id: CrateId) -> Option<Self> {
        let cairo_test = db.crate_macro_plugins(crate_id).iter().any(|plugin_id| {
            plugin_id.lookup_intern(db).plugin_type_id() == TestPlugin::default().plugin_type_id()
        });

        // This will not work with crate renames in `Scarb.toml`, but there is no better way to do this now.
        let snforge = db.crate_config(crate_id)?.settings.dependencies.contains_key("snforge_std");

        Some(Self { cairo_test, snforge })
    }
}

impl TestRunner {
    fn command(
        &self,
        test_path: TestFullQualifiedPath,
        available_runners: AvailableTestRunners,
        custom_command: &str,
    ) -> Option<String> {
        match self {
            Self::Auto => match (available_runners.cairo_test, available_runners.snforge) {
                (true, false) => Some(format!("scarb cairo-test --filter {test_path}")),
                (false, true) => Some(format!("snforge test {test_path} --exact")),
                _ => None,
            },
            Self::CairoTest if available_runners.cairo_test => {
                Some(format!("scarb cairo-test --filter {test_path}"))
            }
            Self::Snforge if available_runners.snforge => {
                Some(format!("snforge test {test_path} --exact"))
            }
            Self::Custom => Some(custom_command.replace("{{TEST_PATH}}", test_path.as_ref())),
            _ => None,
        }
    }
}

fn collect_tests(
    db: &AnalysisDatabase,
    module_items: Vec<ModuleItem>,
    file: FileId,
    file_url: Url,
    file_state: &mut Vec<CodeLens>,
) {
    for item in module_items {
        match item {
            ModuleItem::FreeFunction(func) if func.find_attr(db, "test").is_some() => {
                if let Some(position) = func
                    .stable_ptr()
                    .untyped()
                    .lookup(db)
                    .span_start_without_trivia(db)
                    .position_in_file(db, file)
                {
                    let position = position.to_lsp();

                    make_code_lens(
                        TextDocumentPositionParams {
                            position,
                            text_document: TextDocumentIdentifier { uri: file_url.clone() },
                        },
                        false,
                        file_state,
                    );
                };
            }
            ModuleItem::Module(module) => {
                if let MaybeModuleBody::Some(body) = module.body(db) {
                    let tests_count = file_state.len();

                    collect_tests(
                        db,
                        body.items(db).elements(db),
                        file,
                        file_url.clone(),
                        file_state,
                    );

                    // Append mod only if it contains tests.
                    if tests_count != file_state.len() {
                        if let Some(position) = module
                            .as_syntax_node()
                            .span_start_without_trivia(db)
                            .position_in_file(db, file)
                        {
                            let position = position.to_lsp();

                            make_code_lens(
                                TextDocumentPositionParams {
                                    position,
                                    text_document: TextDocumentIdentifier { uri: file_url.clone() },
                                },
                                true,
                                file_state,
                            );
                        };
                    }
                }
            }
            _ => {}
        }
    }
}

fn make_code_lens(
    position: TextDocumentPositionParams,
    is_plural: bool,
    file_state: &mut Vec<CodeLens>,
) {
    let mut title = "▶ Run test".to_string();

    if is_plural {
        title.push('s');
    }

    let command = Command {
        title,
        command: "cairo.executeCodeLens".to_string(),
        arguments: Some(vec![
            Value::Number(Number::from(file_state.len())),
            Value::String(position.text_document.uri.to_string()),
        ]),
    };
    let range = Range::new(position.position, position.position);

    file_state.push(CodeLens { range, command: Some(command), data: None });
}
