use serde::{Deserialize, Serialize};
use serde_json;

/* stores repository metadata (not needed in minimal prototype). */
#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfig {
    pub name: String,           // Repo name
    pub default_branch: String, // default branch name
}

impl RepositoryConfig {
    pub fn serialize(&self) -> String {
        serde_json::to_string(self)
            .expect(format!("Failed to serialize repository config {:#?}", self).as_str())
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data)
            .expect(format!("Failed to deserialize repository config: {}", data).as_str())
    }
}
