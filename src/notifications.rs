use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::client::Client;

#[derive(Debug)]
pub struct NotificationsSvc<'c> {
    client: &'c Client,
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

impl<'c> NotificationsSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    // FIXME: Implement query params
    pub async fn get_notifications(&self) -> Result<NotificationsResponse, anyhow::Error> {
        let path = "/get_notifications";
        let response = self.client.get(path).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn get_notifications_works() {
        let mock_server = MockServer::start().await;

        // FIXME match not just any
        Mock::given(any()).respond_with(ResponseTemplate::new(200).set_body_raw(r##"
{
  "notifications": [
    {
      "id": 32514315,
      "type": 0,
      "created_at": "2019-08-24T14:15:22Z",
      "created_by": 2,
      "source": {
        "type": "Expense",
        "id": 865077,
        "url": "string"
      },
      "image_url": "https://s3.amazonaws.com/splitwise/uploads/notifications/v2/0-venmo.png",
      "image_shape": "square",
      "content": "<strong>You</strong> paid <strong>Jon H.</strong>.<br><font color=\\\"#5bc5a7\\\">You paid $23.45</font>"
    }
  ]
}
"##, "application/json")).mount(&mock_server).await;

        let result = Client::default()
            .with_base_url(mock_server.uri())
            .notifications()
            .get_notifications()
            .await
            .unwrap();

        assert_eq!(result, result);
    }
}
