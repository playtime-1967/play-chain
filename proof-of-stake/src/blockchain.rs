use super::Block;
use super::Stake;
use super::Validator;
use crate::Transaction;
use anyhow::{anyhow, Ok, Result};
use rand::seq::SliceRandom;
use std::collections::HashMap;
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub validators: HashMap<String, Validator>,
    pub stakes: HashMap<String, Stake>,
    pub balances: HashMap<String, u64>,
    pub pending_transactions: Vec<Transaction>,
    pub reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![],
            stakes: HashMap::new(),
            validators: HashMap::new(),
            balances: HashMap::new(),
            pending_transactions: vec![], // Transactions awaiting inclusion in the next block.
            reward: 5,                    // Default reward per block.
        }
    }

    //Validator becomes eligible to validate blocks by staking a certain amount of cryptocurrency.
    pub fn add_validator(&mut self, validator: Validator, stake: Stake) {
        self.validators.insert(validator.address.clone(), validator);
        self.stakes.insert(stake.owner.clone(), stake.clone());

        self.balances.entry(stake.owner).or_insert(stake.amount); // Initialize balance for the validator.s
    }

    pub fn create_genesis_block(&mut self) {
        self.chain.push(Block::genesis());
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<()> {
        // Ensure sender has sufficient balance.
        if let Some(balance) = self.balances.get_mut(&transaction.sender) {
            if *balance < transaction.amount {
                return Err(anyhow!("Insufficient balance for transaction."));
            }

            *balance -= transaction.amount;
        } else {
            return Err(anyhow!("Sender not found in balances."));
        }

        // Add amount to the recipient.
        let recipient_balance = self
            .balances
            .entry(transaction.recipient.clone())
            .or_insert(0);
        *recipient_balance += transaction.amount;

        // Add the transaction to the pool.
        self.pending_transactions.push(transaction);

        Ok(())
    }

    pub fn add_block(&mut self) -> Option<Block> {
        if self.chain.is_empty() {
            eprintln!("Warning: Blockchain has no genesis block!");
            return None;
        }

        if self.pending_transactions.is_empty() {
            println!("Warning: No transactions to add!");
            return None;
        }

        let last_block = self.chain.last().unwrap();
        let selected_validator = self.select_validator()?;

        // Collect pending transactions.
        let new_block = Block::new(
            last_block.index + 1,
            last_block.hash.clone(),
            self.pending_transactions.drain(..).collect::<Vec<_>>(),
            selected_validator.clone(),
        );

        // Add reward to the validator's balance.
        if let Some(balance) = self.balances.get_mut(&selected_validator) {
            *balance += self.reward;
        } else {
            eprintln!("Validator not found in balances!");
            return None;
        }

        self.chain.push(new_block.clone());
        Some(new_block)
    }

    fn select_validator(&self) -> Option<String> {
        let stakes: Vec<(&String, &Stake)> = self.stakes.iter().collect();
        let total_stake_amount: u64 = stakes.iter().map(|(owner, stake)| stake.amount).sum();

        let mut weighted_pool: Vec<String> = vec![];
        for (address, stake) in stakes {
            let weight = (stake.amount as f64 / total_stake_amount as f64 * 100.0) as usize;
            weighted_pool.extend(vec![address.clone(); weight]); //Appends the contents of the newly created vector, containing the address repeated weight times.
        }

        weighted_pool.choose(&mut rand::thread_rng()).cloned()
    }

    pub fn print_balances(&self) {
        println!("Current Balances:");
        for (address, balance) in &self.balances {
            println!("{}: {}", address, balance);
        }
    }
}
