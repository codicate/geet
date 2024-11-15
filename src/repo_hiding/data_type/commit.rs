use super::Hash;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CommitMetadata {
    pub author: String,
    pub message: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
