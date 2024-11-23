use anyhow::{Ok, Result};
mod domain;
mod sign_helper;
use domain::{Blockchain, Transaction, Wallet};

fn main() -> Result<()> {
    println!("Play Chain!");

    let mut blockchain = Blockchain::new(4, 10);

    let alice_wallet = Wallet::new();
    let bob_wallet: Wallet = Wallet::new();

    let mut alice_transaction = Transaction::new(&alice_wallet.address, &bob_wallet.address, 30.0);
    alice_wallet.sign_transaction(&mut alice_transaction)?;

    if let Err(err) = alice_transaction.verify(&alice_wallet.verifying_key) {
        println!("Transaction verification failed: {}", err);
    } else {
        println!("Transaction signature verified!");
        blockchain.add_transaction(alice_transaction)?;
    }

    blockchain.add_block();

    blockchain.add_transaction(Transaction::new("Jon", "Rick", 70.0))?;
    blockchain.add_block();

    blockchain.add_block(); //no transactions

    blockchain.print_chain();

    if blockchain.is_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid!");
    }

    Ok(())
    //invalidate_chain_Sample(&mut blockchain);
}

fn invalidate_chain_Sample(blockchain: &mut Blockchain) {
    let transactions = vec![Transaction::new("...", "...", 100.0)];
    blockchain.chain[1].transactions = transactions;
    blockchain.chain[1].hash = blockchain.chain[1].calculate_hash();
    if blockchain.is_valid() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid!");
    }
}

fn sign_verify_tr_sample() {
    // let (signing_key, verifying_key) = sign_helper::generate_keypair();
    // transaction1.sign(&signing_key)?;
    // match transaction1.verify(&verifying_key) {
    //     std::result::Result::Ok(_) => println!("Transaction's signature is valid!"),
    //     Err(err) => {
    //         panic!("Transaction verification failed: {}", err)
    //     }
    // }
    // let mut transaction1 = Transaction::new("Alice", "Bob", 30.0);
    //blockchain.add_transaction(transaction1)?;
}
