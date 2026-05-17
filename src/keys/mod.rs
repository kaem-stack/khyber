mod algorithm;
mod config;
mod factory;
mod ml_kem;

pub use algorithm::KemAlgorithm;
pub use config::{Algorithm, KeyGenConfig};
pub use kaem_sdk::modules::khyber::generate_keys::KeysGeneratedEvent;

use anyhow::Result;
use std::fs;

pub fn generate(config: &KeyGenConfig) -> Result<KeysGeneratedEvent> {
    factory::create(&config.algorithm).generate()
}

pub fn save(keypair: &KeysGeneratedEvent, config: &KeyGenConfig) -> Result<()> {
    fs::create_dir_all(&config.out_dir)?;
    fs::write(config.out_dir.join("khyber.pub"), &keypair.public_key)?;
    fs::write(config.out_dir.join("khyber.key"), &keypair.secret_key)?;
    Ok(())
}
