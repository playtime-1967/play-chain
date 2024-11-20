use chrono::prelude::*;
use sha2::{Digest, Sha256};

fn main() {
    println!("Play Chain!");
    let mut blockchain = Blockchain::new(4);
    blockchain.add_block("block1 data".to_string());
    blockchain.add_block("block2 data".to_string());
    blockchain.add_block("block3 data".to_string());

    blockchain.print_chain();

    // Validate the blockchain
    if blockchain.is_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid!");
    }

    //invalidate_chain(&mut blockchain);
}

fn invalidate_chain(blockchain: &mut Blockchain) { //Sample
    blockchain.chain[1].data = "Tampered Data".to_string();
    blockchain.chain[1].hash = blockchain.chain[1].calculate_hash();
    if blockchain.is_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid!");
    }
}

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64, // Used to modify the hash during mining
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            data,
            previous_hash,
            timestamp: Utc::now().to_rfc3339(),
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }

    // Perform Proof-of-Work by finding a hash that starts with a certain number of zeros
    fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined: {} with nonce: {}", self.hash, self.nonce);
    }
}

struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize, // Number of leading zeros required in a valid hash
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let mut block_chain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };
        block_chain.add_genesis_block();
        block_chain
    }

    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn add_genesis_block(&mut self) {
        let mut genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        genesis_block.mine_block(self.difficulty);
        self.chain.push(genesis_block);
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(previous_block.index + 1, data, previous_block.hash.clone());
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Check if the current block's hash is correct
            if current_block.hash != current_block.calculate_hash() {
                println!("Block {} has an invalid hash!", current_block.index);
                return false;
            }

            // Check if the current block's previous_hash matches the hash of the previous block
            if current_block.previous_hash != previous_block.hash {
                println!(
                    "Block {} has an invalid previous hash!",
                    current_block.index
                );
                return false;
            }
        }
        true
    }

    fn print_chain(&self) {
        for block in &self.chain {
            print!("{:?}", block)
        }
    }
}
