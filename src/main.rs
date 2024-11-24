mod behavior_hiding;
mod file_hiding;
mod repo_hiding;

use behavior_hiding::cli_parser::parse_input;
use std::env;

const BASE_DIR: &str = "./test";
const GEET_DIR: &str = "./test/.geet";
const REFS_DIR: &str = "./test/.geet/refs";
const OBJECTS_DIR: &str = "./test/.geet/objects";
const INDEX_FILE: &str = "./test/.geet/index";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    parse_input();
}
