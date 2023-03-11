use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlowView},
    fmt::presentln,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::models::ScribeError;

#[derive(Debug)]
pub struct BuildCmd;

impl BuildCmd {
    pub async fn run<O>(output: &mut O) -> Result<(), ScribeError>
    where
        O: OutputWrite<ScribeError>,
    {
        let workspace = Workspace::new(app_name!(), WorkspaceSpec::WorkingDir);

        Ok(())
    }
}
