use anyhow::bail;

use crate::{
    client::Client,
    model::{
        CreateExpenseEquallyRequest, Expense, ExpenseWrapper, ExpensesWrapper, GetExpensesRequest,
    },
};

#[derive(Debug)]
pub struct ExpensesSvc<'c> {
    client: &'c Client,
}

impl<'c> ExpensesSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub async fn get_expense(&self, id: i64) -> Result<Expense, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path(&format!("get_expense/{}", id));
        let response: ExpenseWrapper = self.client.get(url).await?;
        Ok(response.expense)
    }

    pub async fn get_expenses(
        &self,
        request: GetExpensesRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path("get_expenses");
        let query = serde_qs::to_string(&request)?;
        url.set_query(Some(&query));
        let response: ExpensesWrapper = self.client.get(url).await?;
        Ok(response.expenses)
    }

    pub async fn create_expense_equally(
        &self,
        request: CreateExpenseEquallyRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let request = CreateExpenseEquallyRequest {
            split_equally: true,
            ..request
        };

        let mut url = self.client.base_url.clone();
        url.set_path("create_expense");
        let response: ExpensesWrapper = self.client.post(url, &request).await?;

        // TODO: Clean this up, it'll be needed elsewhere as well
        if let Some(e) = response.errors {
            if !e.is_empty() {
                let mut error_text = String::from("");
                for (k, v) in e {
                    error_text.push_str("* ");
                    error_text.push_str(&k);
                    error_text.push_str(": ");
                    error_text.push_str(&v.join("; "));
                }
                bail!(error_text)
            }
        }
        Ok(response.expenses)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use super::*;

    #[tokio::test]
    async fn get_expense_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_expense/0");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/expenses/get_expense.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .expenses()
            .get_expense(0)
            .await
            .unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn get_expenses_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_expenses");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/expenses/get_expenses.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .expenses()
            .get_expenses(GetExpensesRequest::default())
            .await
            .unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn create_expense_equally_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/create_expense");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/expenses/create_expense.POST.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .expenses()
            .create_expense_equally(CreateExpenseEquallyRequest::default())
            .await
            .unwrap();
        mock.assert();
    }
}
