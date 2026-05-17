use std::path::PathBuf;

use anyhow::{bail, Result};
use base64ct::{Base64, Encoding as _};
use clap::Args;
use kaem_sdk::modules::khyber::decrypt::DecryptCommand;
use kaem_sdk::transport;
use tokio::net::UnixStream;

use kaem_sdk::modules::khyber::ipc::{Request, Response};

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

pub async fn run(args: DecryptArgs, socket: &PathBuf) -> Result<()> {
    let secret_key = std::fs::read(&args.key)?;
    let ciphertext = Base64::decode_vec(&args.input)
        .map_err(|e| anyhow::anyhow!("invalid base64: {e}"))?;

    let req = Request::Decrypt(DecryptCommand {
        ciphertext,
        secret_key,
    });

    let mut stream = UnixStream::connect(socket).await?;
    transport::send(&mut stream, &req).await.map_err(|e| anyhow::anyhow!("{e}"))?;
    let response: Response = transport::receive(&mut stream).await.map_err(|e| anyhow::anyhow!("{e}"))?;

    match response {
        Response::Decrypted(event) => match args.output {
            Some(path) => std::fs::write(path, &event.plaintext)?,
            None => println!("{}", String::from_utf8_lossy(&event.plaintext)),
        },
        Response::Error(e) => bail!("daemon error: {e}"),
        _ => bail!("unexpected response"),
    }

    Ok(())
}
