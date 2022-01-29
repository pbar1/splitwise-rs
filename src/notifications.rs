use crate::errors::{ErrorForbiddenOrNotFound, ErrorUnauthorized};
use crate::Client;
use anyhow::{bail, Error};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone)]
pub struct NotificationsService {
    client: Client,
}

// FIXME: Check if any more fields are nullable
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsResponse {
    pub notifications: Vec<Notification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: u64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub created_at: String,
    pub created_by: u64,
    pub source: Source,
    pub image_url: String,
    pub image_shape: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    pub id: i64,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NotificationType {
    ExpenseAdded,
    ExpenseUpdated,
    ExpenseDeleted,
    CommentAdded,
    AddedToGroup,
    RemovedFromGroup,
    GroupDeleted,
    GroupSettingsChanged,
    AddedAsFriend,
    RemovedAsFriend,
    News, // A URL should be included
    DebtSimplification,
    GroupUndeleted,
    ExpenseUndeleted,
    GroupCurrencyConversion,
    FriendCurrencyConversion,
}

impl NotificationsService {
    pub fn new(client: Client) -> NotificationsService {
        let service = NotificationsService { client };
        service
    }

    // FIXME: Implement query params
    pub async fn get_notifications(&self) -> Result<NotificationsResponse, Error> {
        let path = String::from("/get_notifications");
        let response = self.client.get(path).await?;
        match response.status() {
            StatusCode::OK => {
                let decoded = response.json::<NotificationsResponse>().await?;
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn get_notifications_works() {
        let api_key = std::env::var("SPLITWISE_API_KEY").unwrap();
        let client = crate::Client::new_default_http_client(api_key);
        let result = client.notifications().get_notifications().await.unwrap();
        assert_eq!(result, result);
    }
}
