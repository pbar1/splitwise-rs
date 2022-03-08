use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct NotificationsWrapper {
    pub notifications: Vec<Notification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub created_at: chrono::DateTime<Utc>,
    pub created_by: i64,
    pub source: NotificationSource,
    pub image_url: String,
    pub image_shape: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationSource {
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
    News,
    DebtSimplification,
    GroupUndeleted,
    ExpenseUndeleted,
    GroupCurrencyConversion,
    FriendCurrencyConversion,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNotificationsRequest {
    pub updated_after: Option<chrono::DateTime<Utc>>,
    pub limit: Option<i64>,
}
