use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use lsp_types::notification::ShowMessage;
use lsp_types::{Command, MessageType, Range, ShowMessageParams, Url};
use regex::Regex;

use super::LSCodeLens;
use crate::ide::code_lens::tests::{TestCodeLensInternal, TestFullQualifiedPath};
use crate::ide::code_lens::{CodeLens, CodeLensInterface, CodeLensInternal, make_lens_args};
use crate::lang::db::AnalysisDatabase;
use crate::lang::lsp::LsProtoGroup;
use crate::server::client::Notifier;
use crate::server::commands::ServerCommand;
use crate::state::State;

#[derive(PartialEq, Clone, Debug)]
pub enum GasTarget {
    Test { full_path: String, is_fuzzer: bool },
}

impl From<&TestCodeLensInternal> for GasTarget {
    fn from(value: &TestCodeLensInternal) -> Self {
        GasTarget::Test { full_path: value.full_path.clone(), is_fuzzer: value.is_fuzzer }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct GasCodeLens {
    lens: CodeLens,
    target: GasTarget,
}

impl CodeLensInterface for GasCodeLens {
    fn execute(&self, file_url: Url, state: &State, notifier: &Notifier) -> Option<()> {
        match &self.target {
            GasTarget::Test { full_path, is_on_mod } => {
                test_target_execute(full_path.clone(), *is_on_mod, file_url, state, notifier)
            } // TODO(hakiers) GasTarget::Executable
        }
    }
    fn lens(&self) -> CodeLens {
        self.lens.clone()
    }
}

pub struct GasCodeLensInternal {
    pub range: Range,
    pub file_url: Url,
    pub target: GasTarget,
}

impl CodeLensInternal for GasCodeLensInternal {
    fn into_ls_lens(self, index: usize) -> LSCodeLens {
        let command = Command {
            title: "⛽ Calculate Gas".to_string(),
            command: ServerCommand::ExecuteCodeLens.as_str().to_string(),
            arguments: Some(make_lens_args(self.file_url.clone(), index)),
        };

        LSCodeLens::Gas(GasCodeLens {
            lens: CodeLens { range: self.range, command: Some(command), data: None },
            target: self.target,
        })
    }
}

impl TryFrom<&TestCodeLensInternal> for GasCodeLensInternal {
    type Error = ();

    fn try_from(value: &TestCodeLensInternal) -> Result<Self, Self::Error> {
        Ok(Self {
            range: value.range,
            file_url: value.file_url.clone(),
            target: GasTarget::from(value),
        })
    }
}

pub fn get_gas_code_lenses(
    db: &AnalysisDatabase,
    url: Url,
    test_code_lens: &[TestCodeLensInternal],
) -> Option<Vec<GasCodeLensInternal>> {
    let file = db.file_for_url(&url)?;

    let main_module = *db.file_modules(file).ok()?.first()?;
    let crate_id = main_module.owning_crate(db);

    let is_snforge_available =
        db.crate_config(crate_id)?.settings.dependencies.contains_key("snforge_std");

    let gas_code_lens = if is_snforge_available {
        test_code_lens
            .iter()
            .filter_map(|test_lens| GasCodeLensInternal::try_from(test_lens).ok())
            .collect()
    } else {
        vec![]
    };

    Some(gas_code_lens)
}

fn test_target_execute(
    full_path: String,
    is_on_mod: bool,
    file_url: Url,
    state: &State,
    notifier: &Notifier,
) -> Option<()> {
    let notifier = notifier.clone();

    let path = if is_on_mod {
        TestFullQualifiedPath::Module(full_path.clone())
    } else {
        TestFullQualifiedPath::Function(full_path.clone())
    };
    let command = path.snforge_command();

    let file_path = file_url.to_file_path().ok()?;
    let cwd = state.project_controller.configs_registry().manifest_dir_for_file(&file_path)?;

    // Running `snforge` can take several seconds, so we let it run in the
    // background and report the result later via `ShowMessage` instead of blocking.
    std::thread::spawn(move || {
        let mut parts = command.split_whitespace();
        let program = parts.next().expect("command should not be empty");
        let output = std::process::Command::new(program).args(parts).current_dir(&cwd).output();

        let Ok(output) = output else {
            let message = format!("Failed to run snforge: {}", output.unwrap_err());
            notifier.notify::<ShowMessage>(ShowMessageParams { typ: MessageType::ERROR, message });
            return;
        };

        // All-or-nothing: if any test in the batch failed, show error
        if !output.status.success() {
            let message = "One or more tests failed — gas usage not reported.".to_string();
            notifier.notify::<ShowMessage>(ShowMessageParams { typ: MessageType::ERROR, message });
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);

        let gas = sum_l2_gas(&stdout);
        let message = format!("{full_path} Gas: {gas}");
        notifier.notify::<ShowMessage>(ShowMessageParams { typ: MessageType::INFO, message });
    });

    Some(())
}

fn sum_l2_gas(stdout: &str) -> u64 {
    let re = Regex::new(r"l2_gas:\s*(?:~(\d+)|\{[^}]*?max:\s*~(\d+))").unwrap();

    re.captures_iter(stdout)
        .filter_map(|caps| caps.get(1).or_else(|| caps.get(2)))
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::sum_l2_gas;

    #[test]
    fn sums_plain_gas_values() {
        let stdout = indoc::indoc! {r#"
            [PASS] playground::tests::test_one (l1_gas: ~0, l1_data_gas: ~0, l2_gas: ~13620)
        "#};

        assert_eq!(sum_l2_gas(stdout), 13620);
    }

    #[test]
    fn sums_fuzz_gas_values() {
        let stdout = indoc::indoc! {r#"
            [PASS] playground::tests::test_one (runs: 256, (l1_gas: {max: ~0, min: ~0, mean: ~0, std deviation: ~0}, l1_data_gas: {max: ~0, min: ~0, mean: ~0, std deviation: ~0}, l2_gas: {max: ~54300, min: ~42660, mean: ~51686, std deviation: ~3360}))
        "#};

        assert_eq!(sum_l2_gas(stdout), 54300);
    }

    #[test]
    fn sums_plain_and_fuzz_gas_values() {
        let stdout = indoc::indoc! {r#"
            [PASS] playground::tests::test_one (l1_gas: ~0, l1_data_gas: ~0, l2_gas: ~13620)
            [PASS] playground::tests::test_two (runs: 256, (l1_gas: {max: ~0, min: ~0, mean: ~0, std deviation: ~0}, l1_data_gas: {max: ~0, min: ~0, mean: ~0, std deviation: ~0}, l2_gas: {max: ~54300, min: ~42660, mean: ~51686, std deviation: ~3360}))
        "#};

        assert_eq!(sum_l2_gas(stdout), 13620 + 54300);
    }

    #[test]
    fn return_zero_when_no_gas_info() {
        assert_eq!(sum_l2_gas("no gas info here"), 0);
    }
}
