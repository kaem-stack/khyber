use std::path::PathBuf;

use anyhow::Result;
use base64ct::{Base64, Encoding as _};
use clap::Args;

use crate::crypto::{self, EncryptConfig};

#[derive(Args)]
pub struct EncryptArgs {
    /// Message to encrypt
    pub message: String,

    /// Public key file (khyber.pub)
    #[arg(long, short = 'k')]
    pub key: PathBuf,

    /// Output file (default: print base64 to stdout)
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,
}

pub fn run(args: EncryptArgs) -> Result<()> {
    let public_key = std::fs::read(&args.key)?;
    let config = EncryptConfig::default();

    let event = crypto::encrypt(&config, &public_key, args.message.as_bytes())?;
    let encoded = Base64::encode_string(&event.ciphertext);

    match args.output {
        Some(path) => std::fs::write(path, &encoded)?,
        None => println!("{}", encoded),
    }

    Ok(())
}
