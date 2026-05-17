mod algorithm;
mod config;
mod factory;
mod ml_kem;

pub use algorithm::{KemAlgorithm, KeyPair};
pub use config::{Algorithm, KeyGenConfig};

use anyhow::Result;
use std::fs;

pub fn generate(config: &KeyGenConfig) -> Result<KeyPair> {
    factory::create(&config.algorithm).generate()
}

pub fn save(keypair: &KeyPair, config: &KeyGenConfig) -> Result<()> {
    fs::create_dir_all(&config.out_dir)?;
    fs::write(config.out_dir.join("khyber.pub"), &keypair.public)?;
    fs::write(config.out_dir.join("khyber.key"), &keypair.secret)?;
    Ok(())
}
