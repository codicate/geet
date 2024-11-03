use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub type Hash = String;
const REFS_DIR: &str = "./.geet/refs";
/// Creates a new Ref object and writes it to `./geet/refs`.
pub fn create_ref(name: &str, commit_id: &Hash) -> io::Result<()> {
    let path = format!("{}/{}", REFS_DIR, name);
    let mut file = File::create(&path)?;
    file.write_all(commit_id.as_bytes())?;
    Ok(())
}

/// Retrieves a Ref object from `./geet/refs`.
pub fn get_ref(name: &str) -> io::Result<Option<Hash>> {
    let path = format!("{}/{}", REFS_DIR, name);
    let mut file = File::open(&path)?;

    let mut commit_id = String::new();
    file.read_to_string(&mut commit_id)?;

    let commit_hash = commit_id.trim().to_string();
    if commit_hash.len() > 0 {
        Ok(Some(commit_hash))
    } else {
        Ok(None)
    }
}
