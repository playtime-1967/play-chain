use anyhow::{Ok, Result};
mod domain;
mod sign_helper;
use domain::{Blockchain, Transaction};

fn main() -> Result<()> {
    println!("Play Chain!");

    let (signing_key, verifying_key) = sign_helper::generate_keypair();

    let mut blockchain = Blockchain::new(4, 10);
    let mut transaction1 = Transaction::new("Alice", "Bob", 30.0);
    transaction1.sign(&signing_key)?; //TODO: Sign and Verify other blocks as well.
    match transaction1.verify(&verifying_key) {
        std::result::Result::Ok(_) => println!("Transaction's signature is valid!"),
        Err(err) => {
            panic!("Transaction verification failed: {}", err)
        }
    }
    blockchain.add_transaction(transaction1);
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

    Ok(())
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
