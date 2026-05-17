mod decrypt;
mod encrypt;
mod generate_keys;
mod serve;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "khyber", about = "Khyber encryption toolkit")]
pub struct Cli {
    #[arg(long, global = true, default_value = "/tmp/khyber.sock")]
    pub socket: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    GenerateKeys(generate_keys::GenerateKeysArgs),
    Encrypt(encrypt::EncryptArgs),
    Decrypt(decrypt::DecryptArgs),
    Serve(serve::ServeArgs),
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::GenerateKeys(args) => generate_keys::run(args, &cli.socket).await,
        Command::Encrypt(args) => encrypt::run(args, &cli.socket).await,
        Command::Decrypt(args) => decrypt::run(args, &cli.socket).await,
        Command::Serve(args) => serve::run(args, &cli.socket).await,
    }
}
