use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Splitwise CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync transactions matching rules to Splitwise groups
    Sync {
        /// Path to transaction file; if not set, will read from stdin
        #[clap(short, long, value_parser, value_name = "FILE")]
        file: Option<PathBuf>,
    },
}

fn main() {
    let _cli = Cli::parse();
}
