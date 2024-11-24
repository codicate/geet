use chrono::Utc;

use crate::{
    file_hiding::{file_log::does_object_exist, index},
    repo_hiding::{
        data_type::CommitMetadata,
        operation::{branch::checkout_commit, repo::init_repo, revision::create_revision},
    },
};

pub fn init() -> Result<(), String> {
    init_repo(&"default".to_string(), &"main".to_string())?;
    Ok(())
}

pub fn clone(repo_path: &str) -> Result<(), String> {
    println!("Cloning repository from {}...", repo_path);
    Ok(())
}

pub fn pull(repo_path: &str) -> Result<(), String> {
    println!("Pulling changes from {}...", repo_path);
    Ok(())
}

pub fn push(repo_path: &str) -> Result<(), String> {
    println!("Pushing changes to {}...", repo_path);
    Ok(())
}

pub fn add(file_path: &str) -> Result<(), String> {
    index::add(file_path)?;
    println!("Added file {} to staging area.", file_path);
    Ok(())
}

pub fn remove(file_path: &str) -> Result<(), String> {
    index::remove(file_path)?;
    println!("Removed file {} from staging area.", file_path);
    Ok(())
}

pub fn status() -> Result<(), String> {
    // TODO - Implement this part
    print!("On branch main\n\n");

    println!("Changes to be committed:");
    let files = index::get_staged_files();
    files.iter().for_each(|file| println!("{:?}", file));
    println!();

    println!("Changes not staged for commit:");
    let files = index::get_unstaged_files();
    files.iter().for_each(|file| println!("{:?}", file));

    Ok(())
}

pub fn heads() -> Result<(), String> {
    println!("Listing branch heads...");
    Ok(())
}

pub fn log() -> Result<(), String> {
    println!("Displaying commit log...");
    Ok(())
}

pub fn diff(hash1: &str, hash2: &str) -> Result<(), String> {
    println!("Showing differences between {} and {}...", hash1, hash2);
    Ok(())
}

pub fn cat(file_path: &str) -> Result<(), String> {
    println!("Displaying contents of file {}...", file_path);
    Ok(())
}

pub fn commit(message: &String, author: &String) -> Result<(), String> {
    let metadata = CommitMetadata {
        author: author.clone(),
        message: message.clone(),
        timestamp: Utc::now().to_rfc3339(),
    };

    let commit_id = create_revision(metadata)?;
    println!("Files committed successfully with Commit ID: {}", commit_id);
    Ok(())
}

pub fn checkout(str: &String, branch: &bool) -> Result<(), String> {
    if *branch {
        println!("Creating and switching to new branch '{}'...", str);
    }

    // TODO: handle branches
    let hash = if does_object_exist(&str) { str } else { str };

    checkout_commit(str)?;
    println!("Switched to commit {}", hash);
    Ok(())
}

pub fn merge(branch_name: &str) -> Result<(), String> {
    println!("Merging branch '{}'...", branch_name);
    Ok(())
}

pub fn cleanup() -> Result<(), String> {
    println!("Cleaning up the repository...");
    Ok(())
}
