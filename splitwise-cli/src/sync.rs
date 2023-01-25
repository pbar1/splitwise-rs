use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use regex::{Regex, RegexBuilder};

use crate::mint::MintTransaction;

#[derive(Parser)]
pub(crate) struct Args {
    /// Path to file containing transactions to process
    file: PathBuf,

    /// Splitwise group ID to sync transactions with
    group_id: String,

    /// Regex filter for transaction account name
    #[clap(short, long, default_value = ".*", parse(try_from_str = build_regex_smartcase))]
    account: Regex,

    /// Regex filter for transaction description
    #[clap(short, long, default_value = ".*", parse(try_from_str = build_regex_smartcase))]
    description: Regex,

    /// Show incomes in addition to expenses
    #[clap(long)]
    all: bool,

    /// Only process this many transactions
    #[clap(long)]
    limit: Option<usize>,
}

pub(crate) async fn sync(args: Args) -> Result<()> {
    let txns = read_mint_transactions_from_file(args.file)?;
    let limit = args.limit.unwrap_or_else(|| txns.len());

    txns.iter()
        .filter(|txn| txn.is_expense || args.all)
        .filter(|txn| args.account.is_match(&txn.account_ref.name))
        .filter(|txn| args.description.is_match(&txn.description))
        .take(limit)
        .for_each(|txn| {
            println!(
                "{}: {} @ [{}] {}",
                txn.date, txn.amount, txn.account_ref.name, txn.description
            )
        });

    Ok(())
}

fn build_regex_smartcase(pattern: &str) -> Result<Regex> {
    let case_sensitive = pattern.chars().any(|c| c.is_ascii_uppercase());
    let re = RegexBuilder::new(pattern).case_insensitive(!case_sensitive).build()?;
    Ok(re)
}

fn read_mint_transactions_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<MintTransaction>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let transactions = serde_json::from_reader(reader)?;
    Ok(transactions)
}
