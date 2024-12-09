use crate::file_hiding::file_log::{retrieve_object, store_object};
use crate::file_hiding::index;
use crate::repo_hiding::data_type::{Hash, Tree};
use crate::{BASE_DIR, GEET_DIR};
use std::fs;
use std::io::{self, Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub fn read_cwd() -> Option<Hash> {
    read_cwd_helper(BASE_DIR).unwrap_or(None)
}

fn read_cwd_helper(path: &str) -> Result<Option<Hash>> {
    let children = fs::read_dir(path)?;
    let mut tree = Tree::new();
    let staged_files: HashSet<PathBuf> = index::get_staged_files()
        .into_iter()
        .map(|p| p.canonicalize().unwrap_or(p))
        .collect();

    for child in children {
        let path = child?.path();
        let canonical_path = path.canonicalize()?;
        let path_string = strip_path(&path);

        if path_string.starts_with(GEET_DIR) {
            continue;
        }

        if path.is_dir() || staged_files.contains(&canonical_path) {
            let hash = if path.is_dir() {
                read_cwd_helper(&path_string)?
            } else {
                Some(store_file(&path_string))
            };

            if let Some(hash) = hash {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                tree.add_node(file_name.to_string(), hash, path.is_dir());
            }
        }
    }

    if tree.nodes.is_empty() {
        Ok(None)
    } else {
        let serialized = tree.serialize();
        let hash = store_object(&serialized)?;
        Ok(Some(hash))
    }
}

pub fn update_cwd(hash: &Hash) -> Result<()> {
    println!("Starting checkout process for hash: {}", hash);
    
    // First retrieve and deserialize the tree
    let serialized = retrieve_object(hash).map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to retrieve tree object: {}", e))
    })?;
    
    let tree = Tree::deserialize(&serialized);
    println!("Retrieved tree with {} nodes", tree.nodes.len());
    
    // First, clean up the working directory
    delete_cwd(Path::new(BASE_DIR))?;
    
    // Now process each node directly in the working directory
    for node in &tree.nodes {
        let target_path = PathBuf::from(BASE_DIR).join(&node.name);
        println!("Processing node '{}' to path '{}'", node.name, target_path.display());
        
        if node.is_dir {
            fs::create_dir_all(&target_path)?;
            process_directory(&target_path, &node.hash)?;
        } else {
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let contents = retrieve_object(&node.hash)
                .map_err(|e| Error::new(ErrorKind::Other, 
                    format!("Failed to retrieve file contents for {}: {}", node.name, e)))?;
            
            fs::write(&target_path, contents.as_bytes())?;
            
            // Verify the file was created successfully
            if !target_path.exists() {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to create file: {}", target_path.display())
                ));
            }
            println!("Successfully written file: {}", target_path.display());
        }
    }
    
    Ok(())
}

fn process_directory(path: &Path, hash: &Hash) -> Result<()> {
    let serialized = retrieve_object(hash).map_err(|e| {
        Error::new(ErrorKind::Other, format!("Failed to retrieve directory contents: {}", e))
    })?;
    
    let tree = Tree::deserialize(&serialized);
    println!("Processing directory: {} with {} nodes", path.display(), tree.nodes.len());
    
    for node in &tree.nodes {
        let node_path = path.join(&node.name);
        
        if node.is_dir {
            fs::create_dir_all(&node_path)?;
            process_directory(&node_path, &node.hash)?;
        } else {
            if let Some(parent) = node_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let contents = retrieve_object(&node.hash)
                .map_err(|e| Error::new(ErrorKind::Other, 
                    format!("Failed to retrieve file contents: {}", e)))?;
            
            fs::write(&node_path, contents.as_bytes())?;
            println!("Written file in directory: {}", node_path.display());
        }
    }
    
    Ok(())
}

fn delete_cwd(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        // Skip .geet directory and hidden files
        if path.starts_with(GEET_DIR) || path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("."))
            .unwrap_or(false) {
            continue;
        }
        
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    
    Ok(())
}

fn store_file(path: &String) -> Hash {
    let data = fs::read_to_string(path).unwrap();
    store_object(&data).unwrap()
}

fn strip_path(path: &PathBuf) -> String {
    path.to_str()
        .map(|s| s.trim_start_matches("./").to_string())
        .unwrap_or_default()
}
