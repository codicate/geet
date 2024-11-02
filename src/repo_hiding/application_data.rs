// application_data.rs
// define the structure of commits and other repository-related metadata.
/*
add the following [dependencies] to cargo.toml:
[dependencies] 
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
*/

use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;

/* This is a commit in the DVCS. */
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub id: String,              // Commit hash (e.g., SHA-1)
    pub message: String,         // Commit message
    pub author: String,          // Author's name
    pub date: String,            // Timestamp
    pub parent: Option<String>,  // Hash of the parent commit
}

impl Commit {
    /* creates a new `Commit` instance with the specified fields. */
    pub fn new_commit(id: String, message: String, author: String, date: String, parent: Option<String>) -> Self {
        Commit {
            id,
            message,
            author,
            date,
            parent,
        }
    }

    /* serializes a `Commit` instance to a JSON byte vector. */
    pub fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        serde_json::to_vec(self).map_err(|e| SerializationError::SerdeError(e.to_string()))
    }

    /* deserializes a `Commit` instance from a JSON byte slice. */
    pub fn deserialize(data: &[u8]) -> Result<Self, SerializationError> {
        serde_json::from_slice(data).map_err(|e| SerializationError::SerdeError(e.to_string()))
    }
}


/*  represents a branch or tag reference (not needed in minimal prototype). */
#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    pub name: String,       // Branch or tag name
    pub commit_id: String,  // Associated commit hash
}

impl Ref {
    /*  serializes a `Ref` instance. */
    pub fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        // TODO: Implement serialization for Ref
        todo!()
    }

    /* deserializes a `Ref` instance from data. */
    pub fn deserialize(data: &[u8]) -> Result<Self, SerializationError> {
        // TODO: Implement deserialization for Ref
        todo!()
    }
}

/* stores repository metadata (not needed in minimal prototype). */
#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub repo_name: String,       // Repo name
    pub default_branch: String,  // default branch name
}

impl RepositoryConfig {
    /*  serializes a `RepositoryConfig` instance. */
    pub fn serialize(&self) -> Result<Vec<u8>, SerializationError> {
        // TODO: Implement serialization for RepositoryConfig
        todo!()
    }

    /*  deserializes a `RepositoryConfig` instance from data. */
    pub fn deserialize(data: &[u8]) -> Result<Self, SerializationError> {
        // TODO: Implement deserialization for RepositoryConfig
        todo!()
    }
}

/* errors related to serialization and deserialization. */
#[derive(Debug)]
pub enum SerializationError {
    SerdeError(String),
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            SerializationError::SerdeError(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl Error for SerializationError {}

/*
Test Cases for Commit Serialization/Deserialization:
1. Create a new `Commit` instance and serialize it.
2. Deserialize the serialized data back into a `Commit` instance.
3. Ensure data consistency after serialization and deserialization.
*/

