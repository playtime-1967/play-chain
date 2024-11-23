use anyhow::{Ok, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub is_reward: bool, //miner's reward
    pub signature: Option<Signature>,
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
        let message = self.get_message();
        self.signature = Some(signing_key.sign(message.as_bytes()));
        Ok(())
    }

    pub fn verify(&self, verifying_key: &VerifyingKey) -> Result<()> {
        let signature = self
            .signature
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No signature found"))?;
        verifying_key.verify(self.get_message().as_bytes(), signature)?;
        Ok(())
    }

    pub fn get_message(&self) -> String {
        format!("{}->{}:{}", self.sender, self.receiver, self.amount)
    }
}
