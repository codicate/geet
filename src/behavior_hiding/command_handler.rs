use crate::{
    file_hiding::{
        file_log::{does_object_exist, retrieve_object},
        index,
        ref_log::Hash,
    },
    repo_hiding::{
        data_type::{CommitMetadata, RefType, Commit},
        operation::{
            branch::{
                checkout_commit,
                diff::{get_diffs, Diff},
                list_commits, list_refs,
            },
            repo::{clone_repo, init_repo, pull_repo, push_repo},
            revision::create_revision,
        },
    },
    BASE_DIR, OBJECTS_DIR,
};
use chrono::Utc;
use colored::Colorize;
use std::process::Command;
use std::fs;

pub fn init() -> Result<(), String> {
    init_repo(&"default".to_string(), &"main".to_string())?;
    Ok(())
}

pub fn clone(remote_path: &String) -> Result<(), String> {
    println!("Cloning repository from {}...", remote_path);
    clone_repo(remote_path, &BASE_DIR.to_string())?;
    Ok(())
}

pub fn pull(remote_path: &String) -> Result<(), String> {
    pull_repo(remote_path, &BASE_DIR.to_string())
        .map_err(|e| format!("Failed to pull changes\n{}", e))?;
    println!("Pulled changes from {}.", remote_path);
    Ok(())
}

pub fn push(remote_path: &String) -> Result<(), String> {
    push_repo(&BASE_DIR.to_string(), remote_path)
        .map_err(|e| format!("Failed to push changes\n{}", e))?;
    println!("Pushed changes to {}.", remote_path);
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

// pub fn cat(hash: &Hash) -> Result<(), String> {
//     let content =
//         retrieve_object(hash).map_err(|_| "Object with given hash not found".to_string())?;
//     println!("{}", content);
//     Ok(())
// }
pub fn cat(path_or_hash: &String) -> Result<(), String> {
    // First try to read as a regular file
    if let Ok(content) = std::fs::read_to_string(path_or_hash) {
        println!("{}", content);
        return Ok(());
    }

    // If not a file, try as a hash
    let content = retrieve_object(path_or_hash)
        .map_err(|_| format!("Neither a valid file path nor a valid object hash: {}", path_or_hash))?;

    // Try to parse as commit for better formatting
    if let Ok(commit) = serde_json::from_str::<Commit>(&content) {
        println!("Commit: {}\nAuthor: {}\nDate: {}\nMessage: {}", 
            path_or_hash, 
            commit.metadata.author, 
            commit.metadata.timestamp, 
            commit.metadata.message);
        return Ok(());
    }

    // If not a commit, just print the content
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

// TODO
pub fn merge(from: &Hash) -> Result<(), String> {
    let to = "HEAD".to_string();
    let merged_revision = format!("Merged revision {} to {}", from, to);

    Ok(())
}

fn print_file_diffs(hash1: &String, hash2: &String) {
    // Retrieve content from both versions
    let content1 = retrieve_object(hash1).unwrap_or_else(|_| String::new());
    let content2 = retrieve_object(hash2).unwrap_or_else(|_| String::new());

    // Split content into lines
    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();

    // Print the diff
    println!("@@ -{},{} +{},{} @@", 1, lines1.len(), 1, lines2.len());

    // Simple line-by-line comparison
    let max_lines = lines1.len().max(lines2.len());
    for i in 0..max_lines {
        match (lines1.get(i), lines2.get(i)) {
            (Some(l1), Some(l2)) if l1 == l2 => {
                println!(" {}", l1); // Unchanged line
            }
            (Some(l1), Some(l2)) => {
                println!("{}", format!("-{}", l1).red()); // Modified line (old)
                println!("{}", format!("+{}", l2).green()); // Modified line (new)
            }
            (Some(l1), None) => {
                println!("{}", format!("-{}", l1).red()); // Deleted line
            }
            (None, Some(l2)) => {
                println!("{}", format!("+{}", l2).green()); // Added line
            }
            (None, None) => unreachable!(),
        }
    }
    println!();
}
