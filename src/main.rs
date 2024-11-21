use chrono::prelude::*;
use sha2::{Digest, Sha256};

fn main() {
    println!("Play Chain!");
    let mut blockchain = Blockchain::new(4, 10);

    blockchain.add_transaction(Transaction::new("Alice", "Bob", 30.0));
    blockchain.add_transaction(Transaction::new("Bob", "Charlie", 50.0));
    blockchain.add_block();

    blockchain.add_transaction(Transaction::new("Jon", "Rick", 70.0));
    blockchain.add_block();

    blockchain.add_block(); //no transactions

    blockchain.print_chain();

    if blockchain.is_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid!");
    }

    //invalidate_chain(&mut blockchain); //Sample
}

fn invalidate_chain(blockchain: &mut Blockchain) {
    let transactions = vec![Transaction::new("...", "...", 100.0)];
    blockchain.chain[1].transactions = transactions;
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
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
    is_reward: bool, //miner's reward
}

impl Transaction {
    fn new(sender: &str, receiver: &str, amount: f64) -> Self {
        Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            is_reward: false,
        }
    }
    fn reward(receiver: &str, amount: f64) -> Self {
        Transaction {
            sender: "Network".to_string(),
            receiver: receiver.to_string(),
            amount,
            is_reward: true,
        }
    }
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: &str) -> Self {
        Block {
            index,
            transactions,
            previous_hash: previous_hash.to_string(),
            timestamp: String::new(),
            hash: String::new(),
            nonce: 0,
        }
    }

    fn calculate_hash(&self) -> String {
        let transaction_data: String = self
            .transactions
            .iter()
            .map(|tx| format!("{}->{}:{}", tx.sender, tx.receiver, tx.amount))
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
    fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.timestamp = Utc::now().to_rfc3339();
            self.hash = self.calculate_hash();
        }

        println!("Block mined: {} with nonce: {}", self.hash, self.nonce);
    }
}

struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    difficulty: usize,
    target_time: u64,
}

impl Blockchain {
    fn new(difficulty: usize, target_time: u64) -> Self {
        let mut block_chain = Blockchain {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            difficulty,
            target_time,
        };
        block_chain.add_genesis_block();
        block_chain
    }

    fn add_transaction(&mut self, transaction: Transaction) {
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

    fn add_block(&mut self) {
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

    fn is_valid(&self) -> bool {
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

    fn adjust_difficulty(&mut self) {
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

    fn print_chain(&self) {
        for block in &self.chain {
            println!("{:?}", block)
        }
    }
}
