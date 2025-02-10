//declare submodules
mod block;
mod blockchain;
mod network;
mod transaction;
mod wallet;

//re-export the mods for easier access
pub use block::Block;
pub use blockchain::Blockchain;
pub use network::Network;
pub use transaction::Transaction;
pub use wallet::Wallet;
