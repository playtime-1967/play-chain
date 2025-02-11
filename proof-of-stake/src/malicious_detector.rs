use super::Blockchain;
use std::collections::HashSet;

impl Blockchain {
    //detect double-signing or other malicious activity
    pub fn detect_and_slash(&mut self) {
        let mut seen_blocks: HashSet<(u64, String)> = Self::get_malicious_validators();
        let mut validators_to_slash: Vec<String> = vec![];
        let mut malicious_blocks: Vec<usize> = vec![];

        for block in &self.chain {
            let key = (block.index, block.validator.clone());

            if !seen_blocks.insert(key.clone()) {
                println!(
                    "Validator {} double-signed at block index {}. Initiating slashing.",
                    block.validator, block.index
                );

                validators_to_slash.push(block.validator.clone());
                malicious_blocks.push(block.index.try_into().unwrap());
            }
        }

        //slashing validators
        for validator_address in validators_to_slash {
            self.slash_validator(&validator_address);
        }

        //remove malicious blocks from the chain
        for &index in malicious_blocks.iter().rev() {
            self.chain.remove(index);
            println!("Removed malicious block at index {} from the chain.", index);
        }
    }

    fn slash_validator(&mut self, validator_address: &String) -> bool {
        if let Some(stake) = self.stakes.get_mut(validator_address) {
            // Slash a portion of the stake (e.g., 25%)
            let penalty = stake.amount / 4;
            stake.amount -= penalty;

            // Deduct penalty from balances as well
            if let Some(balance) = self.balances.get_mut(validator_address) {
                *balance = balance.saturating_sub(penalty); // Prevent negative balances(otherwise attempt to subtract with overflow)
            }

            // Log the slashing action
            println!(
                "Validator {} was slashed. Penalty: {}, Remaining Stake: {}",
                validator_address, penalty, stake.amount
            );

            // If the stake drops to zero, remove the validator
            if stake.amount == 0 {
                self.remove_validator(validator_address);
                println!(
                    "Validator {} has been removed due to zero stake.",
                    validator_address
                );
            }

            true
        } else {
            println!("Validator {} does not exist.", validator_address);
            false
        }
    }

    //remove a validator from the system
    fn remove_validator(&mut self, validator_address: &String) {
        self.validators.remove(validator_address);
        self.stakes.remove(validator_address);
    }

    fn get_malicious_validators() -> HashSet<(u64, String)> {
        //a mock set of blocks to represent malicious behavior for demo purposes.
        HashSet::from([(2, String::from("Validator2"))])
    }
}
