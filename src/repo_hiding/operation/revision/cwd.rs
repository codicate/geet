use crate::file_hiding::file_log::{retrieve_object, store_object};
use crate::file_hiding::index;
use crate::repo_hiding::data_type::{Hash, Tree};
use crate::{BASE_DIR, GEET_DIR};
use std::fs;
use std::io::{Result, Write};
use std::path::PathBuf;

pub fn read_cwd() -> Option<Hash> {
    read_cwd_helper(BASE_DIR).unwrap_or(None)
}

fn read_cwd_helper(path: &str) -> Result<Option<Hash>> {
    let children = fs::read_dir(path)?;
    let mut tree = Tree::new();

    for child in children {
        let path = child?.path();
        let path_string = strip_path(&path);

        // ignore the ./geet folder
        if path_string.starts_with(GEET_DIR) {
            continue;
        }

        // ignore files that are not in the index
        if path.is_file() && !index::contains(&path) {
            continue;
        }

        let hash = if path.is_dir() {
            read_cwd_helper(&path_string)?
        } else {
            Some(store_file(&path_string))
        };

        if let Some(hash) = hash {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            tree.add_node(file_name.to_string(), hash, path.is_dir());
        }
    }

    if tree.nodes.is_empty() {
        Ok(None)
    } else {
        let serialized = tree.serialize();
        let hash = store_object(&serialized)?;
        Ok(Some(hash))
    }
}

pub fn update_cwd(hash: &Hash) {
    delete_cwd(BASE_DIR).unwrap();
    update_cwd_helper(BASE_DIR, hash).unwrap();
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
        if path_string.starts_with(GEET_DIR) {
            continue;
        }

        if path.is_dir() {
            delete_cwd(&path_string)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    if path != BASE_DIR {
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
