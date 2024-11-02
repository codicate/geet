mod behavior_hiding;
mod file_hiding;
mod repo_hiding;

fn main() {
    let metadata = repo_hiding::application_data::CommitMetadata {
        author: "Alice".to_string(),
        message: "Initial commit".to_string(),
        timestamp: 0,
    };

    repo_hiding::operation::revision::create_revision(metadata);
}
