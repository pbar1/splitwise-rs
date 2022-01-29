use crate::notifications::NotificationsService;
use crate::users::UserService;
use reqwest::header::AUTHORIZATION;
use serde::Serialize;

pub mod errors;
pub mod notifications;
pub mod users;

const BASE_URL: &str = "https://secure.splitwise.com/api/v3.0";

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    base_url: String,
    authorization: String,
}

impl Client {
    pub fn new(http_client: reqwest::Client, api_key: String) -> Client {
        let authorization = format!("Bearer {}", api_key);
        let client = Client {
            http_client,
            base_url: String::from(BASE_URL),
            authorization,
        };
        client
    }

    pub fn new_default_http_client(api_key: String) -> Client {
        let http_client = reqwest::Client::default();
        let client = Client::new(http_client, api_key);
        client
    }

    pub(crate) async fn get(&self, path: String) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .get(url)
            .header(AUTHORIZATION, &self.authorization)
            .send()
            .await?;
        Ok(response)
    }

    pub(crate) async fn post<T>(
        &self,
        path: String,
        body: &T,
    ) -> Result<reqwest::Response, reqwest::Error>
    where
        T: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .post(url)
            .header(AUTHORIZATION, &self.authorization)
            .json(body)
            .send()
            .await?;
        Ok(response)
    }

    pub(crate) async fn delete(&self, path: String) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .http_client
            .delete(url)
            .header(AUTHORIZATION, &self.authorization)
            .send()
            .await?;
        Ok(response)
    }

    pub fn users(&self) -> UserService {
        let service = UserService::new(self.clone());
        service
    }

    pub fn notifications(&self) -> NotificationsService {
        let service = NotificationsService::new(self.clone());
        service
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
