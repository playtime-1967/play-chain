use super::Transaction;
use crate::util::sign_helper;
use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey};
use hex;

#[derive(Debug)]
pub struct Wallet {
    pub address: String,
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let (signing_key, verifying_key) = sign_helper::generate_keypair();

        Self {
            address: hex::encode(verifying_key.as_bytes()),
            signing_key,
            verifying_key,
        }
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction) -> Result<()> {
        transaction.sign(&self.signing_key)
    }
}
