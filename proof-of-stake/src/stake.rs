use anyhow::{anyhow, Ok, Result};
#[derive(Debug, Clone)]
pub struct Stake {
    pub owner: String,
    pub amount: u64,
}

impl Stake {
    pub fn new(owner: String, amount: u64) -> Self {
        Self { owner, amount }
    }

    pub fn add_stake(&mut self, amount: u64) {
        self.amount += amount;
    }
    pub fn reduce_stake(&mut self, amount: u64) -> Result<()> {
        if self.amount < amount {
            return Err(anyhow!("Insufficient stake"));
        }

        self.amount -= amount;
        Ok(())
    }
}
