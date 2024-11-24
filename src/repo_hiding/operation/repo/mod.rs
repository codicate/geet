use serde::{Deserialize, Serialize};

use crate::file_hiding::file_log::{deserialize_metadata,copy_dir};
use crate::file_hiding::ref_log::store_ref;
use crate::repo_hiding::data_type::RepositoryConfig;
use crate::repo_hiding::data_type::{Hash, RefType};
use crate::repo_hiding::operation::branch::{create_head, create_ref, update_head};
use std::io;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;


#[derive(Debug, Serialize, Deserialize)] 
pub enum RepoError {
    InitializationFailed(String),
    SerializationError(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoError::InitializationFailed(msg) => write!(f, "Initialization Failed: {}", msg),
            RepoError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl RepositoryConfig {
    // Initializes a new repository configuration and prints it to the command aline
    pub fn init_repo(
        name: String,
        path: String,
        default_branch: String,
    ) -> Result<Self, RepoError> {
        // Ensure the directory exists or create it
        if !Path::new(&path).exists() {
            fs::create_dir_all(&path).map_err(|e| {
                RepoError::InitializationFailed(format!("Failed to create directory: {}", e))
            })?;
        }

        // create .geet directory
        let refs_path = format!("{}/.geet/refs", path);
        fs::create_dir_all(&refs_path).unwrap();
        let objects_path = format!("{}/.geet/objects", path);
        fs::create_dir_all(&objects_path).unwrap();

        // Create the HEAD reference
        create_head();

        //Create the default branch reference using `create_ref`
        //Here, None is passed for the hash since no commits exist yet
        let branch_ref = create_ref(RefType::Branch, default_branch.clone(), None);

        // Create the RepositoryConfig instance
        let config = RepositoryConfig {
            name: name.clone(),
            default_branch: default_branch.clone(),
        };

        // Serialize the config to JSON format
        let serialized_config = config.serialize();

        // Print the serialized JSON to the command line
        println!("Repository configuration initialized:");
        println!("{}", serialized_config);

        Ok(config)
    }

    pub fn validate_remote_repo(path: &str) -> std::io::Result<()> {
        let geet_path = format!("{}/.geet", path);
        if !Path::new(&geet_path).exists() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Invalid remote repository: .geet directory not found",
            ))
        } else {
            Ok(())
        }
    }

    fn copy_refs(remote_path: &str, local_path: &str)  -> std::io::Result<()> {
        let remote_refs_path = format!("{}/.geet/refs", remote_path);
        let local_refs_path = format!("{}/.geet/refs", local_path);
        copy_dir(&remote_refs_path, &local_refs_path)
    }

    pub fn clone_repo(
        remote_path: String,
        local_path: String,
    ) -> Result<(), RepoError> {
        // Validate the remote repository
        Self::validate_remote_repo(&remote_path).map_err(|e| {
            RepoError::InitializationFailed(format!("Remote repository validation failed: {}", e))
        })?;
    
        // Ensure the local path does not already exist
        if Path::new(&local_path).exists() {
            return Err(RepoError::InitializationFailed(
                "The destination path already exists.".to_string(),
            ));
        }
    
        // Create the local repository structure
        fs::create_dir_all(&local_path).map_err(|e| {
            RepoError::InitializationFailed(format!("Failed to create directory: {}", e))
        })?;
    
        // Copy the .geet directory
        let remote_geet_path = format!("{}/.geet", remote_path);
        let local_geet_path = format!("{}/.geet", local_path);
        copy_dir(&remote_geet_path, &local_geet_path).map_err(|e| {
            RepoError::InitializationFailed(format!("Failed to copy .geet directory: {}", e))
        })?;
    
        // Copy references
        Self::copy_refs(&remote_path, &local_path).map_err(|e| {
            RepoError::InitializationFailed(format!("Failed to copy refs: {}", e))
        })?;
    
        // Set up HEAD locally
        let remote_head_path = format!("{}/.geet/HEAD", remote_path);
        let local_head_path = format!("{}/.geet/HEAD", local_path);
        fs::copy(&remote_head_path, &local_head_path).map_err(|e| {
            RepoError::InitializationFailed(format!("Failed to copy HEAD reference: {}", e))
        })?;
    
        println!("Repository successfully cloned to {}", local_path);
        Ok(())
    }  
}
