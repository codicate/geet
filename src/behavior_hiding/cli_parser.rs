use clap::{Parser, Subcommand};

mod cmd {
    pub mod init {
        pub fn main() {
            println!("Init command executed");
        }
    }
    pub mod commit {
        pub fn main(message: &str) {
            println!("Commit command executed with message: {}", message);
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
    Init {},
    Commit {
        #[arg(short, required = true)]
        message: String,
    },
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
        match CLI::parse_command(&input) {
            Ok(command) => match command {
                DVCSCommands::Init {} => {
                    println!("Init command called");
                }
                DVCSCommands::Commit { message } => {
                    println!("Commit command called with message: {}", message);
                }
            },
            Err(e) => match e {
                CommandError::InvalidCommand(msg) => eprintln!("Error: {}", msg),
                CommandError::ParseError(msg) => eprintln!("Parse Error: {}", msg),
            },
        }
    }
}
