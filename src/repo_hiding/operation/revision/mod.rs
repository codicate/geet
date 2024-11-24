mod cwd;
use super::branch::{get_head, update_head};
use crate::file_hiding::file_log::{retrieve_object, store_object};
use crate::repo_hiding::data_type::{Commit, CommitMetadata, Hash};
use cwd::{read_cwd, update_cwd};

use crate::file_hiding::index::{clear_index, get_staged_files, is_stage_empty};

// create a new revision with the given metadata
pub fn create_revision(metadata: CommitMetadata) -> Result<Hash, String> {
    if is_stage_empty() {
        return Err("No changes to commit".to_string());
    }

    // create a new commit object
    let tree_hash = read_cwd().ok_or_else(|| "No changes to commit".to_string())?;
    let parent_hash = get_head();
    let commit = Commit::new_commit(tree_hash, parent_hash, metadata);

    // store the commit object
    let serialized = commit.serialize();
    let commit_hash = store_object(&serialized).unwrap();

    // update HEAD
    update_head(&commit_hash);

    // Clear the index after successful commit
    clear_index().unwrap_or_else(|e| eprintln!("Warning: Failed to clear index: {}", e));

    Ok(commit_hash)
}

// get the revision with the given hash
pub fn get_revision(commit_hash: &String) -> Commit {
    let serialized = retrieve_object(&commit_hash).unwrap();
    Commit::deserialize(&serialized)
}

// get the parent that the revision is pointing to
pub fn get_parent_revision(commit_hash: &String) -> Option<Commit> {
    let commit = get_revision(commit_hash);
    if let Some(parent_hash) = commit.parent_hash {
        Some(get_revision(&parent_hash))
    } else {
        None
    }
}

// apply the changes from the revision to the working directory
pub fn apply_revision(commit_hash: &String) {
    let commit = get_revision(commit_hash);
    update_cwd(&commit.tree_hash);
}
