mod behavior_hiding;
mod file_hiding;
mod repo_hiding;

use repo_hiding::operation::revision::{create_revision, get_revision};

fn main() {
    let metadata = repo_hiding::data_type::CommitMetadata {
        author: "Alice".to_string(),
        message: "Initial commit".to_string(),
        timestamp: 0,
    };

    let hash = create_revision(metadata);
    println!("Created revision with hash: {}", hash);

    let revision = get_revision(&hash);
    println!("Retrieved revision: {:?}", revision);
}
