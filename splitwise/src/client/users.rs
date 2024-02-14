use crate::client::client::Client;
use crate::model::users::UpdateUserRequest;
use crate::model::users::User;
use crate::model::users::UserWrapper;

/// Resources to access and modify user information.
///
/// [Splitwise API docs](https://dev.splitwise.com/#tag/users)
#[derive(Debug)]
pub struct UsersSvc<'c> {
    client: &'c Client,
}

impl<'c> UsersSvc<'c> {
    /// Creates an instance of `UsersSvc`.
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get information about the current user.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/users/paths/~1get_current_user/get)
    pub async fn get_current_user(&self) -> Result<User, anyhow::Error> {
        let url = self.client.base_url.join("get_current_user")?;
        let response: UserWrapper = self.client.get(url).await?;
        Ok(response.user)
    }

    /// Get information about another user.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/users/paths/~1get_user~1{id}/get)
    pub async fn get_user(&self, id: i64) -> Result<User, anyhow::Error> {
        let url = self.client.base_url.join(&format!("get_user/{}", id))?;
        let response: UserWrapper = self.client.get(url).await?;
        Ok(response.user)
    }

    /// Update a user.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/users/paths/~1update_user~1{id}/post)
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
    use tracing::debug;

    use super::*;

    #[test(tokio::test)]
    async fn get_current_get_update_user_works() {
        let client = Client::default();

        let current = client.users().get_current_user().await.unwrap();
        debug!("get_current_user: {:#?}", current);

        let id = current.id.unwrap();

        let get = client.users().get_user(id).await.unwrap();
        debug!("get_user: {:#?}", get);
        assert_eq!(id, get.id.unwrap());

        let btc = client
            .users()
            .update_user(
                id,
                UpdateUserRequest {
                    default_currency: Some("BTC".to_string()),
                    ..UpdateUserRequest::default()
                },
            )
            .await
            .unwrap();
        debug!("update_user: {:#?}", btc);
        assert_eq!("BTC".to_string(), btc.default_currency.unwrap());

        let btc = client
            .users()
            .update_user(
                id,
                UpdateUserRequest {
                    default_currency: Some("USD".to_string()),
                    ..UpdateUserRequest::default()
                },
            )
            .await
            .unwrap();
        debug!("update_user: {:#?}", btc);
        assert_eq!("USD".to_string(), btc.default_currency.unwrap());
    }
}
