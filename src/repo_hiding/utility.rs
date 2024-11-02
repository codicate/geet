use std::process::Command;

enum DiffType {
    Add,
    Remove,
    Modify,
}

struct Diff {
    diff_type: DiffType,
    line_number: i32,
    line: String,
}

// returns the list of differences between the two files
pub fn diff(path1: &String, path2: &String) -> String {
    let output = Command::new("diff")
        .arg("-u")
        .arg(path1)
        .arg(path2)
        .output()
        .expect("failed to execute diff command");

    String::from_utf8(output.stdout).unwrap()
}
