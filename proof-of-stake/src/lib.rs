mod block;
mod blockchain;
mod stake;
mod validator;
mod transaction;
mod malicious_detector;

pub use block::Block; //pub use: is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope
pub use blockchain::Blockchain;
pub use stake::Stake;
pub use validator::Validator;
pub use transaction::Transaction;

