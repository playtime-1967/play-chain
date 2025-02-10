

## 1- Proof of Work (PoW)
A lightweight PoW blockchain includes core blockchain functionality, transaction validation, block mining, consensus mechanisms, dynamic difficulty adjustment, and decentralized peer-to-peer network communication.

## Features  

- **Blockchain Core**  
  - Stores a chain of blocks, each containing verified transactions.  
  - Ensures chain integrity by verifying hashes and linking blocks cryptographically.  
- **Mining (Proof-of-Work)**  
  - Miners solve a cryptographic puzzle by finding a hash that starts with a certain number of zeros.  
  - Difficulty is dynamically adjusted based on previous block times.  
  - Reward system for miners upon successful block mining.  
- **Transactions**  
  - Digital signature verification using Ed25519 cryptography.  
  - Secure peer-to-peer transactions with signature validation before inclusion in a block.  
- **Networking**  
  - Asynchronous TCP streaming to handle multiple peer connections without blocking threads.  
  - Each new connection is handled via a spawned task, ensuring efficient concurrency.  
  - When a new block is mined, it is broadcasted to all network peers for synchronization.  
- **Dynamic Difficulty Adjustment**  
  - The mining difficulty is adjusted based on actual block time:  
    - If a block is mined faster than the target time, difficulty increases to slow down mining.  
    - If a block takes too long, difficulty decreases to ensure a stable block rate.  
- **Wallets & Signing**  
  - Uses public-private key cryptography to sign and verify transactions.  
  - Each wallet generates a unique address based on the public key.  
- **Peer-to-Peer Communication**  
  - Nodes can send and receive transactions across the network.  
  - Transactions are propagated to all connected peers.  

## How It Works

1. The blockchain initializes with a genesis block.
2. Transactions are created and signed using Ed25519 keys.
3. The network propagates transactions to peers.
4. The next block is mined and added to the chain.
5. The system verifies block validity and updates the ledger.

## Technologies Used

- **Tokio**: Asynchronous runtime for networking.
- **Sha256**: Hashing for block integrity.
- **Ed25519**: Digital signature scheme for secure transactions.
- **Chrono**: Time management for block timestamps.

## Steps to Run  

To start the blockchain network, open three or more terminal windows in the root directory. 
Each node must be started with its own address and the addresses of its peers as command-line arguments.  

## Start Nodes  

**Terminal 1:**  
```sh
cargo run --bin proof-of-work 127.0.0.1:8050 127.0.0.1:8052,127.0.0.1:8051
```  

**Terminal 2:**  
```sh
cargo run --bin proof-of-work 127.0.0.1:8051 127.0.0.1:8050,127.0.0.1:8052
```  

**Terminal 3:**  
```sh
cargo run --bin proof-of-work 127.0.0.1:8052 127.0.0.1:8051,127.0.0.1:8050
```  

- Each node must include the IP and port of all other peers as arguments.  
- Once started, the nodes will begin communicating, mining, and broadcasting new blocks to each other.  

----------------------------------------------------------------------------------

## 2- Proof of Stake (PoS)
