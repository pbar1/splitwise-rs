use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::model::{Balance, Image};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct UserWrapper {
    pub user: User,
}

/// Splitwise user.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// User's ID.
    pub id: Option<i64>,

    /// User's first name.
    pub first_name: Option<String>,

    /// User's last name.
    pub last_name: Option<String>,

    /// User's email address.
    pub email: Option<String>,

    /// User's registration status. One of:
    /// - `confirmed`
    /// - `dummy`
    /// - `invited`
    pub registration_status: Option<String>,

    /// User's profile picture.
    pub picture: Option<Image>,

    /// Whether the profile picture is user-provided.
    pub custom_picture: Option<bool>,

    /// Timestamp of the last time notifications were read.
    pub notifications_read: Option<chrono::DateTime<chrono::Utc>>,

    /// Number of unread notifications since `notifications_read`.
    pub notifications_count: Option<i64>,

    /// User's notification preferences.
    #[serde(rename = "notifications")]
    pub notification_preferences: Option<HashMap<String, bool>>,

    /// User's default currency.
    pub default_currency: Option<String>,

    /// ISO_639-1 2-letter locale code
    pub locale: Option<String>,

    /// List of balances that the user carries in a group.
    pub balance: Option<Vec<Balance>>,
}

/// Splitwise `update_user` request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    /// User's first name.
    pub first_name: Option<String>,

    /// User's last name.
    pub last_name: Option<String>,

    /// User's email address.
    pub email: Option<String>,

    /// User's password.
    pub password: Option<String>,

    /// User's default currency.
    pub default_currency: Option<String>,

    /// ISO_639-1 2-letter locale code.
    pub locale: Option<String>,
}
