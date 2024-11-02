use serde::{Deserialize, Serialize};

pub type Hash = String;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct CommitMetadata {
    pub author: String,
    pub message: String,
    pub timestamp: i64,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub metadata: CommitMetadata,
    pub hash: Hash,
    pub parent: Option<Hash>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RefType {
    Branch,
    Tag,
    Head,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Ref {
    pub kind: RefType,
    pub name: String,
    pub hash: Hash,
}
