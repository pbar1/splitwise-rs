use crate::{
    client::Client,
    model::{GetNotificationsRequest, Notification, NotificationsWrapper},
};

#[derive(Debug)]
pub struct NotificationsSvc<'c> {
    client: &'c Client,
}

impl<'c> NotificationsSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// [Get notifications](https://dev.splitwise.com/#tag/notifications/paths/~1get_notifications/get)
    pub async fn get_notifications(
        &self,
        request: GetNotificationsRequest,
    ) -> Result<Vec<Notification>, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path("get_notifications");
        let query = serde_qs::to_string(&request)?;
        url.set_query(Some(&query));
        let response: NotificationsWrapper = self.client.get(url).await?;
        Ok(response.notifications)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use super::*;

    #[tokio::test]
    async fn get_notifications_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_notifications");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/notifications/get_notifications.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .notifications()
            .get_notifications(GetNotificationsRequest::default())
            .await
            .unwrap();
        mock.assert();
    }
}
