use super::Block;
use super::Transaction;
use chrono::prelude::*;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
    pub target_time: u64,
}

impl Blockchain {
    pub fn new(difficulty: usize, target_time: u64) -> Self {
        let mut block_chain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
            target_time,
        };
        block_chain.add_genesis_block();
        block_chain
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        if transaction.amount <= 0.0 {
            println!("Invalid transaction: amount must be positive!");
            return;
        }
        // TODO: Validate sender has sufficient balance
        self.pending_transactions.push(transaction);
    }

    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn get_second_last_block(&self) -> &Block {
        &self.chain[self.chain.len() - 2]
    }

    fn add_genesis_block(&mut self) {
        let mut genesis_block = Block::new(0, Vec::new(), "0");
        genesis_block.mine_block(self.difficulty);
        self.chain.push(genesis_block);
    }

    pub fn add_block(&mut self) {
        if self.pending_transactions.is_empty() {
            println!("No transactions to add!");
            return;
        }

        let reward_transaction = Transaction::reward("MinerAddress", 0.5);
        self.pending_transactions.insert(0, reward_transaction);

        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(
            previous_block.index + 1,
            self.pending_transactions.clone(),
            &previous_block.hash,
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);

        self.pending_transactions.clear(); // Clear the pending transactions once included in a block

        self.adjust_difficulty();
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                println!("Block {} has an invalid hash!", current_block.index);
                return false;
            }

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

    pub fn adjust_difficulty(&mut self) {
        if self.chain.len() < 2 {
            return; // No adjustment for the genesis block
        }

        let last_block = self.get_latest_block();
        let second_last_block = self.get_second_last_block();

        let last_block_time = DateTime::parse_from_str(&last_block.timestamp, "%+").unwrap();
        let second_last_block_time =
            DateTime::parse_from_str(&second_last_block.timestamp, "%+").unwrap();

        let actual_time = (last_block_time - second_last_block_time).num_seconds() as u64;

        println!(
            "Actual block time: {} seconds | Target block time: {} seconds",
            actual_time, self.target_time
        );

        if actual_time < self.target_time {
            self.difficulty += 1;
            println!("Difficulty increased to {}", self.difficulty);
        } else if actual_time > self.target_time && self.difficulty > 1 {
            self.difficulty -= 1;
            println!("Difficulty decreased to {}", self.difficulty);
        }
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block)
        }
    }
}
