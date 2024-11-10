use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::repo_hiding::operation::repo;
use crate::repo_hiding::data_type::RepositoryConfig;

pub struct FileSystemCommands;

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
    pub fn init_repository(&self, name: String, path: String, default_branch: String) -> Result<(), CommandError> {
        RepositoryConfig::init_repo(name, path, default_branch)
            .map(|_| ())  
            .map_err(|e| CommandError::ProcessFailed(e.to_string()))
    }
}