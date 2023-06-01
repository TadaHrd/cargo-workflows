use std::collections::HashMap;

use alloc::{string::String, vec::Vec};

#[derive(Debug)]
pub enum Error {
    FileOpenError(std::io::Error),
    TomlParseError(toml::de::Error),
    WorkflowNotFound { workflow: String },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FileOpenError(e) => write!(f, "Cannot open workflows.toml: {}", e),
            Self::TomlParseError(e) => write!(f, "Cannot parse workflows.toml: {}", e),
            Self::WorkflowNotFound { workflow } => {
                if workflow != "default" {
                    write!(f, "workflow \"{}\" doesn't exist", workflow)
                } else {
                    write!(f, "the \"default\" workflow doesn't exist")
                }
            }
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Workflow {
    pub env: Option<HashMap<String, String>>,
    pub commands: HashMap<String, Vec<String>>,
}

impl std::error::Error for Error {}

/// Gets a workflow
pub fn get_workflow(workflow: &str) -> Result<Workflow, Error> {
    let contents = match std::fs::read_to_string("workflows.toml") {
        Ok(v) => v,
        Err(e) => return Err(Error::FileOpenError(e)),
    };

    let workflows: HashMap<String, Workflow> = match toml::de::from_str(&contents) {
        Ok(v) => v,
        Err(e) => return Err(Error::TomlParseError(e)),
    };

    let workflow = match workflows.iter().find_map(|v| {
        if v.0 as &str == workflow {
            Some(v.1)
        } else {
            None
        }
    }) {
        Some(v) => v,
        None => {
            return Err(Error::WorkflowNotFound {
                workflow: workflow.into(),
            })
        }
    };

    Ok(workflow.clone())
}
