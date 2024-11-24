/*B.2.1 CLI Parser (Angie)*/

use crate::behavior_hiding::file_system_commands::{FileSystemCommands, RepositoryCommand};
use crate::behavior_hiding::output_formatting::{FormatStyle, OutputFormatter};
use crate::behavior_hiding::status_command::{
    RepoOptions, RepositoryCommands, RevisionAction, RevisionOptions,
};
use crate::repo_hiding::operation::branch::checkout_commit;
use clap::{Parser, Subcommand};
use std::fmt;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: Option<DVCSCommands>,
}

impl fmt::Display for DVCSCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DVCSCommands::Init { .. } => write!(f, "Init"),
            DVCSCommands::Add { .. } => write!(f, "Add"),
            DVCSCommands::Commit { .. } => write!(f, "Commit"),
            DVCSCommands::Checkout { .. } => write!(f, "Checkout"),
            DVCSCommands::Status { .. } => write!(f, "Status"),
            DVCSCommands::Cleanup { .. } => write!(f, "Cleanup"),
            DVCSCommands::Clone { .. } => write!(f, "Clone"),
        }
    }
}

#[derive(Subcommand)]
pub enum DVCSCommands {
    Init {
        name: String,
        #[arg(short, long, default_value = "./test")]
        path: String,
        #[arg(long, default_value = "main")]
        default_branch: String,
    },
    Add {
        path: String,
    },
    Commit {
        #[arg(short, long)]
        message: String,
        #[arg(short, long, default_value = "Anonymous")]
        author: String,
    },
    Checkout {
        hash: String,
    },
    Clone {
        #[arg(short, long, default_value = "./test_repo")]
        remote_path: String,
        #[arg(short, long, default_value = "./test")]
        local_path: String,
    },
    Status {},
    Cleanup {},
}

pub enum CommandError {
    InvalidCommand(String),
    ParseError(String),
}

impl CLI {
    /// Parses the command-line input and returns a DVCS command or an error.
    pub fn parse_command(input: &[String]) -> Result<DVCSCommands, CommandError> {
        let cli = CLI::try_parse_from(input).map_err(|err| {
            if err.kind() == clap::error::ErrorKind::UnknownArgument
                || err.kind() == clap::error::ErrorKind::InvalidSubcommand
            {
                CommandError::InvalidCommand(err.to_string())
            } else {
                CommandError::ParseError(err.to_string())
            }
        })?;
        cli.command.ok_or(CommandError::InvalidCommand(
            "No command provided.".to_string(),
        ))
    }

    pub fn run() {
        let input: Vec<String> = std::env::args().collect(); // Collect command line arguments
        let formatter = OutputFormatter::new(FormatStyle::Colored);

        let cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        //Dummy variables
        let repo_options = RepoOptions {
            path: Some(cwd),
            current_branch: Some("main".to_string()),
        };
        let revision_options = RevisionOptions::default();

        let fs_commands = FileSystemCommands {};
        let repo_commands = RepositoryCommands {
            repo_options,
            revision_options,
        };

        let command = match CLI::parse_command(&input) {
            Ok(cmd) => cmd,
            Err(e) => {
                match e {
                    CommandError::InvalidCommand(msg) => {
                        formatter.display_syntax_error(&format!("Invalid Command {}", msg))
                    }
                    CommandError::ParseError(msg) => {
                        formatter.display_syntax_error(&format!("Parse Error {}", msg))
                    }
                }
                std::process::exit(1);
            }
        };

        let result = match &command {
            DVCSCommands::Init {
                name,
                path,
                default_branch,
            } => fs_commands.init_repository(name.clone(), path.clone(), default_branch.clone()),

            DVCSCommands::Add { path } => fs_commands.add_file(&path),

            // DVCSCommands::Commit { message, author } => repo_commands.commit_action(&message, &author),
            DVCSCommands::Commit { message, author } => {
                repo_commands.commit_action(&message, &author)
                    .map(|_| ()) // Convert the Result<RevisionResult, String> to Result<(), String>
            },

            // DVCSCommands::Cleanup {} => cleanup_helper(),
            DVCSCommands::Cleanup {} => cleanup_helper()
                .map_err(|e| e.to_string()),

            DVCSCommands::Status {} => {
                let files = fs_commands.get_status().unwrap_or_default();
                status_helper(files, &formatter);
                Ok(())
            }

            DVCSCommands::Clone { remote_path, local_path } => {
                fs_commands.clone_repository(remote_path.to_string(), local_path.to_string())
            }
            
            DVCSCommands::Checkout { hash } => checkout_helper(&hash)
                .map_err(|e| e.to_string()),

            _ => Ok(()),
        };

        match result {
            Ok(_) => formatter.display_command_execution_status(true, &command.to_string()),
            Err(e) => formatter.display_syntax_error(&format!("Error executing command: {:?}", e)),
        }
    }
}

fn cleanup_helper() -> std::io::Result<()> {
    // This is a debug command to clean up the .geet directory
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let path = format!("{}/.geet", path);
    let _ = std::fs::remove_dir_all(path);
    Ok(())
}

fn checkout_helper(hash: &String) -> std::io::Result<()> {
    checkout_commit(hash);
    Ok(())
}

fn status_helper(files: Vec<String>, formatter: &OutputFormatter) {
    if files.is_empty() {
        formatter.display_program_result("No files staged for commit");
    } else {
        let mut status = String::from("Changes to be committed:\n");
        for file in files {
            status.push_str(&format!("  new file: {}\n", file));
        }
        formatter.display_program_result(&status);
    }
}
