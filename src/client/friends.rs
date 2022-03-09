use crate::{
    client::Client,
    model::friends::{
        AddFriendRequest, AddFriendsResponse, DeleteFriendResponse, Friend, FriendWrapper,
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

    pub async fn add_friend(&self, request: AddFriendRequest) -> Result<Friend, anyhow::Error> {
        let url = self.client.base_url.join("create_friend")?;
        let response: FriendWrapper = self.client.post(url, &request).await?;
        Ok(response.friend)
    }

    pub async fn add_friends(
        &self,
        request: AddFriendRequest,
    ) -> Result<AddFriendsResponse, anyhow::Error> {
        let url = self.client.base_url.join("create_friends")?;
        let response: AddFriendsResponse = self.client.post(url, &request).await?;
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
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn list_friends_works() {
        let _response = Client::default().friends().list_friends().await.unwrap();
    }

    #[test(tokio::test)]
    async fn get_friend_works() {
        let _response = Client::default()
            .friends()
            .get_friend(9239275)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn add_delete_friend_works() {
        todo!()
    }

    #[test(tokio::test)]
    async fn add_delete_friends_works() {
        todo!()
    }
}
