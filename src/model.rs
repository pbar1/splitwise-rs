use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

// -----------------------------------------------------------------------------
// Users
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserWrapper {
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub registration_status: Option<String>,
    pub picture: Option<Image>,
    pub custom_picture: Option<bool>,
    pub notifications_read: Option<String>,
    pub notifications_count: Option<i64>,
    pub notifications: Option<UserNotifications>,
    pub default_currency: Option<String>,
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub xlarge: Option<String>,
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
// Groups
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Friends
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Expenses
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ExpenseWrapper {
    pub expense: Expense,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ExpensesWrapper {
    pub expenses: Vec<Expense>,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expense {
    pub cost: Option<String>,
    pub description: Option<String>,
    pub details: Option<String>,
    pub date: Option<chrono::DateTime<Utc>>,
    pub repeat_interval: Option<String>, // TODO: Make this an enum
    pub currency_code: Option<String>,
    pub id: Option<i64>,
    pub group_id: Option<i64>,
    pub friendship_id: Option<i64>,
    pub expense_bundle_id: Option<i64>,
    pub repeats: Option<bool>,
    pub email_reminder: Option<bool>,
    pub email_reminder_in_advance: Option<i64>,
    pub next_repeat: Option<chrono::DateTime<Utc>>,
    pub comments_count: Option<i64>,
    pub payment: Option<bool>,
    pub creation_method: Option<String>,
    pub transaction_method: Option<String>,
    pub transaction_confirmed: Option<bool>,
    pub transaction_id: Option<i64>,
    pub transaction_status: Option<String>,
    pub repayments: Option<Vec<Repayment>>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub created_by: Option<User>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
    pub updated_by: Option<User>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
    pub deleted_by: Option<User>,
    pub category: Option<Category>,
    pub receipt: Option<Receipt>,
    pub users: Option<Vec<ExpenseUser>>,
    pub comments: Option<Vec<Comment>>,
}

pub enum RepeatInterval {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repayment {
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub amount: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Receipt {
    pub large: Option<String>,
    pub original: Option<String>,
}

// TODO: Could this be used for CreateExpenseByShares user?
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpenseUser {
    pub user: Option<User>,
    pub user_id: Option<i64>,
    pub paid_share: Option<String>,
    pub owed_share: Option<String>,
    pub net_balance: Option<String>,
}

// TODO: This may move into the "comments" section
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub content: String,
    pub comment_type: String,
    pub relation_type: String,
    pub relation_id: i64,
    pub created_at: chrono::DateTime<Utc>,
    pub deleted_at: Option<chrono::DateTime<Utc>>,
    pub user: Option<User>, // TODO: Guessing this is the main "User" type
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExpensesRequest {
    pub group_id: Option<i64>,
    pub friend_id: Option<i64>,
    pub dated_after: Option<chrono::DateTime<Utc>>,
    pub dated_before: Option<chrono::DateTime<Utc>>,
    pub updated_after: Option<chrono::DateTime<Utc>>,
    pub updated_before: Option<chrono::DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateExpenseRequest {
    pub cost: String,
    pub description: String,
    pub details: Option<String>,
    pub date: chrono::DateTime<Utc>,
    pub repeat_interval: String,
    pub currency_code: String,
    pub category_id: i64,
    pub group_id: i64,
    pub split_equally: bool,

    #[serde(flatten)]
    #[serde(serialize_with = "users_by_shares_ser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserByShares>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateExpenseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<Utc>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,

    pub group_id: i64,

    #[serde(flatten)]
    #[serde(serialize_with = "users_by_shares_ser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserByShares>>,
}

impl Default for CreateExpenseRequest {
    fn default() -> Self {
        Self {
            cost: "0.00".to_string(),
            description: "".to_string(),
            details: None,
            date: chrono::Utc::now(),
            repeat_interval: "never".to_string(),
            currency_code: "USD".to_string(),
            category_id: 0,
            group_id: 0,
            split_equally: true,
            users: None,
        }
    }
}

fn users_by_shares_ser<S: Serializer>(
    vec: &Option<Vec<UserByShares>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut map = HashMap::new();

    if let Some(v) = vec {
        for (i, user) in v.iter().enumerate() {
            if let Some(ref user_id) = user.user_id {
                map.insert(format!("users__{}__user_id", i), user_id.to_string());
            }
            if let Some(ref first_name) = user.first_name {
                map.insert(format!("users__{}__first_name", i), first_name.to_string());
            }
            if let Some(ref last_name) = user.last_name {
                map.insert(format!("users__{}__last_name", i), last_name.to_string());
            }
            if let Some(ref email) = user.email {
                map.insert(format!("users__{}__email", i), email.to_string());
            }
            if let Some(ref paid_share) = user.paid_share {
                map.insert(format!("users__{}__paid_share", i), paid_share.to_string());
            }
            if let Some(ref owed_share) = user.owed_share {
                map.insert(format!("users__{}__owed_share", i), owed_share.to_string());
            }
        }
    }

    map.serialize(serializer)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserByShares {
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,

    /// Decimal amount as a string with 2 decimal places. The amount this user paid for the expense
    pub paid_share: Option<String>,

    /// Decimal amount as a string with 2 decimal places. The amount this user owes for the expense
    pub owed_share: Option<String>,
}

// -----------------------------------------------------------------------------
// Comments
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Notifications
// -----------------------------------------------------------------------------

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNotificationsRequest {
    pub updated_after: Option<chrono::DateTime<Utc>>,
    pub limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationsWrapper {
    pub notifications: Vec<Notification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub created_at: String, // TODO: chrono
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceResponse {
    pub expense: Option<Expense>,
    pub valid: Option<bool>,
    pub confidence: Option<f64>,
    pub error: Option<String>,
}
