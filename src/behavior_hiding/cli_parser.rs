/*B.2.1 CLI Parser (Angie)*/

use clap::{Parser, Subcommand};

use crate::behavior_hiding::file_system_commands::{FileSystemCommands, RepositoryCommand};
use crate::behavior_hiding::output_formatting::{FormatStyle, OutputFormatter};
use crate::behavior_hiding::status_command::{
    RepoOptions, RepositoryCommands, RevisionAction, RevisionOptions,
};
use crate::repo_hiding::operation::branch::checkout_commit;

mod cmd {
    //This needs to be replaced with the actual init command
    pub mod init {
        pub fn main() {
            println!("Init command executed");
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    command: Option<DVCSCommands>,
}

#[derive(Subcommand)]
pub enum DVCSCommands {
    Init {
        name: String,
        #[arg(short, long, default_value = ".")]
        path: String,
        #[arg(long, default_value = "main")]
        default_branch: String,
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
    Cleanup {},
}

pub enum CommandError {
    InvalidCommand(String),
    ParseError(String),
}

impl CLI {
    /// Parses the command-line input and returns a DVCS command or an error.
    pub fn parse_command(input: &[String]) -> Result<DVCSCommands, CommandError> {
        let matches = CLI::try_parse_from(input);

        match matches {
            Ok(cli) => match cli.command {
                Some(cmd) => Ok(cmd),
                None => Err(CommandError::InvalidCommand(
                    "No command provided.".to_string(),
                )),
            },
            Err(err) => {
                if err.kind() == clap::error::ErrorKind::UnknownArgument
                    || err.kind() == clap::error::ErrorKind::InvalidSubcommand
                {
                    Err(CommandError::InvalidCommand(err.to_string()))
                } else {
                    Err(CommandError::ParseError(err.to_string()))
                }
            }
        }
    }

    pub fn run() {
        let input: Vec<String> = std::env::args().collect(); // Collect command line arguments
        let formatter = OutputFormatter::new(FormatStyle::Colored); // Instantiate OutputFormatter

        match CLI::parse_command(&input) {
            Ok(command) => {
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

                match command {
                    DVCSCommands::Init {
                        name,
                        path,
                        default_branch,
                    } => {
                        let result = fs_commands.repository_calls(RepositoryCommand::Init {
                            name,
                            path,
                            default_branch,
                        });
                        match result {
                            Ok(_) => formatter.display_command_execution_status(true, "Init"),
                            Err(e) => formatter
                                .display_syntax_error(&format!("Error executing command: {:?}", e)),
                        }
                    }
                    DVCSCommands::Commit { message, author } => {
                        let result = repo_commands.commit_action(&message, &author);
                        match result {
                            Ok(_) => formatter.display_command_execution_status(true, "Commit"),
                            Err(e) => formatter
                                .display_syntax_error(&format!("Error executing command: {:?}", e)),
                        }
                    }
                    // TODO: remove this before release, just a debug command
                    DVCSCommands::Cleanup {} => {
                        let result = cleanup_helper();
                    }
                    DVCSCommands::Checkout { hash } => {
                        let result = checkout_helper(&hash);
                        match result {
                            Ok(_) => formatter.display_command_execution_status(true, "Checkout"),
                            Err(e) => formatter
                                .display_syntax_error(&format!("Error executing command: {:?}", e)),
                        }
                    }
                }
            }
            Err(e) => match e {
                CommandError::InvalidCommand(msg) => formatter
                    .display_syntax_error(&format!("You entered an unrecognized command. {}", msg)),
                CommandError::ParseError(msg) => {
                    formatter.display_syntax_error(&format!("It's our fault. {}", msg))
                }
            },
        }
    }
}

fn cleanup_helper() {
    // This is a debug command to clean up the .geet directory
    let path = std::env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let path = format!("{}/.geet", path);
    let _ = std::fs::remove_dir_all(path);
}

fn checkout_helper(hash: &String) -> std::io::Result<()> {
    checkout_commit(hash);
    Ok(())
}
