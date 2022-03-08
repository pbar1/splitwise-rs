use serde::{Deserialize, Serialize};

use crate::model::{Balance, Image};

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
    pub balance: Option<Vec<Balance>>,
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
