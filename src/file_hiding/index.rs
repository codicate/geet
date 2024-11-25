use crate::{BASE_DIR, GEET_DIR, INDEX_FILE, OBJECTS_DIR};
use core::hash;
use serde_json::{self, Value};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::file_log::{does_object_exist, hash_object};

fn read_index() -> std::io::Result<HashSet<PathBuf>> {
    match File::open(INDEX_FILE) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            Ok(serde_json::from_str(&content).unwrap_or_default())
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Create parent directory if it doesn't exist
            if let Some(parent) = Path::new(INDEX_FILE).parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Create and initialize the file with an empty array
            let mut file = File::create(INDEX_FILE)?;
            file.write_all(b"[]")?;
            Ok(HashSet::new())
        },
        Err(e) => Err(e),
    }
}

fn write_index(paths: &HashSet<PathBuf>) -> std::io::Result<()> {
    let serialized = serde_json::to_string(paths)?;
    let mut file = File::create(INDEX_FILE)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn clear_index() -> std::io::Result<()> {
    let index = HashSet::new();
    write_index(&index)
}

fn is_file_changed(path: &Path) -> bool {
    let mut file = File::open(path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let hash = hash_object(&content);
    return !does_object_exist(&hash);
}

fn get_files_recursively(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    // Skip the geet directory
    if path.starts_with(GEET_DIR) {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            files.extend(get_files_recursively(&entry.path())?);
        }
    }

    if path.is_file() && is_file_changed(path) {
        files.push(path.to_path_buf());
    }

    Ok(files)
}

pub fn add(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err("File or directory not found".to_string());
    }

    // Use error handling instead of unwrap
    let mut index = match read_index() {
        Ok(index) => index,
        Err(e) => return Err(format!("Failed to read index: {}", e))
    };
    
    let files = match get_files_recursively(path) {
        Ok(files) => files,
        Err(e) => return Err(format!("Failed to process files: {}", e))
    };
    
    index.extend(files);

    match write_index(&index) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write index: {}", e))
    }
}

pub fn remove(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err("File or directory not found".to_string());
    }

    let mut index = read_index().unwrap();
    let files = get_files_recursively(path).unwrap();
    for file in files {
        index.remove(&file);
    }

    write_index(&index).unwrap();
    Ok(())
}

// return true if file is not changed, or if the file is changed and staged
pub fn contains(path: &Path) -> bool {
    if !is_file_changed(path) {
        return true;
    }

    let index = read_index().unwrap();
    index.contains(&path.to_path_buf())
}

pub fn is_stage_empty() -> bool {
    let index = read_index().unwrap();
    index.is_empty()
}

pub fn get_staged_files() -> Vec<PathBuf> {
    read_index().unwrap().into_iter().collect()
}

pub fn get_unstaged_files() -> Vec<PathBuf> {
    let staged_files = get_staged_files();
    let path = Path::new(BASE_DIR);
    get_files_recursively(path)
        .unwrap()
        .into_iter()
        .filter(|f| {
            !f.to_string_lossy().contains("/.geet/") && 
            !staged_files.contains(f)
        })
        .collect()
}
