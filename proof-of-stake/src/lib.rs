mod block;
mod blockchain;
mod malicious_detector;
mod stake;
mod transaction;
mod validator;

//re-exporting; bringing an item into scope but also making that item available for others to bring into their scope.
pub use block::Block;
pub use blockchain::Blockchain;
pub use stake::Stake;
pub use transaction::Transaction;
pub use validator::Validator;
