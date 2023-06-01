#![no_std]

extern crate alloc;
extern crate std;

use std::{io::Write, process::exit};

use alloc::boxed::Box;

pub mod args;
pub mod run;
pub mod workflow_getter;

/// Creates a new workflows.toml file from a template
fn create_workflows_file() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create("workflows.toml")?;

    file.write_all(include_str!("../text/workflows.toml").as_bytes())?;

    Ok(())
}

/// Runs this program
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    match args.subcmd {
        args::SubCommandEnum::Run(workflow) => run::run(&workflow.workflow, workflow.debug)?,
        args::SubCommandEnum::Init(_) => create_workflows_file()?,
    }

    exit(0)
}
