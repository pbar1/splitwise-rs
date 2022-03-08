use crate::{
    client::Client,
    model::other::{
        CategoriesWrapper, Category, CurrenciesWrapper, Currency, ParseSentenceRequest,
        ParseSentenceResponse,
    },
};

#[derive(Debug)]
pub struct OtherSvc<'c> {
    client: &'c Client,
}

impl<'c> OtherSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// [Supported currencies](https://dev.splitwise.com/#tag/other/paths/~1get_currencies/get)
    pub async fn get_currencies(&self) -> Result<Vec<Currency>, anyhow::Error> {
        let url = self.client.base_url.join("get_currencies")?;
        let response: CurrenciesWrapper = self.client.get(url).await?;
        Ok(response.currencies)
    }

    /// [Supported categories](https://dev.splitwise.com/#tag/other/paths/~1get_categories/get)
    pub async fn get_categories(&self) -> Result<Vec<Category>, anyhow::Error> {
        let url = self.client.base_url.join("get_categories")?;
        let response: CategoriesWrapper = self.client.get(url).await?;
        Ok(response.categories)
    }

    /// [Parse sentence into an expense](https://dev.splitwise.com/#tag/other/paths/~1parse_sentence/post)
    pub async fn parse_sentence(
        &self,
        request: ParseSentenceRequest,
    ) -> Result<ParseSentenceResponse, anyhow::Error> {
        let url = self.client.base_url.join("parse_sentence")?;
        let response: ParseSentenceResponse = self.client.post(url, &request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod integration_tests {
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn get_currencies_works() {
        Client::default().other().get_currencies().await.unwrap();
    }

    #[test(tokio::test)]
    async fn get_categories_works() {
        Client::default().other().get_categories().await.unwrap();
    }

    #[test(tokio::test)]
    async fn parse_sentence_works() {
        let request = ParseSentenceRequest {
            input: "paid $50 for tacos in test-group-1 split evenly".to_string(),
            ..ParseSentenceRequest::default()
        };
        let response = Client::default()
            .other()
            .parse_sentence(request)
            .await
            .unwrap();
        assert!(response.valid.unwrap());
    }
}
