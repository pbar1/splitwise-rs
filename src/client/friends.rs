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
    use std::ops::Index;

    use log::debug;
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn list_get_delete_add_friends_works() {
        let client = Client::default();

        let list = Client::default().friends().list_friends().await.unwrap();
        debug!("list_friends: {:?}", list);
        let id = list.index(0).id.unwrap();

        let get = client.friends().get_friend(id).await.unwrap();
        debug!("get_friend: {:?}", get);
        assert_eq!(id, get.id.unwrap());

        let delete = client.friends().delete_friend(id).await.unwrap();
        debug!("delete_friend: {:?}", delete);
        assert!(delete.success);

        let req = AddFriendsRequest {
            emails: vec![get.email.unwrap()],
            message: None,
            allow_partial_success: None,
        };
        let add = client.friends().add_friends(req).await.unwrap();
        debug!("add_friends: {:?}", add);
        assert_eq!(id, add.users.unwrap().index(0).id.unwrap())
    }
}
