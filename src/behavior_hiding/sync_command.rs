// use crate::repo_hiding::branch_management::get_head;
// use crate::repo_hiding::revision_management::{checkout, create_revision};
// use sha1::{Digest, Sha1};
// use std::collections::HashMap;
// use std::io::{self, Write};

// // Struct for Repository Options
// #[derive(Clone, Debug)]
// pub struct RepoOptions {
//     pub path: Option<String>,           // Path to the repository
//     pub current_branch: Option<String>, // Active branch name
// }

// // Struct for Revision Options
// #[derive(Default, Clone, Debug)]
// pub struct RevisionOptions {
//     pub revision_id: Option<String>,    // ID of a specific revision
//     pub file_path: Option<String>,      // Path of a file to inspect or commit
//     pub commit_message: Option<String>, // Commit message for a new commit
//     pub author: Option<String>,         // Author name for the commit
//     pub parent_commit: Option<String>,  // ID of the parent commit, if applicable
// }

// // Enum for inspection types
// #[derive(Clone, Debug)]
// pub enum InspectionType {
//     Status, // Displays file statuses (modified, untracked)
//     Heads,  // Shows active branches and HEAD commit
// }

// // Enum for inspection results
// #[derive(Clone, Debug)]
// pub enum InspectionResult {
//     FileStatus {
//         modified_files: Vec<String>,
//         untracked_files: Vec<String>,
//     },
//     HeadStatus {
//         branches: Vec<String>,
//         head_commit: String,
//     },
// }

// // Enum for revision actions
// #[derive(Clone, Debug)]
// pub enum RevisionAction {
//     Diff,     // Checks changes between revisions
//     Cat,      // Inspects a file from a given revision
//     Checkout, // Switches to a specific revision
//     Commit,   // Creates a new revision with changes
//     Log,      // Displays the revision history
// }

// // Enum for revision results
// #[derive(Clone, Debug)]
// pub enum RevisionResult {
//     DiffResult { changes: String }, // Summary of changes between revisions
//     CatResult { content: String },  // Content of a file in the specific revision
//     CheckoutResult { success_message: String }, // Confirmation message for a successful checkout
//     CommitResult { commit_id: String }, // ID of the newly created commit
//     LogResult { history: Vec<String> }, // List of revision history entries
// }

// // Enum for status-related errors
// #[derive(Clone, Debug)]
// pub enum StatusError {
//     InvalidCommand,       // Error for unrecognized commands
//     ActionFailed(String), // General error for action failures
// }

// // Main struct for Repository Status Commands
// #[derive(Clone, Debug)]
// pub struct RepositoryCommands {
//     pub repo_options: RepoOptions,
//     pub revision_options: RevisionOptions,
// }

// impl RepositoryCommands {
//     pub fn run_user_interface(&self) {
//         loop {
//             println!("Select an action: ");
//             println!("1. Status - Check the current status of the repository");
//             println!("2. Heads - Show the current heads");
//             println!("3. Diff - Check changes between revisions");
//             println!("4. Cat - Inspect a file of a given revision");
//             println!("5. Checkout - Check out a specific revision");
//             println!("6. Commit - Commit changes and create a new revision");
//             println!("7. Log - View the change log");
//             println!("0. Exit");

//             let action = self.get_user_input("Enter the action number: ");
//             match action.trim() {
//                 "1" => {
//                     // Status action
//                     let result =
//                         self.inspect_repo(self.repo_options.clone(), InspectionType::Status);
//                     println!("{:?}", result);
//                 }
//                 "2" => {
//                     // Heads action
//                     let result =
//                         self.inspect_repo(self.repo_options.clone(), InspectionType::Heads);
//                     println!("{:?}", result);
//                 }
//                 "3" => {
//                     // Diff action
//                     let revision_id = self.get_user_input("Enter the revision ID to diff: ");
//                     let options = RevisionOptions {
//                         revision_id: Some(revision_id),
//                         ..Default::default()
//                     };
//                     let result = self.manage_revisions(options, RevisionAction::Diff);
//                     println!("{:?}", result);
//                 }
//                 "4" => {
//                     // Cat action
//                     let revision_id = self.get_user_input("Enter the revision ID to inspect: ");
//                     let file_path = self.get_user_input("Enter the file path to inspect: ");
//                     let options = RevisionOptions {
//                         revision_id: Some(revision_id),
//                         file_path: Some(file_path),
//                         ..Default::default()
//                     };
//                     let result = self.manage_revisions(options, RevisionAction::Cat);
//                     println!("{:?}", result);
//                 }
//                 "5" => {
//                     // Checkout action
//                     let ref_name = self.get_user_input("Enter the revision ID to check out: ");
//                     let options = RevisionOptions {
//                         revision_id: Some(ref_name),
//                         ..Default::default()
//                     };
//                     let result = self.manage_revisions(options, RevisionAction::Checkout);
//                     println!("{:?}", result);
//                 }
//                 "6" => {
//                     // Commit action
//                     let commit_message = self.get_user_input("Enter the commit message: ");
//                     let author = self.get_user_input("Enter the author name: ");
//                     let parent_commit =
//                         self.get_user_input("Enter the parent commit ID (optional): ");
//                     let parent = if parent_commit.trim().is_empty() {
//                         None
//                     } else {
//                         Some(parent_commit)
//                     };

//                     let options = RevisionOptions {
//                         commit_message: Some(commit_message),
//                         author: Some(author),
//                         parent_commit: parent,
//                         ..Default::default()
//                     };
//                     let result = self.manage_revisions(options, RevisionAction::Commit);
//                     println!("{:?}", result);
//                 }
//                 "7" => {
//                     // Log action
//                     let result = self.manage_revisions(Default::default(), RevisionAction::Log);
//                     println!("{:?}", result);
//                 }
//                 "0" => {
//                     println!("Exiting...");
//                     break;
//                 }
//                 _ => {
//                     println!("Invalid option. Please try again.");
//                 }
//             }
//         }
//     }

//     // Helper function to get user input from the console
//     fn get_user_input(&self, prompt: &str) -> String {
//         print!("{}", prompt);
//         io::stdout().flush().unwrap(); // Ensure prompt is printed before reading input
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         input.trim().to_string()
//     }

//     // Method to inspect repository status or heads
//     pub fn inspect_repo(
//         &self,
//         options: RepoOptions,
//         inspection_type: InspectionType,
//     ) -> Result<InspectionResult, StatusError> {
//         match inspection_type {
//             InspectionType::Status => {
//                 // Mock function call - Replace with actual Repository Hiding function
//                 Ok(InspectionResult::FileStatus {
//                     modified_files: vec!["file1.txt".to_string(), "file2.txt".to_string()],
//                     untracked_files: vec!["newfile.txt".to_string()],
//                 })
//             }
//             InspectionType::Heads => {
//                 // Use get_head function to retrieve the current HEAD commit hash
//                 let head_commit = get_head(); // Calls the actual function to get the HEAD
//                 Ok(InspectionResult::HeadStatus {
//                     branches: vec!["main".to_string(), "feature-branch".to_string()],
//                     head_commit: head_commit.to_string(), // Assuming `Hash` implements `to_string()`
//                 })
//             }
//         }
//     }

//     // Method to manage revisions based on specified actions
//     pub fn manage_revisions(
//         &self,
//         options: RevisionOptions,
//         revision_action: RevisionAction,
//     ) -> Result<RevisionResult, StatusError> {
//         let repo_path = match &self.repo_options.path {
//             Some(path) => path,
//             None => {
//                 return Err(StatusError::ActionFailed(
//                     "Repository path not set".to_string(),
//                 ))
//             }
//         };
//         match revision_action {
//             RevisionAction::Diff => {
//                 // Mock function call - Replace with actual Repository Hiding function
//                 Ok(RevisionResult::DiffResult {
//                     changes: "modified file1.txt".to_string(),
//                 })
//             }
//             RevisionAction::Cat => Ok(RevisionResult::CatResult {
//                 content: "File content from specified revision".to_string(),
//             }),
//             RevisionAction::Checkout => {
//                 // Ensure a revision ID is provided for checkout
//                 if let Some(ref_name) = options.revision_id.clone() {
//                     // Attempt to perform the checkout
//                     checkout(ref_name); // This calls the actual checkout function
//                     Ok(RevisionResult::CheckoutResult {
//                         success_message: format!("Checked out revision {}", ref_name),
//                     })
//                 } else {
//                     Err(StatusError::InvalidCommand) // Return an error if no revision ID is provided
//                 }
//             }

//             RevisionAction::Commit => {
//                 println!(
//                     "Committing all changes in repository at path: {}",
//                     repo_path
//                 );

//                 // Generate a unique SHA-1 ID for the commit
//                 let mut hasher = Sha1::new();
//                 hasher.update(Utc::now().to_string()); // Add timestamp for uniqueness
//                 let commit_id = format!("{:x}", hasher.finalize());

//                 // Pass gathered metadata to `new_commit`
//                 let commit = create_revision(
//                     commit_id.clone(),
//                     options
//                         .commit_message
//                         .unwrap_or("Default commit message".to_string()),
//                     options.author.unwrap_or("Author Name".to_string()),
//                     Utc::now().to_rfc3339(),
//                     options.parent_commit,
//                 );

//                 println!(
//                     "Commit completed successfully for repository at {} with ID: {}",
//                     repo_path, commit.id
//                 );
//                 Ok(RevisionResult::CommitResult {
//                     commit_id: commit.id,
//                 })
//             }
//             _ => Err(StatusError::InvalidCommand),

//             RevisionAction::Log => Ok(RevisionResult::LogResult {
//                 history: vec!["Commit 1".to_string(), "Commit 2".to_string()],
//             }),
//         }
//     }
// }
