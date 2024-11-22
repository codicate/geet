// src/file_hiding/index.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Result};
use std::path::Path;

const INDEX_PATH: &str = "./test/.geet/index";

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexEntry {
    pub path: String,
    pub staged: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Index {
    entries: HashMap<String, IndexEntry>,
}

impl Index {
    pub fn new() -> Self {
        if let Ok(data) = fs::read_to_string(INDEX_PATH) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Index::default()
        }
    }

    fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(self)?;
        if let Some(parent) = Path::new(INDEX_PATH).parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(INDEX_PATH, serialized)?;
        Ok(())
    }

    pub fn add_entry(&mut self, path: &str) -> Result<()> {
        let entry = IndexEntry {
            path: path.to_string(),
            staged: true,
        };

        self.entries.insert(path.to_string(), entry);
        self.save()?;
        Ok(())
    }

    pub fn get_staged_entries(&self) -> Vec<String> {
        self.entries
            .values()
            .filter(|entry| entry.staged)
            .map(|entry| entry.path.clone())
            .collect()
    }

    // method to clear the index after commit
    pub fn clear(&mut self) -> Result<()> {
        self.entries.clear();
        self.save()?;
        Ok(())
    }

    pub fn is_in_index(&self, path: &str) -> bool {
        self.entries.contains_key(path)
    }
}

pub fn add_to_index(path: &str) -> Result<()> {
    let mut index = Index::new();
    index.add_entry(path)
}

pub fn get_staged_files() -> Vec<String> {
    let index = Index::new();
    index.get_staged_entries()
}

// New function to clear the index
pub fn clear_index() -> Result<()> {
    let mut index = Index::new();
    index.clear()
}
