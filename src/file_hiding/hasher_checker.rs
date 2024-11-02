// hasher_checker.rs
// generate and verify SHA-1 hashes to ensure file content integrity.
/*
add the following [dependencies] to cargo.toml:
[dependencies] 
*/

use sha1::{Digest, Sha1};

#[derive(Debug)]
pub struct HasherChecker;

impl HasherChecker {
    // creates an SHA-1 hash for the provided data and returns it as a hexadecimal string
    // enough for small to moderate data inputs
    pub fn generate_hash(data: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /* check the hash of the provided data against the expected hash
     return true if the generated hash matches the expected hash */
    pub fn verify_hash(data: &[u8], expected_hash: &str) -> bool {
        let calculated_hash = Self::generate_hash(data);
        calculated_hash == expected_hash
    }
}

/* 
Test cases: 
1. generate hash
2. verify hash success/failure
3. generate hash with empty data
*/

/*
SPMP - Week 1
1. generate_hash(data: &[u8]) -> String
2. verify_hash(data: &[u8], expected_hash: &str) -> bool
    
for commit, add, and checkout commands

*/

// TODO: Add error handling or logging for hash mismatch cases in `verify_hash` if needed.
// TODO: Abstract SHA-1 implementation to allow easy switching to SHA-256 or other algorithms if needed.
// TODO: Add test cases for:
// 1. Edge case with large data for `generate_hash`.
// 2. `verify_hash` with invalid hash formats (optional, based on use case).
// TODO: Implement test cases:
// 1. generate_hash with various data inputs (including empty and large data).
// 2. verify_hash with matching and non-matching hashes.
// 3. Edge cases for invalid or large inputs if necessary.
