use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::Parser;
use dialoguer::Confirm;
use regex::{Regex, RegexBuilder};
use splitwise::model::expenses::{Expense, ListExpensesRequest};

use crate::mint::MintTransaction;

#[derive(Parser)]
pub(crate) struct Args {
    /// Path to file containing transactions to process
    file: PathBuf,

    /// Splitwise group ID to sync transactions with
    group_id: i64,

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

    /// Assume yes to all prompts (ie, non-interactive)
    #[clap(long)]
    assume_yes: bool,

    /// Don't write any data back to Splitwise
    #[clap(long)]
    dry_run: bool,
}

pub(crate) async fn sync(args: Args) -> Result<()> {
    let txns = read_mint_transactions_from_file(args.file)?;
    let limit = args.limit.unwrap_or_else(|| txns.len());

    // Fetch existing expenses from Splitwise
    let client = splitwise::client::Client::default();
    let existing = client
        .expenses()
        .list_expenses(ListExpensesRequest {
            group_id: Some(args.group_id),
            limit: Some(10000),
            ..Default::default()
        })
        .await?;
    println!(
        "Found {} expenses in Splitwise group {}",
        existing.len(),
        args.group_id
    );

    // Filter Mint transactions and sync them to Splitwise
    txns.iter()
        .filter(|txn| txn.is_expense || args.all)
        .filter(|txn| args.account.is_match(&txn.account_ref.name))
        .filter(|txn| args.description.is_match(&txn.description))
        .take(limit)
        .filter(|txn| {
            let prompt = format!(
                "{}: {} @ [{}] {}  -- Sync?",
                txn.date, txn.amount, txn.account_ref.name, txn.description
            );
            Confirm::new()
                .with_prompt(prompt)
                .interact()
                .unwrap_or(false)
        })
        .for_each(|txn| {
            let _ = expense_exists(&existing, txn);
        });

    Ok(())
}

/// Builds a Regex from the given pattern using smart case sensitivity. If the
/// pattern contains any uppercase characters, then the Regex will be case
/// sensitive, otherwise it will not.
fn build_regex_smartcase(pattern: &str) -> Result<Regex> {
    let has_uppercase = pattern.chars().any(|c| c.is_ascii_uppercase());
    let re = RegexBuilder::new(pattern)
        .case_insensitive(!has_uppercase)
        .build()?;
    Ok(re)
}

fn read_mint_transactions_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<MintTransaction>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let transactions = serde_json::from_reader(reader)?;
    Ok(transactions)
}

fn expense_exists(expenses: &[Expense], _txn: &MintTransaction) -> bool {
    for _expense in expenses {
        break;
    }
    false
}
