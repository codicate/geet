use crate::file_hiding::file_log::{retrieve_object, store_object};
use crate::file_hiding::index::Index;
use crate::repo_hiding::data_type::{Hash, Tree};
use std::fs;
use std::io::{Result, Write};
use std::path::PathBuf;

const CWD: &str = "./";

pub fn read_cwd() -> Hash {
    read_cwd_helper(CWD).unwrap()
}

fn read_cwd_helper(path: &str) -> Result<String> {
    let children = fs::read_dir(path)?;
    let mut tree = Tree::new();

    for child in children {
        let path = child?.path();
        let path_string = strip_path(&path);

        // ignore the ./geet folder
        if path_string.starts_with(".geet") {
            continue;
        }

        let index = Index::new();
        if path.is_file() && !index.is_in_index(path.to_str().unwrap()) {
            continue;
        }

        let hash = if path.is_dir() {
            read_cwd_helper(&path_string).unwrap()
        } else {
            store_file(&path_string)
        };

        let file_name = path.file_name().unwrap().to_str().unwrap();
        tree.add_node(file_name.to_string(), hash, path.is_dir());
    }

    let serialized = tree.serialize();
    let hash = store_object(&serialized).unwrap();
    Ok(hash)
}

pub fn update_cwd(hash: &Hash) {
    delete_cwd(CWD).unwrap();
    update_cwd_helper(CWD, hash).unwrap();
}

fn update_cwd_helper(path: &str, hash: &Hash) -> Result<()> {
    let serialized = retrieve_object(hash)?;
    let tree = Tree::deserialize(&serialized);

    for node in tree.nodes {
        let path = PathBuf::from(path).join(node.name);
        let path_string = strip_path(&path);

        if node.is_dir {
            fs::create_dir(path)?;
            update_cwd_helper(&path_string, &node.hash)?;
        } else {
            let contents = retrieve_object(&node.hash)?;
            let mut file = fs::File::create(path)?;
            file.write_all(contents.as_bytes())?;
            file.flush()?;
        }
    }

    Ok(())
}

fn delete_cwd(path: &str) -> Result<()> {
    for child in fs::read_dir(&path)? {
        let path = child?.path();
        let path_string = strip_path(&path);

        // ignore the ./geet folder
        if path_string.starts_with(".geet") {
            continue;
        }

        if path.is_dir() {
            delete_cwd(&path_string)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    if path != CWD {
        fs::remove_dir(path)?;
    }
    Ok(())
}

fn store_file(path: &String) -> Hash {
    let data = fs::read_to_string(path).unwrap();
    store_object(&data).unwrap()
}

fn strip_path(path: &PathBuf) -> String {
    path.to_str()
        .map(|s| s.trim_start_matches("./").to_string())
        .unwrap_or_default()
}
