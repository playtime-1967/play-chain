#![allow(warnings)]
use anyhow::{Ok, Result};
mod domain;
pub mod util;
use clap::{builder::Str, Arg, ArgMatches, Command};
use domain::{Blockchain, Network, Transaction, Wallet};
use tokio::time::{sleep, Duration};
use util::converter;
#[tokio::main]
async fn main() -> Result<()> {
    let args = get_args();
    let listen_addr = args.get_one::<String>("listen").unwrap().clone();
    let peers_addr = args
        .get_one::<String>("peers")
        .unwrap_or_else(|| panic!("peers!"))
        .split(',')
        .collect::<Vec<&str>>();

    let mut blockchain = Blockchain::new(2, 10).unwrap();
    let mut network = Network::new();
    network
        .add_peers(converter::convert_vec_of_str_to_vec_of_string(peers_addr))
        .await;
    blockchain.set_network(network.clone());

    tokio::spawn(async move {
        network.start_listening(&listen_addr).await;
    });

    let mut block_counter = 1;
    loop {
        let alice_wallet = Wallet::new();
        let bob_wallet: Wallet = Wallet::new();
        let mut alice_transaction =
            Transaction::new(alice_wallet.address.clone(), bob_wallet.address, 30.0);

        alice_wallet.sign_transaction(&mut alice_transaction)?;

        if let Err(err) = alice_transaction.verify(&alice_wallet.verifying_key) {
            println!("Transaction verification failed: {}", err);
        } else {
            println!("Transaction signature verified!");
            blockchain.add_transaction(alice_transaction).await?;
        }
        let miner_addr = String::from("miner_addr");
        blockchain.add_block(miner_addr.clone());

        let john_wallet = Wallet::new();
        let chris_wallet: Wallet = Wallet::new();

        blockchain
            .add_transaction(Transaction::new(
                john_wallet.address,
                chris_wallet.address,
                70.0,
            ))
            .await?; //skip sign_transaction and verify as it's a sample.
        blockchain.add_block(miner_addr.clone());

        blockchain.add_block(miner_addr.clone()); //no transaction sample

        //blockchain.print_chain();

        if blockchain.is_valid_chain() {
            println!("The blockchain is valid.");
        } else {
            println!("The blockchain is invalid!");
        }

        println!("Block Counter: {block_counter} Completed. ---------------------------------------------------------------------------------------------------");
        sleep(Duration::from_secs(5)).await;
        block_counter = block_counter + 1;
    }

    Ok(())
}

fn get_args() -> ArgMatches {
    let app = Command::new("Blockchain Peer")
        .arg(
            Arg::new("listen")
                .help("The address to listen on")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("peers")
                .help("Comma-separated list of peer addresses")
                .required(true)
                .index(2),
        );
    app.get_matches()
}

fn invalidate_chain_Sample(blockchain: &mut Blockchain) {
    let transactions = vec![Transaction::new(
        String::from("..."),
        String::from("..."),
        100.0,
    )];
    blockchain.chain[1].transactions = transactions;
    blockchain.chain[1].hash = blockchain.chain[1].calculate_hash();
    if blockchain.is_valid_chain() {
        println!("The blockchain is valid.");
    } else {
        println!("The blockchain is invalid!");
    }
}
