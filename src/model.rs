use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// -----------------------------------------------------------------------------
// Common/Shared
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub xlarge: Option<String>,
}

// -----------------------------------------------------------------------------
// Users
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserWrapper {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub registration_status: Option<String>,
    pub picture: Image,
    pub notifications_read: Option<String>,
    pub notifications_count: Option<i64>,
    pub notifications: Option<UserNotifications>,
    pub default_currency: Option<String>,
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserNotifications {
    pub added_as_friend: Option<bool>,
    pub expense_added: Option<bool>,
    pub expense_updated: Option<bool>,
    pub bills: Option<bool>,
    pub payments: Option<bool>,
    pub monthly_summary: Option<bool>,
    pub announcements: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub locale: Option<String>,
    pub default_currency: Option<String>,
}

// -----------------------------------------------------------------------------
// Notifications
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsWrapper {
    pub notifications: Vec<Notification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub created_at: String,
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

// -----------------------------------------------------------------------------
// Other
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CurrenciesWrapper {
    pub currencies: Vec<Currency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub currency_code: String,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CategoriesWrapper {
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: Option<String>,
    pub icon_types: Option<HashMap<String, Image>>,
    pub subcategories: Option<Vec<Category>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceRequest {
    pub input: String,
    pub friend_id: Option<i64>,
    pub group_id: Option<i64>,
    pub autosave: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceResponse {
    pub expense: Expense,
    pub valid: bool,
    pub confidence: f64,
    pub error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expense {
    pub cost: String,
    pub description: String,
    pub details: String,
    pub date: String,
    pub repeat_interval: String,
    pub currency_code: String,
    pub category_id: i64,
    pub id: i64,
    pub group_id: i64,
    pub friendship_id: i64,
    pub expense_bundle_id: i64,
    pub repeats: bool,
    pub email_reminder: bool,
    pub email_reminder_in_advance: Option<i64>,
    pub next_repeat: String,
    pub comments_count: i64,
    pub payment: bool,
    pub transaction_confirmed: bool,
    pub repayments: Vec<Repayment>,
    pub created_at: String,
    pub created_by: User,
    pub updated_at: String,
    pub updated_by: User,
    pub deleted_at: String,
    pub deleted_by: User,
    pub category: Category,
    pub receipt: Receipt,
    pub users: Vec<ExpenseUser>,
    pub comments: Vec<Comment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repayment {
    pub from: i64,
    pub to: i64,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub large: String,
    pub original: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpenseUser {
    pub user: User,
    pub user_id: i64,
    pub paid_share: String,
    pub owed_share: String,
    pub net_balance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub content: String,
    pub comment_type: String,
    pub relation_type: String,
    pub relation_id: i64,
    pub created_at: String,
    pub deleted_at: String,
    pub user: Option<User>, // TODO: Guessing this is the main "User" type
}
