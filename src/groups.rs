use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupsResponse {
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupResponse {
    pub group: Group,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub group_type: String,
    pub updated_at: String,
    pub simplify_by_default: bool,
    pub members: Vec<Member>,
    pub original_debts: Vec<OriginalDebt>,
    pub simplified_debts: Vec<SimplifiedDebt>,
    pub avatar: Avatar,
    pub custom_avatar: bool,
    pub cover_photo: CoverPhoto,
    pub invite_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub registration_status: String,
    pub picture: Picture,
    pub balance: Vec<Balance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Picture {
    pub small: String,
    pub medium: String,
    pub large: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    pub currency_code: String,
    pub amount: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OriginalDebt {
    pub from: i64,
    pub to: i64,
    pub amount: String,
    pub currency_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimplifiedDebt {
    pub from: i64,
    pub to: i64,
    pub amount: String,
    pub currency_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Avatar {
    pub original: Option<String>,
    pub xxlarge: String,
    pub xlarge: String,
    pub large: String,
    pub medium: String,
    pub small: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoverPhoto {
    pub xxlarge: String,
    pub xlarge: String,
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
