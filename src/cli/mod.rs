mod decrypt;
mod encrypt;
mod generate_keys;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "khyber", about = "Khyber encryption toolkit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    GenerateKeys(generate_keys::GenerateKeysArgs),
    Encrypt(encrypt::EncryptArgs),
    Decrypt(decrypt::DecryptArgs),
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::GenerateKeys(args) => generate_keys::run(args),
        Command::Encrypt(args) => encrypt::run(args),
        Command::Decrypt(args) => decrypt::run(args),
    }
}
