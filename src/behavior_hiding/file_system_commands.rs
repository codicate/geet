use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::repo_hiding::operation::repo;
use crate::repo_hiding::data_type::RepositoryConfig;

pub struct FileSystemCommands {}

pub enum RepositoryCommand {
    Init {
        name: String,
        path: String,
        default_branch: String,
    },
}

#[derive(Debug)]
pub enum CommandError {
    InvalidPath,
    ProcessFailed(String),
}

impl FileSystemCommands {
    pub fn repository_calls(&self, command: RepositoryCommand) -> Result<(), CommandError> {
        match command {
            RepositoryCommand::Init { name, path, default_branch } => {
                // Call the init_repo function with the correct types
                match RepositoryConfig::init_repo(name, path, default_branch) {  // Adjust here
                    Ok(_) => Ok(()),
                    Err(e) => Err(CommandError::ProcessFailed(e.to_string())),
                }
            }
        }
    }
}
