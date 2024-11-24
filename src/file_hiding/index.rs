use crate::{INDEX_FILE, OBJECTS_DIR};
use core::hash;
use serde_json::{self, Value};
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::file_log::hash_object;

fn read_index() -> std::io::Result<HashSet<PathBuf>> {
    let mut file = File::open(INDEX_FILE)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let paths: HashSet<PathBuf> = serde_json::from_str(&content).unwrap_or_default();
    Ok(paths)
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
    let object_path = format!("{}/{}", OBJECTS_DIR, hash);
    let path = Path::new(path);
    return !path.exists();
}

fn get_files_recursively(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            files.extend(get_files_recursively(&entry.path())?);
        }
    }

    if !is_file_changed(path) {
        files.push(path.to_path_buf());
    }

    Ok(files)
}

pub fn add(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    if !path.exists() {
        return Err("File or directory not found".to_string());
    }

    let mut index = read_index().unwrap();
    let files = get_files_recursively(path).unwrap();
    index.extend(files);

    write_index(&index).unwrap();
    Ok(())
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

pub fn contains(path: &Path) -> bool {
    let index = read_index().unwrap();
    index.contains(&path.to_path_buf())
}

pub fn get_staged_files() -> Vec<String> {
    let index = read_index().unwrap();
    index
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}
