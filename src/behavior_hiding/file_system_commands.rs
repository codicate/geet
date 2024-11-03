/*  C.2.2.1 File Systems Commands (Angie) */

use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct FileSystemCommands{
}

pub enum RepositoryCommand{
    Init{
        path: String,
        default_branch: Option<String>,
    }
}
#[derive(Debug)]
pub enum CommandError{
    InvalidPath,
    ProcessFailed(String),
}

impl FileSystemCommands {
      /// Repository stuff
      pub fn repository_calls(&self, command: RepositoryCommand) -> Result<(), CommandError> {
        match command{
            RepositoryCommand::Init {path, default_branch} => {
                 //this is where the actual init logic would be called
                // println!("Initializing repo at: {}", path);
                // if let Some(branch) = default_branch {
                //     println!("Default branch: {}", branch);
                // }
                Ok(())
            }
        }
    }
}