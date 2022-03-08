use crate::{
    client::Client,
    model::{UpdateUserRequest, User, UserWrapper},
};

#[derive(Debug)]
pub struct UsersSvc<'c> {
    client: &'c Client,
}

impl<'c> UsersSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// [Get information about the current user](https://dev.splitwise.com/#tag/users/paths/~1get_current_user/get)
    pub async fn get_current_user(&self) -> Result<User, anyhow::Error> {
        let url = self.client.base_url.join("get_current_user")?;
        let response: UserWrapper = self.client.get(url).await?;
        Ok(response.user)
    }

    /// [Get information about another user](https://dev.splitwise.com/#tag/users/paths/~1get_user~1{id}/get)
    pub async fn get_user(&self, id: i64) -> Result<User, anyhow::Error> {
        let url = self.client.base_url.join(&format!("get_user/{}", id))?;
        let response: UserWrapper = self.client.get(url).await?;
        Ok(response.user)
    }

    /// [Update a user](https://dev.splitwise.com/#tag/users/paths/~1update_user~1{id}/post)
    pub async fn update_user(
        &self,
        id: i64,
        updates: UpdateUserRequest,
    ) -> Result<User, anyhow::Error> {
        let url = self.client.base_url.join(&format!("update_user/{}", id))?;
        let response: UserWrapper = self.client.post(url, &updates).await?;
        Ok(response.user)
    }
}

#[cfg(test)]
mod integration_tests {
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn get_current_user_works() {
        Client::default().users().get_current_user().await.unwrap();
    }

    #[test(tokio::test)]
    async fn get_user_works() {
        Client::default().users().get_user(47829677).await.unwrap();
    }

    #[test(tokio::test)]
    async fn update_user_works() {
        let client = Client::default();
        let request = UpdateUserRequest {
            default_currency: Some("BTC".to_string()),
            ..UpdateUserRequest::default()
        };
        client.users().update_user(47829677, request).await.unwrap();
        let request = UpdateUserRequest {
            default_currency: Some("USD".to_string()),
            ..UpdateUserRequest::default()
        };
        client.users().update_user(47829677, request).await.unwrap();
    }
}
