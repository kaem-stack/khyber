use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct DecryptArgs {
    pub input: String,
}

pub fn run(args: DecryptArgs) -> Result<()> {
    println!("decrypt: decrypting \"{}\"", args.input);
    Ok(())
}
