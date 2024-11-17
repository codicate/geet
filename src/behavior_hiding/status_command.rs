use chrono::Utc;

use crate::repo_hiding::{
    data_type::CommitMetadata,
    operation::{branch::{checkout_ref, get_head, list_commits}, revision::create_revision},
};

#[derive(Clone)]
pub struct RepoOptions {
    pub path: Option<String>,
    pub current_branch: Option<String>,
}

#[derive(Default, Debug)]
pub struct RevisionOptions {
    pub commit_message: Option<String>,
    pub author: Option<String>,
    pub ref_name: Option<String>, 
    pub count: Option<i32>,
}

#[derive(Debug)]
pub enum InspectionType {
    Status,
    Heads,
}

#[derive(Debug)]
pub enum RevisionAction {
    Diff,
    Cat,
    Checkout,
    Commit,
    Log,
}

#[derive(Debug)]
pub enum InspectionResult {
    FileStatus {
        modified_files: Vec<String>,
        untracked_files: Vec<String>,
    },
    HeadStatus {
        branches: Vec<String>,
        head_commit: String,
    },
}

#[derive(Debug)]
pub enum RevisionResult {
    DiffResult { changes: String },
    CatResult { content: String },
    CheckoutResult { success_message: String },
    CommitResult { commit_id: String },
    LogResult { history: Vec<String> },
}

#[derive(Debug)]
pub enum StatusError {
    InvalidCommand,
    ActionFailed(String),
}



pub struct RepositoryCommands {
    pub repo_options: RepoOptions,
    pub revision_options: RevisionOptions,
}

impl RepositoryCommands {
    pub fn commit_action(
        &self,
        commit_message: &str,
        author: &str,
    ) -> Result<RevisionResult, StatusError> {
        let options = RevisionOptions {
            commit_message: Some(commit_message.to_string()),
            author: Some(author.to_string()),
            ref_name: None,
            count: None,
        };
        let result = self.manage_revisions(options, RevisionAction::Commit);
        println!("{:?}", result);
        result
    }

    pub fn checkout_action(&self, ref_name: &str) -> Result<RevisionResult, StatusError> {
        let options = RevisionOptions {
            commit_message: None,
            author: None,
            ref_name: Some(ref_name.to_string()), // Pass the reference name
            count: None,
        };
        self.manage_revisions(options, RevisionAction::Checkout)
    }

    pub fn log_action(&self, ref_name: Option<String>, count: Option<i32>) -> Result<RevisionResult, StatusError> {
        // Prepare the RevisionOptions with ref_name and count
        let options = RevisionOptions {
            commit_message: None, 
            author: None,         
            ref_name,             // Pass the ref_name provided by user input
            count,                // Pass the count (number of commits to list) provided by user input
        };
    
        // Call manage_revisions with the prepared options and the Log action
        self.manage_revisions(options, RevisionAction::Log)
    }
    

    fn inspect_repo(
        &self,
        _options: RepoOptions,
        inspection_type: InspectionType,
    ) -> Result<InspectionResult, StatusError> {
        match inspection_type {
            InspectionType::Status => Ok(InspectionResult::FileStatus {
                modified_files: vec!["file1.txt".to_string(), "file2.txt".to_string()],
                untracked_files: vec!["newfile.txt".to_string()],
            }),
            InspectionType::Heads => {
                let head_commit = get_head().unwrap_or_default();
                Ok(InspectionResult::HeadStatus {
                    branches: vec!["main".to_string(), "feature-branch".to_string()],
                    head_commit: head_commit.to_string(),
                })
            }
        }
    }

    fn manage_revisions(
        &self,
        options: RevisionOptions,
        revision_action: RevisionAction,
    ) -> Result<RevisionResult, StatusError> {
        let repo_path = match &self.repo_options.path {
            Some(path) => path,
            None => {
                return Err(StatusError::ActionFailed(
                    "Repository path not set".to_string(),
                ))
            }
        };
        match revision_action {
            RevisionAction::Checkout => {
                if let Some(ref_name) = options.ref_name {
                    // If ref_name is provided, call checkout_ref
                    checkout_ref(&ref_name);
                    Ok(RevisionResult::CheckoutResult {
                        success_message: format!("Checked out ref: {}", ref_name),
                    })
                } else {
                    // If no ref_name is provided, it might be an error
                    Err(StatusError::ActionFailed("Reference name is required".to_string()))
                }
            },
            RevisionAction::Diff => todo!(),
            RevisionAction::Cat => todo!(),
            RevisionAction::Log => {
                // Get the ref_name and count from options
                let ref_name = options.ref_name.unwrap_or_else(|| "HEAD".to_string()); // Default to HEAD if ref_name is not provided
                let count = options.count.unwrap_or(i32::MAX); // Default to all commits if count is not provided
    
                // Call the list_commits function to get the commit history
                let commits = list_commits(ref_name, Some(count));
    
                let commit_history: Vec<String> = commits
                    .into_iter()
                    .map(|commit| format!(
                        "Commit ID: {}, Author: {}, Message: {}, Timestamp: {}", 
                        commit.tree_hash,
                        commit.metadata.author,
                        commit.metadata.message,
                        commit.metadata.timestamp
                    ))
                    .collect();

                // Return the result as a LogResult
                Ok(RevisionResult::LogResult { history: commit_history })
            }
            _ => Err(StatusError::InvalidCommand),
    
            RevisionAction::Commit => {
                println!(
                    "Committing all changes in repository at path: {}",
                    repo_path
                );

                // Create commit metadata
                let metadata = CommitMetadata {
                    author: options.author.unwrap_or("Author Name".to_string()),
                    message: options
                        .commit_message
                        .unwrap_or("Default commit message".to_string()),
                    timestamp: Utc::now().to_rfc3339(),
                };

                // Step 4: Create a new revision using the commit metadata
                let commit_id = create_revision(metadata);
                println!(
                    "Files committed successfully for repository at {} with Commit ID: {}",
                    repo_path, commit_id
                );
                println!("Files committed successfully with Commit ID: {}", commit_id);
                Ok(RevisionResult::CommitResult { commit_id })
            }
            _ => Err(StatusError::InvalidCommand),
        }
    }
}

// // Mock function for head retrieval
// fn get_head() -> String {
//     "mock-head-hash".to_string()
// }

// fn main() {
//     let repo_options = RepoOptions {
//         path: Some("path/to/your/repository".to_string()),
//         current_branch: Some("main".to_string()),
//     };

//     let revision_options = RevisionOptions::default();
//     let repository_commands = RepositoryCommands {
//         repo_options,
//         revision_options,
//     };

//     repository_commands.commit_action("Initial commit", "Author Name");
// }
