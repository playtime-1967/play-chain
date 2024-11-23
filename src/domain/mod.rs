//Declare submodules
mod block;
mod blockchain;
mod transaction;
mod wallet;

//Re-export them for easier access
pub use block::Block;
pub use blockchain::Blockchain;
pub use transaction::Transaction;
pub use wallet::Wallet;

