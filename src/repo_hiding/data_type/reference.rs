use super::Hash;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RefType {
    Branch,
    Tag,
    Head,
}

/*  represents a branch or tag reference (not needed in minimal prototype). */
#[derive(Serialize, Deserialize, Debug)]
pub struct Ref {
    pub ref_type: RefType,         // Type of reference (branch or tag)
    pub name: String,              // Branch or tag name
    pub commit_hash: Option<Hash>, // Associated commit hash
}

impl Ref {
    // TODO (Optional): change it back to serialize to vectcor
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect(format!("Failed to serialize ref {:#?}", self).as_str())
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).expect(format!("Failed to deserialize ref: {}", data).as_str())
    }
}
