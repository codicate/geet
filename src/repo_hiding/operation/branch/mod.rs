use super::revision::{apply_revision, get_revision};
use crate::file_hiding::ref_log::{delete_ref as _delete_ref, retrieve_ref, store_ref};
use crate::repo_hiding::data_type::{Commit, Hash, Ref, RefType};
use crate::REFS_DIR;

// create a new ref with the given name
// hash is optional. If it is None, the ref will point to HEAD
pub fn create_ref(ref_type: RefType, name: String, hash: Option<Hash>) -> Ref {
    let hash = if let Some(hash) = hash {
        Some(hash)
    } else {
        get_head()
    };

    let data = Ref {
        ref_type,
        name: name.clone(),
        commit_hash: hash,
    };

    let serialized = data.serialize();
    store_ref(&name, &serialized).unwrap();
    data
}

// get the ref with the given name
pub fn get_ref(name: &String) -> Ref {
    let serialized = retrieve_ref(&name).unwrap();
    Ref::deserialize(&serialized)
}

// update the ref with the given name to point to the new hash
pub fn update_ref(name: &String, new_hash: Hash) -> Ref {
    let mut data = get_ref(name);
    data.commit_hash = Some(new_hash.clone());

    let serialized = data.serialize();
    store_ref(&name, &serialized).unwrap();
    data
}

// rename the ref with the old name to the new name
pub fn rename_ref(old_name: &String, new_name: &String) -> Ref {
    let data = get_ref(old_name);
    let serialized = data.serialize();
    store_ref(&new_name, &serialized).unwrap();
    delete_ref(old_name);
    data
}

// delete the ref with the given name
pub fn delete_ref(name: &String) {
    _delete_ref(name.as_str()).unwrap();
}

pub fn create_head() {
    let data = Ref {
        ref_type: RefType::Head,
        name: "HEAD".to_string(),
        commit_hash: None,
    };

    let serialized = data.serialize();
    store_ref(&data.name, &serialized).unwrap();
}

// get the hash that the HEAD is pointing to
pub fn get_head() -> Option<Hash> {
    let head = get_ref(&"HEAD".to_string());
    if let Some(hash) = head.commit_hash {
        Some(hash)
    } else {
        None
    }
}

// update the HEAD to point to the new hash
pub fn update_head(new_hash: &Hash) {
    update_ref(&"HEAD".to_string(), new_hash.clone());
}

// apply the changes from the revision to the working directory
pub fn checkout_commit(commit_hash: &String) -> Result<(), String> {
    apply_revision(commit_hash)?;
    update_head(commit_hash);
    Ok(())
}

// checkout the given ref by calling update_head() and apply_revision()
pub fn checkout_ref(ref_name: &String) {
    let ref_data = get_ref(ref_name);
    let hash = ref_data
        .commit_hash
        .expect(format!("Ref {} does not point to any commit", ref_name).as_str());
    checkout_commit(&hash);
}

// list all refs of the given kind (Branch, Tag, or Head)
pub fn list_refs(kind: RefType) -> Vec<Ref> {
    let mut ref_list = Vec::new();
    let dir = std::fs::read_dir(REFS_DIR).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let data = get_ref(&name);
        if data.ref_type == kind {
            ref_list.push(data);
        }
    }
    ref_list
}

// list all commits of the given ref
// count is optional. If it is None, all commits will be listed
pub fn list_commits(ref_name: String, count: Option<i32>) -> Result<Vec<Commit>, String> {
    let mut current_commit_hash = get_ref(&ref_name).commit_hash.unwrap();
    let mut commit_list = Vec::new();

    for _ in 0..count.unwrap_or(i32::MAX) {
        let commit = get_revision(&current_commit_hash)?;
        commit_list.push(commit.clone());
        match commit.parent_hash {
            Some(hash) => current_commit_hash = hash,
            None => break,
        }
    }

    Ok(commit_list)
}

#[test]
fn test() {
    let commit_list = list_commits("HEAD".to_string(), None);
    println!("{:#?}", commit_list);

    let ref_list = list_refs(RefType::Head);
    println!("{:#?}", ref_list);
}
