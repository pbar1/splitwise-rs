use crate::client::client::Client;
use crate::model::comments::Comment;
use crate::model::comments::CommentWrapper;
use crate::model::comments::CommentsWrapper;
use crate::model::comments::CreateCommentRequest;

/// Comments.
///
/// [Splitwise API docs](https://dev.splitwise.com/#tag/comments)
#[derive(Debug)]
pub struct CommentsSvc<'c> {
    client: &'c Client,
}

impl<'c> CommentsSvc<'c> {
    /// Creates a new instance of `CommentsSvc`.
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Get expense comments.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/comments/paths/~1get_comments/get)
    pub async fn get_comments(&self, expense_id: i64) -> Result<Vec<Comment>, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("get_comments?expense_id={}", expense_id))?;
        let response: CommentsWrapper = self.client.get(url).await?;
        Ok(response.comments)
    }

    /// Create a comment.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/comments/paths/~1create_comment/post)
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

    /// Deletes a comment. Returns the deleted comment.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/comments/paths/~1delete_comment/post)
    // NOTE: API docs are currently ambiguous on whether to pass the comment ID
    // via a path or query parameter. However, one of the maintainers has noted
    // that either work.
    // See: https://github.com/pbar1/splitwise-rs/commit/0095c1d7a3a601ddc2dc76419f58649cfe13cd9a#r68338969
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

    // TODO: Remove hardcoded expense ID
    #[test(tokio::test)]
    async fn get_comments_works() {
        let _response = Client::default()
            .comments()
            .get_comments(1606307156)
            .await
            .unwrap();
    }

    // TODO: Remove hardcoded expense ID
    #[test(tokio::test)]
    async fn create_delete_comment_works() {
        let client = Client::default();

        let created = client
            .comments()
            .create_comment(1606307156, "this is a fake comment".to_string())
            .await
            .unwrap();
        let deleted = client.comments().delete_comment(created.id).await.unwrap();
        assert_eq!(created.id, deleted.id)
    }
}
