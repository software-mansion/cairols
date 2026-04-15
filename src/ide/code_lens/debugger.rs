use std::path::PathBuf;
use std::process::Stdio;

use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use lsp_types::notification::ShowMessage;
use lsp_types::{CodeLens, Command, MessageType, Range, ShowMessageParams, Url};

use crate::ide::code_lens::tests::{
    TestCodeLensInternal, get_full_path_and_module_id, sanitize_test_case_name,
};
use crate::ide::code_lens::{CodeLensInterface, CodeLensInternal, LSCodeLens, make_lens_args};
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::lsp::capabilities::client::ClientCapabilitiesExt;
use crate::lsp::ext::{LaunchDebugger, LaunchDebuggerParams};
use crate::server::client::Notifier;
use crate::server::commands::ServerCommand;
use crate::state::State;

#[derive(PartialEq, Clone, Debug)]
pub struct DebuggerCodeLens {
    lens: CodeLens,
    full_path: String,
}

impl CodeLensInterface for DebuggerCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        let (full_qualified_path, _) =
            get_full_path_and_module_id(&file_url, state, &self.lens, &self.full_path)?;
        let full_path = sanitize_test_case_name(full_qualified_path.as_ref());

        let file_path = file_url.to_file_path().ok()?;
        let cwd = state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?;

        let command = format!("snforge test {full_path} --exact --launch-debugger");
        send_launch_debugger(state, notifier, command, cwd, full_path);
        Some(())
    }

    fn lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub struct DebuggerLensInternal {
    range: Range,
    file_url: Url,
    full_path: String,
}

impl CodeLensInternal for DebuggerLensInternal {
    fn into_ls_lens(self, index: usize) -> LSCodeLens {
        LSCodeLens::Debugger(DebuggerCodeLens {
            lens: CodeLens {
                range: self.range,
                command: Some(Command {
                    title: String::from("▶ Debug test"),
                    command: ServerCommand::ExecuteCodeLens.as_str().to_string(),
                    arguments: Some(make_lens_args(self.file_url.clone(), index)),
                }),
                data: None,
            },
            full_path: self.full_path,
        })
    }
}

impl TryFrom<&TestCodeLensInternal> for DebuggerLensInternal {
    type Error = ();

    fn try_from(value: &TestCodeLensInternal) -> Result<Self, Self::Error> {
        if value.is_on_mod || value.is_fuzzer {
            Err(())
        } else {
            Ok(Self {
                range: value.range,
                file_url: value.file_url.clone(),
                full_path: value.full_path.clone(),
            })
        }
    }
}

pub fn get_debugger_code_lenses(
    db: &AnalysisDatabase,
    url: Url,
    test_code_lenses: &[TestCodeLensInternal],
) -> Option<Vec<DebuggerLensInternal>> {
    let file = db.file_for_url(&url)?;

    let main_module = *db.file_modules(file).ok()?.first()?;
    let crate_id = main_module.owning_crate(db);

    // TODO(software-mansion/scarb#2347): This will not work with crate renames in `Scarb.toml`, but there is no better way to do this now.
    let is_snforge_available =
        db.crate_config(crate_id)?.settings.dependencies.contains_key("snforge_std");

    let snforge_help_stdout_bytes = std::process::Command::new("snforge")
        .stdout(Stdio::piped())
        .arg("test")
        .arg("--help")
        .output()
        .map(|output| output.stdout)
        .unwrap_or_default();
    let snforge_supports_debugger = std::str::from_utf8(&snforge_help_stdout_bytes)
        .map(|s| s.contains("--launch-debugger"))
        .unwrap_or_default();

    let debugger_test_code_lens = if is_snforge_available && snforge_supports_debugger {
        test_code_lenses
            .iter()
            .filter_map(|test_lens| DebuggerLensInternal::try_from(test_lens).ok())
            .collect()
    } else {
        vec![]
    };

    Some(debugger_test_code_lens)
}

fn send_launch_debugger(
    state: &State,
    notifier: &Notifier,
    command: String,
    cwd: PathBuf,
    test_name: String,
) {
    if state.client_capabilities.launch_debugger_support() {
        notifier.notify::<LaunchDebugger>(LaunchDebuggerParams { cwd, command, test_name });
    } else {
        notifier.notify::<ShowMessage>(ShowMessageParams {
            typ: MessageType::INFO,
            message: format!(
                "To launch the debug adapter process, run command: `{command}` in directory {}",
                cwd.display()
            ),
        });
    }
}
