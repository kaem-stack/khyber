use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use kaem_sdk::modules::khyber::Algorithm;
use kaem_sdk::transport;
use tokio::net::{UnixListener, UnixStream};

use crate::crypto::{self, EncryptConfig};
use kaem_sdk::modules::khyber::ipc::{Request, Response};
use crate::keys::{self, Algorithm as LocalAlgorithm, KeyGenConfig};

#[derive(Args)]
pub struct ServeArgs {}

pub async fn run(_args: ServeArgs, socket: &PathBuf) -> Result<()> {
    if socket.exists() {
        std::fs::remove_file(socket)?;
    }

    let listener = UnixListener::bind(socket)?;
    eprintln!("listening on {}", socket.display());

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle(stream).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}

async fn handle(mut stream: UnixStream) -> Result<()> {
    let request: Request = transport::receive(&mut stream)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    let response = dispatch(request);

    transport::send(&mut stream, &response)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(())
}

fn dispatch(request: Request) -> Response {
    match request {
        Request::GenerateKeys(cmd) => {
            let local_alg = match cmd.algorithm {
                Algorithm::MlKem768 => LocalAlgorithm::MlKem768,
            };
            let config = KeyGenConfig::new(PathBuf::new()).with_algorithm(local_alg);
            match keys::generate(&config) {
                Ok(event) => Response::KeysGenerated(event),
                Err(e) => Response::Error(e.to_string()),
            }
        }
        Request::Encrypt(cmd) => {
            let config = EncryptConfig::default();
            match crypto::encrypt(&config, &cmd.public_key, &cmd.message) {
                Ok(event) => Response::Encrypted(event),
                Err(e) => Response::Error(e.to_string()),
            }
        }
        Request::Decrypt(cmd) => {
            let config = EncryptConfig::default();
            match crypto::decrypt(&config, &cmd.secret_key, &cmd.ciphertext) {
                Ok(event) => Response::Decrypted(event),
                Err(e) => Response::Error(e.to_string()),
            }
        }
    }
}
