use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::model::{
    comments::Comment,
    other::Category,
    shared::{Debt, Image},
    users::User,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ExpenseWrapper {
    pub expense: Expense,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ExpensesWrapper {
    pub expenses: Vec<Expense>,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

/// Splitwise expense.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expense {
    /// A string representation of a decimal value, limited to 2 decimal places.
    pub cost: Option<String>,

    /// A short description of the expense.
    pub description: Option<String>,

    /// Also known as "notes".
    pub details: Option<String>,

    /// The date and time the expense took place. May differ from `created_at`.
    pub date: Option<chrono::DateTime<chrono::Utc>>,

    // TODO: Make this an enum
    /// Cadence at which the expense repeats. One of:
    /// - `never`
    /// - `weekly`
    /// - `fortnightly`
    /// - `monthly`
    /// - `yearly`
    pub repeat_interval: Option<String>,

    /// A currency code. Must be in the list from `get_currencies`.
    pub currency_code: Option<String>,

    /// A category ID from `get_categories`.
    pub category_id: Option<i64>,

    /// Expense ID.
    pub id: Option<i64>,

    /// Null if the expense is not associated with a group.
    pub group_id: Option<i64>,

    /// Null if the expense is not associated with a friendship.
    pub friendship_id: Option<i64>,

    /// TODO: Unknown.
    pub expense_bundle_id: Option<i64>,

    /// Whether the expense recurs automatically.
    pub repeats: Option<bool>,

    /// Whether a reminder will be sent to involved users in advance of the next
    /// occurrence of a recurring expense. Only applicable if the expense
    /// recurs.
    pub email_reminder: Option<bool>,

    /// Number of days in advance to remind involved users about the next
    /// occurrence of a new expense. Only applicable if the expense recurs. One
    /// of:
    /// - `null`
    /// - `-1`
    /// - `0`
    /// - `1`
    /// - `2`
    /// - `3`
    /// - `4`
    /// - `5`
    /// - `6`
    /// - `7`
    /// - `14`
    pub email_reminder_in_advance: Option<i64>,

    /// The date of the next occurrence of a recurring expense. Only applicable
    /// if the expense recurs.
    pub next_repeat: Option<chrono::DateTime<chrono::Utc>>,

    /// Number of comments left on the expense.
    pub comments_count: Option<i64>,

    /// Whether this was a payment between users.
    pub payment: Option<bool>,

    /// Creation method.
    pub creation_method: Option<String>,

    /// Transaction method.
    pub transaction_method: Option<String>,

    /// If a payment was made via an integrated third party service, whether it
    /// was confirmed by that service.
    pub transaction_confirmed: Option<bool>,

    /// Transaction ID.
    pub transaction_id: Option<i64>,

    /// Transaction status.
    pub transaction_status: Option<String>,

    /// List of debts between users.
    pub repayments: Option<Vec<Debt>>,

    /// The date and time the expense was created on Splitwise.
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// User that created the expense.
    pub created_by: Option<User>,

    /// The last time the expense was updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// User that updated the expense.
    pub updated_by: Option<User>,

    /// If the expense was deleted, when it was deleted.
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,

    /// User that deleted the expense.
    pub deleted_by: Option<User>,

    /// Category of the expense.
    pub category: Option<Category>,

    /// Image of the receipt for the expense.
    pub receipt: Option<Image>,

    /// Users with share information associated with the expense.
    pub users: Option<Vec<UserShare>>,

    /// Comments on the expense.
    pub comments: Option<Vec<Comment>>,
}

/// Splitwise `get_expenses` request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListExpensesRequest {
    /// If provided, only expenses in that group will be returned, and
    /// `friend_id` will be ignored.
    pub group_id: Option<i64>,

    /// ID of another user. If provided, only expenses between the current and
    /// provided user will be returned.
    pub friend_id: Option<i64>,

    /// Filter to expenses after this date.
    pub dated_after: Option<chrono::DateTime<chrono::Utc>>,

    /// Filter to expenses before this date.
    pub dated_before: Option<chrono::DateTime<chrono::Utc>>,

    /// Filter to expenses updated after this date.
    pub updated_after: Option<chrono::DateTime<chrono::Utc>>,

    /// Filter to expenses updated before this date.
    pub updated_before: Option<chrono::DateTime<chrono::Utc>>,

    /// Maximum number of expenses to return.
    /// Default: `20`
    pub limit: Option<i64>,

    /// Offset in the returned set of expenses.
    /// Default: `0`
    pub offset: Option<i64>,
}

/// Splitwise `create_expense` request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateExpenseRequest {
    /// A string representation of a decimal value, limited to 2 decimal places.
    pub cost: String,

    /// A short description of the expense.
    pub description: String,

    /// Also known as "notes."
    pub details: Option<String>,

    /// The date and time the expense took place. May differ from `created_at`.
    pub date: chrono::DateTime<chrono::Utc>,

    // TODO: Make this an enum
    /// Cadence at which the expense repeats. One of:
    /// - `never`
    /// - `weekly`
    /// - `fortnightly`
    /// - `monthly`
    /// - `yearly`
    pub repeat_interval: String,

    /// A currency code. Must be in the list from `get_currencies`.
    pub currency_code: String,

    /// A category id from `get_categories`.
    pub category_id: i64,

    /// The group to put this expense in.
    pub group_id: i64,

    /// Whether to split the expense equally among users.
    pub split_equally: bool,

    /// Users by share if not splitting the expense equally.
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_option_vec_user_by_shares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserShare>>,
}

/// Splitwise `update_expense` request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateExpenseRequest {
    /// A string representation of a decimal value, limited to 2 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,

    /// A short description of the expense.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Also known as "notes."
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// The date and time the expense took place. May differ from `created_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::Utc>>,

    // TODO: Make this an enum
    /// Cadence at which the expense repeats. One of:
    /// - `never`
    /// - `weekly`
    /// - `fortnightly`
    /// - `monthly`
    /// - `yearly`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<String>,

    /// A currency code. Must be in the list from `get_currencies`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,

    /// A category id from `get_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,

    /// The group to put this expense in.
    pub group_id: i64,

    /// Users by share if not splitting the expense equally.
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_option_vec_user_by_shares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserShare>>,
}

/// User with share information associated with the expense.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserShare {
    /// User associated with the expense.
    pub user: Option<User>,

    /// User ID.
    pub user_id: Option<i64>,

    /// User first name.
    pub first_name: Option<String>,

    /// User last name.
    pub last_name: Option<String>,

    /// User email address.
    pub email: Option<String>,

    /// Decimal amount as a string with 2 decimal places. The amount this user
    /// paid for the expense
    pub paid_share: Option<String>,

    /// Decimal amount as a string with 2 decimal places. The amount this user
    /// owes for the expense
    pub owed_share: Option<String>,

    /// Net balance of the expense.
    pub net_balance: Option<String>,
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

fn serialize_option_vec_user_by_shares<S: Serializer>(
    vec: &Option<Vec<UserShare>>,
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
