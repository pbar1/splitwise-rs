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
        let mut url = self.client.base_url.clone();
        url.set_path("get_current_user");
        let response: UserWrapper = self.client.get(url).await?;
        Ok(response.user)
    }

    /// [Get information about another user](https://dev.splitwise.com/#tag/users/paths/~1get_user~1{id}/get)
    pub async fn get_user(&self, id: i64) -> Result<User, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path(&format!("get_user/{}", id));
        let response: UserWrapper = self.client.get(url).await?;
        Ok(response.user)
    }

    /// [Update a user](https://dev.splitwise.com/#tag/users/paths/~1update_user~1{id}/post)
    pub async fn update_user(
        &self,
        id: i64,
        updates: UpdateUserRequest,
    ) -> Result<User, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path(&format!("update_user/{}", id));
        let response: UserWrapper = self.client.post(url, &updates).await?;
        Ok(response.user)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use super::*;

    #[tokio::test]
    async fn get_current_user_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_current_user");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/users/get_current_user.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .users()
            .get_current_user()
            .await
            .unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn get_user_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_user/0");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/users/get_user.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .users()
            .get_user(0)
            .await
            .unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn update_user_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/update_user/0");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/users/update_user.POST.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .users()
            .update_user(0, UpdateUserRequest::default())
            .await
            .unwrap();
        mock.assert();
    }
}
