use chrono::TimeZone;
use splitwise::model::expenses::ListExpensesRequest;

#[tokio::main]
async fn main() {
    let client = splitwise::client::Client::default();

    // NOTE: This ID is non-group expenses. Change it to your actual group ID.
    let group_id = 0;

    let expenses = client
        .expenses()
        .list_expenses(ListExpensesRequest {
            group_id: Some(group_id),
            dated_after: Some(chrono::Utc.ymd(2022, 1, 1).and_hms(0, 0, 0)),
            ..ListExpensesRequest::default()
        })
        .await
        .unwrap();

    for expense in expenses.iter() {
        let id = expense.id.unwrap();
        let date = expense.date.unwrap();
        let cost = expense.cost.as_ref().unwrap();
        let desc = expense.description.as_ref().unwrap();

        println!(
            "Delete expense? --> Date: {}, Cost: {}, Desc: {} [y/N]",
            date, cost, desc
        );
        let y: String = text_io::read!();
        let y = y.to_lowercase();
        if y != *"y" {
            continue;
        }

        client.expenses().delete_expense(id).await.unwrap();
        println!("Successfully deleted expense");
    }
}
