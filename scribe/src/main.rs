use clap::Parser;
use peace::miette::{Report, Result};
use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{output::CliOutput, WorkspaceSpec},
};
use scribe::models::CliArgs;
use scribe::models::ScribeError;

fn run() -> Result<(), ScribeError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        // .enable_io()
        // .enable_time()
        .build()
        .map_err(ScribeError::TokioRuntimeInit)?;

    let CliArgs {
        command,
        format,
        color,
    } = CliArgs::parse();
    #[allow(unused_assignments)]
    runtime.block_on(async {
        let _workspace_spec = WorkspaceSpec::WorkingDir;
        let _profile = profile!("default");
        let _flow_id = flow_id!("file");
        let mut cli_output = {
            let mut builder = CliOutput::builder().with_colorize(color);

            if let Some(format) = format {
                builder = builder.with_outcome_format(format);
            }
            builder.build()
        };

        match command {
            scribe::models::cli_args::ScribeCommand::Init { name, location } => todo!(),
            scribe::models::cli_args::ScribeCommand::Build => todo!(),
            scribe::models::cli_args::ScribeCommand::Clean => todo!(),
        }

        Ok::<_, ScribeError>(())
    })
}

fn main() -> Result<(), Report> {
    run().map_err(|Scribe_error| match Scribe_error {
        // ScribeError::PyProjectParseError(err) => Report::from(err.into()),
        ScribeError::PeaceRtError(err) => Report::from(err),
        other => Report::from(other),
    })
}
