#![allow(warnings)]
use proof_of_stake::{Block, Blockchain, Stake, Transaction, Validator};

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.create_genesis_block();
    println!("genesis block was created.");

    let validator1 = Validator::new("Validator1".to_string());
    let validator2 = Validator::new("Validator2".to_string());
    println!("Validator1 was registered.");
    println!("Validator2 was registered.");

    blockchain.add_validator(validator1, Stake::new("Validator1".to_string(), 50));
    println!("Validator1 was added to the blockchain with an amount of 50.");

    blockchain.add_validator(validator2, Stake::new("Validator2".to_string(), 30));
    println!("Validator2 was added to the blockchain with an amount of 30.");

    //add transactions.
    blockchain
        .add_transaction(Transaction::new(
            "Validator1".to_string(),
            "Alice".to_string(),
            40,
        ))
        .unwrap();
    println!("A transaction of 40 was added to Alice by Validator1.");

    blockchain
        .add_transaction(Transaction::new("Alice".to_string(), "Bob".to_string(), 15))
        .unwrap();
    println!("A transaction of 15 was added to Bob by Alice.");

    blockchain.add_block();
    println!("A block was created by collecting all pending transactions and rewarding the validators.");

    blockchain.add_block(); //simulate of no transactions to add.

    //simulate malicious behavior: Validator2 double-signs block 2.
    let malicious_block = Block::new(
        2,
        blockchain.chain[1].hash.clone(),
        vec![Transaction::new("X".to_string(), "Y".to_string(), 15)],
        "Validator2".to_string(),
    );
    blockchain.chain.push(malicious_block);
    blockchain.detect_and_slash();

    println!("Blocks in the blockchain:-------------------------------------------------------------------------------");
    println!("{:#?}", blockchain.chain);

    println!("Users/Validators Balance:--------------------------------------------------------------------------------");
    blockchain.print_balances();
}
