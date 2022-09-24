use chrono::{NaiveDate, TimeZone, Utc};
use serde::{de::Deserializer, Deserialize};
use splitwise::model::expenses::{CreateExpenseRequest, ListExpensesRequest};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ChaseTransaction {
    #[serde(rename = "Transaction Date")]
    #[serde(deserialize_with = "deserialize")]
    transaction_date: chrono::NaiveDate,

    #[serde(rename = "Post Date")]
    #[serde(deserialize_with = "deserialize")]
    post_date: chrono::NaiveDate,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Category")]
    category: String,

    #[serde(rename = "Type")]
    transaction_type: String,

    #[serde(rename = "Amount")]
    amount: f32,

    #[serde(rename = "Memo")]
    memo: String,
}

fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}

#[tokio::main]
async fn main() {
    println!("Sync Chase transactions to Splitwise!");

    let group_id = 16263273;
    let client = splitwise::client::Client::default();

    // Get a list of all expenses in a group since a certain date
    let expenses = client
        .expenses()
        .list_expenses(ListExpensesRequest {
            group_id: Some(group_id),
            dated_after: Some(chrono::Utc.ymd(2021, 12, 31).and_hms(0, 0, 0)),
            ..ListExpensesRequest::default()
        })
        .await
        .unwrap();

    let mut rdr = csv::Reader::from_path("example.csv").unwrap();

    'outer: for result in rdr.deserialize() {
        let txn: ChaseTransaction = result.unwrap();

        // Constrain to only food/drink over $30
        if !txn.category.to_lowercase().contains("food") || -txn.amount < 30.00 {
            continue;
        }

        // Check is expense already exists
        // TODO: Big bad nested loop, fix this
        for expense in expenses.iter() {
            if expense.id == None
                || expense.date == None
                || expense.description == None
                || expense.cost == None
            {
                continue;
            }
            let cost = expense.cost.as_ref().unwrap().parse::<f32>().unwrap();
            let expense_ndate = expense.date.unwrap().naive_utc().date();
            let desc = expense.description.as_ref().unwrap();

            let amount_delta = (-txn.amount - cost).abs();
            let days_delta = txn
                .transaction_date
                .signed_duration_since(expense_ndate)
                .num_days()
                .abs();

            if amount_delta < 1.00 && days_delta < 2 {
                // Expense has already been submitted
                println!(
                    "Expense possibly already in -> have: '{}', see: '{}'",
                    desc, txn.description
                );
                continue 'outer;
            }
        }

        // Propose creating expense
        println!("Create expense? {:?} [y/N]", txn);
        let y: String = text_io::read!();
        let y = y.to_lowercase();
        if y != "y".to_string() {
            continue;
        }

        let date = Utc
            .from_local_date(&txn.transaction_date)
            .unwrap()
            .and_hms(0, 0, 0)
            + chrono::Duration::days(1);
        let res = client
            .expenses()
            .create_expense(CreateExpenseRequest {
                cost: format!("{:.2}", -txn.amount),
                description: txn.description,
                details: None,
                date,
                repeat_interval: "never".to_string(),
                currency_code: "USD".to_string(),
                category_id: 0,
                group_id,
                split_equally: true,
                users: None,
            })
            .await;

        match res {
            Ok(_) => println!("Successfully created expense!"),
            Err(e) => println!("Error creating expense: {}", e),
        }
    }
}
