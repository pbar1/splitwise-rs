use crate::{
    client::Client,
    model::{
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
        let mut url = self.client.base_url.clone();
        url.set_path("get_currencies");
        let response: CurrenciesWrapper = self.client.get(url).await?;
        Ok(response.currencies)
    }

    /// [Supported categories](https://dev.splitwise.com/#tag/other/paths/~1get_categories/get)
    pub async fn get_categories(&self) -> Result<Vec<Category>, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path("get_categories");
        let response: CategoriesWrapper = self.client.get(url).await?;
        Ok(response.categories)
    }

    /// [Parse sentence into an expense](https://dev.splitwise.com/#tag/other/paths/~1parse_sentence/post)
    pub async fn parse_sentence(
        &self,
        request: ParseSentenceRequest,
    ) -> Result<ParseSentenceResponse, anyhow::Error> {
        let mut url = self.client.base_url.clone();
        url.set_path("parse_sentence");
        let response: ParseSentenceResponse = self.client.post(url, &request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;

    use super::*;

    #[tokio::test]
    async fn get_currencies_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_currencies");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/other/get_currencies.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .other()
            .get_currencies()
            .await
            .unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn get_categories_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET).path("/get_categories");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/other/get_categories.GET.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .other()
            .get_categories()
            .await
            .unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn parse_sentence_success() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST).path("/parse_sentence");
            then.status(200)
                .header("Content-Type", "application/json")
                .body_from_file("test/other/parse_sentence.POST.200.success.json");
        });
        Client::default()
            .with_base_url(server.base_url().as_str())
            .unwrap()
            .other()
            .parse_sentence(ParseSentenceRequest::default())
            .await
            .unwrap();
        mock.assert();
    }
}
