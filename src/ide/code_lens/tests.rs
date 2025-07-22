use std::ops::Not;

use super::{
    AnnotatedNode, FileCodeLens, LSCodeLens, LSCodeLensInterface, collect_functions_with_attrs,
    get_original_node_and_file, make_lens_args,
};
use crate::config::{Config, TestRunner};
use crate::lang::db::AnalysisDatabase;
use crate::lang::db::LsSemanticGroup;
use crate::lang::db::LsSyntaxGroup;
use crate::lang::lsp::ToCairo;
use crate::lang::lsp::{LsProtoGroup, ToLsp};
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
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::ids::SyntaxStablePtrId;
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode, ast::ModuleItem};
use cairo_lang_test_plugin::TestPlugin;
use cairo_lang_utils::Intern;
use lsp_types::Range;
use lsp_types::notification::ShowMessage;
use lsp_types::{CodeLens, Url};
use lsp_types::{Command, ShowMessageParams};
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use lsp_types::{MessageType, Position};
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


#[derive(PartialEq, Clone, Debug)]
pub struct TestCodeLens {
    pub lens: CodeLens,
}

impl LSCodeLensInterface for TestCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        let db = &state.db;

        let position = self.lens.range.start.to_cairo();

        let file = db.file_for_url(&file_url)?;
        let file_path = file_url.to_file_path().ok()?;

        let node = db.find_syntax_node_at_position(file, position)?;

        let module_item = node.ancestor_of_type::<ModuleItem>(db)?;
        let module_file_id = db.find_module_file_containing_node(module_item.as_syntax_node())?;

        let command = state.config.test_runner.command(
            TestFullQualifiedPath::new(db, module_item, module_file_id)?,
            AvailableTestRunners::new(db, module_file_id.0.owning_crate(db))?,
            &state.config.run_test_command,
        )?;

        let cwd = state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?;
        if state.client_capabilities.execute_in_terminal_support() {
            notifier.notify::<ExecuteInTerminal>(ExecuteInTerminalParams { cwd, command });
        } else {
            notifier.notify::<ShowMessage>(ShowMessageParams {
                typ: MessageType::INFO,
                message: format!(
                    "To execute the code lens, run command: `{command}` in directory {}",
                    cwd.display()
                ),
            });
        }

        Some(())
    }

    fn get_lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub fn push_test_code_lenses(
    file_code_lens: &mut FileCodeLens,
    url: Url,
    db: &AnalysisDatabase,
    config: &Config,
) -> Option<()> {
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

    if is_runner_available {
        collect_test_lenses(db, main_module, url, file_code_lens);
    }

    Some(())
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
    pub fn new<'db>(db: &'db AnalysisDatabase, crate_id: CrateId<'db>) -> Option<Self> {
        let cairo_test = db.crate_macro_plugins(crate_id).iter().any(|plugin_id| {
            plugin_id.long(db).plugin_type_id() == TestPlugin::default().plugin_type_id()
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

const TEST_EXECUTABLES: [&str; 2] = ["test", "snforge_internal_test_executable"];
fn collect_test_functions(db: &AnalysisDatabase, module: ModuleId) -> Vec<AnnotatedNode> {
    collect_functions_with_attrs(db, module, &TEST_EXECUTABLES)
}

fn collect_tests<'db>(
    db: &'db AnalysisDatabase,
    module: ModuleId<'db>,
    file_url: Url,
    file_state: &mut FileCodeLens,
) {
    for node in collect_test_functions(db, module) {
        let index = file_state.len();
        maybe_push_code_lens(
            db,
            file_state,
            |position| make_test_code_lens(&file_url, position, false, index),
            node,
        );
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return };

    for submodule in modules.iter().copied() {
        let is_inline = db.is_submodule_inline(submodule);

        let has_tests = if is_inline {
            let tests_count = file_state.len();

            collect_test_lenses(db, ModuleId::Submodule(submodule), file_url.clone(), file_state);

            // Append mod only if it contains tests.
            tests_count != file_state.len()
        } else {
            has_any_test(db, ModuleId::Submodule(submodule))
        };

        if has_tests {
            let ptr = submodule.stable_ptr(db).untyped();

            let full_path = submodule.full_path(db);
            let module_node = AnnotatedNode { full_path: full_path.clone(), attribute_ptr: ptr };
            let index = file_state.len();
            maybe_push_code_lens(
                db,
                file_state,
                |position| make_test_code_lens(&file_url, position, true, index),
                module_node,
            );
        }
    }
}

fn has_any_test<'db>(db: &'db AnalysisDatabase, module: ModuleId<'db>) -> bool {
    if collect_test_functions(db, module).is_empty().not() {
        return true;
    }

    let Ok(modules) = db.module_submodules_ids(module) else { return false };

    modules.iter().copied().map(ModuleId::Submodule).any(|submodule| {
        collect_test_functions(db, submodule).is_empty().not() || has_any_test(db, submodule)
    })
}

fn get_test_lens_position<'db>(db: &'db AnalysisDatabase, ptr: SyntaxStablePtrId<'db>) -> Option<Position> {
    let (original_node, original_file) = get_original_node_and_file(db, ptr)?;
    original_node
        .find_attr(db, "test")
        .map(|attr| attr.as_syntax_node())
        // If attr is not found we are probably on mod.
        .unwrap_or(original_node)
        .span_start_without_trivia(db)
        .position_in_file(db, original_file)
        .map(|position| position.to_lsp())
}

fn maybe_push_code_lens(
    db: &AnalysisDatabase,
    file_state: &mut FileCodeLens,
    make_code_lens: impl FnOnce(Position) -> CodeLens,
    annotated_function: AnnotatedNode,
) {
    let AnnotatedNode { attribute_ptr, full_path: _ } = annotated_function;
    if let Some(position) = get_test_lens_position(db, attribute_ptr) {
        let lens = make_code_lens(position);

        file_state.push(LSCodeLens::Test(TestCodeLens { lens }))
    }
}

fn make_test_code_lens(
    file_url: &Url,
    position: Position,
    is_plural: bool,
    index: usize,
) -> CodeLens {
    let mut title = "▶ Run test".to_string();

    if is_plural {
        title.push('s');
    }

    let command = Command {
        title,
        command: "cairo.executeCodeLens".to_string(),
        arguments: Some(make_lens_args(file_url.clone(), index)),
    };
    let range = Range::new(position, position);

    CodeLens { range, command: Some(command), data: None }
}
