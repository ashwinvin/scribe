use std::path::PathBuf;

use clap::{Parser, Subcommand};
use peace::{rt_model::output::OutputFormat};

use peace::rt_model::output::CliColorizeOpt;

#[derive(Parser)]
#[clap(
    author,
    version,
    about = "Manages an application's lifecycle.",
    long_about = "Manages deployment, configuration, upgrading, and cleaning up of an application."
)]
pub struct CliArgs {
    /// Command to run.
    #[command(subcommand)]
    pub command: ScribeCommand,
    /// The format of the command output.
    ///
    /// At this level, this needs to be specified before the subcommand.
    /// <https://github.com/clap-rs/clap/issues/3002> needs to be implemented
    /// for the argument to be passed in after the subcommand.
    #[arg(long)]
    pub format: Option<OutputFormat>,
    /// Whether output should be colorized.
    ///
    /// * "auto" (default): Colorize when used interactively.
    /// * "always": Always colorize output.
    /// * "never": Never colorize output.
    #[arg(long, default_value = "auto")]
    pub color: CliColorizeOpt,
}

#[derive(Subcommand)]
pub enum ScribeCommand {
    /// Initializes a profile to deploy a web application.
    Init {
        // Name of the project.
        name: String,
        // Path to the project
        location: PathBuf,
    },
    // Build the project
    Build, 
    /// Cleans the current environment.
    Clean,
}