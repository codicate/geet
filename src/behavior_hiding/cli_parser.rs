/*B.2.1 CLI Parser (Angie)*/

use super::command_handler;
use crate::{
    behavior_hiding::output_formatting::{FormatStyle, OutputFormatter},
    GEET_DIR,
};
use clap::{Parser, Subcommand};
use std::{default, fmt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create an empty Git repository or reinitialize an existing one
    Init {},

    /// Clone a repository from a remote location to a local path
    Clone {
        /// Path to the remote repository to clone
        remote_path: String,
    },

    /// Pull changes from the remote repository and merge them into the current branch
    Pull {
        /// Path to the remote repository to pull from
        remote_path: String,
    },

    /// Push local changes to the remote repository
    Push {
        /// Path to the remote repository to push to
        remote_path: String,
    },

    /// Add a file or files to the staging area
    Add {
        /// Path to the file to add
        file_path: String,
    },

    /// Remove a file from the staging area or the repository
    Remove {
        /// Path to the file to remove
        file_path: String,
    },

    /// Show all open branch heads in the repository
    Heads {},

    /// Display the working tree and staging area status
    Status {},

    /// Show the commit logs of the repository
    Log {},

    /// Show the differences between two commits or a commit and the working directory
    Diff {
        /// Hash of the first commit
        hash1: String,
        /// Hash of the second commit
        hash2: String,
    },

    /// Display the contents of a file at a specific commit
    Cat {
        /// Path to the file to display
        file_path: String,
    },

    /// Create a new commit with a message and author
    Commit {
        /// Commit message to include in the commit
        #[arg(short, long)]
        message: String,

        /// Author of the commit; defaults to "Anonymous"
        #[arg(short, long, default_value = "Anonymous")]
        author: String,
    },

    /// Switch branches or restore working tree files
    Checkout {
        /// Branch name or commit hash to check out
        str: String,

        /// Flag to create a new branch named <BRANCHNAME>
        #[arg(short, help = "Create a new branch named <BRANCHNAME>")]
        branch: bool,
    },

    /// Merge the changes from another branch into the current branch
    Merge { from: String },

    /// Clean up unnecessary files and optimize the repository TODO: remove from production
    Cleanup {},
}

pub fn parse_input() {
    let cli = match CLI::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            e.print();
            return;
        }
    };

    let formatter = OutputFormatter::new(FormatStyle::Colored);

    if cli.command.is_none() {
        println!("No command provided. Use -h to see usage.");
        return;
    }

    let command = cli.command.unwrap();
    let command_str = format!("{:?}", command);
    let result = execute_command(&command);

    if result.is_err() {
        formatter.display_syntax_error(&format!("Error executing command: {}", command_str));
        println!("Error: {}", result.unwrap_err());
    };
}

pub fn execute_command(command: &Commands) -> Result<(), String> {
    let formatter = OutputFormatter::new(FormatStyle::Colored);

    match command {
        Commands::Init {} => command_handler::init(),
        Commands::Clone { remote_path } => command_handler::clone(remote_path),
        Commands::Pull { remote_path } => command_handler::pull(remote_path),
        Commands::Push { remote_path } => command_handler::push(remote_path),
        Commands::Add { file_path } => command_handler::add(file_path),
        Commands::Remove { file_path } => command_handler::remove(file_path),
        Commands::Heads {} => command_handler::heads(),
        Commands::Status {} => command_handler::status(),
        Commands::Log {} => command_handler::log(),
        Commands::Diff { hash1, hash2 } => command_handler::diff(hash1, hash2),
        Commands::Cat { file_path } => command_handler::cat(file_path),
        Commands::Commit { message, author } => command_handler::commit(message, author),
        Commands::Checkout { str, branch } => command_handler::checkout(str, branch),
        Commands::Merge { from } => command_handler::merge(from),
        Commands::Cleanup {} => cleanup_helper(),
    }
}

// This is a debug command to clean up the .geet directory TODO: remove from production
fn cleanup_helper() -> Result<(), String> {
    std::fs::remove_dir_all(GEET_DIR);
    Ok(())
}
