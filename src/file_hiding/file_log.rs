// file_hiding/file_log.rs
use super::index;
use crate::OBJECTS_DIR;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{self, Read, Result, Write};
use std::path::Path;

pub fn hash_object(data: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

pub fn store_object(data: &String) -> Result<String> {
    // Generate SHA-1 hash
    let hash_string = hash_object(data);

    // Write data to a file named with its hash
    let file_path = format!("{}/{}", OBJECTS_DIR, hash_string);
    let mut file = File::create(&file_path)?;
    file.write_all(data.as_bytes())?;

    Ok(hash_string)
}

pub fn retrieve_object(hash: &String) -> Result<String> {
    if hash.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Invalid hash provided",
        ));
    }

    let file_path = format!("{}/{}", OBJECTS_DIR, hash);

    if !Path::new(&file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let mut file = File::open(&file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

// Store a file and add it to the index
// pub fn store_file(path: &str) -> Result<String> {
//     let data = fs::read_to_string(path)?;
//     let hash = store_object(&data)?;
//     index::add(path).unwrap();
//     Ok(hash)
// }

// Get all staged files and their contents
// pub fn get_staged_contents() -> Result<Vec<(String, String)>> {
//     let mut contents = Vec::new();
//     for path in get_staged_files() {
//         let data = fs::read_to_string(&path)?;
//         contents.push((path, data));
//     }
//     Ok(contents)
// }

// Keep existing functions...
pub fn delete_data(path: &str) -> Result<()> {
    todo!()
}

pub fn list_files(directory: &str) -> Result<Vec<String>> {
    todo!()
}

pub fn serialize_metadata<T: Serialize>(metadata: &T) -> Result<Vec<u8>> {
    todo!()
}

/*
deserializes metadata for additional information about file versions.
*/
pub fn deserialize_metadata<T: DeserializeOwned>(data: &[u8]) -> Result<T> {
    // TODO: Implement metadata deserialization functionality
    todo!()
}

// TODO
trait DeserializeOwned: for<'de> Deserialize<'de> {}
// type DeserializationError = SerializationError;

// 4 new methods
// Stores a serializable object and returns its SHA-1 hash.
// create store_object retrieve_object to settle the path once for all, so the store_data and retrieve data would be simpler instead of calling path

// redesign retrieve_data to use the first two characters of the hash from the store data instead of the path.

/*
Test cases:
1. store/retrieve data
2. list files (to be implemented)
3. serialize/deserialize metadata (to be implemented)
*/

/*
SPMP - week 1:
store_data method: stores file data by generating an SHA-1 hash and saving the data under a hash-named file.
relavent commands: add and commit commands, as it allows storing snapshots of file data.

retrieve_data method: reads and returns data from a specified file path.
relavent commands: checkout and cat commands, as it allows retrieving stored file versions.

*/

// TODO: Implement delete_data function for file deletion functionality.
// TODO: Implement list_files function to list files in a directory.
// TODO: Implement serialize_metadata and deserialize_metadata for handling file version metadata.
// TODO: Unify error handling by incorporating SerializationError into FileSystemError more thoroughly if needed.
// TODO: Extend `get_file_path` to handle flexible directory structures if the hashing method changes.
// TODO: Add an integrity check function, possibly using the HasherChecker module, to verify stored files against their hash.
