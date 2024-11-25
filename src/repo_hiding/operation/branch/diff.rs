use crate::file_hiding::file_log::retrieve_object;
use crate::file_hiding::ref_log::Hash;
use crate::repo_hiding::data_type::{Commit, Tree};
use crate::{BASE_DIR, OBJECTS_DIR};
use std::{collections::HashMap, path::PathBuf};

pub struct Diff {
    pub deleted_files: Vec<String>,
    pub modified_files: Vec<(String, String, String)>,
    pub new_files: Vec<String>,
}

pub fn get_diffs(hash1: &String, hash2: &String) -> Diff {
    let file_list1 = get_file_list(hash1).unwrap();
    let file_list2 = get_file_list(hash2).unwrap();

    let mut deleted_files: Vec<String> = Vec::new();
    let mut modified_files: Vec<(String, String, String)> = Vec::new();
    let mut new_files: Vec<String> = Vec::new();

    for (path, cur_hash) in &file_list1 {
        if file_list2.contains_key(path) {
            let parent_hash = file_list2.get(path).unwrap();
            if parent_hash != cur_hash {
                modified_files.push((
                    path.to_string(),
                    parent_hash.to_string(),
                    cur_hash.to_string(),
                ));
            }
        } else {
            new_files.push(path.to_string());
        }
    }

    for (path, _) in &file_list2 {
        if !file_list1.contains_key(path) {
            deleted_files.push(path.to_string());
        }
    }

    return Diff {
        deleted_files,
        modified_files,
        new_files,
    };
}

fn get_file_list(hash: &Hash) -> Result<HashMap<String, String>, String> {
    let serialized =
        retrieve_object(&hash).map_err(|_| format!("commit with hash {} not found", hash))?;
    let commit = Commit::deserialize(&serialized);
    let mut file_list: HashMap<String, String> = HashMap::new();
    get_file_list_helper(BASE_DIR, &commit.tree_hash, &mut file_list);
    Ok(file_list)
}

fn get_file_list_helper(
    dir: &str,
    tree_hash: &String,
    file_list: &mut HashMap<String, String>,
) -> Result<(), String> {
    let serialized = retrieve_object(tree_hash).map_err(|_| "Object with given hash not found")?;
    let tree = Tree::deserialize(&serialized);

    for node in tree.nodes {
        let path = PathBuf::from(dir)
            .join(node.name)
            .to_str()
            .unwrap()
            .to_string();

        if node.is_dir {
            get_file_list_helper(&path, &node.hash, file_list);
        } else {
            file_list.insert(path, node.hash);
        }
    }

    Ok(())
}
