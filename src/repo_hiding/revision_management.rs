use std::fs;
use std::io::Result;
use std::path::PathBuf;

use crate::file_hiding::file_log::{retrieve_data, store_data};
use crate::repo_hiding::application_data::{Commit, CommitMetadata, Hash};
use crate::repo_hiding::branch_management::{get_head, update_head};

use super::application_data::Tree;

// create a new revision with the given metadata
pub fn create_revision(metadata: CommitMetadata) -> Hash {
    // create a new commit object
    let commit_hash = read_cwd();
    let parent_hash = get_head();
    let commit = Commit::new_commit(commit_hash.clone(), parent_hash, metadata);

    // store the commit object
    let serialized = commit.serialize();
    store_data("./geet/objects/", &serialized).unwrap();

    // update HEAD
    update_head(&commit_hash);
    commit_hash
}

// get the revision with the given hash
pub fn get_revision(commit_hash: &String) -> Commit {
    let serialized = retrieve_data(&format!("./geet/objects/{}", commit_hash)).unwrap();
    Commit::deserialize(&serialized)
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
    navigate_folders_recursively(&"./test".to_string()).unwrap()
}

fn update_cwd(commit: &Commit) {}

fn navigate_folders_recursively(path: &String) -> Result<String> {
    println!("Navigating folder: {}", path);
    let children = fs::read_dir(path)?;
    let mut tree = Tree::new();

    for child in children {
        let path = child?.path();
        let path_string = strip_path(&path);

        let hash = if path.is_dir() {
            navigate_folders_recursively(&path_string).unwrap()
        } else {
            store_file(&path_string)
        };

        let file_name = path.file_name().unwrap().to_str().unwrap();
        if path.is_dir() {
            tree.add_dir_node(hash, file_name.to_string());
        } else {
            tree.add_file_node(hash, file_name.to_string());
        }
    }

    let serialized = tree.serialize();
    let hash = store_data("./geet/objects/", &serialized).unwrap();
    Ok(hash)
}

fn store_file(path: &String) -> Hash {
    println!("Storing file: {}", path);
    let data = fs::read_to_string(path).unwrap();
    store_data("./geet/objects/", &data).unwrap()
}

fn strip_path(path: &PathBuf) -> String {
    path.to_str()
        .map(|s| s.trim_start_matches("./").to_string())
        .unwrap_or_default()
}
