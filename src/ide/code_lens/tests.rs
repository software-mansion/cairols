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
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::ids::SubmoduleLongId;
use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::db::FilesGroup;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_filesystem::ids::FileId;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{
    TypedStablePtr, TypedSyntaxNode, ast::ModuleItem, helpers::QueryAttrs,
};
use cairo_lang_test_plugin::TestPlugin;
use cairo_lang_utils::Intern;
use cairo_lang_utils::LookupIntern;
use lsp_types::Command;
use lsp_types::Position;
use lsp_types::Range;
use lsp_types::{CodeLens, Url};
use serde_json::Number;
use serde_json::Value;
use std::fmt::Display;
use std::ops::Not;

pub struct TestCodeLensProvider;

impl CodeLensProvider for TestCodeLensProvider {
    fn calculate_code_lens(
        &self,
        url: Url,
        db: &AnalysisDatabase,
        config: &Config,
    ) -> Option<Vec<CodeLens>> {
        let file = db.file_for_url(&url)?;

        let main_module = *db.file_modules(file).ok()?.first()?;

        let is_runner_available = config
            .test_runner
            .command(
                TestFullQualifiedPath::default(), // We can substitute with anything here.
                AvailableTestRunners::new(db, main_module.owning_crate(db))?,
                &config.run_test_command,
            )
            .is_some();

        let mut result = vec![];

        if is_runner_available {
            collect_tests(db, main_module, file, url, &mut result);
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
            cwd: state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?,
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
    module: ModuleId,
    origin_file: FileId,
    file_url: Url,
    file_state: &mut Vec<CodeLens>,
) {
    for test_fn in collect_functions(db, module) {
        maybe_push_code_lens(db, origin_file, &file_url, file_state, false, test_fn);
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return };

    for submodule in modules.iter().copied() {
        let has_tests = if db.is_submodule_inline(submodule) {
            let tests_count = file_state.len();

            collect_tests(
                db,
                ModuleId::Submodule(submodule),
                origin_file,
                file_url.clone(),
                file_state,
            );

            // Append mod only if it contains tests.
            tests_count != file_state.len()
        } else {
            has_any_test(db, ModuleId::Submodule(submodule))
        };

        if has_tests {
            let ptr = submodule.stable_ptr(db).untyped();

            maybe_push_code_lens(db, origin_file, &file_url, file_state, true, ptr);
        }
    }
}

fn has_any_test(db: &AnalysisDatabase, module: ModuleId) -> bool {
    if collect_functions(db, module).is_empty().not() {
        return true;
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return false };

    modules.iter().copied().map(ModuleId::Submodule).any(|submodule| {
        collect_functions(db, submodule).is_empty().not() || has_any_test(db, submodule)
    })
}

fn get_position(db: &AnalysisDatabase, file: FileId, ptr: SyntaxStablePtrId) -> Option<Position> {
    ptr.lookup(db)
        .span_start_without_trivia(db)
        .position_in_file(db, file)
        .map(|position| position.to_lsp())
}

fn collect_functions(db: &AnalysisDatabase, module: ModuleId) -> Vec<SyntaxStablePtrId> {
    let mut result = vec![];

    if let Ok(functions) = db.module_free_functions(module) {
        for function in functions.values() {
            result.extend(
                ["test", "snforge_internal_test_executable"]
                    .iter()
                    .filter_map(|test_attr| function.find_attr(db, test_attr))
                    .map(|test_attr| test_attr.stable_ptr().untyped())
                    // If for some weird reason we found both, push only first (prefer `#[test]`).
                    .next(),
            );
        }
    }

    result
}

fn maybe_push_code_lens(
    db: &AnalysisDatabase,
    file: FileId,
    file_url: &Url,
    file_state: &mut Vec<CodeLens>,
    is_plural: bool,
    ptr: SyntaxStablePtrId,
) {
    if ptr.file_id(db) == file {
        if let Some(position) = get_position(db, file, ptr) {
            let mut title = "▶ Run test".to_string();

            if is_plural {
                title.push('s');
            }

            let command = Command {
                title,
                command: "cairo.executeCodeLens".to_string(),
                arguments: Some(vec![
                    Value::Number(Number::from(file_state.len())),
                    Value::String(file_url.to_string()),
                ]),
            };
            let range = Range::new(position, position);

            file_state.push(CodeLens { range, command: Some(command), data: None });
        };
    }
}
