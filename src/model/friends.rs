use std::collections::HashMap;

use serde::{Deserialize, Serialize, Serializer};

use crate::model::{users::User, Balance};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct FriendsWrapper {
    pub friends: Vec<User>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct FriendWrapper {
    pub friend: User,
}

/// List of balances and their associated group.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupBalance {
    /// Group ID of the group that the balances are in.
    pub group_id: Option<i64>,

    /// List of balances in the group.
    pub balance: Option<Vec<Balance>>,
}

/// Splitwise `add_friends` request.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendsRequest {
    /// List of emails of users to add as friends.
    #[serde(flatten)]
    #[serde(serialize_with = "serialize_vec_email")]
    pub emails: Vec<String>,

    /// Message to send to users being added as a friend.
    pub message: Option<String>,

    /// Whether to allow the request to succeed if not every friend is able to
    /// be added.
    pub allow_partial_success: Option<bool>,
}

/// Splitwise `add_friends` response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddFriendsResponse {
    /// List of users that were added as a friend.
    pub users: Option<Vec<User>>,

    /// Errors that occurred during the request.
    pub errors: Option<HashMap<String, Vec<String>>>,
}

/// Splitwise `delete_friend` response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteFriendResponse {
    /// Whether the request was successful.
    pub success: bool,

    /// Errors that occurred during the request.
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
