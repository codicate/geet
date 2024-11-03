use super::Hash;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeNode {
    pub name: String, // Name of the file or directory
    pub hash: Hash,   // Hash of the tree node
    pub is_dir: bool, // Indicates if it's a directory or file
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {
    pub nodes: Vec<TreeNode>, // List of nodes in the tree
}

impl Tree {
    /* Creates a new, empty `Tree` instance. */
    pub fn new() -> Self {
        Tree { nodes: Vec::new() }
    }

    /* Adds a directory node (Tree) to the `Tree`. */
    pub fn add_node(&mut self, name: String, hash: Hash, is_dir: bool) {
        let node = TreeNode { name, hash, is_dir };
        self.nodes.push(node);
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect(format!("Failed to serialize tree {:#?}", self).as_str())
    }

    pub fn deserialize(data: &str) -> Self {
        serde_json::from_str(data).expect(format!("Failed to deserialize tree: {}", data).as_str())
    }
}
