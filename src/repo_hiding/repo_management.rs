use std::fs;
use std::process::Command;

// Struct representing a repository
pub struct Repository {
    pub name: String,                // Name of the repository
    pub path: String,                // Local path of the repository
    pub remote_url: Option<String>,  // URL of the remote repository (if applicable)
    pub default_branch: String,      // Default branch name
}

// Enum for repository errors
pub enum RepoError {
    InitializationFailed(String),   // Error when repository initialization fails
    CloneFailed(String),            // Error when cloning fails
    SyncFailed(String),             // Error when pull or push fails
}

// Repository Level Operations
impl Repository {
    // Initializes a new repository
    pub fn init_repo(name: String, path: String, default_branch: String) -> Result<Self, RepoError> {
        // Attempt to create a directory for the repository
        match fs::create_dir(&path) {
            Ok(_) => Ok(Repository {
                name,
                path,
                remote_url: None,
                default_branch,
            }),
            Err(e) => Err(RepoError::InitializationFailed(format!("Failed to initialize: {}", e))),
        }
    }
}