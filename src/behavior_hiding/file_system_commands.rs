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
    Clone {
        remote_path: String,
        local_path: String
    },
    Status,
}


impl FileSystemCommands {
    pub fn init_repository(&self, name: String, path: String, default_branch: String) -> Result<(), String> {
        RepositoryConfig::init_repo(name, path, default_branch)
            .map(|_| ()) // If successful, return Ok(())
            .map_err(|e| e.to_string())
    }

    pub fn clone_repository(&self, remote_path: String, local_path: String) -> Result<(), String> {
        RepositoryConfig::clone_repo(remote_path, local_path).map(|_| ()) // If successful, return Ok(())
        .map_err(|e| e.to_string())
    }
    
    pub fn add_file(&self, path: &str) -> Result<(), String> {
        // check if file exist first
        if !Path::new(path).exists() {
            return Err(format!("Error: file '{}' does not exist", path));
        }
        add_to_index(path).map_err(|e| e.to_string())
    }

    pub fn get_status(&self) -> Result<Vec<String>, String> {
        Ok(get_staged_files())
    }
}
