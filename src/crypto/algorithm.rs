use anyhow::Result;
use kaem_sdk::modules::khyber::{decrypt::DecryptedEvent, encrypt::EncryptedEvent};

pub trait EncryptionAlgorithm {
    fn encrypt(&self, public_key: &[u8], plaintext: &[u8]) -> Result<EncryptedEvent>;
    fn decrypt(&self, secret_key: &[u8], ciphertext: &[u8]) -> Result<DecryptedEvent>;
    fn name(&self) -> &'static str;
}
