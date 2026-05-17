use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::keys::{self, Algorithm, KeyGenConfig};

#[derive(Args)]
pub struct GenerateKeysArgs {
    #[arg(long, short = 'o')]
    pub out: PathBuf,

    #[arg(long, value_enum)]
    pub algorithm: Option<Algorithm>,
}

pub fn run(args: GenerateKeysArgs) -> Result<()> {
    let config = KeyGenConfig::new(args.out).with_algorithm(args.algorithm.unwrap_or_default());
    let keypair = keys::generate(&config)?;
    keys::save(&keypair, &config)?;

    println!("algorithm:  {}", config.algorithm.name());
    println!(
        "public key: {} ({} bytes)",
        config.out_dir.join("khyber.pub").display(),
        keypair.public_key.len()
    );
    println!(
        "secret key: {} ({} bytes)",
        config.out_dir.join("khyber.key").display(),
        keypair.secret_key.len()
    );

    Ok(())
}
