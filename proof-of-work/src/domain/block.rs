use super::Transaction;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: &str) -> Self {
        Self {
            index,
            transactions,
            previous_hash: previous_hash.to_string(),
            timestamp: String::new(),
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn genesis(difficulty: usize) -> Self {
        let mut block = Block {
            index: 0,
            transactions: Vec::new(),
            previous_hash: String::from("0"),
            timestamp: Utc::now().to_rfc3339(),
            hash: String::new(),
            nonce: 0,
        };

        block.mine_block(difficulty);
        block
    }

    pub fn calculate_hash(&self) -> String {
        let transaction_data: String = self
            .transactions
            .iter()
            .map(|tx| tx.get_raw_transaction_data())
            .collect();

        let block_data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, transaction_data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }

    // Perform Proof-of-Work by finding a hash that starts with a certain number of zeros
    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.timestamp = Utc::now().to_rfc3339();
            self.hash = self.calculate_hash();
        }

        println!("Block mined: {} with nonce: {}", self.hash, self.nonce);
    }
}
