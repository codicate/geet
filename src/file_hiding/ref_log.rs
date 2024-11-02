use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub type Hash = String;

/// Creates a new Ref object and writes it to `./geet/refs`.
pub fn create_ref(name: &str, commit_id: &Hash) -> io::Result<()> {
    let path = format!("./geet/refs/{}", name);
    let mut file = File::create(&path)?;
    file.write_all(commit_id.as_bytes())?;
    Ok(())
}

/// Retrieves a Ref object from `./geet/refs`.
pub fn get_ref(name: &str) -> io::Result<Option<Hash>> {
    let path = format!("./geet/refs/{}", name);
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

pub fn update_ref(name: &str, new_commit_id: &Hash) -> io::Result<()> {
    let path = format!("./geet/refs/{}", name);

    println!("Updating ref: {} to {}", path, new_commit_id);
    // Check if the ref file exists
    if Path::new(&path).exists() {
        // Open the file in write mode and overwrite with the new commit hash
        let mut file = File::create(&path)?;
        file.write_all(new_commit_id.as_bytes())?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Ref '{}' does not exist.", name),
        ))
    }
}
