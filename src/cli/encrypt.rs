use std::path::PathBuf;

use anyhow::{bail, Result};
use base64ct::{Base64, Encoding as _};
use clap::Args;
use kaem_sdk::modules::khyber::encrypt::EncryptCommand;
use kaem_sdk::transport;
use tokio::net::UnixStream;

use kaem_sdk::modules::khyber::ipc::{Request, Response};

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

pub async fn run(args: EncryptArgs, socket: &PathBuf) -> Result<()> {
    let public_key = std::fs::read(&args.key)?;
    let req = Request::Encrypt(EncryptCommand {
        message: args.message.into_bytes(),
        public_key,
    });

    let mut stream = UnixStream::connect(socket).await?;
    transport::send(&mut stream, &req).await.map_err(|e| anyhow::anyhow!("{e}"))?;
    let response: Response = transport::receive(&mut stream).await.map_err(|e| anyhow::anyhow!("{e}"))?;

    match response {
        Response::Encrypted(event) => {
            let encoded = Base64::encode_string(&event.ciphertext);
            match args.output {
                Some(path) => std::fs::write(path, &encoded)?,
                None => println!("{}", encoded),
            }
        }
        Response::Error(e) => bail!("daemon error: {e}"),
        _ => bail!("unexpected response"),
    }

    Ok(())
}
