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

pub type Hash = String;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CommitMetadata {
    pub author: String,
    pub message: String,
    pub timestamp: i64,
}

/* This is a commit in the DVCS. */
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub tree_hash: Hash,           // Hash of the root tree the commit points to
    pub parent_hash: Option<Hash>, // Hash of the parent commit
    pub metadata: CommitMetadata,  // Commit metadata (author, message, timestamp)
}

impl Commit {
    pub fn new_commit(
        tree_hash: Hash,
        parent_hash: Option<Hash>,
        metadata: CommitMetadata,
    ) -> Self {
        Self {
            tree_hash,
            parent_hash,
            metadata,
        }
    }

    // TODO (Optional): change it back to serialize to vectcor
    pub fn serialize(&self) -> String {
        serde_json::to_string(self)
            .expect(format!("Failed to serialize commit {:#?}", self).as_str())
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data)
            .expect(format!("Failed to deserialize commit: {}", data).as_str())
    }
}

/*  represents a branch or tag reference (not needed in minimal prototype). */
#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    pub name: String,      // Branch or tag name
    pub commit_id: String, // Associated commit hash
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

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RefType {
    Branch,
    Tag,
    Head,
}

/* stores repository metadata (not needed in minimal prototype). */
#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub repo_name: String,      // Repo name
    pub default_branch: String, // default branch name
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

/*
Test Cases for Commit Serialization/Deserialization:
1. Create a new `Commit` instance and serialize it.
2. Deserialize the serialized data back into a `Commit` instance.
3. Ensure data consistency after serialization and deserialization.
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeNode {
    pub is_dir: bool, // Indicates if it's a blob or a tree
    pub hash: Hash,   // Hash of the node
    pub name: String, // Name of the node
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {
    pub nodes: Vec<TreeNode>, // List of nodes in the tree
}

impl Tree {
    /* Creates a new, empty `Tree` instance. */
    pub fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    /* Adds a directory node (Tree) to the `Tree`. */
    pub fn add_node(&mut self, name: String, hash: Hash, is_dir: bool) {
        let node = TreeNode { is_dir, hash, name };
        self.nodes.push(node);
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).unwrap()
    }
}

/// Custom serialization error type.
#[derive(Debug)]
pub enum SerializationError {
    SerdeJsonError(serde_json::Error),
    BincodeError(bincode::Error),
    InvalidHashLength,
    FileNotFound,
}

impl std::fmt::Display for SerializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            SerializationError::SerdeJsonError(msg) => write!(f, "Serde JSON Error: {}", msg),
            SerializationError::BincodeError(msg) => write!(f, "Bincode Error: {}", msg),
            SerializationError::InvalidHashLength => write!(f, "Invalid hash length provided."),
            SerializationError::FileNotFound => write!(f, "File not found for the given hash."),
        }
    }
}

impl Error for SerializationError {}
