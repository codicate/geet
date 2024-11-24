use crate::repo_hiding::operation::repo::{pull_repo, push_repo};


/// Represents options related to the repository
#[derive(Clone)]
pub struct RepoOptions {
    pub local_path: Option<String>, // Local repository path
    pub remote_path: Option<String>, // Remote repository path
}

/// Represents options related to revisions
pub struct RevisionOptions {
    pub revision_id: Option<String>, // ID of a specific revision
    pub merge_base: Option<String>,  // Base revision for merging
}

/// Enum for synchronization actions
pub enum SyncAction {
    Push, // Pushes changes from local to remote
    Pull, // Pulls changes from remote to local
}

#[derive(Debug)]
pub enum SyncResult {
    SyncSuccess { message: String }, // Success message for sync operation
    MergeSuccess { merged_revision: String }, // Success message with the merged revision ID
}

/// Main struct to manage repository synchronization
pub struct RepositoryCommands;

impl RepositoryCommands {
    /// Pull action to synchronize changes from remote to local
pub fn pull_action(&self, local_path: &str, remote_path: &str) -> Result<SyncResult, String> {
    // Use the RepoOptions to specify the local and remote repository paths
    let options = RepoOptions {
        local_path: Some(local_path.to_string()),
        remote_path: Some(remote_path.to_string()),
    };

    // Call sync_changes with SyncAction::Pull
    self.sync_changes(options, SyncAction::Pull)
}

/// Push action to synchronize changes from local to remote
pub fn push_action(&self, local_path: &str, remote_path: &str) -> Result<SyncResult, String> {
    // Use the RepoOptions to specify the local and remote repository paths
    let options = RepoOptions {
        local_path: Some(local_path.to_string()),
        remote_path: Some(remote_path.to_string()),
    };

    // Call sync_changes with SyncAction::Push
    self.sync_changes(options, SyncAction::Push)
}


    /// Merge action to merge revisions or branches
    pub fn merge_action(&self, revision_id: &str, merge_base: Option<String>) -> Result<SyncResult, String> {
        // Prepare the RevisionOptions with the given revision ID and merge base
        let options = RevisionOptions {
            revision_id: Some(revision_id.to_string()),
            merge_base,
        };

        // Call the merge_revisions method with the prepared options
        self.merge_revisions(options)
    }

    /// Synchronizes the repository based on the specified SyncAction
    pub fn sync_changes(
        &self,
        options: RepoOptions,
        action: SyncAction,
    ) -> Result<SyncResult, String> {
        let local_path = options.local_path.ok_or_else(|| "Local repository path is not set".to_string())?;
        let remote_path = options.remote_path.ok_or_else(|| "Remote repository path is not set".to_string())?;
    
        match action {
            SyncAction::Push => {
                push_repo(local_path.clone(), remote_path.clone()).map_err(|e| {
                    format!("Failed to push changes: {}", e)
                })?;
                Ok(SyncResult::SyncSuccess {
                    message: "Repository successfully pushed.".to_string(),
                })
            }
            SyncAction::Pull => {
                pull_repo(remote_path.clone(), local_path.clone()).map_err(|e| {
                    format!("Failed to pull changes: {}", e)
                })?;
                Ok(SyncResult::SyncSuccess {
                    message: "Repository successfully pulled.".to_string(),
                })
            }
        }
    }
    
    /// Merges revisions using user-defined inputs
    pub fn merge_revisions(
        &self,
        options: RevisionOptions,
    ) -> Result<SyncResult, String> {
        let revision_id = options.revision_id.ok_or_else(|| "Revision ID is required for merging".to_string())?;

        let merge_base = options.merge_base.unwrap_or_else(|| "HEAD".to_string());

        // Example placeholder for actual merge logic
        let merged_revision = format!(
            "Merged revision {} into base {}",
            revision_id, merge_base
        );

        Ok(SyncResult::MergeSuccess {
            merged_revision,
        })
    }
}

fn main() {
    let repo_cmd = RepositoryCommands;

    // Define the local and remote repository paths
    let local_repo_path = "path/to/local/repo";
    let remote_repo_path = "path/to/remote/repo";

    // Sync Pull
    match repo_cmd.pull_action(local_repo_path, remote_repo_path) {
        Ok(result) => println!("Pull Success: {:?}", result),
        Err(err) => eprintln!("Pull Failed: {}", err),
    }

    // Sync Push
    match repo_cmd.push_action(local_repo_path, remote_repo_path) {
        Ok(result) => println!("Push Success: {:?}", result),
        Err(err) => eprintln!("Push Failed: {}", err),
    }
}

