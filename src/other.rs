use crate::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct OtherSvc<'c> {
    client: &'c Client,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CurrenciesWrapper {
    pub currencies: Vec<Currency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub currency_code: String,
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CategoriesWrapper {
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub icon_types: IconTypes,
    pub subcategories: Option<Vec<Category>>,
}

// FIXME: Are specific "IconTypes" actually defined concretely?
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IconTypes {
    pub slim: Slim,
    pub square: Square,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Slim {
    pub small: String,
    pub large: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Square {
    pub large: String,
    pub xlarge: String,
}

// FIXME: Implement parse_sentence
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSentenceRequest {
    pub input: String,
    pub friend_id: Option<i64>,
    pub group_id: Option<i64>,
    pub autosave: bool,
}

impl<'c> OtherSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub async fn get_currencies(&self) -> Result<Vec<Currency>, anyhow::Error> {
        let path = "/get_currencies";
        let response: CurrenciesWrapper = self.client.get(path).await?;
        Ok(response.currencies)
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>, anyhow::Error> {
        let path = "/get_categories";
        let response: CategoriesWrapper = self.client.get(path).await?;
        Ok(response.categories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use wiremock::{matchers, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn get_currencies_success() {
        let mock_server = MockServer::start().await;

        let body = fs::read("test/other/get_currencies.GET.response.200.json").unwrap();
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/json"))
            .mount(&mock_server)
            .await;

        Client::default()
            .with_base_url(mock_server.uri())
            .other()
            .get_currencies()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_categories_success() {
        let mock_server = MockServer::start().await;

        let body = fs::read("test/other/get_categories.GET.response.200.json").unwrap();
        Mock::given(matchers::any())
            .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/json"))
            .mount(&mock_server)
            .await;

        Client::default()
            .with_base_url(mock_server.uri())
            .other()
            .get_categories()
            .await
            .unwrap();
    }
}
