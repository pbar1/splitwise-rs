use crate::{errors, notifications::NotificationsSvc, other::OtherSvc, users::UsersSvc};
use anyhow::bail;
use reqwest::{header, StatusCode};
use secrecy::{ExposeSecret, Secret};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    api_key: Secret<String>,
}

impl Default for Client {
    fn default() -> Self {
        let http_client = reqwest::Client::default();
        let base_url = String::from("https://secure.splitwise.com/api/v3.0");
        let api_key: Secret<String> = std::env::var("SPLITWISE_API_KEY")
            .unwrap_or(String::from(""))
            .into();
        Self {
            http_client,
            base_url,
            api_key,
        }
    }
}

impl Client {
    pub fn with_http_client(self, http_client: reqwest::Client) -> Self {
        Self {
            http_client,
            base_url: self.base_url,
            api_key: self.api_key,
        }
    }

    pub fn with_base_url(self, base_url: String) -> Self {
        Self {
            http_client: self.http_client,
            base_url,
            api_key: self.api_key,
        }
    }

    pub fn with_api_key(self, api_key: Secret<String>) -> Self {
        Self {
            http_client: self.http_client,
            base_url: self.base_url,
            api_key,
        }
    }

    pub(crate) async fn parse_response<T>(
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
                let decoded = response.json::<errors::ErrorUnauthorized>().await?;
                bail!(decoded.error)
            }
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => {
                let decoded = response.json::<errors::ErrorForbiddenOrNotFound>().await?;
                bail!(decoded.errors.base.join("; "))
            }
            _ => bail!("unexpected HTTP status code: {}", response.status()),
        }
    }

    /// Perform an HTTP GET wrapped with auth.
    pub(crate) async fn get<T>(&self, path: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .get(url)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", &self.api_key.expose_secret()),
            )
            .send()
            .await?;
        let payload = self.parse_response(response).await?;
        Ok(payload)
    }

    /// Perform an HTTP GET wrapped with auth.
    pub(crate) async fn get_with_query<T, S>(
        &self,
        path: &str,
        query: &S,
    ) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
        S: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .get(url)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", &self.api_key.expose_secret()),
            )
            .query(query)
            .send()
            .await?;
        let payload = self.parse_response(response).await?;
        Ok(payload)
    }

    /// Perform an HTTP POST wrapped with auth.
    pub(crate) async fn post<T, S>(&self, path: &str, body: &S) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
        S: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .post(url)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", &self.api_key.expose_secret()),
            )
            .json(body)
            .send()
            .await?;
        let payload = self.parse_response(response).await?;
        Ok(payload)
    }

    /// Users API group.
    pub fn users(&self) -> UsersSvc {
        UsersSvc::new(self)
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
