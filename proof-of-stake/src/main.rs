#![allow(warnings)]
use proof_of_stake::{Blockchain, Stake, Validator};
fn main() {
    println!("Play Chain!--------------------------------PROOF OF STAKE!");
    
    let mut blockchain = Blockchain::new();
    blockchain.create_genesis_block();

    let validator1 = Validator::new("Validator1".to_string());
    let validator2 = Validator::new("Validator2".to_string());

    blockchain.add_validator(validator1, Stake::new("Validator1".to_string(), 50));
    blockchain.add_validator(validator2, Stake::new("Validator2".to_string(), 30));

    blockchain.add_block("Transaction 1".to_string());
    blockchain.add_block("Transaction 2".to_string());

    println!("{:#?}", blockchain.chain);
}
