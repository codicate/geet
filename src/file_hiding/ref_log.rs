use crate::REFS_DIR;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;


pub type Hash = String;

pub fn store_ref(name: &String, data: &String) -> io::Result<()> {
    // Ensure the refs directory exists
    fs::create_dir_all(REFS_DIR)?;

    // Construct the path to the reference file using PathBuf
    let path = PathBuf::from(REFS_DIR).join(name);

    // Open the file and write the serialized reference data
    let mut file = File::create(&path)?;
    file.write_all(data.as_bytes())?;

    Ok(())
}
/*
/// Creates a new Ref object and writes it to `./geet/refs`.
pub fn store_ref(name: &String, data: &String) -> io::Result<()> {
    // Create all necessary parent directories
    fs::create_dir_all(REFS_DIR)?;
    let path = format!("{}\\{}", REFS_DIR, name);
    let mut file = File::create(&path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}*/

/// Retrieves a Ref object from `./geet/refs`.
pub fn retrieve_ref(name: &str) -> io::Result<String> {
    let path = PathBuf::from(REFS_DIR).join(name);
   // let path = format!("{}\\{}", REFS_DIR, name);
    let mut file = File::open(&path)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

/// Deletes a Ref object from `./geet/refs`.
pub fn delete_ref(name: &str) -> io::Result<()> {
    let path = PathBuf::from(REFS_DIR).join(name);
    //let path = format!("{}\\{}", REFS_DIR, name);
    std::fs::remove_file(&path)?;
    Ok(())
}
