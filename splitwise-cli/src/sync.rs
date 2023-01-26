use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Result;
use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use clap::Parser;
use dialoguer::Confirm;
use regex::{Regex, RegexBuilder};
use splitwise::model::expenses::{CreateExpenseRequest, Expense, ListExpensesRequest};

use crate::mint::MintTransaction;

#[derive(Parser)]
pub(crate) struct Args {
    /// Path to file containing transactions to process
    file: PathBuf,

    /// Splitwise group ID to sync transactions with
    group_id: i64,

    /// Limit to transactions after this date, inclusive
    #[clap(long)]
    after: Option<NaiveDate>,

    /// Limit to transactions before this date, inclusive
    #[clap(long)]
    before: Option<NaiveDate>,

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
    let after_date = args.after.unwrap_or(NaiveDate::MIN);
    let before_date = args.before.unwrap_or(NaiveDate::MAX);

    // Fetch existing expenses from Splitwise
    let client = splitwise::client::Client::default();
    let expenses = client
        .expenses()
        .list_expenses(ListExpensesRequest {
            group_id: Some(args.group_id),
            limit: Some(10000),
            ..Default::default()
        })
        .await?;
    println!(
        "Found {} expenses in Splitwise group {}",
        expenses.len(),
        args.group_id
    );

    // Filter Mint transactions and sync them to Splitwise
    for t in txns
        .iter()
        .filter(|txn| txn.date.signed_duration_since(after_date).num_days() >= 0)
        .filter(|txn| txn.date.signed_duration_since(before_date).num_days() <= 0)
        .filter(|txn| txn.is_expense || args.all)
        .filter(|txn| args.account.is_match(&txn.account_ref.name))
        .filter(|txn| args.description.is_match(&txn.description))
        .take(limit)
        .filter(|txn| !expense_exists(&expenses, txn))
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
    {
        if args.dry_run {
            continue;
        }

        let res = client
            .expenses()
            .create_expense(CreateExpenseRequest {
                cost: format!("{:.2}", -t.amount),
                description: t.description.clone(),
                details: Some(format!("mint:{}", t.id)),
                date: Utc.from_utc_datetime(&t.date.and_time(NaiveTime::default())),
                repeat_interval: "never".to_string(),
                currency_code: "USD".to_string(), // FIXME: Don't hardcode USD
                category_id: 0,
                group_id: args.group_id,
                split_equally: true,
                users: None,
            })
            .await;

        if let Err(e) = res {
            println!("Failed creating expense: {}", e);
        }
    }

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

// TODO: Concept of strict vs fuzzy matching of transaction to expense. Making
// a Splitwise expense could prepend `mint:<transaction_id>` to details, then
// when listing it could be used as a key to build a hashmap for check.
fn expense_exists(expenses: &[Expense], txn: &MintTransaction) -> bool {
    let days_tolerance = 2;
    let amount_tolerance = 1.0;

    for expense in expenses {
        // Short-circuit if the Mint transaction ID is found in the expense details
        if expense.details.is_some()
            && expense
                .details
                .clone()
                .unwrap()
                .contains(&format!("mint:{}", txn.id))
        {
            return true;
        }

        let expense_date = match expense.date {
            Some(datetime) => datetime.date_naive(),
            None => continue,
        };
        let expense_cost = match expense.cost.iter().flat_map(|s| s.parse::<f32>()).next() {
            Some(cost) => cost,
            None => continue,
        };

        let days_delta = txn
            .date
            .signed_duration_since(expense_date)
            .num_days()
            .saturating_abs();
        let amount_delta = (-txn.amount - expense_cost as f64).abs();

        // Check if the transaction roughly matches the expense
        if days_delta < days_tolerance && amount_delta < amount_tolerance {
            // FIXME: Fuzzy match on description
            return true;
        }
    }

    false
}
