use crate::{
    client::client::Client,
    model::other::{
        CategoriesWrapper, Category, CurrenciesWrapper, Currency, ParseSentenceRequest,
        ParseSentenceResponse,
    },
};

/// Other.
///
/// [Splitwise API docs](https://dev.splitwise.com/#tag/other)
#[derive(Debug)]
pub struct OtherSvc<'c> {
    client: &'c Client,
}

impl<'c> OtherSvc<'c> {
    /// Creates a new instance of `OtherSvc`.
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Returns a list of all currencies allowed by the system. These are mostly
    /// ISO 4217 codes, but we do sometimes use pending codes or unofficial,
    /// colloquial codes (like BTC instead of XBT for Bitcoin).
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/other/paths/~1get_currencies/get)
    pub async fn get_currencies(&self) -> Result<Vec<Currency>, anyhow::Error> {
        let url = self.client.base_url.join("get_currencies")?;
        let response: CurrenciesWrapper = self.client.get(url).await?;
        Ok(response.currencies)
    }

    /// Returns a list of all categories Splitwise allows for expenses. There
    /// are parent categories that represent groups of categories with
    /// subcategories for more specific categorization. When creating expenses,
    /// you must use a subcategory, not a parent category. If you intend for an
    /// expense to be represented by the parent category and nothing more
    /// specific, please use the "Other" subcategory.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/other/paths/~1get_categories/get)
    pub async fn get_categories(&self) -> Result<Vec<Category>, anyhow::Error> {
        let url = self.client.base_url.join("get_categories")?;
        let response: CategoriesWrapper = self.client.get(url).await?;
        Ok(response.categories)
    }

    /// Attempts to create an expense from the input as an English natural
    /// language phrase like "groceries $20" or "Jon paid me $50". If `valid` is
    /// `true`, the `expense` value will be a complete and valid expense. If it
    /// is `false`, the `expense` value may be missing some values.
    ///
    /// **Note:** By default, the expense is only parsed, not persisted. See the
    /// `autosave` parameter.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/other/paths/~1parse_sentence/post)
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
