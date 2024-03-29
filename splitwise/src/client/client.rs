use std::collections::HashMap;

use anyhow::bail;
use reqwest::header;
use reqwest::StatusCode;
use secrecy::ExposeSecret;
use secrecy::Secret;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::client::comments::CommentsSvc;
use crate::client::expenses::ExpensesSvc;
use crate::client::friends::FriendsSvc;
use crate::client::groups::GroupsSvc;
use crate::client::notifications::NotificationsSvc;
use crate::client::other::OtherSvc;
use crate::client::users::UsersSvc;
use crate::model::shared::ErrorForbiddenOrNotFound;
use crate::model::shared::ErrorUnauthorized;

/// Splitwise API client.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    pub(crate) base_url: Url,
    authorization: Secret<String>,
}

impl Default for Client {
    /// Creates a default Splitwise API client using a default Reqwest HTTP
    /// client, the official Splitwise API URL, and an API key sourced from
    /// the environment variable `SPLITWISE_API_KEY`.
    fn default() -> Self {
        let http_client = reqwest::Client::default();
        let base_url = Url::parse("https://secure.splitwise.com/api/v3.0/").unwrap();
        let api_key: String =
            std::env::var("SPLITWISE_API_KEY").unwrap_or_else(|_| String::from(""));
        let authorization = format!("Bearer {}", api_key).into();
        Self {
            http_client,
            base_url,
            authorization,
        }
    }
}

impl Client {
    /// Builds a new Splitwise client from the current one, with the given HTTP
    /// client as an override.
    pub fn with_http_client(self, http_client: reqwest::Client) -> Self {
        Self {
            http_client,
            base_url: self.base_url,
            authorization: self.authorization,
        }
    }

    /// Builds a new Splitwise client from the current one, with the given API
    /// base URL as an override.
    pub fn with_base_url(self, base_url: &str) -> Result<Self, anyhow::Error> {
        let mut ensured_base_url = base_url.to_string();
        if !ensured_base_url.ends_with('/') {
            ensured_base_url.push('/');
        }
        let base_url = Url::parse(&ensured_base_url)?;
        Ok(Self {
            http_client: self.http_client,
            base_url,
            authorization: self.authorization,
        })
    }

    /// Builds a new Splitwise client from the current one, with the given API
    /// key as an override.
    pub fn with_api_key(self, api_key: Secret<String>) -> Self {
        let authorization = format!("Bearer {}", api_key.expose_secret()).into();
        Self {
            http_client: self.http_client,
            base_url: self.base_url,
            authorization,
        }
    }

    /// Builds a new Splitwise client from the current one, performing an OAuth
    /// 2.0 Authorization Code flow.
    // pub fn with_oauth(self, )

    /// Decodes HTTP response into Splitwise API types or errors.
    pub(crate) async fn process_response<T>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK => {
                let decoded = response.json::<T>().await?;
                Ok(decoded)
            }
            StatusCode::UNAUTHORIZED => {
                let decoded = response.json::<ErrorUnauthorized>().await?;
                bail!(decoded.error)
            }
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => {
                let decoded = response.json::<ErrorForbiddenOrNotFound>().await?;
                bail!(decoded.errors.base.join("; "))
            }
            _ => bail!("unexpected HTTP status code: {}", response.status()),
        }
    }

    /// Perform an HTTP GET wrapped with auth.
    pub(crate) async fn get<T>(&self, url: Url) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .get(url)
            .header(header::AUTHORIZATION, self.authorization.expose_secret())
            .send()
            .await?;
        let payload = self.process_response(response).await?;
        Ok(payload)
    }

    // TODO: Can body be consumed rather than be a reference?
    /// Perform an HTTP POST wrapped with auth.
    pub(crate) async fn post<T, S>(&self, url: Url, body: &S) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
        S: Serialize + ?Sized,
    {
        let response = self
            .http_client
            .post(url)
            .header(header::AUTHORIZATION, self.authorization.expose_secret())
            .json(body)
            .send()
            .await?;
        let payload = self.process_response(response).await?;
        Ok(payload)
    }

    // TODO: Merge this with post
    /// Perform an HTTP POST wrapped with auth, with no request body.
    pub(crate) async fn post_form<T, S>(&self, url: Url, body: &S) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
        S: Serialize + ?Sized,
    {
        let response = self
            .http_client
            .post(url)
            .header(header::AUTHORIZATION, self.authorization.expose_secret())
            .form(body)
            .send()
            .await?;
        let payload = self.process_response(response).await?;
        Ok(payload)
    }

    // TODO: Merge this with post
    /// Perform an HTTP POST wrapped with auth, with no request body.
    pub(crate) async fn post_no_body<T>(&self, url: Url) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http_client
            .post(url)
            .header(header::AUTHORIZATION, self.authorization.expose_secret())
            .send()
            .await?;
        let payload = self.process_response(response).await?;
        Ok(payload)
    }

    /// Users API group.
    pub fn users(&self) -> UsersSvc {
        UsersSvc::new(self)
    }

    /// Groups API group.
    pub fn groups(&self) -> GroupsSvc {
        GroupsSvc::new(self)
    }

    /// Friends API group.
    pub fn friends(&self) -> FriendsSvc {
        FriendsSvc::new(self)
    }

    /// Expenses API group.
    pub fn expenses(&self) -> ExpensesSvc {
        ExpensesSvc::new(self)
    }

    /// Comments API group.
    pub fn comments(&self) -> CommentsSvc {
        CommentsSvc::new(self)
    }

    /// Notifications API group.
    pub fn notifications(&self) -> NotificationsSvc {
        NotificationsSvc::new(self)
    }

    /// Other API group.
    pub fn other(&self) -> OtherSvc {
        OtherSvc::new(self)
    }
}

pub(crate) fn join_errors(errors: &HashMap<String, Vec<String>>) -> String {
    let mut error_text = String::from("");
    for (k, v) in errors {
        error_text.push_str(&format!("{}: [{}];", k, v.join("; ")));
    }
    error_text
}
