use cairo_lang_defs::db::DefsGroup;
use cairo_lang_filesystem::db::FilesGroup;
use lsp_types::{Command, Range, Url};
use tracing::trace;

use super::LSCodeLens;
use crate::ide::code_lens::tests::TestCodeLensInternal;
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
    fn execute(&self, _file_url: Url, _state: &State, _notifier: &Notifier) -> Option<()> {
        trace!("Execute Calculate Gas");
        None
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
