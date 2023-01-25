mod mint;
mod sync;

use anyhow::Result;
use clap::Parser;

use crate::sync::sync;

/// Splitwise CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
enum Cli {
    Sync(sync::Args),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Sync(args) => sync(args).await?,
    };

    Ok(())
}
