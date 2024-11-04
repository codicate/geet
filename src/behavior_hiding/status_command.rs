use chrono::Utc;


use crate::repo_hiding::{data_type::CommitMetadata, operation::revision::create_revision};

#[derive(Clone)]
pub struct RepoOptions {
    pub path: Option<String>,
    pub current_branch: Option<String>,
}

#[derive(Default, Debug)]
pub struct RevisionOptions {
    pub commit_message: Option<String>,
    pub author: Option<String>
}

#[derive(Debug)]
pub enum InspectionType {
    Status,
    Heads,
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
pub enum RevisionAction {
    Diff,
    Cat,
    Checkout,
    Commit,
    Log,
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

    pub fn commit_action(&self, commit_message: &str, author: &str) {
        let options = RevisionOptions {
            commit_message: Some(commit_message.to_string()),
            author: Some(author.to_string()),
        };
        let result = self.manage_revisions(options, RevisionAction::Commit);
        println!("{:?}", result);
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
                let head_commit = get_head();
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
        
            RevisionAction::Checkout => todo!(),
            RevisionAction::Diff => todo!(),
            RevisionAction::Cat => todo!(),
            RevisionAction::Commit => todo!(),
            RevisionAction::Log => todo!(),
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

// Mock function for head retrieval
fn get_head() -> String {
    "mock-head-hash".to_string()
}

fn main() {
    let repo_options = RepoOptions {
        path: Some("path/to/your/repository".to_string()),
        current_branch: Some("main".to_string()),
    };

    let revision_options = RevisionOptions::default();
    let repository_commands = RepositoryCommands {
        repo_options,
        revision_options,
    };

    repository_commands.commit_action("Initial commit", "Author Name");
}
