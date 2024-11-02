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
    pub id: Hash,                 // Commit hash (e.g., SHA-1)
    pub parent: Option<Hash>,     // Hash of the parent commit
    pub metadata: CommitMetadata, // Commit metadata (author, message, timestamp)
}

impl Commit {
    /* creates a new `Commit` instance with the specified fields. */
    pub fn new_commit(id: Hash, parent: Option<Hash>, metadata: CommitMetadata) -> Self {
        Commit {
            id,
            parent,
            metadata,
        }
    }

    // TODO (Optional): change it back to serialize to vectcor
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).unwrap()
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

/* Enum to define the type of content in the `TreeNode`. */
#[derive(Serialize, Deserialize, Debug)]
pub enum ContentType {
    File,
    Dir,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TreeNode {
    pub content_type: ContentType, // Indicates if it's a blob or a tree
    pub hash: Hash,                // Hash of the node
    pub name: String,              // Name of the node
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
    pub fn add_dir_node(&mut self, hash: Hash, name: String) {
        let node = TreeNode {
            content_type: ContentType::Dir,
            hash,
            name,
        };
        self.nodes.push(node);
    }

    /* Adds a file node (Blob) to the `Tree`. */
    pub fn add_file_node(&mut self, hash: Hash, name: String) {
        let node = TreeNode {
            content_type: ContentType::File,
            hash,
            name,
        };
        self.nodes.push(node);
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).unwrap()
    }
}
