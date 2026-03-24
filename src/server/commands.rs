use anyhow::bail;

pub enum ServerCommand {
    Reload,
    ExecuteCodeLens,
}

const RELOAD: &str = "cairo.reload";
const EXECUTE_CODE_LENS: &str = "cairo.executeCodeLens";

impl TryFrom<String> for ServerCommand {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Self> {
        match value.as_str() {
            RELOAD => Ok(ServerCommand::Reload),
            EXECUTE_CODE_LENS => Ok(ServerCommand::ExecuteCodeLens),
            _ => bail!("Unrecognized command: {value}"),
        }
    }
}

impl ServerCommand {
    pub fn as_str(&self) -> &str {
        match self {
            ServerCommand::Reload => RELOAD,
            ServerCommand::ExecuteCodeLens => EXECUTE_CODE_LENS,
        }
    }
}
