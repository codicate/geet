use crate::repo_hiding::data_type::*;

// create a new ref with the given name
// hash is optional. If it is None, the ref will point to HEAD
pub fn create_ref(name: String, hash: Option<Hash>) -> Ref {
    todo!()
}

// get the ref with the given name
pub fn get_ref(name: String) -> Ref {
    todo!()
}

// update the ref with the given name to point to the new hash
pub fn update_ref(name: String, new_hash: Hash) -> Ref {
    todo!()
}

// rename the ref with the old name to the new name
pub fn rename_ref(old_name: String, new_name: String) -> Ref {
    todo!()
}

// delete the ref with the given name
pub fn delete_ref(name: String) {}

// get the hash that the HEAD is pointing to
pub fn get_head() -> Option<Hash> {
    todo!()
}

// update the HEAD to point to the new hash
pub fn update_head(new_hash: Hash) {
    todo!()
}

// checkout the given ref by calling update_head() and apply_revision()
pub fn checkout(ref_name: String) {
    todo!()
}

// list all refs of the given kind (Branch, Tag, or Head)
pub fn list_refs(kind: RefType) -> Vec<Ref> {
    todo!()
}

// list all commits of the given ref
// count is optional. If it is None, all commits will be listed
pub fn list_commits(ref_name: String, count: Option<i32>) -> Vec<Commit> {
    todo!()
}
