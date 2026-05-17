use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "khyber", about = "Khyber encryption toolkit")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generate a new key pair
    GenerateKeys,
    /// Encrypt a message
    Encrypt {
        /// Input to encrypt
        input: String,
    },
    /// Decrypt a message
    Decrypt {
        /// Input to decrypt
        input: String,
    },
}

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Command::GenerateKeys => {
            println!("generate-keys: generating key pair...");
        }
        Command::Encrypt { input } => {
            println!("encrypt: encrypting \"{}\"", input);
        }
        Command::Decrypt { input } => {
            println!("decrypt: decrypting \"{}\"", input);
        }
    }
}
