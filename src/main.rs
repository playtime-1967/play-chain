use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    // Create a new block
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            data,
            previous_hash,
            timestamp: Utc::now().to_rfc3339(),
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }
}

struct Blockchain {
    chain: Vec<Block>,
}
impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.get_latest_block();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        self.chain.push(new_block);
    }
    fn print_chain(&self) {
        for block in &self.chain {
            print!("{:?}", block)
        }
    }
}
fn main() {
    println!("Play Chain!");
    let mut chain = Blockchain::new();
    chain.add_block("block1 data".to_string());
    chain.add_block("block2 data".to_string());
    chain.print_chain();
}
