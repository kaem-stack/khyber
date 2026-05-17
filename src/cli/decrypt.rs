use std::path::PathBuf;

use anyhow::Result;
use base64ct::{Base64, Encoding as _};
use clap::Args;

use crate::crypto::{self, EncryptConfig};

#[derive(Args)]
pub struct DecryptArgs {
    /// Base64-encoded ciphertext to decrypt
    pub input: String,

    /// Secret key file (khyber.key)
    #[arg(long, short = 'k')]
    pub key: PathBuf,

    /// Output file (default: print plaintext to stdout)
    #[arg(long, short = 'o')]
    pub output: Option<PathBuf>,
}

pub fn run(args: DecryptArgs) -> Result<()> {
    let secret_key = std::fs::read(&args.key)?;
    let ciphertext = Base64::decode_vec(&args.input)
        .map_err(|e| anyhow::anyhow!("invalid base64: {e}"))?;

    let config = EncryptConfig::default();
    let event = crypto::decrypt(&config, &secret_key, &ciphertext)?;

    match args.output {
        Some(path) => std::fs::write(path, &event.plaintext)?,
        None => println!("{}", String::from_utf8_lossy(&event.plaintext)),
    }

    Ok(())
}
