use std::fs::File;
use std::io::{self, Read, Write};

pub type Hash = String;
const REFS_DIR: &str = "./.geet/refs";
/// Creates a new Ref object and writes it to `./geet/refs`.
pub fn store_ref(name: &String, data: &String) -> io::Result<()> {
    let path = format!("{}/{}", REFS_DIR, name);
    let mut file = File::create(&path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Retrieves a Ref object from `./geet/refs`.
pub fn retrieve_ref(name: &str) -> io::Result<String> {
    let path = format!("{}/{}", REFS_DIR, name);
    let mut file = File::open(&path)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

/// Deletes a Ref object from `./geet/refs`.
pub fn delete_ref(name: &str) -> io::Result<()> {
    let path = format!("{}/{}", REFS_DIR, name);
    std::fs::remove_file(&path)?;
    Ok(())
}
