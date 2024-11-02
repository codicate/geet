// user_data.rs
// generate and verify SHA-1 hashes to ensure file content integrity.

use std::fs::{File};
use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct UserDataManager;

impl UserDataManager {
    /* creates a new `UserDataManager`. */
    pub fn new() -> Self {
        Self
    }

    /* stores configuration data in plain text (non-sensitive data). */
    pub fn store_config(&self, path: &str, data: &[u8]) -> Result<(), FileSystemError> {
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }

    /* retrieves configuration data. */
    pub fn retrieve_config(&self, path: &str) -> Result<Vec<u8>, FileSystemError> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /* stores sensitive data by encrypting it before writing to a file (not needed in minimal prototype).*/
    pub fn store_sensitive_data(&self, path: &str, data: &[u8]) -> Result<(), FileSystemError> {
        // TODO: write encryption for sensitive data storage.
        todo!()
    }

    /* retrieves and decrypts sensitive data from a file (not needed in minimal prototype). */
    pub fn retrieve_sensitive_data(&self, path: &str) -> Result<Vec<u8>, FileSystemError> {
        // TODO: write decryption for retrieving sensitive data.
        todo!()
    }
}

#[derive(Debug)]
pub enum FileSystemError {
    IoError(io::Error),
    EncryptionError,
    DecryptionError,
}

impl From<io::Error> for FileSystemError {
    fn from(error: io::Error) -> Self {
        FileSystemError::IoError(error)
    }
}

/*
SPMP - Week 1
1. new() -> Self
2. store_config(&self, path: &str, data: &[u8]) -> Result<(), FileSystemError>
3. retrieve_config(&self, path: &str) -> Result<Vec<u8>, FileSystemError>

for recording commit metadata

*/

/*
TODOs for Future Extensions:
1. Replace the hardcoded nonce in `store_sensitive_data` with a dynamically generated one and store it with the ciphertext.
2. Implement test cases for:
   - Storing and retrieving sensitive data with encryption and decryption.
   - Handling errors, such as incorrect encryption key or missing files.
   - Verifying functionality of storing and retrieving configuration data.
3. Add error handling for missing or corrupted files in `retrieve_sensitive_data`.
*/
