use std::fs;

use crate::file_hiding::file_management::file_log::{get_object, store_object};
use crate::repo_hiding::branch_management::get_head;
use crate::repo_hiding::data_type::{Commit, CommitMetadata, Hash};

// create a new revision with the given metadata
pub fn create_revision(metadata: &CommitMetadata) -> Hash {
    // create a new commit object
    let commit_hash = read_cwd();
    let parent_hash = get_head();
    let commit = Commit {
        metadata: metadata.clone(),
        hash: commit_hash,
        parent: parent_hash,
    };

    // store the commit object
    let serialized = serde_json::to_string(&commit).unwrap();
    store_object(&serialized);

    // update HEAD
    update_head(commit_hash);
    commit_hash
}

// get the revision with the given hash
pub fn get_revision(commit_hash: &String) -> Commit {
    let serialized = get_object(&commit_hash).unwrap();
    serde_json::from_str(&serialized).unwrap()
}

// get the parent that the revision is pointing to
pub fn get_parent_revision(commit_hash: &String) -> Option<Commit> {
    let commit = get_revision(commit_hash);
    if let Some(parent_hash) = commit.parent {
        Some(get_revision(&parent_hash))
    } else {
        None
    }
}

// apply the changes from the revision to the working directory
pub fn checkout(commit_hash: &String) {
    let commit = get_revision(commit_hash);
    update_cwd(&commit);
}

fn read_cwd() -> Hash {
    todo!()
}

fn update_cwd(commit: &Commit) {}

fn navigate_folders_recursively(path: &String) -> Result<String> {
    let children = fs::read_dir(path)?;
    let mut tree_content = String::new();

    for child in children {
        let path = child?.path();
        let path_string = gitty::strip_path(&path);

        let hash = if path.is_dir() {
            navigate_folders_recursively(path).unwrap()
        } else {
            store_object(&path_string).unwrap()
        };

        let content_type = if path.is_dir() { "tree" } else { "blob" };
        let file_name = path.file_name().unwrap().to_str().unwrap();
        tree_content.push_str(&format!("{} {} {}\n", content_type, hash, file_name));
    }

    let hash = store_object(&tree_content).unwrap();
    Ok(hash)
}
