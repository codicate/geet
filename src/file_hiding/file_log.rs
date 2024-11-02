// file_log.rs
// store, retrieve, and manage file versions in the repository.
/*
add the following [dependencies] to cargo.toml:
[dependencies]
sha1 = "0.6.0"
serde = { version = "1.0", features = ["derive"] }
*/

use crate::repo_hiding::application_data::SerializationError;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::{self, Read, Result, Write};
use std::path::Path;

const BASE_PATH: &str = "./.geet/objects";

pub fn store_object(data: &String) -> Result<String> {
    // Convert data to JSON format
    let json_data = serde_json::to_string(data)?;

    // Generate SHA-1 hash
    let mut hasher = Sha1::new();
    hasher.update(json_data.as_bytes());
    let hash = hasher.finalize();
    let hash_string = format!("{:x}", hash);

    // Directory structure based on the first two characters of the hash
    let dir_path = format!("{}/{}", BASE_PATH, &hash_string[..2]);
    fs::create_dir_all(&dir_path)?;

    // Write data to a file named with its hash
    let file_path = format!("{}/{}", dir_path, hash_string);
    let mut file = File::create(&file_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(hash_string)
}

pub fn retrieve_object(hash: &String) -> Result<String> {
    // Ensure the hash is at least two characters long
    if hash.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Invalid hash provided",
        ));
    }

    // Directory structure based on the first two characters of the hash
    let dir_path = format!("{}/{}", BASE_PATH, &hash[..2]);
    let file_path = format!("{}/{}", dir_path, hash);

    // Check if the file exists
    if !Path::new(&file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    // Read data from the file
    let mut file = File::open(&file_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Convert JSON string back to data
    let json_data: String = serde_json::from_str(&data)?;

    Ok(json_data)
}

/* deletes a file at a specified path. */
pub fn delete_data(path: &str) -> Result<()> {
    // TODO: Implement file deletion functionality
    todo!()
}

/* lists all files within a directory, filtering for files only. */
pub fn list_files(directory: &str) -> Result<Vec<String>> {
    // TODO: Implement directory file listing functionality
    todo!()
}

/*
serializes metadata for additional information about file versions.
*/
pub fn serialize_metadata<T: Serialize>(metadata: &T) -> Result<Vec<u8>> {
    // TODO: Implement metadata serialization functionality
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
type DeserializationError = SerializationError;

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
