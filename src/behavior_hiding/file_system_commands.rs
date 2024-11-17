use std::fs;
use std::io::{self, Write};
use std::path::Path;
use crate::repo_hiding::operation::repo;
use crate::repo_hiding::data_type::RepositoryConfig;

use crate::file_hiding::file_log::{store_object, store_file};
use crate::file_hiding::index::{add_to_index, get_staged_files};

pub struct FileSystemCommands;

pub enum RepositoryCommand {
    Init {
        name: String,
        path: String,
        default_branch: String,
    },
    Add {
        path: String,
    },
    Status,
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

    pub fn add_file(&self, path: &str) -> Result<(), CommandError> {
        add_to_index(path)
            .map_err(|e| CommandError::ProcessFailed(e.to_string()))
    }

    pub fn get_status(&self) -> Result<Vec<String>, CommandError> {
        Ok(get_staged_files())
    }
}
