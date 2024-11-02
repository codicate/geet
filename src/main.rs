mod behavior_hiding;
mod file_hiding;
mod repo_hiding;

use repo_hiding::operation::revision::{create_revision, get_revision};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  create - Creates a new revision with hardcoded metadata.");
        println!("  get <hash> - Retrieves a revision based on the given hash.");
        return;
    }

    match args[1].as_str() {
        "create" => {
            // Create a new revision with hardcoded metadata
            let metadata = repo_hiding::data_type::CommitMetadata {
                author: "Alice".to_string(),
                message: "Initial commit".to_string(),
                timestamp: 0,
            };

            let hash = create_revision(metadata);
            println!("Created revision with hash: {}", hash);
        }
        "get" => {
            // Ensure a hash is provided
            if args.len() < 3 {
                eprintln!("Error: Missing hash for `get` command.");
                println!("Usage: get <hash>");
                return;
            }

            let hash = &args[2];
            let revision = get_revision(hash);
            println!("Retrieved revision: {:?}", revision);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            println!("Usage:");
            println!("  create - Creates a new revision with hardcoded metadata.");
            println!("  get <hash> - Retrieves a revision based on the given hash.");
        }
    }
}
