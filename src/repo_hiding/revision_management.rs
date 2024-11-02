use crate::file_hiding::file_management::file_log::store_commit;
use crate::repo_hiding::branch_management::get_head;
use crate::repo_hiding::data_type::{Commit, CommitMetadata, Hash};

// create a new revision with the given metadata
pub fn create_revision(metadata: &CommitMetadata) -> Hash {}

// get the revision with the given hash
pub fn get_revision(commit_hash: &String) -> Commit {
    todo!()
}

// get the parent that the revision is pointing to
pub fn get_parent_revision(commit_hash: &String) -> Option<Commit> {}

// apply the changes from the revision to the working directory
pub fn checkout(commit_hash: &String) {
    todo!()
}
