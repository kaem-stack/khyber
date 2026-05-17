mod cli;
mod crypto;
mod keys;

fn main() -> anyhow::Result<()> {
    cli::run()
}
