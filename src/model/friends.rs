use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};

use crate::model::{Balance, Image};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct FriendsWrapper {
    pub friends: Vec<Friend>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct FriendWrapper {
    pub friend: Friend,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Friend {
    pub id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub registration_status: Option<String>,
    pub picture: Option<Image>,
    pub groups: Option<Vec<GroupBalance>>,
    pub balance: Option<Vec<Balance>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupBalance {
    pub group_id: Option<i64>,
    pub balance: Option<Vec<Balance>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendsRequest {
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_vec_email")]
    pub emails: Vec<String>,
    pub message: Option<String>,
    pub allow_partial_success: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendsResponse {
    pub users: Option<Vec<Friend>>,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteFriendResponse {
    pub success: bool,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

fn serialize_vec_email<S: Serializer>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error> {
    let mut map = HashMap::new();

    for (i, email) in vec.iter().enumerate() {
        map.insert(format!("users[{}][email]", i), email.to_owned());
        map.insert(format!("users[{}][name]", i), email.to_owned());
    }

    map.serialize(serializer)
}
