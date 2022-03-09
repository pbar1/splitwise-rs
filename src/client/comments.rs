use crate::{
    client::Client,
    model::comments::{Comment, CommentWrapper, CommentsWrapper, CreateCommentRequest},
};

#[derive(Debug)]
pub struct CommentsSvc<'c> {
    client: &'c Client,
}

impl<'c> CommentsSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub async fn list_comments(&self, expense_id: i64) -> Result<Vec<Comment>, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("get_comments?expense_id={}", expense_id))?;
        let response: CommentsWrapper = self.client.get(url).await?;
        Ok(response.comments)
    }

    pub async fn create_comment(
        &self,
        expense_id: i64,
        content: String,
    ) -> Result<Comment, anyhow::Error> {
        let url = self.client.base_url.join("create_comment")?;
        let request = CreateCommentRequest {
            expense_id,
            content,
        };
        let response: CommentWrapper = self.client.post(url, &request).await?;
        Ok(response.comment)
    }

    // TODO: API docs are ambiguous on this one. Assuming ID is passed as a path param.
    pub async fn delete_comment(&self, id: i64) -> Result<Comment, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("delete_comment/{}", id))?;
        let response: CommentWrapper = self.client.post_no_body(url).await?;
        Ok(response.comment)
    }
}

#[cfg(test)]
mod integration_tests {
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn get_comments_works() {
        let _response = Client::default()
            .comments()
            .list_comments(1603994775)
            .await
            .unwrap();
    }

    #[test(tokio::test)]
    async fn create_delete_comment_works() {
        let client = Client::default();
        let created = client
            .comments()
            .create_comment(1603994775, "this is a fake comment".to_string())
            .await
            .unwrap();
        let deleted = client.comments().delete_comment(created.id).await.unwrap();
        assert_eq!(created.id, deleted.id)
    }
}
