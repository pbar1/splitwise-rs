use anyhow::bail;

use crate::{
    client::{join_errors, Client},
    model::{
        expenses::{
            CreateExpenseRequest, Expense, ExpenseWrapper, ExpensesWrapper, GetExpensesRequest,
            UpdateExpenseRequest,
        },
        Success,
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
        let url = self.client.base_url.join(&format!("get_expense/{}", id))?;
        let response: ExpenseWrapper = self.client.get(url).await?;
        Ok(response.expense)
    }

    pub async fn get_expenses(
        &self,
        request: GetExpensesRequest,
    ) -> Result<Vec<Expense>, anyhow::Error> {
        let mut url = self.client.base_url.join("get_expenses")?;
        let query = serde_qs::to_string(&request)?;
        url.set_query(Some(&query));
        let response: ExpensesWrapper = self.client.get(url).await?;
        Ok(response.expenses)
    }

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

    pub async fn undelete_expense(&self, id: i64) -> Result<(), anyhow::Error> {
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
    use crate::model::expenses::UserByShares;

    #[test(tokio::test)]
    async fn create_update_get_delete_undelete_list_expense_works() {
        let client = Client::default();

        let request = CreateExpenseRequest {
            cost: "50.00".to_string(),
            description: format!("Fake full-flow expense {}", chrono::Utc::now()),
            group_id: 30331347,
            ..CreateExpenseRequest::default()
        };
        let response = client.expenses().create_expense(request).await.unwrap();

        let created_expense = response.index(0);
        let id = created_expense.id.unwrap();

        let request = UpdateExpenseRequest {
            description: Some("new fake description".to_string()),
            group_id: 30331347,
            ..UpdateExpenseRequest::default()
        };
        client.expenses().update_expense(id, request).await.unwrap();

        client.expenses().get_expense(id).await.unwrap();
        client.expenses().delete_expense(id).await.unwrap();
        client.expenses().undelete_expense(id).await.unwrap();
        client.expenses().delete_expense(id).await.unwrap();
        client
            .expenses()
            .get_expenses(GetExpensesRequest::default())
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn create_expense_equally_works() {
        let request = CreateExpenseRequest {
            cost: "50.00".to_string(),
            description: "Fake equal expense".to_string(),
            group_id: 30331347,
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
            group_id: 30331347,
            users: Some(vec![UserByShares {
                user_id: Some(47829677),
                owed_share: Some("1".to_string()),
                paid_share: Some("1".to_string()),
                ..UserByShares::default()
            }]),
            ..CreateExpenseRequest::default()
        };
        let client = Client::default();
        let response = client.expenses().create_expense(request).await.unwrap();
        let expense_id = response.index(0).id.unwrap();
        let _ = client.expenses().delete_expense(expense_id).await.unwrap();
    }
}
