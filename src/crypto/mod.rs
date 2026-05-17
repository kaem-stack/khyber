mod algorithm;
mod config;
mod factory;
mod kyber_chacha;

pub use algorithm::EncryptionAlgorithm;
pub use config::EncryptConfig;

use anyhow::Result;

pub fn encrypt(config: &EncryptConfig, public_key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    factory::create(&config.algorithm).encrypt(public_key, plaintext)
}
