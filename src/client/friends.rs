use crate::{
    client::Client,
    model::friends::{
        AddFriendsRequest, AddFriendsResponse, DeleteFriendResponse, Friend, FriendWrapper,
        FriendsWrapper,
    },
};

#[derive(Debug)]
pub struct FriendsSvc<'c> {
    client: &'c Client,
}

impl<'c> FriendsSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub async fn list_friends(&self) -> Result<Vec<Friend>, anyhow::Error> {
        let url = self.client.base_url.join("get_friends")?;
        let response: FriendsWrapper = self.client.get(url).await?;
        Ok(response.friends)
    }

    pub async fn get_friend(&self, id: i64) -> Result<Friend, anyhow::Error> {
        let url = self.client.base_url.join(&format!("get_friend/{}", id))?;
        let response: FriendWrapper = self.client.get(url).await?;
        Ok(response.friend)
    }

    // NOTE: This endpoint behaves a bit differently than the API documents suggest.
    // After inspection in the browser debugger, we'll be using that flow instead.
    pub async fn add_friends(
        &self,
        request: AddFriendsRequest,
    ) -> Result<AddFriendsResponse, anyhow::Error> {
        let url = self.client.base_url.join("create_friends")?;
        let response: AddFriendsResponse = self.client.post_form(url, &request).await?;
        Ok(response)
    }

    pub async fn delete_friend(&self, id: i64) -> Result<DeleteFriendResponse, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("delete_friend/{}", id))?;
        let response: DeleteFriendResponse = self.client.post_no_body(url).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod integration_tests {
    use log::debug;
    use test_log::test;

    use super::*;
    use crate::model::groups::{GroupCreateRequest, GroupUser};

    // NOTE: This test also contains `create_group`, `delete_group`,
    // `add_user_to_group`, and `remove_user_from_group`, which are from the
    // `groups` API.
    //
    // Splitwise users cannot be added to a group unless they are friends with
    // the user that is attempting to add them. Likewise, a user cannot be
    // unfriended if they remain in a group with the current user.
    //
    // Similarly, Splitwise groups cannot be deleted if they still contain
    // users other than the user that created them.
    //
    // Thus, this test runs the following flow:
    //   1. Add friend
    //   2. List friends
    //   3. Get friend
    //   4. -> Create group
    //   5. ---> Add user to group
    //   6. ---> Remove user from group
    //   7. -> Delete group
    //   8. Delete friend
    #[test(tokio::test)]
    async fn add_list_get_delete_friend_and_add_remove_user_group_works() {
        let client = Client::default();

        let email = "kmalbwid@sharklasers.com".to_string();
        let req = AddFriendsRequest {
            emails: vec![email.clone()],
            ..AddFriendsRequest::default()
        };
        let add = client.friends().add_friends(req).await.unwrap();
        debug!("add_friends: {:?}", add);
        assert_eq!(
            &email,
            add.users
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .email
                .as_ref()
                .unwrap()
        );

        let list = Client::default().friends().list_friends().await.unwrap();
        debug!("list_friends: {:?}", list);
        let id = list.first().unwrap().id.unwrap();

        let get = client.friends().get_friend(id).await.unwrap();
        debug!("get_friend: {:?}", get);
        assert_eq!(id, get.id.unwrap());

        // BEGIN GROUP TESTING -------------------------------------------------

        let group = client
            .groups()
            .create_group(GroupCreateRequest {
                name: "happy-happy-friends".to_string(),
                group_type: Some("apartment".to_string()),
                simplify_by_default: Some(true),
                users: None,
            })
            .await
            .unwrap();
        debug!("create_group: {:#?}", group);
        let group_id = group.id.unwrap();

        let add_to_group = client
            .groups()
            .add_user_to_group(
                group_id,
                GroupUser {
                    user_id: Some(id),
                    ..GroupUser::default()
                },
            )
            .await
            .unwrap();
        debug!("add_to_group: {:#?}", add_to_group);
        assert!(add_to_group.success);

        let remove_from_group = client
            .groups()
            .remove_user_from_group(group_id, id)
            .await
            .unwrap();
        debug!("remove_from_group: {:#?}", remove_from_group);
        assert!(remove_from_group.success);

        let delete_group = client.groups().delete_group(group_id).await.unwrap();
        debug!("delete_group: {:#?}", delete_group);
        assert!(delete_group.success);

        // END GROUP TESTING ---------------------------------------------------

        let delete = client.friends().delete_friend(id).await.unwrap();
        debug!("delete_friend: {:?}", delete);
        assert!(delete.success);
    }
}
