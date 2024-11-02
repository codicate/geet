// file_log.rs
// store, retrieve, and manage file versions in the repository.
/*
add the following [dependencies] to cargo.toml:
[dependencies] 
sha1 = "0.6.0"
serde = { version = "1.0", features = ["derive"] }
*/

use std::fs::{self, File};
use std::io::{self, Read, Write};
use sha1::{Digest, Sha1};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct FileLog;

impl FileLog {
    /* 
    stores file data at a specified path, 
    generates an SHA-1 hash for the content, 
    and saves it to a hash-named file or directory.
    */
    pub fn store_data(path: &str, data: &[u8]) -> Result<String, FileSystemError> {
        // generate SHA-1 hash
        let mut hasher = Sha1::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let hash_string = format!("{:x}", hash);

        // directory structure based on the first two characters of the hash
        let dir_path = format!("{}/{}", path, &hash_string[..2]);
        fs::create_dir_all(&dir_path)?;

        // write data to a file named with its hash
        let file_path = format!("{}/{}", dir_path, hash_string);
        let mut file = File::create(&file_path)?;
        file.write_all(data)?;

        Ok(hash_string)
    }

    /* reads and returns data from a specified file path. */
    pub fn retrieve_data(path: &str) -> Result<Vec<u8>, FileSystemError> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        Ok(buffer)
    }

    /* deletes a file at a specified path. */
    pub fn delete_data(path: &str) -> Result<(), FileSystemError> {
        // TODO: Implement file deletion functionality
        todo!()
    }

    /* lists all files within a directory, filtering for files only. */
    pub fn list_files(directory: &str) -> Result<Vec<String>, FileSystemError> {
        // TODO: Implement directory file listing functionality
        todo!()
    }

    /* 
    serializes metadata for additional information about file versions.
    */
    pub fn serialize_metadata<T: Serialize>(metadata: &T) -> Result<Vec<u8>, SerializationError> {
        // TODO: Implement metadata serialization functionality
        todo!()
    }

    /* 
    deserializes metadata for additional information about file versions.
    */
    pub fn deserialize_metadata<T: DeserializeOwned>(data: &[u8]) -> Result<T, DeserializationError> {
        // TODO: Implement metadata deserialization functionality
        todo!()
    }
}

#[derive(Debug)]
pub enum FileSystemError {
    IoError(io::Error),
    SerializationError(bincode::Error),
}

impl From<io::Error> for FileSystemError {
    fn from(error: io::Error) -> Self {
        FileSystemError::IoError(error)
    }
}

impl From<bincode::Error> for FileSystemError {
    fn from(error: bincode::Error) -> Self {
        FileSystemError::SerializationError(error)
    }
}

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
