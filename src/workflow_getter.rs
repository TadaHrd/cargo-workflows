use std::collections::HashMap;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot open workflows.toml: {0}")]
    FileOpenError(std::io::Error),
    #[error("cannot parse workflows.toml: {0}")]
    TomlParseError(toml::de::Error),
    #[error("workflow \"{workflow}\" doesn't exist")]
    WorkflowNotFound { workflow: String },
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Workflow {
    pub env: Option<HashMap<String, String>>,
    pub commands: BTreeMap<String, Vec<String>>,
}

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
