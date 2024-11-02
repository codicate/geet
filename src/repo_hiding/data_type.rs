pub type Hash = String;

#[derive(PartialEq, Debug)]
pub struct CommitMetadata {
    pub author: String,
    pub message: String,
    pub timestamp: i64,
}
pub struct Commit {
    pub metadata: CommitMetadata,
    pub hash: Hash,
    pub parent: Option<Hash>,
}

#[derive(PartialEq, Debug)]
pub enum RefType {
    Branch,
    Tag,
    Head,
}

#[derive(PartialEq, Debug)]
pub struct Ref {
    pub kind: RefType,
    pub name: String,
    pub hash: Hash,
}
