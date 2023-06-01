use core::fmt::Debug;
use std::{
    eprintln,
    process::{Command, Stdio},
};

use alloc::{boxed::Box, format, string::String};

use crate::workflow_getter;

#[derive(Debug)]
pub enum Error {
    FailedToExecuteCommand(std::io::Error),
    CommandListTooShort { name: String },
    Other(Box<dyn std::error::Error>),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FailedToExecuteCommand(e) => write!(f, "Failed to execute command: {}", e),
            Self::CommandListTooShort { name } => {
                write!(f, "Command list is empty in {} (workspaces.toml)", name)
            }
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

/// Executes a workflow
pub fn run(workflow_name: &str, debug: bool) -> Result<(), Error> {
    let workflow = match workflow_getter::get_workflow(workflow_name) {
        Ok(v) => v,
        Err(e) => return Err(Error::Other(Box::new(e))),
    };

    let (env, commands) = (workflow.env, workflow.commands);

    let env = env.unwrap_or(Default::default());

    for (k, v) in commands {
        if debug {
            eprintln!("[running \"{}\"]", k)
        }
        if v.is_empty() {
            return Err(Error::CommandListTooShort {
                name: format!("{}.{}", workflow_name, k),
            });
        }

        let _output = match Command::new(&v[0])
            .args(&v[1..])
            .envs(&env)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
        {
            Ok(v) => v,
            Err(e) => return Err(Error::FailedToExecuteCommand(e)),
        };
    }

    Ok(())
}
