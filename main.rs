//! # Blockchain


use std::time::SystemTime;
use sha2::{Digest, Sha256};
use serde::{Serialize, Deserialize};

/// Struct representing a block in the blockchain.
#[derive(Serialize, Deserialize, Debug)]
struct Block {
    /// The index of the block in the chain.
    index: u32,
    /// The timestamp of when the block was created.
    timestamp: u128,
    /// The data stored in the block.
    data: String,
    /// The hash of the previous block in the chain.
    previous_hash: String,
    /// The hash of the current block.
    hash: String,
}

impl Block {
    /// Create a new block.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the block.
    /// * `data` - The data to be stored in the block.
    /// * `previous_hash` - The hash of the previous block in the chain.
    ///
    /// # Returns
    ///
    /// A new `Block` instance.
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get system time.")
            .as_millis();

        let hash = calculate_hash(index, &data, &previous_hash, timestamp);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

/// Calculate the hash value for a block.
fn calculate_hash(index: u32, data: &str, previous_hash: &str, timestamp: u128) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(data);
    hasher.update(previous_hash);
    hasher.update(timestamp.to_string());
    let result = hasher.finalize();
    hex::encode(result)
}

/// Main function to demonstrate the usage of the blockchain.
fn main() {
    let block = Block::new(0, String::from("Genesis Block"), String::from(""));

    println!("{:?}", block);
}
