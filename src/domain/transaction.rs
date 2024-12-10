use anyhow::{Ok, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub is_reward: bool, //miner's reward
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64) -> Self {
        Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            is_reward: false,
            signature: None,
        }
    }
    pub fn reward(receiver: &str, amount: f64) -> Self {
        Transaction {
            sender: "Network".to_string(),
            receiver: receiver.to_string(),
            amount,
            is_reward: true,
            signature: None,
        }
    }

    pub fn sign(&mut self, signing_key: &SigningKey) -> anyhow::Result<()> {
        let raw_transaction_data = self.get_raw_transaction_data();
        let signature = signing_key.sign(raw_transaction_data.as_bytes());
        self.signature = Some(hex::encode(signature.to_bytes()));

        Ok(())
    }

    pub fn verify(&self, verifying_key: &VerifyingKey) -> Result<()> {
        let signature_str = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No signature found"))?;

        let signature_bytes = hex::decode(signature_str)?;
        let signature_array: &[u8; 64] = signature_bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Signature must be exactly 64 bytes"))?;

        let signature = Signature::from_bytes(signature_array);

        verifying_key.verify(self.get_raw_transaction_data().as_bytes(), &signature)?;

        Ok(())
    }

    pub fn get_raw_transaction_data(&self) -> String {
        format!("{}->{}:{}", self.sender, self.receiver, self.amount)
    }
}
