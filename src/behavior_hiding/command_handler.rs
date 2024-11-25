use crate::{
    file_hiding::{
        file_log::{does_object_exist, retrieve_object},
        index,
        ref_log::Hash,
    },
    repo_hiding::{
        data_type::{CommitMetadata, RefType},
        operation::{
            branch::{
                checkout_commit,
                diff::{get_diffs, Diff},
                list_commits, list_refs,
            },
            repo::init_repo,
            revision::create_revision,
        },
    },
    OBJECTS_DIR,
};
use chrono::Utc;
use colored::Colorize;
use std::process::Command;

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
    let ref_list = list_refs(RefType::Branch)?;
    println!("Active branches:");
    for _ref in ref_list {
        println!("{}", _ref.name);
    }
    Ok(())
}

pub fn log() -> Result<(), String> {
    let commit_map = list_commits("HEAD".to_string(), None)?;

    for (commit_hash, commit) in &commit_map {
        println!("commit {}", commit_hash);
        println!("\tAuthor: {}", commit.metadata.author);
        println!("\tDate: {}", commit.metadata.timestamp);
        println!("\tMessage: {}", commit.metadata.message);
        println!();
    }

    Ok(())
}

pub fn diff(hash1: &Hash, hash2: &Hash) -> Result<(), String> {
    let Diff {
        deleted_files,
        modified_files,
        new_files,
    } = get_diffs(hash1, hash2);

    if !deleted_files.is_empty() {
        println!("Deleted files:");
        for file in &deleted_files {
            let line = format!("- {}", file).red();
            println!("{}", line);
        }
        println!();
    }

    if !new_files.is_empty() {
        println!("New files:");
        for file in &new_files {
            let line = format!("+ {}", file).green();
            println!("{}", line);
        }
        println!();
    }

    if !modified_files.is_empty() {
        for (file, hash1, hash2) in &modified_files {
            let line = format!("> {}", file).yellow();
            println!("{}", line);
            print_file_diffs(&hash1, &hash2);
        }
    }

    if deleted_files.is_empty() && new_files.is_empty() && modified_files.is_empty() {
        println!("No changes found between the commits.");
    }

    Ok(())
}

pub fn cat(hash: &Hash) -> Result<(), String> {
    let content =
        retrieve_object(hash).map_err(|_| "Object with given hash not found".to_string())?;
    println!("{}", content);
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

fn print_file_diffs(hash1: &String, hash2: &String) {
    let path1 = format!("{}\\{}", OBJECTS_DIR, hash1);
    let path2 = format!("{}\\{}", OBJECTS_DIR, hash2);

    let output = Command::new("diff")
        .arg("-u")
        .arg(path1)
        .arg(path2)
        .output()
        .expect("failed to execute diff command");

    let diff = String::from_utf8(output.stdout).unwrap();
    println!("{}", diff);
}
