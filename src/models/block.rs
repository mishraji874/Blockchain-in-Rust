use super::transaction::Transaction;
use chrono::prelude::*;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// A block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    index: usize,
    timestamp: u64,
    date_time: String,
    pub proof_of_work: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    pub hash: String,
}

// Mining followed by multi-threads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mining {
    pub mined: bool,
    pub proof_of_work: u64,
    pub hash: String,
    pub consensus: u64,
}

impl Mining {
    pub fn new() -> Self {
        Self {
            mined: false,
            proof_of_work: u64::default(),
            hash: String::default(),
            consensus: u64::default(),
        }
    }
}

impl Block {
    // Create new block with the hash of the previous block and the accepted transactions.
    pub fn new(index: usize, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        Self {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            date_time: Utc::now().format("%Y-%m-%d %H:%M:%S:%3f").to_string(),
            proof_of_work: u64::default(),
            transactions,
            previous_hash,
            hash: String::default(),
        }
    }

    // Calculate and return SHA-256 hash value of the block
    fn generate_block_hash(&mut self) -> String {
        self.hash = String::default();
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(&self).unwrap().as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Mine block hash with given the difficulty and nonce and return the proof of work
    pub fn mine(&mut self, difficulty: usize, nonce: u64) {
        // add nonce to block
        self.proof_of_work = nonce;

        // Generate block hash until it starts with difficulty number of zeros
        loop {
            if !self.hash.starts_with(&"0".repeat(difficulty)) {
                self.proof_of_work += 1;
                self.hash = self.generate_block_hash();
            } else {
                if nonce == 0 {
                    println!(
                        "Genesis Block mined: Prof of work is: {}",
                        self.proof_of_work.to_string().green().bold()
                    );
                }
                break;
            }
        }
    }

    // Verify block once mined
    pub fn verify(&self) -> bool {
        let mut cloned_block = self.clone();
        cloned_block.generate_block_hash() == self.hash
    }
}
