mod branch_management;
mod data_type;
mod revision_management;

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
pub fn diff(file1: Vec<String>, file2: Vec<String>) -> Vec<Diff> {
    todo!()
}
