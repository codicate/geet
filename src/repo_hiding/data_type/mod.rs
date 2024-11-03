// application_data.rs
// define the structure of commits and other repository-related metadata.
/*
add the following [dependencies] to cargo.toml:
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
*/

pub type Hash = String;
mod commit;
pub use commit::{Commit, CommitMetadata};
mod tree;
pub use tree::Tree;
mod reference;
pub use reference::{Ref, RefType};
mod repo;
pub use repo::RepoConfig;

/*
Test Cases for Commit Serialization/Deserialization:
1. Create a new `Commit` instance and serialize it.
2. Deserialize the serialized data back into a `Commit` instance.
3. Ensure data consistency after serialization and deserialization.
*/

// /// Custom serialization error type.
// #[derive(Debug)]
// pub enum SerializationError {
//     SerdeJsonError(serde_json::Error),
//     BincodeError(bincode::Error),
//     InvalidHashLength,
//     FileNotFound,
// }

// impl std::fmt::Display for SerializationError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match &self {
//             SerializationError::SerdeJsonError(msg) => write!(f, "Serde JSON Error: {}", msg),
//             SerializationError::BincodeError(msg) => write!(f, "Bincode Error: {}", msg),
//             SerializationError::InvalidHashLength => write!(f, "Invalid hash length provided."),
//             SerializationError::FileNotFound => write!(f, "File not found for the given hash."),
//         }
//     }
// }

// impl Error for SerializationError {}
