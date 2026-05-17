use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::Args;
use kaem_sdk::modules::khyber::{
    generate_keys::GenerateKeysCommand,
    ipc::{Request, Response},
    Algorithm as SdkAlgorithm,
};
use kaem_sdk::transport;
use tokio::net::UnixStream;

use crate::keys::{self, Algorithm, KeyGenConfig};

#[derive(Args)]
pub struct GenerateKeysArgs {
    #[arg(long, short = 'o')]
    pub out: PathBuf,

    #[arg(long, value_enum)]
    pub algorithm: Option<Algorithm>,
}

pub async fn run(args: GenerateKeysArgs, socket: &PathBuf) -> Result<()> {
    let config = KeyGenConfig::new(args.out).with_algorithm(args.algorithm.unwrap_or_default());
    let sdk_algorithm = match config.algorithm {
        Algorithm::MlKem768 => SdkAlgorithm::MlKem768,
    };
    let req = Request::GenerateKeys(GenerateKeysCommand { algorithm: sdk_algorithm });

    let mut stream = UnixStream::connect(socket).await?;
    transport::send(&mut stream, &req).await.map_err(|e| anyhow::anyhow!("{e}"))?;
    let response: Response = transport::receive(&mut stream).await.map_err(|e| anyhow::anyhow!("{e}"))?;

    match response {
        Response::KeysGenerated(keypair) => {
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
        }
        Response::Error(e) => bail!("daemon error: {e}"),
        _ => bail!("unexpected response"),
    }

    Ok(())
}
