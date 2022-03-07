use chrono::Utc;
use url::Url;

use crate::{
    client::Client,
    model::{Notification, NotificationsWrapper},
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
        updated_after: Option<chrono::DateTime<Utc>>,
        limit: Option<i64>,
    ) -> Result<Vec<Notification>, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path("get_notifications");

        let mut q = Vec::new();
        if let Some(x) = updated_after {
            q.push(("updated_after", x.to_rfc3339()));
        }
        if let Some(x) = limit {
            q.push(("limit", format!("{}", x)));
        }
        let url = Url::parse_with_params(url.as_str(), q)?;

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
            .get_notifications(None, Some(10))
            .await
            .unwrap();
        mock.assert();
    }
}
