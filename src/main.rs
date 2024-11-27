mod behavior_hiding;
mod file_hiding;
mod repo_hiding;

use behavior_hiding::cli_parser::parse_input;
use std::{env, path::Path};

const BASE_DIR: &str = ".";
const GEET_DIR: &str = ".\\.geet";
const REFS_DIR: &str = ".\\.geet\\refs";
const OBJECTS_DIR: &str = ".\\.geet\\objects";
const INDEX_FILE: &str = ".\\.geet\\index";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    parse_input();
}
