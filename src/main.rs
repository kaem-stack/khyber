mod cli;
mod keys;

fn main() -> anyhow::Result<()> {
    cli::run()
}
