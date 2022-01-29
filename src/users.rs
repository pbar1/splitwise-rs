use crate::errors::{ErrorForbiddenOrNotFound, ErrorUnauthorized};
use crate::Client;
use anyhow::{bail, Error};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct UserService {
    client: Client,
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

impl UserService {
    pub fn new(client: Client) -> UserService {
        let service = UserService { client };
        service
    }

    pub async fn get_current_user(&self) -> Result<UserResponse, Error> {
        let path = String::from("/get_current_user");
        let response = self.client.get(path).await?;
        match response.status() {
            StatusCode::OK => {
                let decoded = response.json::<UserResponse>().await?;
                Ok(decoded)
            }
            StatusCode::UNAUTHORIZED => {
                let decoded = response.json::<ErrorUnauthorized>().await?;
                bail!(decoded.error)
            }
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => {
                let decoded = response.json::<ErrorForbiddenOrNotFound>().await?;
                bail!(decoded.errors.base.join("; "))
            }
            _ => bail!("unexpected HTTP status code: {}", response.status()),
        }
    }

    pub async fn get_user(&self, id: String) -> Result<UserResponse, Error> {
        let path = format!("{}/{}", "/get_user", id);
        let response = self.client.get(path).await?;
        match response.status() {
            StatusCode::OK => {
                let decoded = response.json::<UserResponse>().await?;
                Ok(decoded)
            }
            StatusCode::UNAUTHORIZED => {
                let decoded = response.json::<ErrorUnauthorized>().await?;
                bail!(decoded.error)
            }
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => {
                let decoded = response.json::<ErrorForbiddenOrNotFound>().await?;
                bail!(decoded.errors.base.join("; "))
            }
            _ => bail!("unexpected HTTP status code: {}", response.status()),
        }
    }

    pub async fn update_user(
        &self,
        id: u64,
        updates: UpdateUserRequest,
    ) -> Result<UserResponse, Error> {
        let path = format!("{}/{}", "/update_user", id);
        let response = self.client.post(path, &updates).await?;
        match response.status() {
            StatusCode::OK => {
                let decoded = response.json::<UserResponse>().await?;
                Ok(decoded)
            }
            StatusCode::UNAUTHORIZED => {
                let decoded = response.json::<ErrorUnauthorized>().await?;
                bail!(decoded.error)
            }
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => {
                let decoded = response.json::<ErrorForbiddenOrNotFound>().await?;
                bail!(decoded.errors.base.join("; "))
            }
            _ => bail!("unexpected HTTP status code: {}", response.status()),
        }
    }
}
