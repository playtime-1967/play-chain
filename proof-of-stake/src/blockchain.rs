use super::Block;
use super::Stake;
use super::Validator;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub validators: HashMap<String, Validator>,
    pub stakes: HashMap<String, Stake>,
    pub balances: HashMap<String, u64>,
    pub reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            chain: vec![],
            stakes: HashMap::new(),
            validators: HashMap::new(),
            balances: HashMap::new(),
            reward: 10, // Default reward per block.
        }
    }

    //Validator becomes eligible to validate blocks by staking a certain amount of cryptocurrency.
    pub fn add_validator(&mut self, validator: Validator, stake: Stake) {
        self.validators.insert(validator.address.clone(), validator);
        self.stakes.insert(stake.owner.clone(), stake.clone());
        self.balances.insert(stake.owner, stake.amount); // Initialize balance for the validator.s
    }

    pub fn create_genesis_block(&mut self) {
        self.chain.push(Block::genesis());
    }

    pub fn add_block(&mut self, data: String) -> Option<Block> {
        if self.chain.is_empty() {
            eprintln!("Blockchain has no genesis block!");
            return None;
        }

        let last_block = self.chain.last().unwrap();
        let selected_validator = self.select_validator()?;

        // Add reward to the validator's balance.
        if let Some(balance) = self.balances.get_mut(&selected_validator) {
            *balance += self.reward;
        } else {
            eprintln!("Validator not found in balances!");
            return None;
        }

        let new_block = Block::new(
            last_block.index + 1,
            last_block.hash.clone(),
            data,
            selected_validator,
        );

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
