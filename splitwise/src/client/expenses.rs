use anyhow::bail;

use crate::client::client::join_errors;
use crate::client::client::Client;
use crate::model::expenses::CreateExpenseRequest;
use crate::model::expenses::Expense;
use crate::model::expenses::ExpenseWrapper;
use crate::model::expenses::ExpensesWrapper;
use crate::model::expenses::ListExpensesRequest;
use crate::model::expenses::UpdateExpenseRequest;
use crate::model::shared::Success;

/// Expenses.
///
/// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses)
#[derive(Debug)]
pub struct ExpensesSvc<'c> {
    client: &'c Client,
}

impl<'c> ExpensesSvc<'c> {
    /// Creates an instance of `ExpensesSvc`.
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get expense information.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses/paths/~1get_expense~1{id}/get)
    pub async fn get_expense(&self, id: i64) -> Result<Expense, anyhow::Error> {
        let url = self.client.base_url.join(&format!("get_expense/{}", id))?;
        let response: ExpenseWrapper = self.client.get(url).await?;
        Ok(response.expense)
    }

    /// List the current user's expenses.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses/paths/~1get_expenses/get)
    pub async fn list_expenses(
        &self,
        request: ListExpensesRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let mut url = self.client.base_url.join("get_expenses")?;
        let query = serde_qs::to_string(&request)?;
        url.set_query(Some(&query));
        let response: ExpensesWrapper = self.client.get(url).await?;
        Ok(response.expenses)
    }

    /// Creates an expense. You may either split an expense equally (only with
    /// `group_id` provided), or supply a list of shares.
    ///
    /// When splitting equally,
    /// the authenticated user is assumed to be the payer.
    ///
    /// When providing a list of shares, each share must include `paid_share`
    /// and `owed_share`, and must be identified by one of the following:
    /// - `email`, `first_name`, and `last_name`
    /// - `user_id`
    ///
    /// Note: The operation was successful only if `errors` is empty.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses/paths/~1create_expense/post)
    pub async fn create_expense(
        &self,
        request: CreateExpenseRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let url = self.client.base_url.join("create_expense")?;

        // User shares always take priority over equal split
        let mut request = request.clone();
        if request.users.is_some() {
            request.split_equally = false;
        } else {
            request.split_equally = true;
        }

        let response: ExpensesWrapper = self.client.post(url, &request).await?;

        if let Some(e) = response.errors {
            if !e.is_empty() {
                bail!(join_errors(&e))
            }
        }

        Ok(response.expenses)
    }

    /// Updates an expense. Parameters are the same as in `create_expense`, but
    /// you only need to include parameters that are changing from the previous
    /// values. If any values is supplied for `users`, all shares for the
    /// expense will be overwritten with the provided values.
    ///
    /// **Note:** The operation was successful only if `errors` is empty.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses/paths/~1update_expense~1{id}/post)
    pub async fn update_expense(
        &self,
        id: i64,
        request: UpdateExpenseRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("update_expense/{}", id))?;

        let response: ExpensesWrapper = self.client.post(url, &request).await?;

        if let Some(e) = response.errors {
            if !e.is_empty() {
                bail!(join_errors(&e))
            }
        }

        Ok(response.expenses)
    }

    /// Delete an expense.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses/paths/~1delete_expense~1{id}/post)
    pub async fn delete_expense(&self, id: i64) -> Result<(), anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("delete_expense/{}", id))?;

        let response: Success = self.client.post_no_body(url).await?;

        if response.success {
            return Ok(());
        }
        if let Some(e) = response.errors {
            if !e.is_empty() {
                bail!(join_errors(&e))
            }
        }
        bail!("unknown error deleting expense: {}", id)
    }

    /// Restore an expense.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/expenses/paths/~1undelete_expense~1{id}/post)
    pub async fn restore_expense(&self, id: i64) -> Result<(), anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("undelete_expense/{}", id))?;

        let response: Success = self.client.post_no_body(url).await?;

        if response.success {
            return Ok(());
        }

        if let Some(e) = response.errors {
            if !e.is_empty() {
                bail!(join_errors(&e))
            }
        }

        bail!("unknown error undeleting expense")
    }
}

#[cfg(test)]
mod integration_tests {
    use std::ops::Index;

    use test_log::test;

    use super::*;
    use crate::model::expenses::UserShare;

    #[test(tokio::test)]
    async fn create_update_get_delete_undelete_list_expense_works() {
        let client = Client::default();

        let request = CreateExpenseRequest {
            cost: "50.00".to_string(),
            description: format!("Fake full-flow expense {}", chrono::Utc::now()),
            group_id: 0,
            ..CreateExpenseRequest::default()
        };
        let response = client.expenses().create_expense(request).await.unwrap();

        let created_expense = response.index(0);
        let id = created_expense.id.unwrap();

        let request = UpdateExpenseRequest {
            description: Some("new fake description".to_string()),
            group_id: 0,
            ..UpdateExpenseRequest::default()
        };
        client.expenses().update_expense(id, request).await.unwrap();

        client.expenses().get_expense(id).await.unwrap();
        client.expenses().delete_expense(id).await.unwrap();
        client.expenses().restore_expense(id).await.unwrap();
        client.expenses().delete_expense(id).await.unwrap();
        client
            .expenses()
            .list_expenses(ListExpensesRequest::default())
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn create_expense_equally_works() {
        let request = CreateExpenseRequest {
            cost: "50.00".to_string(),
            description: "Fake equal expense".to_string(),
            group_id: 0,
            ..CreateExpenseRequest::default()
        };
        let client = Client::default();
        let response = client.expenses().create_expense(request).await.unwrap();
        let expense_id = response.index(0).id.unwrap();
        let _ = client.expenses().delete_expense(expense_id).await.unwrap();
    }

    #[test(tokio::test)]
    async fn create_expense_by_shares_works() {
        let request = CreateExpenseRequest {
            cost: "1".to_string(),
            description: "Fake by-shares expense".to_string(),
            group_id: 0,
            users: Some(vec![UserShare {
                user_id: Some(48903837),
                owed_share: Some("1".to_string()),
                paid_share: Some("1".to_string()),
                ..UserShare::default()
            }]),
            ..CreateExpenseRequest::default()
        };
        let client = Client::default();
        let response = client.expenses().create_expense(request).await.unwrap();
        let expense_id = response.index(0).id.unwrap();
        let _ = client.expenses().delete_expense(expense_id).await.unwrap();
    }
}
