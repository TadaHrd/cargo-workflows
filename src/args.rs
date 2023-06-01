use alloc::format;
use std::string::String;

use clap::{Parser, Subcommand};

#[must_use]
#[derive(Parser, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcmd: SubCommandEnum,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[must_use]
#[derive(Subcommand, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubCommandEnum {
    /// Runs a specific workflow in the workflows.toml file
    #[clap(name = "run")]
    Run(RunCommand),

    /// Initializes a new workflow.toml file
    #[clap(name = "init")]
    Init(InitCommand),
}

#[must_use]
#[derive(clap::Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RunCommand {
    /// Workflow to run
    #[arg(long, short, default_value_t = { "default".into() })]
    pub workflow: String,

    /// Print debug information
    #[arg(long, short, default_value_t = false)]
    pub debug: bool,
}

#[derive(clap::Args, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InitCommand;
