#![allow(warnings)]
use proof_of_stake::{Blockchain, Stake, Transaction, Validator};
fn main() {
    println!("Play Chain!--> PROOF OF STAKE!");

    let mut blockchain = Blockchain::new();
    blockchain.create_genesis_block();

    let validator1 = Validator::new("Validator1".to_string());
    let validator2 = Validator::new("Validator2".to_string());

    blockchain.add_validator(validator1, Stake::new("Validator1".to_string(), 50));
    blockchain.add_validator(validator2, Stake::new("Validator2".to_string(), 30));

    // Add transactions.
    blockchain
        .add_transaction(Transaction::new(
            "Validator1".to_string(),
            "Alice".to_string(),
            40,
        ))
        .unwrap();

    blockchain
        .add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 15))
        .unwrap();

    blockchain.add_block();
    blockchain.add_block(); //no transactions to add!

    println!("Blockchain:--------------------------------");
    println!("{:#?}", blockchain.chain);

    println!("Balances:----------------------------------");
    blockchain.print_balances();
}
