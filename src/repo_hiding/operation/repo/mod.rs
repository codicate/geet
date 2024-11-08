use crate::file_hiding::ref_log::store_ref;
use crate::repo_hiding::data_type::RepositoryConfig;
use crate::repo_hiding::data_type::{Hash, RefType};
use crate::repo_hiding::operation::branch::{create_head, create_ref, update_head};
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
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
}
