mod algorithm;
mod config;
mod factory;
mod kyber_chacha;

pub use algorithm::EncryptionAlgorithm;
pub use config::EncryptConfig;
pub use kaem_sdk::modules::khyber::{decrypt::DecryptedEvent, encrypt::EncryptedEvent};

use anyhow::Result;

pub fn encrypt(config: &EncryptConfig, public_key: &[u8], plaintext: &[u8]) -> Result<EncryptedEvent> {
    factory::create(&config.algorithm).encrypt(public_key, plaintext)
}

pub fn decrypt(config: &EncryptConfig, secret_key: &[u8], ciphertext: &[u8]) -> Result<DecryptedEvent> {
    factory::create(&config.algorithm).decrypt(secret_key, ciphertext)
}
