mod cli;
mod crypto;
mod keys;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::run().await
}
