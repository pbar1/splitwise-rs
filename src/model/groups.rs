use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub group_type: Option<String>,
    pub simplify_by_default: Option<bool>,

    #[serde(flatten)]
    #[serde(serialize_with = "serialize_option_vec_create_group_user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<CreateGroupUser>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateGroupUser {
    pub user_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteGroupResponse {
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestoreGroupResponse {
    pub success: bool,
    pub errors: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddUserToGroupRequest {
    pub group_id: i64,
    pub user_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddUserToGroupResponse {
    pub success: bool,
    pub user: Option<User>,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveUserFromGroupRequest {
    pub group_id: i64,
    pub user_id: i64,
}

// TODO: "errors" is [] when none, {} when some - this is endemic to all of Splitwise
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveUserFromGroupResponse {
    pub success: bool,
    pub errors: Option<Vec<String>>,
}

fn serialize_option_vec_create_group_user<S: Serializer>(
    vec: &Option<Vec<CreateGroupUser>>,
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
