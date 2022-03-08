use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::model::{users::User, Image};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct GroupsWrapper {
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct GroupWrapper {
    pub group: Group,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub group_type: Option<String>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
    pub simplify_by_default: Option<bool>,
    pub members: Option<Vec<User>>,
    pub original_debts: Option<Vec<Debt>>,
    pub simplified_debts: Option<Vec<Debt>>,
    pub avatar: Option<Image>,
    pub custom_avatar: Option<bool>,
    pub cover_photo: Option<Image>,
    pub invite_link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Debt {
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub amount: Option<String>,
    pub currency_code: Option<String>,
}

// FIXME: Properly serialize `users` field by index
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub group_type: Option<String>,
    pub simplify_by_default: Option<bool>,
    pub users: Option<Vec<CreateGroupUser>>,
}

// FIXME: Handle requiring one of: `user_id`, `email`
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupUser {
    pub user_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
