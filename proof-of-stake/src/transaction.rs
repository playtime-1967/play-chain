use serde::Serialize;

#[derive(Debug, Clone,Serialize)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        Self {
            sender,
            recipient,
            amount,
        }
    }
}
