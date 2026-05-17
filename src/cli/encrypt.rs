use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct EncryptArgs {
    pub input: String,
}

pub fn run(args: EncryptArgs) -> Result<()> {
    println!("encrypt: encrypting \"{}\"", args.input);
    Ok(())
}
