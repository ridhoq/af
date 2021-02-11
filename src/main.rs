use anyhow::Result;
use clap::Clap;

mod cli;
mod fetch;

use cli::Opts;
use fetch::fetch;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Opts = Opts::parse();
    fetch(args).await?;
    Ok(())
}
