use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NotificationsWrapper {
    pub notifications: Vec<Notification>,
}

/// Notification of activity on the user's account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID.
    pub id: Option<i64>,

    /// Notification type.
    #[serde(rename = "type")]
    pub notification_type: Option<NotificationType>,

    /// Timestamp of when the notification was created.
    pub created_at: Option<chrono::DateTime<Utc>>,

    /// ID of the user who created the notification.
    pub created_by: Option<i64>,

    /// Notification source.
    pub source: Option<NotificationSource>,

    /// URL of the notification image.
    pub image_url: Option<String>,

    /// Shape of the image. One of:
    /// - `square`
    /// - `circle`
    pub image_shape: Option<String>,

    /// Notification content.
    pub content: Option<String>,
}

/// Notification source.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationSource {
    /// Notification source type.
    #[serde(rename = "type")]
    pub source_type: Option<String>,

    /// Notification source ID.
    pub id: Option<i64>,

    /// Notification source URL.
    pub url: Option<String>,
}

/// Indicates what the notification is about.
///
/// **Note:** Notification types may be added in the future without warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NotificationType {
    /// Expense added.
    ExpenseAdded,

    /// Expense updated.
    ExpenseUpdated,

    /// Expense deleted.
    ExpenseDeleted,

    /// Comment added.
    CommentAdded,

    /// Added to group.
    AddedToGroup,

    /// Removed from group.
    RemovedFromGroup,

    /// Group deleted.
    GroupDeleted,

    /// Group settings changed.
    GroupSettingsChanged,

    /// Added as friend.
    AddedAsFriend,

    /// Removed as friend.
    RemovedAsFriend,

    /// News. A URL should be included.
    News,

    /// Debt simplification.
    DebtSimplification,

    /// Group restored.
    GroupUndeleted,

    /// Expense restored.
    ExpenseUndeleted,

    /// Group currency conversion.
    GroupCurrencyConversion,

    /// Friend currency conversion.
    FriendCurrencyConversion,
}

/// Splitwise `get_notifications` request.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetNotificationsRequest {
    /// If provided, returns only notifications after this time.
    pub updated_after: Option<chrono::DateTime<Utc>>,

    /// Omit (or provide 0) to get the maximum number of notifications.
    /// Default: `0`.
    pub limit: Option<i64>,
}
