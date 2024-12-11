use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng {};
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}
