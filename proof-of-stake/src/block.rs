use super::Transaction;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub validator: String,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        validator: String,
    ) -> Self {
        let mut block = Self {
            index,
            previous_hash,
            transactions,
            validator,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();

        block
    }

    pub fn genesis() -> Self {
        Self {
            index: 0,
            previous_hash: "0".to_string(),
            timestamp: 0,
            transactions: vec![],
            validator: "System".to_string(),
            hash: "0".to_string(),
        }
    }

    fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.hash,
            self.timestamp,
            self.previous_hash,
            self.validator,
            serde_json::to_string(&self.transactions).unwrap_or_default()
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }
}
