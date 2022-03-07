use serde::{Deserialize, Serialize};

use crate::client::Client;

#[derive(Debug)]
pub struct UsersSvc<'c> {
    client: &'c Client,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserResponse {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub registration_status: String,
    pub picture: Picture,
    pub notifications_read: Option<String>,
    pub notifications_count: Option<u64>,
    pub notifications: Option<Notifications>,
    pub default_currency: Option<String>,
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Picture {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Notifications {
    pub added_as_friend: bool,
    pub expense_added: bool,
    pub expense_updated: bool,
    pub bills: bool,
    pub payments: bool,
    pub monthly_summary: bool,
    pub announcements: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub locale: Option<String>,
    pub default_currency: Option<String>,
}

impl<'c> UsersSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub async fn get_current_user(&self) -> Result<UserResponse, anyhow::Error> {
        let path = "/get_current_user";
        let response = self.client.get(path).await?;
        Ok(response)
    }

    pub async fn get_user(&self, id: &str) -> Result<UserResponse, anyhow::Error> {
        let path = format!("{}/{}", "/get_user", id);
        let response = self.client.get(&path).await?;
        Ok(response)
    }

    pub async fn update_user(
        &self,
        id: u64,
        updates: UpdateUserRequest,
    ) -> Result<UserResponse, anyhow::Error> {
        let path = format!("{}/{}", "/update_user", id);
        let response = self.client.post(&path, &updates).await?;
        Ok(response)
    }
}
