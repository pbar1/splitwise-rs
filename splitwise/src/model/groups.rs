use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::model::{
    shared::{Debt, Image},
    users::User,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GroupsWrapper {
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GroupWrapper {
    pub group: Group,
}

/// Splitwise group.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    /// Group ID.
    pub id: Option<i64>,

    /// Group name.
    pub name: Option<String>,

    /// What is the group used for? One of:
    /// - `apartment`
    /// - `house`
    /// - `trip`
    /// - `other`
    pub group_type: Option<String>,

    /// Timestamp of when the group was last updated.
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Turn on simplify debts?
    pub simplify_by_default: Option<bool>,

    /// List of users that are members of the group.
    pub members: Option<Vec<User>>,

    /// List of debts between users in the group.
    pub original_debts: Option<Vec<Debt>>,

    /// List of simplified debts between users in the group.
    pub simplified_debts: Option<Vec<Debt>>,

    /// Avatar image for the group.
    pub avatar: Option<Image>,

    /// Whether the group's avatar is user-provided.
    pub custom_avatar: Option<bool>,

    /// Cover photo for the group.
    pub cover_photo: Option<Image>,

    /// Invite link to the group.
    pub invite_link: Option<String>,
}

/// Splitwise `create_group` request.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupCreateRequest {
    /// Group name.
    pub name: String,

    /// What is the group used for? One of:
    /// - `apartment`
    /// - `house`
    /// - `trip`
    /// - `other`
    pub group_type: Option<String>,

    /// Turn on simplify debts?
    pub simplify_by_default: Option<bool>,

    /// List of users to invite to the group.
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_option_vec_create_group_user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GroupUser>>,
}

/// Information to invite a user to a group. The user's email or ID must be
/// provided.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupUser {
    /// User ID.
    pub user_id: Option<i64>,

    /// User first name.
    pub first_name: Option<String>,

    /// User last name.
    pub last_name: Option<String>,

    /// User email address.
    pub email: Option<String>,
}

// TODO: does this also have an error field?
/// Splitwise `delete_group` response.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupDeleteResponse {
    /// Whether the request was successful.
    pub success: bool,
}

/// Splitwise `restore_group` response.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupRestoreResponse {
    /// Whether the request was successful.
    pub success: bool,

    /// Errors that occurred during the request.
    pub errors: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GroupAddUserRequest {
    pub group_id: i64,
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

/// Splitwise `add_user_to_group` response.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupAddUserResponse {
    /// Whether the request was successful.
    pub success: bool,

    /// Users added to the group.
    pub user: Option<User>,

    /// Errors that occurred during the request.
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct GroupRemoveUserRequest {
    pub group_id: i64,
    pub user_id: i64,
}

/// Splitwise `remove_user_from_group` response.
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupRemoveUserResponse {
    /// Whether the request was successful.
    pub success: bool,

    /// Errors that occurred during the request.
    pub errors: Option<Vec<String>>,
}

fn serialize_option_vec_create_group_user<S: Serializer>(
    vec: &Option<Vec<GroupUser>>,
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
        }
    }

    map.serialize(serializer)
}
