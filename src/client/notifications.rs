use crate::{
    client::client::Client,
    model::notifications::{GetNotificationsRequest, Notification, NotificationsWrapper},
};

/// Notifications.
///
/// [Splitwise API docs](https://dev.splitwise.com/#tag/notifications)
#[derive(Debug)]
pub struct NotificationsSvc<'c> {
    client: &'c Client,
}

impl<'c> NotificationsSvc<'c> {
    /// Creates an instance of `NotificationsSvc`.
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Return a list of recent activity on the users account with the most
    /// recent items first. `content` will be suitable for display in HTML and
    /// uses only the `<strong>`, `<strike>`, `<small>`, `<br>` and `<font
    /// color="#FFEE44">` tags.
    ///
    /// The `type` value indicates what the notification is about. Notification
    /// types may be added in the future without warning. Below is an
    /// incomplete list of notification types.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/notifications/paths/~1get_notifications/get)
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
