use std::ops::Not;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::FreeFunctionLongId;
use cairo_lang_defs::ids::ModuleFileId;
use cairo_lang_defs::ids::ModuleId;
use cairo_lang_defs::ids::ModuleItemId;
use cairo_lang_defs::ids::SubmoduleLongId;
use cairo_lang_defs::ids::TopLevelLanguageElementId;
use cairo_lang_defs::plugin::MacroPlugin;
use cairo_lang_filesystem::db::{FilesGroup, get_originating_location};
use cairo_lang_filesystem::ids::CrateId;
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
                TestFullQualifiedPath::Function(String::new()), // We can substitute with anything here.
                AvailableTestRunners::new(db, main_module.owning_crate(db))?,
                &config.run_test_command,
            )
            .is_some();

        let mut result = vec![];

        if is_runner_available {
            collect_tests(db, main_module, url, &mut result);
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
        let module_file_id = db.find_module_file_containing_node(module_item.as_syntax_node())?;

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

enum TestFullQualifiedPath {
    Function(String),
    Module(String),
}

impl TestFullQualifiedPath {
    fn cairo_test_command(&self) -> String {
        format!("scarb cairo-test --filter {}", self.as_ref())
    }

    fn snforge_command(&self) -> String {
        match self {
            TestFullQualifiedPath::Function(path) => {
                format!("snforge test {path} --exact")
            }
            TestFullQualifiedPath::Module(path) => {
                format!("snforge test {path}")
            }
        }
    }
}

impl AsRef<str> for TestFullQualifiedPath {
    fn as_ref(&self) -> &str {
        match self {
            TestFullQualifiedPath::Function(path) | TestFullQualifiedPath::Module(path) => path,
        }
    }
}

impl TestFullQualifiedPath {
    pub fn new(
        db: &AnalysisDatabase,
        module_item: ModuleItem,
        module_file_id: ModuleFileId,
    ) -> Option<Self> {
        match module_item {
            ModuleItem::FreeFunction(function_with_body) => {
                let path = ModuleItemId::FreeFunction(
                    FreeFunctionLongId(module_file_id, function_with_body.stable_ptr(db))
                        .intern(db),
                )
                .full_path(db);

                Some(TestFullQualifiedPath::Function(path))
            }
            ModuleItem::Module(item_module) => {
                let path = ModuleItemId::Submodule(
                    SubmoduleLongId(module_file_id, item_module.stable_ptr(db)).intern(db),
                )
                .full_path(db);

                Some(TestFullQualifiedPath::Module(path))
            }
            _ => None,
        }
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
                (true, false) => Some(test_path.cairo_test_command()),
                (false, true) => Some(test_path.snforge_command()),
                _ => None,
            },
            Self::CairoTest if available_runners.cairo_test => Some(test_path.cairo_test_command()),
            Self::Snforge if available_runners.snforge => Some(test_path.snforge_command()),
            Self::Custom => Some(custom_command.replace("{{TEST_PATH}}", test_path.as_ref())),
            _ => None,
        }
    }
}

fn collect_tests(
    db: &AnalysisDatabase,
    module: ModuleId,
    file_url: Url,
    file_state: &mut Vec<CodeLens>,
) {
    for test_fn in collect_functions(db, module) {
        maybe_push_code_lens(
            db,
            file_state,
            |position, index| make_code_lens(&file_url, index, position, false),
            test_fn,
        );
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return };

    for submodule in modules.iter().copied() {
        let is_inline = db.is_submodule_inline(submodule);

        let has_tests = if is_inline {
            let tests_count = file_state.len();

            collect_tests(db, ModuleId::Submodule(submodule), file_url.clone(), file_state);

            // Append mod only if it contains tests.
            tests_count != file_state.len()
        } else {
            has_any_test(db, ModuleId::Submodule(submodule))
        };

        if has_tests {
            let ptr = submodule.stable_ptr(db).untyped();

            maybe_push_code_lens(
                db,
                file_state,
                |position, index| make_code_lens(&file_url, index, position, true),
                ptr,
            );
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

fn get_position(db: &AnalysisDatabase, ptr: SyntaxStablePtrId) -> Option<Position> {
    let (file, span) =
        get_originating_location(db, ptr.file_id(db), ptr.lookup(db).span_without_trivia(db), None);

    let module_item = db
        .find_syntax_node_at_offset(file, span.start)?
        .ancestors_with_self(db)
        .find(|n| ModuleItem::cast(db, *n).is_some())?;

    module_item
        // In original code it is always `#[test]`.
        .find_attr(db, "test")
        .map(|test| test.as_syntax_node())
        // If attr is not found we are probably on mod.
        .unwrap_or(module_item)
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
                    .map(|test_attr| test_attr.stable_ptr(db).untyped())
                    // If for some weird reason we found both, push only first (prefer `#[test]`).
                    .next(),
            );
        }
    }

    result
}

fn maybe_push_code_lens(
    db: &AnalysisDatabase,
    file_state: &mut Vec<CodeLens>,
    make_code_lens: impl FnOnce(Position, usize) -> CodeLens,
    ptr: SyntaxStablePtrId,
) {
    if let Some(position) = get_position(db, ptr) {
        let lens = make_code_lens(position, file_state.len());

        file_state.push(lens);
    }
}

fn make_code_lens(file_url: &Url, index: usize, position: Position, is_plural: bool) -> CodeLens {
    let mut title = "▶ Run test".to_string();

    if is_plural {
        title.push('s');
    }

    let command = Command {
        title,
        command: "cairo.executeCodeLens".to_string(),
        arguments: Some(vec![
            Value::Number(Number::from(index)),
            Value::String(file_url.to_string()),
        ]),
    };
    let range = Range::new(position, position);

    CodeLens { range, command: Some(command), data: None }
}
