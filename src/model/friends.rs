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
pub struct AddFriendRequest {
    pub email: String,
    pub user_first_name: Option<String>,
    pub user_last_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendsRequest {
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_vec_add_friend_request")]
    pub friends: Vec<AddFriendRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendsResponse {
    pub users: Option<Vec<Friend>>, // TODO: Is the API doc wrong? This should be "friends"
    pub errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteFriendResponse {
    pub success: bool,
    pub errors: Option<HashMap<String, Vec<String>>>,
}

fn serialize_vec_add_friend_request<S: Serializer>(
    vec: &[AddFriendRequest],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut map = HashMap::new();

    for (i, friend) in vec.iter().enumerate() {
        if let Some(ref first_name) = friend.user_first_name {
            map.insert(
                format!("friends__{}__first_name", i),
                first_name.to_string(),
            );
        }
        if let Some(ref last_name) = friend.user_last_name {
            map.insert(format!("friends__{}__last_name", i), last_name.to_string());
        }
        map.insert(format!("friends__{}__email", i), friend.email.to_string());
    }

    map.serialize(serializer)
}
