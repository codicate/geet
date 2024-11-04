// repository.rs
use crate::repo_hiding::data_type::RepositoryConfig;
use std::fs;
use std::path::Path;
use std::fmt; 

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
