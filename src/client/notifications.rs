use crate::{
    client::Client,
    model::notifications::{GetNotificationsRequest, Notification, NotificationsWrapper},
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
        let mut url = self.client.base_url.join("get_notifications")?;
        let query = serde_qs::to_string(&request)?;
        url.set_query(Some(&query));
        let response: NotificationsWrapper = self.client.get(url).await?;
        Ok(response.notifications)
    }
}

#[cfg(test)]
mod integration_tests {
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn get_notifications_works() {
        let request = GetNotificationsRequest::default();
        Client::default()
            .notifications()
            .get_notifications(request)
            .await
            .unwrap();
    }
}
